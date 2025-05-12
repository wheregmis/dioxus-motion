use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use std::path::PathBuf;

use crate::models::{Category, Expense};
use crate::services::{ExpenseService, ExpenseServiceError};
use crate::utils::{first_day_of_current_month, last_day_of_current_month};

/// Filter type for expenses
#[derive(Clone, Debug, PartialEq)]
pub enum FilterType {
    None,
    DateRange(NaiveDate, NaiveDate),
    Category(Category),
    AmountRange(f64, f64),
}

/// Error type for expense state
#[derive(Clone, Debug, PartialEq)]
pub enum ExpenseStateError {
    ServiceError(String),
    DatabaseError(String),
    ValidationError(String),
}

impl std::fmt::Display for ExpenseStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpenseStateError::ServiceError(msg) => write!(f, "Service error: {}", msg),
            ExpenseStateError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ExpenseStateError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl From<ExpenseServiceError> for ExpenseStateError {
    fn from(err: ExpenseServiceError) -> Self {
        match err {
            ExpenseServiceError::DatabaseError(msg) => ExpenseStateError::DatabaseError(msg),
            ExpenseServiceError::ValidationError(msg) => ExpenseStateError::ValidationError(msg),
        }
    }
}

/// The expense state
#[derive(Debug)]
pub struct ExpenseState {
    service: ExpenseService,
    expenses: Signal<Vec<Expense>>,
    filtered_expenses: Signal<Vec<Expense>>,
    current_filter: Signal<FilterType>,
    error: Signal<Option<ExpenseStateError>>,
    loading: Signal<bool>,
}

impl ExpenseState {
    /// Creates a new expense state with a database at the specified path
    pub fn new(db_path: PathBuf) -> Result<Self, ExpenseStateError> {
        let service = ExpenseService::new(db_path)
            .map_err(|e: ExpenseServiceError| -> ExpenseStateError { e.into() })?;

        Ok(Self {
            service,
            expenses: Signal::new(Vec::new()),
            filtered_expenses: Signal::new(Vec::new()),
            current_filter: Signal::new(FilterType::None),
            error: Signal::new(None),
            loading: Signal::new(false),
        })
    }

    /// Creates a new expense state with an in-memory database (for testing)
    pub fn new_in_memory() -> Result<Self, ExpenseStateError> {
        let service = ExpenseService::new_in_memory()
            .map_err(|e: ExpenseServiceError| -> ExpenseStateError { e.into() })?;

        Ok(Self {
            service,
            expenses: Signal::new(Vec::new()),
            filtered_expenses: Signal::new(Vec::new()),
            current_filter: Signal::new(FilterType::None),
            error: Signal::new(None),
            loading: Signal::new(false),
        })
    }

    /// Loads all expenses from the database
    pub async fn load_expenses(&mut self) {
        self.loading.set(true);
        self.error.set(None);

        match self.service.get_all_expenses() {
            Ok(expenses) => {
                self.expenses.set(expenses.clone());
                self.apply_filter();
            }
            Err(err) => {
                let error: ExpenseStateError = err.into();
                self.error.set(Some(error));
            }
        }

        self.loading.set(false);
    }

    /// Adds a new expense
    pub async fn add_expense(&mut self, expense: Expense) -> Result<(), ExpenseStateError> {
        self.loading.set(true);
        self.error.set(None);

        match self.service.add_expense(expense.clone()) {
            Ok(_) => {
                // Update the local state
                let mut expenses = self.expenses.read().clone();
                expenses.push(expense);
                expenses.sort_by(|a, b| b.date.cmp(&a.date));
                self.expenses.set(expenses);
                self.apply_filter();
                Ok(())
            }
            Err(err) => {
                let error: ExpenseStateError = err.into();
                self.error.set(Some(error.clone()));
                Err(error)
            }
        }
    }

    /// Updates an existing expense
    pub async fn update_expense(&mut self, expense: Expense) -> Result<(), ExpenseStateError> {
        self.loading.set(true);
        self.error.set(None);

        match self.service.update_expense(expense.clone()) {
            Ok(_) => {
                // Update the local state
                let mut expenses = self.expenses.read().clone();
                if let Some(index) = expenses.iter().position(|e| e.id == expense.id) {
                    expenses[index] = expense;
                }
                expenses.sort_by(|a, b| b.date.cmp(&a.date));
                self.expenses.set(expenses);
                self.apply_filter();
                Ok(())
            }
            Err(err) => {
                let error: ExpenseStateError = err.into();
                self.error.set(Some(error.clone()));
                Err(error)
            }
        }
    }

    /// Deletes an expense
    pub async fn delete_expense(&mut self, id: &str) -> Result<(), ExpenseStateError> {
        self.loading.set(true);
        self.error.set(None);

        match self.service.delete_expense(id) {
            Ok(_) => {
                // Update the local state
                let mut expenses = self.expenses.read().clone();
                expenses.retain(|e| e.id != id);
                self.expenses.set(expenses);
                self.apply_filter();
                Ok(())
            }
            Err(err) => {
                let error: ExpenseStateError = err.into();
                self.error.set(Some(error.clone()));
                Err(error)
            }
        }
    }

    /// Sets the current filter
    pub fn set_filter(&mut self, filter: FilterType) {
        self.current_filter.set(filter);
        self.apply_filter();
    }

    /// Clears the current filter
    pub fn clear_filter(&mut self) {
        self.current_filter.set(FilterType::None);
        self.filtered_expenses.set(self.expenses.read().clone());
    }

    /// Sets the filter to show expenses for the current month
    pub fn filter_current_month(&mut self) {
        let start_date = first_day_of_current_month();
        let end_date = last_day_of_current_month();
        self.set_filter(FilterType::DateRange(start_date, end_date));
    }

    /// Sets the filter to show expenses for a specific category
    pub fn filter_by_category(&mut self, category: Category) {
        self.set_filter(FilterType::Category(category));
    }

    /// Sets the filter to show expenses within a date range
    pub fn filter_by_date_range(&mut self, start_date: NaiveDate, end_date: NaiveDate) {
        self.set_filter(FilterType::DateRange(start_date, end_date));
    }

    /// Sets the filter to show expenses within an amount range
    pub fn filter_by_amount_range(&mut self, min_amount: f64, max_amount: f64) {
        self.set_filter(FilterType::AmountRange(min_amount, max_amount));
    }

    /// Applies the current filter to the expenses
    fn apply_filter(&mut self) {
        let expenses = self.expenses.read().clone();
        let filter = self.current_filter.read().clone();

        let filtered = match filter {
            FilterType::None => expenses,
            FilterType::DateRange(start_date, end_date) => expenses
                .into_iter()
                .filter(|e| e.date >= start_date && e.date <= end_date)
                .collect(),
            FilterType::Category(category) => expenses
                .into_iter()
                .filter(|e| match (&category, &e.category) {
                    (Category::Other(_), Category::Other(_)) => true,
                    _ => e.category == category,
                })
                .collect(),
            FilterType::AmountRange(min_amount, max_amount) => expenses
                .into_iter()
                .filter(|e| e.amount >= min_amount && e.amount <= max_amount)
                .collect(),
        };

        self.filtered_expenses.set(filtered);
    }

    /// Gets the total amount of all expenses
    pub fn get_total_amount(&self) -> f64 {
        self.filtered_expenses.read().iter().map(|e| e.amount).sum()
    }

    /// Gets the total amount by category
    pub fn get_total_by_category(&self) -> Vec<(Category, f64)> {
        let mut result = Vec::new();
        let expenses = self.filtered_expenses.read().clone();

        // Group expenses by category
        let mut category_map = std::collections::HashMap::new();
        for expense in expenses {
            let entry = category_map.entry(expense.category.clone()).or_insert(0.0);
            *entry += expense.amount;
        }

        // Convert to vector of tuples
        for (category, amount) in category_map {
            result.push((category, amount));
        }

        // Sort by amount (descending)
        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        result
    }

    /// Gets the expenses for a specific month
    pub fn get_expenses_for_month(&self, year: i32, month: u32) -> Vec<Expense> {
        let expenses = self.expenses.read().clone();

        expenses
            .into_iter()
            .filter(|e| e.date.year() == year && e.date.month() == month)
            .collect()
    }

    /// Gets the expenses signal
    pub fn expenses(&self) -> Signal<Vec<Expense>> {
        self.expenses
    }

    /// Gets the filtered expenses signal
    pub fn filtered_expenses(&self) -> Signal<Vec<Expense>> {
        self.filtered_expenses
    }

    /// Gets the current filter signal
    pub fn current_filter(&self) -> Signal<FilterType> {
        self.current_filter
    }

    /// Gets the error signal
    pub fn error(&self) -> Signal<Option<ExpenseStateError>> {
        self.error
    }

    /// Gets the loading signal
    pub fn loading(&self) -> Signal<bool> {
        self.loading
    }

    /// Gets an expense by ID
    pub async fn get_expense(&self, id: &str) -> Result<Option<Expense>, ExpenseStateError> {
        // Since we're using signals, we can modify them even with an immutable reference
        let mut loading = self.loading;
        let mut error = self.error;

        tracing::info!("ExpenseState.get_expense called with ID: {}", id);
        loading.set(true);

        // First try to find the expense in the local state
        let local_expenses = self.expenses.read().clone();
        if let Some(expense) = local_expenses.iter().find(|e| e.id == id) {
            tracing::info!("Found expense in local state: {}", expense.title);
            loading.set(false);
            return Ok(Some(expense.clone()));
        }

        tracing::info!("Expense not found in local state, querying service");
        let result: Result<Option<Expense>, ExpenseStateError> = match self.service.get_expense(id)
        {
            Ok(expense) => {
                if let Some(ref e) = expense {
                    tracing::info!("Service returned expense: {}", e.title);
                } else {
                    tracing::warn!("Service did not find expense with ID: {}", id);
                }
                Ok(expense)
            }
            Err(err) => {
                tracing::error!("Service error getting expense: {}", err);
                Err(err.into())
            }
        };

        loading.set(false);

        if let Err(ref err) = result {
            error.set(Some(err.clone()));
        } else {
            error.set(None);
        }

        result
    }
}

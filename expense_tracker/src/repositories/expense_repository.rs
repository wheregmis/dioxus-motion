use chrono::{Datelike, Local, NaiveDate};
use dioxus::prelude::*;
use std::path::PathBuf;
use thiserror::Error;

use crate::models::{Budget, Category, Expense};
use crate::services::{Database, DatabaseError};
use crate::utils::{first_day_of_current_month, last_day_of_current_month};

/// Filter type for expenses
#[derive(Clone, Debug, PartialEq)]
pub enum FilterType {
    /// No filter applied - show all expenses
    None,
    /// Filter by date range
    DateRange(NaiveDate, NaiveDate),
    /// Filter by category
    Category(Category),
    /// Filter by amount range
    AmountRange(f64, f64),
}

/// Error type for expense repository
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ExpenseRepositoryError {
    /// Error from the database layer
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Error in the expense validation logic
    #[error("Invalid expense data: {0}")]
    ValidationError(String),
}

impl From<DatabaseError> for ExpenseRepositoryError {
    fn from(err: DatabaseError) -> Self {
        ExpenseRepositoryError::DatabaseError(err.to_string())
    }
}

/// Result type for expense repository operations
pub type ExpenseRepositoryResult<T> = Result<T, ExpenseRepositoryError>;

/// Unified expense repository with reactive state
///
/// This repository serves as the main data access layer for the application,
/// combining database operations with reactive state management through Signals.
/// It handles CRUD operations on expenses, filtering, and validation.
#[derive(Debug)]
pub struct ExpenseRepository {
    /// Underlying database connection
    database: Database,
    /// Signal containing all expenses
    expenses: Signal<Vec<Expense>>,
    /// Signal containing filtered expenses based on the current filter
    filtered_expenses: Signal<Vec<Expense>>,
    /// Current filter being applied
    current_filter: Signal<FilterType>,
    /// Current error state, if any
    error: Signal<Option<ExpenseRepositoryError>>,
    /// Loading state indicator
    loading: Signal<bool>,
    budgets: std::collections::HashMap<Category, Budget>,
}

impl ExpenseRepository {
    /// Creates a new expense repository with a database at the specified path
    pub fn new(db_path: PathBuf) -> ExpenseRepositoryResult<Self> {
        let database = Database::new(db_path)?;

        Ok(Self {
            database,
            expenses: Signal::new(Vec::new()),
            filtered_expenses: Signal::new(Vec::new()),
            current_filter: Signal::new(FilterType::None),
            error: Signal::new(None),
            loading: Signal::new(false),
            budgets: Category::default_budgets(),
        })
    }

    /// Creates a new expense repository with an in-memory database (for testing)
    pub fn new_in_memory() -> ExpenseRepositoryResult<Self> {
        let database = Database::new_in_memory()?;

        Ok(Self {
            database,
            expenses: Signal::new(Vec::new()),
            filtered_expenses: Signal::new(Vec::new()),
            current_filter: Signal::new(FilterType::None),
            error: Signal::new(None),
            loading: Signal::new(false),
            budgets: Category::default_budgets(),
        })
    }

    /// Loads all expenses from the database
    pub async fn load_expenses(&mut self) {
        self.loading.set(true);
        self.error.set(None);

        match self.get_all_expenses() {
            Ok(expenses) => {
                self.expenses.set(expenses);
                self.apply_filter();
            }
            Err(err) => {
                self.error.set(Some(err));
            }
        }

        self.loading.set(false);
    }

    /// Adds a new expense
    pub async fn add_expense(&mut self, expense: Expense) -> ExpenseRepositoryResult<()> {
        self.loading.set(true);
        self.error.set(None);

        match self.validate_and_add_expense(expense.clone()) {
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
                self.error.set(Some(err.clone()));
                Err(err)
            }
        }
    }

    /// Updates an existing expense
    pub async fn update_expense(&mut self, expense: Expense) -> ExpenseRepositoryResult<()> {
        self.loading.set(true);
        self.error.set(None);

        match self.validate_and_update_expense(expense.clone()) {
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
                self.error.set(Some(err.clone()));
                Err(err)
            }
        }
    }

    /// Deletes an expense
    pub async fn delete_expense(&mut self, id: &str) -> ExpenseRepositoryResult<()> {
        self.loading.set(true);
        self.error.set(None);

        match self.database.delete_expense(id) {
            Ok(_) => {
                // Update the local state
                let mut expenses = self.expenses.read().clone();
                expenses.retain(|e| e.id != id);
                self.expenses.set(expenses);
                self.apply_filter();
                Ok(())
            }
            Err(err) => {
                let error = ExpenseRepositoryError::from(err);
                self.error.set(Some(error.clone()));
                Err(error)
            }
        }
    }

    /// Gets an expense by ID (first from local state, then from database if needed)
    pub async fn get_expense(&self, id: &str) -> ExpenseRepositoryResult<Option<Expense>> {
        // Since we're using signals, we can modify them even with an immutable reference
        let mut loading = self.loading;
        let mut error = self.error;

        loading.set(true);

        // First try to find the expense in the local state
        let local_expenses = self.expenses.read().clone();
        if let Some(expense) = local_expenses.iter().find(|e| e.id == id) {
            loading.set(false);
            return Ok(Some(expense.clone()));
        }

        // If not found in local state, query the database
        let result = match self.database.get_expense(id) {
            Ok(expense) => Ok(expense),
            Err(err) => Err(ExpenseRepositoryError::from(err)),
        };

        loading.set(false);

        if let Err(ref err) = result {
            error.set(Some(err.clone()));
        } else {
            error.set(None);
        }

        result
    }

    /// Gets all expenses
    fn get_all_expenses(&self) -> ExpenseRepositoryResult<Vec<Expense>> {
        match self.database.get_all_expenses() {
            Ok(expenses) => Ok(expenses),
            Err(err) => Err(ExpenseRepositoryError::from(err)),
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
    ///
    /// This method updates the filtered_expenses signal based on the current filter.
    /// It supports filtering by:
    /// - Date range (expenses between two dates)
    /// - Category (expenses in a specific category)
    /// - Amount range (expenses within a min/max amount range)
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
                    // Special handling for "Other" category
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
    pub fn error(&self) -> Signal<Option<ExpenseRepositoryError>> {
        self.error
    }

    /// Gets the loading signal
    pub fn loading(&self) -> Signal<bool> {
        self.loading
    }

    pub fn get_budget(&self, category: &Category) -> f64 {
        self.budgets.get(category).map(|b| b.amount).unwrap_or(0.0)
    }

    pub fn set_budget(&mut self, category: Category, amount: f64) {
        self.budgets.insert(category, Budget::new(amount));
    }

    pub fn get_remaining_budget_for_month(
        &self,
        category: &Category,
        year: i32,
        month: u32,
    ) -> f64 {
        let budget = self.get_budget(category);
        let spent: f64 = self
            .expenses
            .read()
            .iter()
            .filter(|e| &e.category == category && e.date.year() == year && e.date.month() == month)
            .map(|e| e.amount)
            .sum();
        budget - spent
    }

    // Private methods

    /// Validates and adds a new expense
    fn validate_and_add_expense(&self, expense: Expense) -> ExpenseRepositoryResult<()> {
        self.validate_expense(&expense)?;
        self.database.add_expense(&expense)?;
        Ok(())
    }

    /// Validates and updates an existing expense
    fn validate_and_update_expense(&self, expense: Expense) -> ExpenseRepositoryResult<()> {
        self.validate_expense(&expense)?;

        // Check if the expense exists
        if self.database.get_expense(&expense.id)?.is_none() {
            return Err(ExpenseRepositoryError::ValidationError(format!(
                "Expense with ID {} does not exist",
                expense.id
            )));
        }

        self.database.update_expense(&expense)?;
        Ok(())
    }

    /// Validates an expense
    fn validate_expense(&self, expense: &Expense) -> ExpenseRepositoryResult<()> {
        // Validate title
        if expense.title.trim().is_empty() {
            return Err(ExpenseRepositoryError::ValidationError(
                "Expense title cannot be empty".to_string(),
            ));
        }

        // Validate amount
        if expense.amount <= 0.0 {
            return Err(ExpenseRepositoryError::ValidationError(
                "Expense amount must be greater than zero".to_string(),
            ));
        }

        // Validate date (not in the future for new expenses)
        let today = Local::now().date_naive();
        if expense.date > today {
            // For existing expenses (that we're updating), we'll allow future dates
            // This allows for correcting errors in existing data
            if self.database.get_expense(&expense.id)?.is_some() {
                // Existing expense - allowing future date
                tracing::warn!("Allowing future date for existing expense: {}", expense.id);
            } else {
                return Err(ExpenseRepositoryError::ValidationError(
                    "Expense date cannot be in the future".to_string(),
                ));
            }
        }

        Ok(())
    }
}

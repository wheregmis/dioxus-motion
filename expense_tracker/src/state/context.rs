use dioxus::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::models::{Category, Expense};
use crate::state::{ExpenseState, ExpenseStateError, FilterType};

/// The expense tracker application context
#[derive(Debug)]
pub struct ExpenseContext {
    pub state: ExpenseState,
}

impl ExpenseContext {
    /// Creates a new expense context with a database at the specified path
    pub fn new(db_path: PathBuf) -> Result<Self, ExpenseStateError> {
        let state = ExpenseState::new(db_path)?;
        Ok(Self { state })
    }

    /// Creates a new expense context with an in-memory database (for testing)
    pub fn new_in_memory() -> Result<Self, ExpenseStateError> {
        let state = ExpenseState::new_in_memory()?;
        Ok(Self { state })
    }

    /// Loads all expenses from the database
    pub async fn load_expenses(&mut self) {
        self.state.load_expenses().await;
    }

    /// Adds a new expense
    pub async fn add_expense(&mut self, expense: Expense) -> Result<(), ExpenseStateError> {
        self.state.add_expense(expense).await
    }

    /// Updates an existing expense
    pub async fn update_expense(&mut self, expense: Expense) -> Result<(), ExpenseStateError> {
        self.state.update_expense(expense).await
    }

    /// Deletes an expense
    pub async fn delete_expense(&mut self, id: &str) -> Result<(), ExpenseStateError> {
        self.state.delete_expense(id).await
    }

    /// Sets the current filter
    pub fn set_filter(&mut self, filter: FilterType) {
        self.state.set_filter(filter);
    }

    /// Clears the current filter
    pub fn clear_filter(&mut self) {
        self.state.clear_filter();
    }

    /// Sets the filter to show expenses for the current month
    pub fn filter_current_month(&mut self) {
        self.state.filter_current_month();
    }

    /// Sets the filter to show expenses for a specific category
    pub fn filter_by_category(&mut self, category: Category) {
        self.state.filter_by_category(category);
    }

    /// Sets the filter to show expenses within a date range
    pub fn filter_by_date_range(
        &mut self,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
    ) {
        self.state.filter_by_date_range(start_date, end_date);
    }

    /// Sets the filter to show expenses within an amount range
    pub fn filter_by_amount_range(&mut self, min_amount: f64, max_amount: f64) {
        self.state.filter_by_amount_range(min_amount, max_amount);
    }

    /// Gets the total amount of all expenses
    pub fn get_total_amount(&self) -> f64 {
        self.state.get_total_amount()
    }

    /// Gets the total amount by category
    pub fn get_total_by_category(&self) -> Vec<(Category, f64)> {
        self.state.get_total_by_category()
    }

    /// Gets the expenses for a specific month
    pub fn get_expenses_for_month(&self, year: i32, month: u32) -> Vec<Expense> {
        self.state.get_expenses_for_month(year, month)
    }

    /// Gets the expenses signal
    pub fn expenses(&self) -> Signal<Vec<Expense>> {
        self.state.expenses()
    }

    /// Gets the filtered expenses signal
    pub fn filtered_expenses(&self) -> Signal<Vec<Expense>> {
        self.state.filtered_expenses()
    }

    /// Gets the current filter signal
    pub fn current_filter(&self) -> Signal<FilterType> {
        self.state.current_filter()
    }

    /// Gets the error signal
    pub fn error(&self) -> Signal<Option<ExpenseStateError>> {
        self.state.error()
    }

    /// Gets the loading signal
    pub fn loading(&self) -> Signal<bool> {
        self.state.loading()
    }

    /// Gets an expense by ID
    pub async fn get_expense(&self, id: &str) -> Result<Option<Expense>, ExpenseStateError> {
        self.state.get_expense(id).await
    }
}

/// Provider component for the expense context
#[component]
pub fn ExpenseContextProvider(db_path: PathBuf, children: Element) -> Element {
    // Create the context
    let context =
        ExpenseContext::new(db_path.clone()).expect("Failed to initialize expense context");
    let context = Arc::new(Mutex::new(context));

    provide_context(context.clone());

    rsx! {
        {children}
    }
}

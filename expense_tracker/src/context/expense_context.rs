use dioxus::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::models::{Category, Expense};
use crate::repositories::{ExpenseRepository, ExpenseRepositoryError, FilterType};

/// The expense tracker application context
#[derive(Debug)]
pub struct ExpenseContext {
    pub repository: ExpenseRepository,
}

impl ExpenseContext {
    /// Creates a new expense context with a database at the specified path
    pub fn new(db_path: PathBuf) -> Result<Self, ExpenseRepositoryError> {
        let repository = ExpenseRepository::new(db_path)?;
        Ok(Self { repository })
    }

    /// Creates a new expense context with an in-memory database (for testing)
    pub fn new_in_memory() -> Result<Self, ExpenseRepositoryError> {
        let repository = ExpenseRepository::new_in_memory()?;
        Ok(Self { repository })
    }

    /// Loads all expenses from the database
    pub async fn load_expenses(&mut self) {
        self.repository.load_expenses().await;
    }

    /// Adds a new expense
    pub async fn add_expense(&mut self, expense: Expense) -> Result<(), ExpenseRepositoryError> {
        self.repository.add_expense(expense).await
    }

    /// Updates an existing expense
    pub async fn update_expense(&mut self, expense: Expense) -> Result<(), ExpenseRepositoryError> {
        self.repository.update_expense(expense).await
    }

    /// Deletes an expense
    pub async fn delete_expense(&mut self, id: &str) -> Result<(), ExpenseRepositoryError> {
        self.repository.delete_expense(id).await
    }

    /// Sets the current filter
    pub fn set_filter(&mut self, filter: FilterType) {
        self.repository.set_filter(filter);
    }

    /// Clears the current filter
    pub fn clear_filter(&mut self) {
        self.repository.clear_filter();
    }

    /// Sets the filter to show expenses for the current month
    pub fn filter_current_month(&mut self) {
        self.repository.filter_current_month();
    }

    /// Sets the filter to show expenses for a specific category
    pub fn filter_by_category(&mut self, category: Category) {
        self.repository.filter_by_category(category);
    }

    /// Sets the filter to show expenses within a date range
    pub fn filter_by_date_range(
        &mut self,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
    ) {
        self.repository.filter_by_date_range(start_date, end_date);
    }

    /// Sets the filter to show expenses within an amount range
    pub fn filter_by_amount_range(&mut self, min_amount: f64, max_amount: f64) {
        self.repository
            .filter_by_amount_range(min_amount, max_amount);
    }

    /// Gets the total amount of all expenses
    pub fn get_total_amount(&self) -> f64 {
        self.repository.get_total_amount()
    }

    /// Gets the total amount by category
    pub fn get_total_by_category(&self) -> Vec<(Category, f64)> {
        self.repository.get_total_by_category()
    }

    /// Gets the expenses for a specific month
    pub fn get_expenses_for_month(&self, year: i32, month: u32) -> Vec<Expense> {
        self.repository.get_expenses_for_month(year, month)
    }

    /// Gets the expenses signal
    pub fn expenses(&self) -> Signal<Vec<Expense>> {
        self.repository.expenses()
    }

    /// Gets the filtered expenses signal
    pub fn filtered_expenses(&self) -> Signal<Vec<Expense>> {
        self.repository.filtered_expenses()
    }

    /// Gets the current filter signal
    pub fn current_filter(&self) -> Signal<FilterType> {
        self.repository.current_filter()
    }

    /// Gets the error signal
    pub fn error(&self) -> Signal<Option<ExpenseRepositoryError>> {
        self.repository.error()
    }

    /// Gets the loading signal
    pub fn loading(&self) -> Signal<bool> {
        self.repository.loading()
    }

    /// Gets an expense by ID
    pub async fn get_expense(&self, id: &str) -> Result<Option<Expense>, ExpenseRepositoryError> {
        self.repository.get_expense(id).await
    }

    pub fn get_budgets(&self) -> std::collections::HashMap<Category, f64> {
        Category::all()
            .into_iter()
            .map(|cat| {
                let amt = self.repository.get_budget(&cat);
                (cat, amt)
            })
            .collect()
    }

    pub fn get_remaining_budgets_for_month(
        &self,
        year: i32,
        month: u32,
    ) -> std::collections::HashMap<Category, f64> {
        Category::all()
            .into_iter()
            .map(|cat| {
                let amt = self
                    .repository
                    .get_remaining_budget_for_month(&cat, year, month);
                (cat, amt)
            })
            .collect()
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

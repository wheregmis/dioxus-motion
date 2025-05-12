use chrono::{Datelike, Local, NaiveDate};
use std::path::PathBuf;
use thiserror::Error;

use crate::models::{Category, Expense};
use crate::services::database::{Database, DatabaseError};

/// Error type for expense service
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ExpenseServiceError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Invalid expense data: {0}")]
    ValidationError(String),
}

impl From<DatabaseError> for ExpenseServiceError {
    fn from(err: DatabaseError) -> Self {
        ExpenseServiceError::DatabaseError(err.to_string())
    }
}

/// Result type for expense service operations
pub type ExpenseServiceResult<T> = Result<T, ExpenseServiceError>;

/// Expense service
#[derive(Debug)]
pub struct ExpenseService {
    database: Database,
}

impl ExpenseService {
    /// Creates a new expense service with a database at the specified path
    pub fn new(db_path: PathBuf) -> ExpenseServiceResult<Self> {
        let database = Database::new(db_path)?;
        Ok(Self { database })
    }

    /// Creates a new expense service with an in-memory database (for testing)
    pub fn new_in_memory() -> ExpenseServiceResult<Self> {
        let database = Database::new_in_memory()?;
        Ok(Self { database })
    }

    /// Adds a new expense
    pub fn add_expense(&self, expense: Expense) -> ExpenseServiceResult<()> {
        self.validate_expense(&expense)?;
        self.database.add_expense(&expense)?;
        Ok(())
    }

    /// Updates an existing expense
    pub fn update_expense(&self, expense: Expense) -> ExpenseServiceResult<()> {
        // DEBUG: Log all expenses before updating
        let all_expenses = self.get_all_expenses();
        match &all_expenses {
            Ok(expenses) => {
                tracing::info!("All expenses before update: {:#?}", expenses);
            }
            Err(e) => {
                tracing::error!("Failed to fetch all expenses before update: {}", e);
            }
        }

        self.validate_expense(&expense)?;

        // Check if the expense exists
        if self.database.get_expense(&expense.id)?.is_none() {
            return Err(ExpenseServiceError::ValidationError(format!(
                "Expense with ID {} does not exist",
                expense.id
            )));
        }

        self.database.update_expense(&expense)?;
        Ok(())
    }

    /// Deletes an expense
    pub fn delete_expense(&self, id: &str) -> ExpenseServiceResult<()> {
        self.database.delete_expense(id)?;
        Ok(())
    }

    /// Gets an expense by ID
    pub fn get_expense(&self, id: &str) -> ExpenseServiceResult<Option<Expense>> {
        Ok(self.database.get_expense(id)?)
    }

    /// Gets all expenses
    pub fn get_all_expenses(&self) -> ExpenseServiceResult<Vec<Expense>> {
        Ok(self.database.get_all_expenses()?)
    }

    /// Gets expenses for the current month
    pub fn get_current_month_expenses(&self) -> ExpenseServiceResult<Vec<Expense>> {
        let now = Local::now();
        let year = now.year();
        let month = now.month();

        // First day of the month
        let start_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

        // Last day of the month (calculate by getting the first day of next month and subtracting 1 day)
        let end_month = if month == 12 { 1 } else { month + 1 };
        let end_year = if month == 12 { year + 1 } else { year };
        let next_month_first_day = NaiveDate::from_ymd_opt(end_year, end_month, 1).unwrap();
        let end_date = next_month_first_day.pred_opt().unwrap();

        Ok(self
            .database
            .get_expenses_by_date_range(start_date, end_date)?)
    }

    /// Gets expenses for a specific month
    pub fn get_expenses_by_month(
        &self,
        year: i32,
        month: u32,
    ) -> ExpenseServiceResult<Vec<Expense>> {
        // First day of the month
        let start_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

        // Last day of the month
        let end_month = if month == 12 { 1 } else { month + 1 };
        let end_year = if month == 12 { year + 1 } else { year };
        let next_month_first_day = NaiveDate::from_ymd_opt(end_year, end_month, 1).unwrap();
        let end_date = next_month_first_day.pred_opt().unwrap();

        Ok(self
            .database
            .get_expenses_by_date_range(start_date, end_date)?)
    }

    /// Gets expenses by category
    pub fn get_expenses_by_category(
        &self,
        category: &Category,
    ) -> ExpenseServiceResult<Vec<Expense>> {
        let all_expenses = self.database.get_all_expenses()?;
        let filtered_expenses = all_expenses
            .into_iter()
            .filter(|expense| match (category, &expense.category) {
                (Category::Other(_), Category::Other(_)) => true,
                _ => &expense.category == category,
            })
            .collect();

        Ok(filtered_expenses)
    }

    /// Gets expenses by date range
    pub fn get_expenses_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> ExpenseServiceResult<Vec<Expense>> {
        if start_date > end_date {
            return Err(ExpenseServiceError::ValidationError(
                "Start date cannot be after end date".to_string(),
            ));
        }

        Ok(self
            .database
            .get_expenses_by_date_range(start_date, end_date)?)
    }

    /// Gets expenses by amount range
    pub fn get_expenses_by_amount_range(
        &self,
        min_amount: f64,
        max_amount: f64,
    ) -> ExpenseServiceResult<Vec<Expense>> {
        if min_amount > max_amount {
            return Err(ExpenseServiceError::ValidationError(
                "Minimum amount cannot be greater than maximum amount".to_string(),
            ));
        }

        let all_expenses = self.database.get_all_expenses()?;
        let filtered_expenses = all_expenses
            .into_iter()
            .filter(|expense| expense.amount >= min_amount && expense.amount <= max_amount)
            .collect();

        Ok(filtered_expenses)
    }

    /// Validates an expense
    fn validate_expense(&self, expense: &Expense) -> ExpenseServiceResult<()> {
        if expense.title.trim().is_empty() {
            return Err(ExpenseServiceError::ValidationError(
                "Expense title cannot be empty".to_string(),
            ));
        }

        if expense.amount <= 0.0 {
            return Err(ExpenseServiceError::ValidationError(
                "Expense amount must be greater than zero".to_string(),
            ));
        }

        // Ensure the date is not in the future
        let today = Local::now().date_naive();
        if expense.date > today {
            tracing::warn!("Expense date {} is in the future", expense.date);
            // For now, we'll allow future dates for existing expenses
            // This is a temporary workaround until we fix the data
            if self.database.get_expense(&expense.id)?.is_some() {
                tracing::info!("Allowing future date for existing expense");
            } else {
                return Err(ExpenseServiceError::ValidationError(
                    "Expense date cannot be in the future".to_string(),
                ));
            }
        }

        Ok(())
    }
}

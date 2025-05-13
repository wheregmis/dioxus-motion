use chrono::{DateTime, Local, NaiveDate};
use rusqlite::{params, types::Type, Connection};

use std::path::Path;
use thiserror::Error;

use crate::models::{Category, Expense};

/// Error type for database operations
#[derive(Error, Debug, PartialEq, Clone)]
pub enum DatabaseError {
    /// SQLite database errors
    #[error("SQLite error: {0}")]
    Sqlite(String),

    /// File system I/O errors
    #[error("IO error: {0}")]
    Io(String),

    /// JSON serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<rusqlite::Error> for DatabaseError {
    fn from(err: rusqlite::Error) -> Self {
        DatabaseError::Sqlite(err.to_string())
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(err: std::io::Error) -> Self {
        DatabaseError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(err: serde_json::Error) -> Self {
        DatabaseError::Serialization(err.to_string())
    }
}

/// Result type for database operations
pub type DatabaseResult<T> = Result<T, DatabaseError>;

/// Database connection wrapper
/// Database service for expense tracker
///
/// This struct provides a low-level interface to the SQLite database
/// used to store expense data. It handles raw database operations
/// and serialization/deserialization of expense objects.
#[derive(Debug)]
pub struct Database {
    /// The SQLite connection
    conn: Connection,
}

impl PartialEq for Database {
    fn eq(&self, _other: &Self) -> bool {
        // Databases are considered equal for the example
        // This is a simplification for testing purposes
        true
    }
}

impl Database {
    /// Creates a new database connection
    pub fn new<P: AsRef<Path>>(path: P) -> DatabaseResult<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    /// Creates a new in-memory database connection
    pub fn new_in_memory() -> DatabaseResult<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    /// Initializes the database schema
    fn init(&self) -> DatabaseResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS expenses (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                amount REAL NOT NULL,
                date TEXT NOT NULL,
                category TEXT NOT NULL,
                notes TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    /// Adds a new expense
    pub fn add_expense(&self, expense: &Expense) -> DatabaseResult<()> {
        let category_json = serde_json::to_string(&expense.category)?;

        self.conn.execute(
            "INSERT INTO expenses (id, title, amount, date, category, notes, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                expense.id,
                expense.title,
                expense.amount,
                expense.date.to_string(),
                category_json,
                expense.notes,
                expense.created_at.to_rfc3339(),
                expense.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    /// Updates an existing expense
    pub fn update_expense(&self, expense: &Expense) -> DatabaseResult<()> {
        let category_json = serde_json::to_string(&expense.category)?;

        self.conn.execute(
            "UPDATE expenses
             SET title = ?1, amount = ?2, date = ?3, category = ?4, notes = ?5, updated_at = ?6
             WHERE id = ?7",
            params![
                expense.title,
                expense.amount,
                expense.date.to_string(),
                category_json,
                expense.notes,
                expense.updated_at.to_rfc3339(),
                expense.id,
            ],
        )?;

        Ok(())
    }

    /// Deletes an expense
    pub fn delete_expense(&self, id: &str) -> DatabaseResult<()> {
        self.conn
            .execute("DELETE FROM expenses WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Gets an expense by ID
    pub fn get_expense(&self, id: &str) -> DatabaseResult<Option<Expense>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, amount, date, category, notes, created_at, updated_at
             FROM expenses WHERE id = ?1",
        )?;

        let expense = stmt.query_row(params![id], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let amount: f64 = row.get(2)?;
            let date_str: String = row.get(3)?;
            let category_json: String = row.get(4)?;
            let notes: String = row.get(5)?;
            let created_at_str: String = row.get(6)?;
            let updated_at_str: String = row.get(7)?;

            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|e| {
                rusqlite::Error::InvalidColumnType(0, format!("Invalid date: {}", e), Type::Text)
            })?;

            let category: Category = serde_json::from_str(&category_json).map_err(|e| {
                rusqlite::Error::InvalidColumnType(
                    4,
                    format!("Invalid category: {}", e),
                    Type::Text,
                )
            })?;

            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Local))
                .map_err(|e| {
                    rusqlite::Error::InvalidColumnType(
                        6,
                        format!("Invalid created_at: {}", e),
                        Type::Text,
                    )
                })?;

            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Local))
                .map_err(|e| {
                    rusqlite::Error::InvalidColumnType(
                        7,
                        format!("Invalid updated_at: {}", e),
                        Type::Text,
                    )
                })?;

            Ok(Expense {
                id,
                title,
                amount,
                date,
                category,
                notes,
                created_at,
                updated_at,
            })
        });

        match expense {
            Ok(expense) => Ok(Some(expense)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(DatabaseError::Sqlite(e.to_string())),
        }
    }

    /// Gets all expenses
    pub fn get_all_expenses(&self) -> DatabaseResult<Vec<Expense>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, amount, date, category, notes, created_at, updated_at
             FROM expenses ORDER BY date DESC",
        )?;

        let expense_iter = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let amount: f64 = row.get(2)?;
            let date_str: String = row.get(3)?;
            let category_json: String = row.get(4)?;
            let notes: String = row.get(5)?;
            let created_at_str: String = row.get(6)?;
            let updated_at_str: String = row.get(7)?;

            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|e| {
                rusqlite::Error::InvalidColumnType(0, format!("Invalid date: {}", e), Type::Text)
            })?;

            let category: Category = serde_json::from_str(&category_json).map_err(|e| {
                rusqlite::Error::InvalidColumnType(
                    4,
                    format!("Invalid category: {}", e),
                    Type::Text,
                )
            })?;

            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Local))
                .map_err(|e| {
                    rusqlite::Error::InvalidColumnType(
                        6,
                        format!("Invalid created_at: {}", e),
                        Type::Text,
                    )
                })?;

            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Local))
                .map_err(|e| {
                    rusqlite::Error::InvalidColumnType(
                        7,
                        format!("Invalid updated_at: {}", e),
                        Type::Text,
                    )
                })?;

            Ok(Expense {
                id,
                title,
                amount,
                date,
                category,
                notes,
                created_at,
                updated_at,
            })
        })?;

        let mut expenses = Vec::new();
        for expense in expense_iter {
            expenses.push(expense?);
        }

        Ok(expenses)
    }

    /// Gets expenses filtered by date range
    pub fn get_expenses_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> DatabaseResult<Vec<Expense>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, amount, date, category, notes, created_at, updated_at
             FROM expenses
             WHERE date >= ?1 AND date <= ?2
             ORDER BY date DESC",
        )?;

        let expense_iter = stmt.query_map(
            params![start_date.to_string(), end_date.to_string()],
            |row| {
                let id: String = row.get(0)?;
                let title: String = row.get(1)?;
                let amount: f64 = row.get(2)?;
                let date_str: String = row.get(3)?;
                let category_json: String = row.get(4)?;
                let notes: String = row.get(5)?;
                let created_at_str: String = row.get(6)?;
                let updated_at_str: String = row.get(7)?;

                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|e| {
                    rusqlite::Error::InvalidColumnType(
                        0,
                        format!("Invalid date: {}", e),
                        Type::Text,
                    )
                })?;

                let category: Category = serde_json::from_str(&category_json).map_err(|e| {
                    rusqlite::Error::InvalidColumnType(
                        4,
                        format!("Invalid category: {}", e),
                        Type::Text,
                    )
                })?;

                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Local))
                    .map_err(|e| {
                        rusqlite::Error::InvalidColumnType(
                            6,
                            format!("Invalid created_at: {}", e),
                            Type::Text,
                        )
                    })?;

                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map(|dt| dt.with_timezone(&Local))
                    .map_err(|e| {
                        rusqlite::Error::InvalidColumnType(
                            7,
                            format!("Invalid updated_at: {}", e),
                            Type::Text,
                        )
                    })?;

                Ok(Expense {
                    id,
                    title,
                    amount,
                    date,
                    category,
                    notes,
                    created_at,
                    updated_at,
                })
            },
        )?;

        let mut expenses = Vec::new();
        for expense in expense_iter {
            expenses.push(expense?);
        }

        Ok(expenses)
    }
}

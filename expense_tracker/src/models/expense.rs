use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an expense category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    Food,
    Transportation,
    Housing,
    Utilities,
    Entertainment,
    Healthcare,
    Shopping,
    Education,
    Travel,
    Other(String),
}

impl Category {
    /// Returns a list of all predefined categories
    pub fn all() -> Vec<Self> {
        vec![
            Self::Food,
            Self::Transportation,
            Self::Housing,
            Self::Utilities,
            Self::Entertainment,
            Self::Healthcare,
            Self::Shopping,
            Self::Education,
            Self::Travel,
            Self::Other("".to_string()),
        ]
    }

    /// Returns the display name of the category
    pub fn display_name(&self) -> String {
        match self {
            Self::Food => "Food".to_string(),
            Self::Transportation => "Transportation".to_string(),
            Self::Housing => "Housing".to_string(),
            Self::Utilities => "Utilities".to_string(),
            Self::Entertainment => "Entertainment".to_string(),
            Self::Healthcare => "Healthcare".to_string(),
            Self::Shopping => "Shopping".to_string(),
            Self::Education => "Education".to_string(),
            Self::Travel => "Travel".to_string(),
            Self::Other(name) => {
                if name.is_empty() {
                    "Other".to_string()
                } else {
                    format!("Other ({})", name)
                }
            }
        }
    }

    /// Returns the category from a string representation
    pub fn from_string(s: &str) -> Self {
        match s {
            "Food" => Self::Food,
            "Transportation" => Self::Transportation,
            "Housing" => Self::Housing,
            "Utilities" => Self::Utilities,
            "Entertainment" => Self::Entertainment,
            "Healthcare" => Self::Healthcare,
            "Shopping" => Self::Shopping,
            "Education" => Self::Education,
            "Travel" => Self::Travel,
            s if s.starts_with("Other") => {
                if s == "Other" {
                    Self::Other("".to_string())
                } else {
                    // Extract the custom category name from "Other (name)"
                    let name = s
                        .strip_prefix("Other (")
                        .and_then(|s| s.strip_suffix(")"))
                        .unwrap_or("")
                        .to_string();
                    Self::Other(name)
                }
            }
            _ => Self::Other("".to_string()),
        }
    }
}

/// Represents an expense entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Expense {
    pub id: String,
    pub title: String,
    pub amount: f64,
    pub date: NaiveDate,
    pub category: Category,
    pub notes: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Expense {
    /// Creates a new expense with the given details
    pub fn new(
        title: String,
        amount: f64,
        date: NaiveDate,
        category: Category,
        notes: String,
    ) -> Self {
        let now = Local::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            amount,
            date,
            category,
            notes,
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates a new expense with default values
    pub fn default() -> Self {
        let now = Local::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title: String::new(),
            amount: 0.0,
            date: now.date_naive(),
            category: Category::Other("".to_string()),
            notes: String::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

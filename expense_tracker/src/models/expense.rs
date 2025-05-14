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

    /// Returns a canonical value for the category (for use in select value attributes)
    pub fn as_value(&self) -> String {
        match self {
            Self::Food => "food".to_string(),
            Self::Transportation => "transportation".to_string(),
            Self::Housing => "housing".to_string(),
            Self::Utilities => "utilities".to_string(),
            Self::Entertainment => "entertainment".to_string(),
            Self::Healthcare => "healthcare".to_string(),
            Self::Shopping => "shopping".to_string(),
            Self::Education => "education".to_string(),
            Self::Travel => "travel".to_string(),
            Self::Other(name) => format!("other:{}", name),
        }
    }

    /// Returns the category from a canonical value
    pub fn from_value(s: &str) -> Self {
        match s {
            "food" => Self::Food,
            "transportation" => Self::Transportation,
            "housing" => Self::Housing,
            "utilities" => Self::Utilities,
            "entertainment" => Self::Entertainment,
            "healthcare" => Self::Healthcare,
            "shopping" => Self::Shopping,
            "education" => Self::Education,
            "travel" => Self::Travel,
            s if s.starts_with("other:") => {
                let name = s.strip_prefix("other:").unwrap_or("").to_string();
                Self::Other(name)
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

// Core components
mod expense_form;
mod expense_item;
mod expense_list;
mod filter_bar;
mod summary_view;

// Feature modules
pub mod charts; // Charts module (placeholder for now)
pub mod primitives; // UI primitives

// Public exports
pub use expense_form::ExpenseForm;
pub use expense_item::ExpenseItem;
pub use expense_list::ExpenseList;
pub use filter_bar::FilterBar;
pub use summary_view::SummaryView;

// Export charts
pub use charts::ChartPlaceholder;

// We don't need to re-export primitives here since they're
// imported directly in the files that need them

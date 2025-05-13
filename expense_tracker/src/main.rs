use dioxus::prelude::*;
use tailwind::include_tailwind_stylesheet;
use tracing::Level;

mod components;
mod context;
mod models;
mod pages;
mod repositories;
mod services;
mod tailwind;
mod utils;

use context::ExpenseContextProvider;
use pages::{DashboardPage, ExpenseFormPage, NotFoundPage};

fn main() {
    // Initialize logger
    dioxus_logger::init(Level::INFO).expect("Failed to initialize logger");

    // Install panic hook for better error messages
    console_error_panic_hook::set_once();

    // Launch the application with desktop features
    launch(app);
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Dashboard {},
    #[route("/expense")]
    ExpenseForm { id: Option<String> },
    #[route("/expense/:id")]
    ExpenseFormWithId { id: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
fn Dashboard() -> Element {
    rsx! {
        DashboardPage {}
    }
}

#[component]
fn ExpenseForm(id: Option<String>) -> Element {
    rsx! {
        ExpenseFormPage { id }
    }
}

#[component]
fn ExpenseFormWithId(id: String) -> Element {
    rsx! {
        ExpenseFormPage { id: Some(id) }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        NotFoundPage { route }
    }
}

fn app() -> Element {
    // Create a database path in the user's home directory
    let db_path = dirs::home_dir()
        .unwrap_or_default()
        .join(".expense_tracker")
        .join("expenses.db");

    // Create the directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }

    rsx! {
        include_tailwind_stylesheet {}
        // Provide the expense context to the entire application
        ExpenseContextProvider { db_path, Router::<Route> {} }
    }
}

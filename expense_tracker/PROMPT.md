You are a senior Rust developer and software architect. I need you to design and generate the implementation of a desktop expense tracker application using Dioxus. Follow these strict technical rules and use the context7 tool where specified:
🧩 App Type
Desktop application using Dioxus
Cross-platform support (Windows, macOS, Linux).
📋 Core Features
Add, edit, delete, and view expenses.
Categorize expenses.
Filter by date, amount, and category.
Generate monthly summary view.
Persist expenses locally (file-based or embedded DB).
🔍 Library Selection Rules
Only use Rust crates that:
Are actively maintained (check context7 for commit activity in last 6 months).
Have ≥500 downloads/week and clear documentation (verified via context7).
Are not deprecated or unmaintained.
For local storage, prefer using sled or sqlite via rusqlite. Justify your choice using context7.
Use dioxus-motion for animations
For state management, evaluate using dioxus-signals or dioxus-hooks. Use context7 to compare and recommend the better fit.
UI components must use idiomatic Dioxus patterns. Avoid using web-based JS components.
🗂 Project Structure
Follow idiomatic Rust project structure.
Separate concerns clearly (UI, state, storage, services).
Include a Cargo.toml with all dependencies and feature flags.
✅ Additional Requirements
Implement error handling with Result<> and thiserror.
Use chrono or similar for date/time operations (check latest crate via context7).
Write unit tests for logic (especially filtering and storage).
Include instructions for running the app on all platforms.
📖 Usage of context7
Use context7 to:
Retrieve up-to-date popularity and maintenance status of crates.
Compare options for local storage and state management in Rust desktop apps.
Justify selection of each major dependency with actual findings.
🧾 Output Format
Project structure and folder layout.
Cargo.toml with dependencies.
Code for:
Main app setup.
UI components.
State handling.
Local storage implementation.
Utility functions and models.
Unit test samples.
Setup and build instructions for desktop environments.
Provide your answer in Markdown with code blocks. Summarize each section before showing the code.
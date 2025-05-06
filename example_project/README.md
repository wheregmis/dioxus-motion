# Dioxus Motion Dashboard Example

This project demonstrates how to use the Dioxus Motion library to create fluid animations in a dashboard interface. It also shows how to integrate Tailwind CSS for styling.

## Features

- Animated UI components with spring physics
- Drag and drop functionality
- Responsive layout
- Dark/light mode support
- Tailwind CSS integration

## Project Structure

The project is organized into modular components:

```
example_project/
├── src/
│   ├── components/       # Reusable UI components
│   │   ├── dashboard/    # Dashboard-specific components
│   │   ├── layout/       # Layout components (sidebar, header)
│   │   └── pages/        # Page components (analytics, reports, settings)
│   ├── tailwind.rs       # Tailwind CSS integration
│   └── main.rs           # Main application entry point
├── assets/               # Static assets and compiled CSS
├── input.css             # Tailwind CSS input file
├── tailwind.config.js    # Tailwind configuration
└── tailwind-watch.sh     # Script to watch and compile Tailwind CSS
```

## Getting Started

1. Make sure you have Rust and Cargo installed
2. Install the Dioxus CLI: `cargo install dioxus-cli`
3. Install Node.js and npm: [https://nodejs.org/](https://nodejs.org/)
4. Install Tailwind CSS CLI: `npm install -g tailwindcss`

## Running the Project

1. Start the Tailwind CSS compiler in watch mode:
   ```
   ./tailwind-watch.sh
   ```

2. In a separate terminal, run the Dioxus application:
   ```
   dx serve --platform desktop
   ```

   For web:
   ```
   dx serve
   ```

## Customizing Styles

You can customize the Tailwind CSS configuration in `tailwind.config.js`. The base styles are defined in `input.css`.

## Adding New Components

1. Create a new component file in the appropriate directory under `src/components/`
2. Export the component in the corresponding `mod.rs` file
3. Import and use the component in your application

## License

MIT

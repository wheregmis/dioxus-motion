#!/bin/bash
# Script to watch for changes and compile Tailwind CSS

# Make sure the assets directory exists
mkdir -p assets

# Run Tailwind CSS compiler in watch mode
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

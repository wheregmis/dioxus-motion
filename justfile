# Development commands for dioxus-motion

# Show all available commands
default:
    @just --list

# === Testing ===

# Run all tests with web features
test:
    cargo test --features web

# Run all tests with desktop features  
test-desktop:
    cargo test --features desktop

# Run all tests with transitions features
test-transitions:
    cargo test --features transitions

# Run tests for a specific feature combination
test-features features:
    cargo test --features {{features}}

# Run tests with output (no capture)
test-verbose:
    cargo test --features web -- --nocapture

# Run a specific test by name
test-name name:
    cargo test --features web {{name}} -- --nocapture

# Run loop mode tests specifically
test-loops:
    cargo test --features web test_loop_mode -- --nocapture

# === Code Quality ===

# Format all code
fmt:
    cargo fmt --all

# Check formatting without making changes
fmt-check:
    cargo fmt --all -- --check

# Run clippy with web features
clippy:
    cargo clippy --features web -- -D warnings

# Run clippy with desktop features
clippy-desktop:
    cargo clippy --features desktop -- -D warnings

# Run clippy with all features
clippy-all:
    cargo clippy --all-features -- -D warnings

# Run clippy and automatically fix issues
clippy-fix:
    cargo clippy --features web --fix --allow-dirty -- -D warnings

# === Building ===

# Check compilation with web features
check:
    cargo check --features web

# Check compilation with desktop features
check-desktop:
    cargo check --features desktop

# Check compilation with all features
check-all:
    cargo check --all-features

# Check the entire workspace
check-workspace:
    cargo check --workspace --all-features

# Build with web features
build:
    cargo build --features web

# Build with desktop features
build-desktop:
    cargo build --features desktop

# Build release version
build-release:
    cargo build --release --features web

# === Documentation ===

# Generate and open documentation
docs:
    cargo doc --no-deps --features web --open

# Generate documentation for all features
docs-all:
    cargo doc --no-deps --all-features --open

# Check documentation without opening
docs-check:
    cargo doc --no-deps --all-features

# === CI Simulation ===

# Run the same checks as CI
ci: fmt-check clippy test docs-check
    @echo "✅ All CI checks passed!"

# Run CI checks for desktop
ci-desktop: fmt-check clippy-desktop test-desktop docs-check
    @echo "✅ All desktop CI checks passed!"

# === Maintenance ===

# Remove unused dependencies
remove_deps:
    cargo machete --with-metadata

# Check build timing
check_timing:
    cargo build --timings

# Remove unused features
remove_unused_features:
    cargo features prune

# Clean build artifacts
clean:
    cargo clean

# Update dependencies
update:
    cargo update

# === Benchmarks ===

# Run benchmark tests
bench:
    cargo test --features web --release animations::benchmarks::tests -- --nocapture

# Run performance regression tests
test-perf:
    cargo test --features web test_performance_regression -- --nocapture

# Run config pool performance test
test-pool-perf:
    cargo test --features web test_config_pool_performance -- --nocapture

# === CI Debugging ===

# Test workspace check (like CI does)
test-workspace:
    cargo check --workspace --all-features
    cargo clippy --workspace --all-features -- -D warnings

# Install system dependencies (Ubuntu/Debian)
install-deps:
    #!/usr/bin/env bash
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "Installing system dependencies for Linux..."
        sudo apt-get update
        sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macOS detected - no additional system dependencies needed"
    else
        echo "Unsupported OS: $OSTYPE"
        exit 1
    fi

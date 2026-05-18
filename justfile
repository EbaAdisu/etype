# etype — developer convenience commands
# Install just: https://github.com/casey/just

# List available commands
default:
    @just --list

# Format code
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt --check

# Run Clippy (warnings treated as errors, matches CI)
lint:
    cargo clippy -- -D warnings

# Run all tests (unit + integration)
test:
    cargo test

# Run only unit tests
test-unit:
    cargo test --lib

# Run only integration tests
test-integration:
    cargo test --test db

# Full CI check: format, lint, test
ci: fmt-check lint test

# Build debug binary
build:
    cargo build

# Build release binary
release:
    cargo build --release

# Run in development mode
run:
    cargo run

# Run with debug logging
run-debug:
    RUST_LOG=debug cargo run

# Remove build artifacts
clean:
    cargo clean

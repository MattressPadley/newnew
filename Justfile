# List all available commands
default:
    @just --list

# Install project dependencies
install:
    cargo install --path .

# Build release version
build-release:
    cargo build --release

# Clean build artifacts
clean:
    cargo clean

# Run tests
test:
    cargo test

# Build and run development version
dev:
    cargo build
    cargo run

# Install dependencies and build release version
setup: install build-release

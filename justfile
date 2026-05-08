# Run the app
run:
    cargo run

# Format code
fmt:
    cargo +nightly fmt

# Check formatting
fmt-check:
    cargo +nightly fmt -- --check

# Run clippy
lint:
    cargo clippy

# Format + lint
check: fmt lint

# Build
build:
    cargo build

# Build release
release:
    cargo build --release

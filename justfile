# Run clippy with all warnings
clippy:
    cargo clippy -- -W clippy::pedantic -W clippy::nursery -D warnings

# Run clippy with auto-fix
clippy-fix:
    cargo clippy --fix --allow-dirty -- -W clippy::pedantic -W clippy::nursery -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt -- --check

# Run tests
test:
    cargo test

# Build release
build:
    cargo build --release

# Run the app
run:
    cargo run

# Run with local config
run-local:
    cargo run -- --config ./radioboat.toml

# Run in small mode
run-small:
    cargo run -- --ui-size small

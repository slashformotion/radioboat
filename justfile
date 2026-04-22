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

# Check warnings and cross-compile for all target platforms
check-all:
    cargo check --target x86_64-unknown-linux-gnu 2>&1
    cargo check --target aarch64-unknown-linux-gnu 2>&1
    cargo zigbuild --target x86_64-apple-darwin 2>&1 || true
    cargo zigbuild --target aarch64-apple-darwin 2>&1 || true

# Clippy with warnings for all target platforms
clippy-all:
    cargo clippy --target x86_64-unknown-linux-gnu -- -W clippy::pedantic -W clippy::nursery -D warnings 2>&1
    cargo clippy --target aarch64-unknown-linux-gnu -- -W clippy::pedantic -W clippy::nursery -D warnings 2>&1
    cargo zigbuild --target x86_64-apple-darwin 2>&1 || true
    cargo zigbuild --target aarch64-apple-darwin 2>&1 || true

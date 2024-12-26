test *FLAGS: 
    cargo test {{FLAGS}}
    cargo test --features unstable-builder {{FLAGS}}

benchmark *FLAGS: 
    cargo criterion {{FLAGS}}

ci-dev: lint format test
    cargo build --release

lint *args:
    cargo clippy {{args}} -- -D warnings

format:
    cargo +nightly fmt --all -- --check

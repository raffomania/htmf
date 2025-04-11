test *FLAGS: 
    cargo test --features pretty-print {{FLAGS}}
    cargo test --features unstable-builder,pretty-print {{FLAGS}}

benchmark *FLAGS: 
    cargo criterion --features pretty-print {{FLAGS}}

ci-dev: lint format test
    cargo build --release

lint *args:
    cargo clippy {{args}} -- -D warnings
    cargo semver-checks

mutants *args:
    cargo mutants --all-features --package htmf {{args}}

format:
    cargo +nightly fmt --all -- --check

release *args:
    cargo release {{args}}

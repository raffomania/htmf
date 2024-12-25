test *FLAGS: 
    cargo test {{FLAGS}}
    cargo test --features unstable-builder {{FLAGS}}

benchmark *FLAGS: 
    cargo criterion {{FLAGS}}

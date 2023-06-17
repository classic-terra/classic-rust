A library to interact with classic chain in Rust!

This repository contains:
1. classic-proto: a proto library for interacting with classic chain
2. classic-test-tube: a library for testing your smart contract against simulated classic chain logic

Import:
1. `classic-rust = {version = "0.1.0", git = "https://github.com/classic-terra/classic-rust.git", package = "classic-rust", rev = "5e10211700b813285abf2ba07735b9d52cb1fd7c"}`
2. `classic-test-tube = {version = "0.1.0", git = "https://github.com/classic-terra/classic-rust.git", package = "classic-test-tube", rev = "5e10211700b813285abf2ba07735b9d52cb1fd7c"}`

# Build proto
run `cargo build && cargo run classic-rust-generator` to generate proto

to update dependencies, go to: [main](src/main.rs)
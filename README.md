A library to interact with classic chain in Rust!

This repository contains:
1. classic-proto: a proto library for interacting with classic chain
2. classic-test-tube: a library for testing your smart contract against simulated classic chain logic. This library will allow smart contract developers to test contract against real chain logic without the need for spinning up a node. This will save tremendous amount of time by rapid testing.

Import:
1. `classic-rust = {version = "0.1.0", git = "https://github.com/classic-terra/classic-rust.git", package = "classic-rust", rev = "11bc73ebc96d1f5521df6af60e57c1aeed8649f4"}`
2. `classic-test-tube = {version = "0.1.0", git = "https://github.com/classic-terra/classic-rust.git", package = "classic-test-tube", rev = "11bc73ebc96d1f5521df6af60e57c1aeed8649f4"}`

# Build proto
clone submodules: `git submodule update --init --recursive`

run `cargo build && cargo run classic-rust-generator` to generate proto

to update dependencies, go to: [main](src/main.rs)

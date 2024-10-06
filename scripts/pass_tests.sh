#!/bin/bash -xe

cargo test --workspace --all-features
cargo fmt --all --check --verbose
cargo clippy --workspace --all-features

#!/usr/bin/env bash

# Versions used
SVD2RUST_VERSION=0.18.0
FORM_VERSION=0.8.0

# Install required programs
cargo install --version $SVD2RUST_VERSION svd2rust
cargo install --version $FORM_VERSION form

svd2rust -i svd/CC3200.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt

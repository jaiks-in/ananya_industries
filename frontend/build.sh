#!/bin/sh
set -e

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Load Cargo into PATH
. "$HOME/.cargo/env"

# Set Rust stable as default
rustup default stable

# Install the WebAssembly target
rustup target add wasm32-unknown-unknown

# Install trunk
cargo install --locked trunk

# Build Leptos project
trunk build

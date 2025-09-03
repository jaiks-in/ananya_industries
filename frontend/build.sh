#!/bin/sh
set -e

echo "🚀 Installing Rust toolchain..."
# Install Rust non-interactively
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Load cargo into PATH (works with /bin/sh)
. "$HOME/.cargo/env"

echo "✅ Rust installed. Version:"
rustc --version

echo "📦 Setting Rust default toolchain..."
rustup default stable

echo "🌐 Adding WebAssembly target..."
rustup target add wasm32-unknown-unknown

echo "🔧 Installing Trunk..."
cargo install --locked trunk

echo "🏗️ Building project with Trunk..."
trunk build --release

echo "🎉 Build finished. Files are in target/site"

#!/bin/bash
# Quick setup script for greetme development

set -e

echo "🚀 greetme Development Setup"
echo "=============================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed successfully!"
else
    echo "✅ Rust is already installed ($(rustc --version))"
fi

echo ""

# Check if git is installed
if ! command -v git &> /dev/null; then
    echo "❌ Git is not installed!"
    echo "Please install git and try again."
    exit 1
else
    echo "✅ Git is available"
fi

echo ""
echo "📋 Installing development tools..."

# Install cargo-deb for Debian packaging
if ! command -v cargo-deb &> /dev/null; then
    echo "  Installing cargo-deb..."
    cargo install cargo-deb
    echo "  ✅ cargo-deb installed"
else
    echo "  ✅ cargo-deb already installed"
fi

# Install rustfmt and clippy
echo "  Installing rustfmt and clippy..."
rustup component add rustfmt clippy 2>/dev/null || echo "  ✅ Already installed"

echo ""
echo "🔨 Building greetme..."
cargo build --release

echo ""
echo "🧪 Running tests..."
cargo test

echo ""
echo "✨ Running linter..."
cargo clippy -- -D warnings

echo ""
echo "📦 Creating test configuration..."
./target/release/greetme -c

echo ""
echo "🎉 Setup complete!"
echo ""
echo "Quick commands:"
echo "  cargo build          - Build debug version"
echo "  cargo build --release - Build optimized version"
echo "  cargo test           - Run all tests"
echo "  cargo clippy         - Run linter"
echo "  cargo fmt            - Format code"
echo "  make install         - Install system-wide"
echo "  make package-deb     - Create .deb package"
echo "  make package-rpm     - Create .rpm package"
echo ""
echo "Try it out:"
echo "  ./target/release/greetme -t 'Hello, Developer!'"
echo ""
echo "Happy hacking! 🎨"

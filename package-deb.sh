#!/bin/bash
# Package greetme as a Debian .deb package

set -e

echo "Building greetme Debian package..."

# Check if cargo-deb is installed
if ! command -v cargo-deb &> /dev/null; then
    echo "cargo-deb not found. Installing..."
    cargo install cargo-deb
fi

# Compress man page
echo "Compressing man page..."
mkdir -p target/man
gzip -c man/greetme.1 > target/man/greetme.1.gz

# Build the package
echo "Building release binary..."
cargo build --release

echo "Creating .deb package..."
cargo deb

echo "✓ Debian package created successfully!"
echo "Package location: target/debian/greetme_*.deb"
ls -lh target/debian/greetme_*.deb

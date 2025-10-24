#!/bin/bash
# Quick setup script for greetme development

set -e

echo "🚀 greetme Development Setup"
echo "=============================="
echo ""

# Detect OS
detect_os() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$NAME
        if [[ "$OS" == "openSUSE Tumbleweed" ]] || [[ "$OS" == "openSUSE Leap" ]]; then
            OS=$NAME
        elif [[ "$ID" == "opensuse-tumbleweed" ]]; then
            OS="openSUSE Tumbleweed"
        elif [[ "$ID" == "opensuse-leap" ]]; then
            OS="openSUSE Leap"
        fi
    elif [ -f /etc/debian_version ]; then
        OS="Debian"
    elif [ -f /etc/redhat-release ]; then
        OS="Red Hat"
    elif [ -f /etc/arch-release ]; then
        OS="Arch Linux"
    else
        OS="Unknown"
    fi
    echo "💻 Detected OS: $OS"
}

# Install build essentials based on OS
install_build_essentials() {
    echo "📦 Checking build essentials..."
    case $OS in
        "Ubuntu"|"Debian")
            if ! dpkg -l build-essential &> /dev/null; then
                echo "Installing build-essential..."
                sudo apt-get update
                sudo apt-get install -y build-essential
            fi
            ;;
        "Fedora"|"Red Hat")
            if ! rpm -q gcc make &> /dev/null; then
                echo "Installing development tools..."
                sudo dnf groupinstall -y "Development Tools"
            fi
            ;;
        "openSUSE Tumbleweed"|"openSUSE Leap")
            if ! rpm -q patterns-devel-base-devel_basis &> /dev/null; then
                echo "Installing development tools..."
                sudo zypper install -y patterns-devel-base-devel_basis
            fi
            ;;
        "Arch Linux")
            if ! pacman -Q base-devel &> /dev/null; then
                echo "Installing base-devel..."
                sudo pacman -S --noconfirm base-devel
            fi
            # Ensure makepkg tools are available
            if ! pacman -Q archlinux-keyring &> /dev/null; then
                sudo pacman -S --noconfirm archlinux-keyring
            fi
            ;;
        *)
            echo "⚠️ Unknown distribution. Please install build tools manually."
            ;;
    esac
    echo "✅ Build tools verified"
}

# Check and install Rust/Cargo
install_rust() {
    if ! command -v cargo &> /dev/null; then
        echo "❌ Rust is not installed!"
        echo "📦 Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        echo "✅ Rust installed successfully!"
    else
        echo "✅ Rust is already installed ($(rustc --version))"
    fi
}

# Run detection and installation
detect_os
install_build_essentials
install_rust

echo ""

# Check for git
if ! command -v git &> /dev/null; then
    echo "❌ Git is not installed!"
    case $OS in
        "Ubuntu"|"Debian")
            sudo apt-get install -y git
            ;;
        "Fedora"|"Red Hat")
            sudo dnf install -y git
            ;;
        "openSUSE Tumbleweed"|"openSUSE Leap")
            sudo zypper install -y git
            ;;
        "Arch Linux")
            sudo pacman -S --noconfirm git
            ;;
        *)
            echo "Please install git manually and try again."
            exit 1
            ;;
    esac
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

# Attempt to build
if ! cargo build --release; then
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi

echo "✅ Setup completed successfully!"
echo ""
echo "🎉 You can now run greetme with: ./target/release/greetme"

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

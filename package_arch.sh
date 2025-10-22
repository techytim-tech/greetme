#!/bin/bash
# Package greetme for Arch Linux

set -e

echo "Building greetme Arch Linux package..."

# Check if makepkg is available
if ! command -v makepkg &> /dev/null; then
    echo "Error: makepkg not found. Please install base-devel:"
    echo "  sudo pacman -S base-devel"
    exit 1
fi

# Check if we're in the project root
if [ ! -f "PKGBUILD" ]; then
    echo "Error: PKGBUILD not found. Run this script from the project root."
    exit 1
fi

echo "Building package..."
makepkg -sf

echo ""
echo "✓ Arch Linux package created successfully!"
echo "Package location:"
ls -lh greetme-*.pkg.tar.zst

echo ""
echo "To install:"
echo "  sudo pacman -U greetme-*.pkg.tar.zst"
echo ""
echo "To install to AUR:"
echo "  1. Create an AUR account at https://aur.archlinux.org"
echo "  2. Clone your AUR repository: git clone ssh://aur@aur.archlinux.org/greetme.git"
echo "  3. Copy PKGBUILD and .SRCINFO to the AUR repo"
echo "  4. Generate .SRCINFO: makepkg --printsrcinfo > .SRCINFO"
echo "  5. Commit and push: git add PKGBUILD .SRCINFO && git commit -m 'Initial commit' && git push"

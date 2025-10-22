# Installation Guide

This guide provides detailed installation instructions for greetme on various Linux distributions.

## Table of Contents

- [Debian/Ubuntu](#debianubuntu)
- [Fedora](#fedora)
- [CentOS/RHEL](#centosrhel)
- [openSUSE](#opensuse)
- [Arch Linux](#arch-linux)
- [From Source](#from-source)
- [Verification](#verification)
- [Uninstallation](#uninstallation)

## Debian/Ubuntu

### Using .deb Package

1. Download the latest .deb package:

```bash
wget https://github.com/user/greetme/releases/latest/download/greetme_1.0.0_amd64.deb
```

2. Install the package:

```bash
sudo dpkg -i greetme_1.0.0_amd64.deb
```

3. If there are dependency issues, fix them:

```bash
sudo apt-get install -f
```

### Manual Installation

```bash
# Download and extract
wget https://github.com/user/greetme/releases/latest/download/greetme-linux-x86_64
chmod +x greetme-linux-x86_64
sudo mv greetme-linux-x86_64 /usr/local/bin/greetme
```

## Fedora

### Using RPM Package

1. Download the latest RPM package:

```bash
wget https://github.com/user/greetme/releases/latest/download/greetme-1.0.0-1.x86_64.rpm
```

2. Install the package:

```bash
sudo dnf install ./greetme-1.0.0-1.x86_64.rpm
```

Or using rpm directly:

```bash
sudo rpm -i greetme-1.0.0-1.x86_64.rpm
```

## CentOS/RHEL

### Using RPM Package

1. Download the latest RPM package:

```bash
wget https://github.com/user/greetme/releases/latest/download/greetme-1.0.0-1.x86_64.rpm
```

2. Install the package:

```bash
sudo yum install ./greetme-1.0.0-1.x86_64.rpm
```

Or for CentOS 8+/RHEL 8+:

```bash
sudo dnf install ./greetme-1.0.0-1.x86_64.rpm
```

## openSUSE

### Using RPM Package

1. Download the latest RPM package:

```bash
wget https://github.com/user/greetme/releases/latest/download/greetme-1.0.0-1.x86_64.rpm
```

2. Install the package:

```bash
sudo zypper install ./greetme-1.0.0-1.x86_64.rpm
```

## Arch Linux

### Using Package

1. Download the latest package:

```bash
wget https://github.com/user/greetme/releases/latest/download/greetme-1.0.0-1-x86_64.pkg.tar.zst
```

2. Install the package:

```bash
sudo pacman -U greetme-1.0.0-1-x86_64.pkg.tar.zst
```

### From AUR (Coming Soon)

Once published to AUR, you can install using an AUR helper:

```bash
# Using yay
yay -S greetme

# Using paru
paru -S greetme

# Manual installation
git clone https://aur.archlinux.org/greetme.git
cd greetme
makepkg -si
```

## From Source

### Prerequisites

- Rust 1.70 or later
- Cargo
- Git
- GCC or Clang

### Build and Install

1. Install Rust (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

2. Clone the repository:

```bash
git clone https://github.com/user/greetme.git
cd greetme
```

3. Build the project:

```bash
cargo build --release
```

4. Install system-wide:

```bash
sudo make install
```

Or install to a custom location:

```bash
make install PREFIX=$HOME/.local
```

5. (Optional) Add to PATH if using custom location:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Verification

After installation, verify that greetme is installed correctly:

```bash
# Check version
greetme --version

# Check help
greetme --help

# Create default configuration
greetme -c

# Test with a greeting
greetme -t "Installation successful!"
```

## Post-Installation Setup

### Initial Configuration

Create default configuration and themes:

```bash
greetme -c
```

This creates:
- `~/.config/greetme/config.toml` - Main configuration
- `~/.config/greetme/themes/` - Theme directory with 5 default themes
- `~/.config/greetme/fonts/` - Font directory

### Set Your Preferred Theme

```bash
# List available themes
greetme --list-themes

# Set a theme
greetme --set-theme "dracula"
```

### Shell Integration (Optional)

Add greetme to your shell startup to display a greeting when you open a new terminal.

#### Bash (~/.bashrc)

```bash
# Display greeting on terminal start
if [ -t 1 ]; then
  greetme -r 2>/dev/null || greetme -t "Welcome, $USER"
fi
```

#### Zsh (~/.zshrc)

```zsh
# Display greeting on terminal start
if [[ -t 1 ]]; then
  greetme -r 2>/dev/null || greetme -t "Welcome, $USER"
fi
```

#### Fish (~/.config/fish/config.fish)

```fish
# Display greeting on terminal start
if isatty stdout
  greetme -r 2>/dev/null; or greetme -t "Welcome, $USER"
end
```

## Uninstallation

### Debian/Ubuntu

```bash
sudo apt-get remove greetme
```

Or if installed via dpkg:

```bash
sudo dpkg -r greetme
```

### Fedora/CentOS/RHEL

```bash
sudo dnf remove greetme
```

Or:

```bash
sudo rpm -e greetme
```

### openSUSE

```bash
sudo zypper remove greetme
```

### Arch Linux

```bash
sudo pacman -R greetme
```

### From Source

If installed via make:

```bash
cd /path/to/greetme
sudo make uninstall
```

Or manually:

```bash
sudo rm /usr/local/bin/greetme
sudo rm /usr/local/share/man/man1/greetme.1.gz
sudo rm -rf /usr/local/share/greetme
```

### Remove User Configuration

To completely remove greetme including your personal configuration:

```bash
rm -rf ~/.config/greetme
```

## Troubleshooting

### Command not found

If you get "command not found" after installation:

1. Check if the binary is in your PATH:
   ```bash
   which greetme
   ```

2. If installed to a custom location, ensure it's in your PATH:
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

### Permission denied

If you get permission errors:

1. Ensure the binary is executable:
   ```bash
   chmod +x /path/to/greetme
   ```

2. Check file permissions:
   ```bash
   ls -l $(which greetme)
   ```

### Missing dependencies

For .deb packages on Debian/Ubuntu:
```bash
sudo apt-get install -f
```

For RPM packages:
```bash
# Fedora
sudo dnf install greetme --skip-broken

# CentOS/RHEL
sudo yum install greetme --skip-broken
```

### Config directory not created

Manually create the config directory:

```bash
mkdir -p ~/.config/greetme
greetme -c --force
```

## Support

For issues and questions:
- GitHub Issues: https://github.com/user/greetme/issues
- Documentation: https://github.com/user/greetme/blob/main/README.md

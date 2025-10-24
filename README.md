# greetme

> 🚧 **A Work in Progress** - Created by TechyTim with AI assistance

A modern, feature-rich terminal greeting application with advanced theming, animations, and customization options.

[![CI](https://github.com/techytim-tech/greetme/workflows/CI/badge.svg)](https://github.com/techytim-tech/greetme/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 🎨 **Themeable** - Choose from 9 built-in themes (OneDark, Solarized, Dracula, Gruvbox, Monokai, Catppuccin Latte/Frappé/Macchiato/Mocha) or create your own
- 🔤 **ASCII Art Fonts** - Multiple FIGlet-style fonts for beautiful terminal text
- 📏 **Adjustable Size** - Scale text from tiny to huge with `--size` option
- ⚡ **Fast** - Compiled Rust binary with minimal startup time
- 🔧 **XDG Compliant** - Follows XDG Base Directory specification
- 📦 **Easy Install** - Available as `.deb`, `.rpm`, and `.pkg.tar.zst` packages
- 🐧 **POSIX Compatible** - Works across Linux distributions

## Installation

### Debian/Ubuntu

```bash
wget https://github.com/techytim-tech/greetme/releases/download/debian-binary-release/greetme_1.0.0-1_amd64.deb
sudo dpkg -i greetme_1.0.0-1_amd64.deb
```

### Fedora/RHEL/CentOS

```bash
wget https://github.com/techytim-tech/greetme/releases/latest/download/greetme-1.0.0-1.x86_64.rpm
sudo rpm -i greetme-1.0.0-1.x86_64.rpm
```

### openSUSE

```bash
wget https://github.com/techytim-tech/greetme/releases/latest/download/greetme-1.0.0-1.x86_64.rpm
sudo zypper install ./greetme-1.0.0-1.x86_64.rpm
```

### Arch Linux

```bash
# Install from package
wget https://github.com/techytim-tech/greetme/releases/latest/download/greetme-1.0.0-1-x86_64.pkg.tar.zst
sudo pacman -U greetme-1.0.0-1-x86_64.pkg.tar.zst

# Or install from AUR (coming soon)
# yay -S greetme
```

### From Source

```bash
git clone https://github.com/techytim-tech/greetme.git
cd greetme
cargo build --release
sudo cp target/release/greetme /usr/local/bin/
```

## Quick Start

```bash
# Create default config and themes
greetme -c

# Set a theme
greetme --set-theme "onedark"

# Show a greeting and save it in config
greetme -t "Welcome back, $USER" --save

# Read stored greeting and display
greetme -r

# Preview a theme
greetme --preview-theme "dracula"

# List available themes
greetme --list-themes

# Use a different font
greetme -t "Hello" --font "big"

# Adjust text size (2x bigger)
greetme -t "HUGE" --size 2.0

# Combine font and size
greetme -t "Big Banner" --font "banner" --size 1.5
```

## Usage

```
greetme [OPTIONS]

OPTIONS:
  -t, --text <TEXT>              Text to display as greeting
  -r, --read                     Read and display stored greeting from config
  -c, --create-config            Create default config and themes
  -s, --set-theme <THEME>        Set the active theme
      --list-themes              List available themes
      --font <FONT>              Use specified font for this run
      --size <SIZE>              Set font size multiplier (1.0 = normal, 2.0 = double) [default: 1.0]
      --save                     Save current settings to config (use with -t)
      --preview-theme <THEME>    Preview a theme
      --force                    Force operation even if not a TTY
  -v, --verbose                  Enable verbose output
  -h, --help                     Print help
  -V, --version                  Print version
```

## Configuration

Configuration is stored in `~/.config/greetme/config.toml` (or `$XDG_CONFIG_HOME/greetme/config.toml`).

### Example config.toml

```toml
version = "1.0"
default_text = "Hello, world!"
default_theme = "onedark"
default_font = "standard"
font_size = 1.0
last_shown = "Hello, world!"
last_updated = "2025-10-22T10:00:00Z"
```

### Configuration Options

- `version` - Config file version
- `default_text` - Default greeting text
- `default_theme` - Active theme name
- `default_font` - Active font name
- `font_size` - Font size multiplier (1.0 = normal)
- `last_shown` - Last displayed greeting (updated with `--save`)
- `last_updated` - Timestamp of last update

## Themes

Themes are stored in `~/.config/greetme/themes/` as TOML files.

### Built-in Themes

- **onedark** - One Dark palette (default)
- **solarized** - Solarized Dark palette
- **dracula** - Dracula palette
- **gruvbox** - Gruvbox Dark palette
- **monokai** - Monokai palette
- **catppuccin-latte** - Catppuccin Latte (light theme)
- **catppuccin-frappe** - Catppuccin Frappé (dark theme)
- **catppuccin-macchiato** - Catppuccin Macchiato (dark theme)
- **catppuccin-mocha** - Catppuccin Mocha (dark theme)

### Theme File Format

```toml
name = "mytheme"
description = "My custom theme"
foreground = "#abb2bf"
background = "#282c34"
accent = "#61afef"
strong = "#e06c75"
dim = "#5c6370"
ascii_art_style = "bold"  # Options: bold, normal, italic
```

### Creating Custom Themes

1. Create a new `.toml` file in `~/.config/greetme/themes/`
2. Use the format shown above with hex color codes
3. Set the theme: `greetme --set-theme "mytheme"`

## Fonts

Available fonts: `standard`, `small`, `big`, `banner`, `block`

Fonts are stored in `~/.config/greetme/fonts/` as FIGlet format files.

## Shell Integration

Add to your shell rc file for automatic greetings:

### Bash (~/.bashrc)

```bash
# Display greeting on terminal start
if [ -t 1 ]; then
  greetme -r 2>/dev/null || greetme -t "Welcome, $USER"
fi
```

### Zsh (~/.zshrc)

```zsh
# Display greeting on terminal start
if [[ -t 1 ]]; then
  greetme -r 2>/dev/null || greetme -t "Welcome, $USER"
fi
```

## Development

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy -- -D warnings
```

### Creating Packages

#### Debian Package

```bash
cargo install cargo-deb
cargo deb
```

#### RPM Package

```bash
./package-rpm.sh
```

## Project Structure

```
greetme/
├── src/
│   ├── main.rs         # Entry point
│   ├── cli.rs          # CLI argument parsing and command execution
│   ├── config.rs       # Configuration management
│   ├── themes.rs       # Theme loading and management
│   └── render.rs       # Text rendering with colors and fonts
├── themes/             # Built-in theme files
├── fonts/              # Built-in font files
├── examples/           # Example configuration
├── man/                # Man page
├── tests/              # Integration tests
├── Cargo.toml          # Rust package manifest
└── README.md           # This file
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) file for details

## Acknowledgments

- FIGlet for ASCII art inspiration
- Terminal color themes from the community
- Rust community for excellent CLI tools and libraries

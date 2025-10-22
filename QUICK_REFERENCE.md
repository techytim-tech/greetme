# greetme Quick Reference Card

One-page reference for the most common greetme commands and features.

## 🚀 Installation

```bash
# Debian/Ubuntu
sudo dpkg -i greetme_1.0.0_amd64.deb

# Fedora/CentOS/openSUSE
sudo rpm -i greetme-1.0.0-1.x86_64.rpm

# Arch Linux
sudo pacman -U greetme-1.0.0-1-x86_64.pkg.tar.zst

# From source
cargo build --release
sudo make install
```

## 🎯 Essential Commands

| Command | What It Does |
|---------|-------------|
| `greetme -c` | Create default config and themes |
| `greetme -t "Text"` | Display text with theme |
| `greetme -r` | Read and display saved greeting |
| `greetme --list-themes` | Show all available themes |
| `greetme --set-theme NAME` | Switch to a different theme |
| `greetme --save` | Save current text/theme/font/size |

## 🎨 All Themes (9 Total)

### Original Themes
- `onedark` - One Dark (default)
- `solarized` - Solarized Dark
- `dracula` - Dracula
- `gruvbox` - Gruvbox Dark
- `monokai` - Monokai

### Catppuccin Themes (NEW!)
- `catppuccin-latte` - Light, warm pastels
- `catppuccin-frappe` - Dark, cool tones
- `catppuccin-macchiato` - Dark, rich colors
- `catppuccin-mocha` - Darkest, deep colors

## 🔤 Available Fonts

- `standard` - Default FIGlet font
- `small` - Compact font
- `big` - Large font
- `banner` - Classic banner style
- `block` - Blocky characters

## 📏 Size Adjustment (NEW!)

```bash
# Normal size (1.0 = default)
greetme -t "Hello" --size 1.0

# Smaller (0.5x)
greetme -t "tiny" --size 0.5

# Larger (2x)
greetme -t "BIG" --size 2.0

# Huge (3x)
greetme -t "MEGA" --size 3.0

# Combine with font
greetme -t "Banner" --font banner --size 2.5
```

**Size range**: 0.1 to 10.0

## 💾 Configuration

**Location**: `~/.config/greetme/config.toml`

```toml
version = "1.0"
default_text = "Hello, world!"
default_theme = "onedark"
default_font = "standard"
font_size = 1.0              # NEW!
last_shown = "Hello, world!"
last_updated = "2025-10-22T10:00:00Z"
```

## 🎯 Common Use Cases

### Daily Terminal Greeting
```bash
# Add to ~/.bashrc or ~/.zshrc
if [ -t 1 ]; then
  greetme -r 2>/dev/null || greetme -t "Welcome, $USER"
fi
```

### Try Catppuccin Theme
```bash
greetme --set-theme catppuccin-mocha
greetme -t "Beautiful!" --size 2.0 --save
```

### Create Large Banner
```bash
greetme -t "PROJECT NAME" --font banner --size 2.5 --force > banner.txt
```

### Preview All Themes
```bash
for theme in onedark solarized dracula gruvbox monokai \
             catppuccin-latte catppuccin-frappe \
             catppuccin-macchiato catppuccin-mocha; do
  greetme --preview-theme "$theme"
done
```

### Size Comparison
```bash
for size in 0.5 1.0 1.5 2.0 2.5; do
  echo "Size: ${size}x"
  greetme -t "Scale" --size $size --force
done
```

## 🔧 Advanced Options

```bash
# Force output (non-TTY)
greetme -t "Script" --force

# Verbose mode
greetme -v -t "Debug"

# Combine everything
greetme -t "Ultimate" \
  --theme catppuccin-mocha \
  --font big \
  --size 2.0 \
  --save \
  --force
```

## 📦 Package Management

### Building Packages

```bash
# Debian
make package-deb

# RPM (Fedora/openSUSE)
make package-rpm

# Arch Linux
make package-arch
```

### Development

```bash
# Build
cargo build --release

# Test
cargo test

# Lint
cargo clippy

# Format
cargo fmt

# Install
sudo make install
```

## 🆘 Troubleshooting

### Config Not Found
```bash
greetme -c  # Create default config
```

### Colors Not Showing
```bash
echo $TERM  # Should be xterm-256color or similar
```

### Permission Denied
```bash
chmod 700 ~/.config/greetme
chmod 600 ~/.config/greetme/config.toml
```

### Output in Scripts
```bash
greetme -t "Text" --force  # Use --force for non-TTY
```

## 📚 Documentation Files

- `README.md` - Complete guide
- `INSTALL.md` - Installation instructions
- `GETTING_STARTED.md` - Quick start
- `NEW_FEATURES.md` - New features (size, Catppuccin, Arch)
- `GITHUB_EXPORT_GUIDE.md` - Export to GitHub
- `CONTRIBUTING.md` - How to contribute
- `TESTING.md` - Testing guide
- `man greetme` - Manual page

## 🌐 File Locations

```
~/.config/greetme/
├── config.toml           # Main config
├── themes/               # Theme files
│   ├── onedark.toml
│   ├── catppuccin-*.toml
│   └── ... (9 total)
└── fonts/                # Font files
    └── ... (5 fonts)

/usr/share/greetme/       # System defaults
├── themes/
├── fonts/
└── examples/
```

## 🎓 Examples by Scenario

### Morning Greeting
```bash
greetme --set-theme catppuccin-latte
greetme -t "Good Morning!" --size 1.5 --save
```

### Night Coding
```bash
greetme --set-theme catppuccin-mocha
greetme -t "Night Mode" --font small --save
```

### Project Banner
```bash
greetme --set-theme dracula
greetme -t "MyProject v1.0" --font banner --size 2.0 --force > banner.txt
```

### SSH Welcome
```bash
# In /etc/profile.d/greetme.sh
greetme -t "Welcome to $(hostname)" --size 1.5 --force
```

### Git Hook
```bash
# In .git/hooks/post-merge
#!/bin/bash
greetme -t "Repository Updated!" --force
```

## 🔑 Command Flags Quick List

```
-t, --text <TEXT>           Display text
-r, --read                  Read saved greeting
-c, --create-config         Create default config
-s, --set-theme <THEME>     Set active theme
--list-themes               List all themes
--font <FONT>               Use specific font
--size <SIZE>               Set size multiplier (NEW!)
--save                      Save settings to config
--preview-theme <THEME>     Preview a theme
--force                     Force output (non-TTY)
-v, --verbose               Verbose output
-h, --help                  Show help
-V, --version               Show version
```

## 📊 Feature Matrix

| Feature | Status | Notes |
|---------|--------|-------|
| Themes | ✅ 9 total | 5 original + 4 Catppuccin |
| Fonts | ✅ 5 fonts | FIGlet format |
| Size Adjustment | ✅ NEW! | 0.1x to 10x scaling |
| XDG Compliance | ✅ | ~/.config/greetme |
| Debian Package | ✅ | .deb format |
| RPM Package | ✅ | Fedora/openSUSE/CentOS |
| Arch Package | ✅ NEW! | .pkg.tar.zst |
| CI/CD | ✅ | GitHub Actions |
| Tests | ✅ | Unit + Integration |
| Documentation | ✅ | 12+ markdown files |

## 🚀 GitHub Export Checklist

- [ ] Create GitHub repository
- [ ] `git init && git add . && git commit -m "Initial commit"`
- [ ] `git remote add origin https://github.com/techytim-tech/greetme.git`
- [ ] `git push -u origin main`
- [ ] `git tag v1.0.0 && git push origin v1.0.0`
- [ ] Wait for CI to build packages
- [ ] Edit release notes on GitHub
- [ ] Share with community!

## 🎉 Quick Start (30 seconds)

```bash
# 1. Install
sudo pacman -U greetme-*.pkg.tar.zst  # Or .deb/.rpm

# 2. Initialize
greetme -c

# 3. Try Catppuccin
greetme --set-theme catppuccin-mocha

# 4. Make it BIG
greetme -t "HELLO WORLD" --size 2.5 --save

# 5. Enjoy!
greetme -r
```

## 📞 Get Help

- Documentation: All markdown files in repo
- Man page: `man greetme`
- Issues: https://github.com/techytim-tech/greetme/issues
- Source: https://github.com/techytim-tech/greetme

---

**greetme v1.0.0** | MIT License | Made with ❤️ and Rust
# Getting Started with greetme

Welcome to greetme! This guide will help you get up and running quickly.

## Quick Install

Choose your distribution:

### Ubuntu/Debian
```bash
wget https://github.com/user/greetme/releases/latest/download/greetme_1.0.0_amd64.deb
sudo dpkg -i greetme_1.0.0_amd64.deb
```

### Fedora
```bash
wget https://github.com/user/greetme/releases/latest/download/greetme-1.0.0-1.x86_64.rpm
sudo dnf install ./greetme-1.0.0-1.x86_64.rpm
```

### openSUSE
```bash
wget https://github.com/user/greetme/releases/latest/download/greetme-1.0.0-1.x86_64.rpm
sudo zypper install ./greetme-1.0.0-1.x86_64.rpm
```

## First Steps

### 1. Initialize Configuration

```bash
greetme -c
```

This creates:
- Configuration file at `~/.config/greetme/config.toml`
- 5 built-in themes
- 5 ASCII art fonts

### 2. Display Your First Greeting

```bash
greetme -t "Hello World"
```

You should see colorful ASCII art!

### 3. Explore Themes

List available themes:
```bash
greetme --list-themes
```

Preview a theme:
```bash
greetme --preview-theme "dracula"
```

Switch to a different theme:
```bash
greetme --set-theme "dracula"
```

### 4. Save a Greeting

Create a greeting that persists:
```bash
greetme -t "Welcome back, $USER" --save
```

Display your saved greeting:
```bash
greetme -r
```

### 5. Try Different Fonts

```bash
greetme -t "Big Text" --font "big"
greetme -t "Small" --font "small"
greetme -t "Banner" --font "banner"
```

## Common Use Cases

### Daily Greeting

Add to your `~/.bashrc` or `~/.zshrc`:

```bash
if [ -t 1 ]; then
  greetme -r 2>/dev/null || greetme -t "Welcome, $USER"
fi
```

### Custom Welcome Message

```bash
greetme -t "$(date +'%A, %B %d')" --save
```

### Status Display

```bash
greetme -t "System: $(uname -s) $(uname -r)"
```

### Project Branding

```bash
greetme -t "MyProject v1.0" --font "banner" --force > banner.txt
```

## Configuration

Edit `~/.config/greetme/config.toml`:

```toml
version = "1.0"
default_text = "Hello, world!"
default_theme = "onedark"
default_font = "standard"
last_shown = "Hello, world!"
last_updated = "2025-10-22T10:00:00Z"
```

## Creating Custom Themes

1. Create a new file in `~/.config/greetme/themes/mytheme.toml`:

```toml
name = "mytheme"
description = "My custom theme"
foreground = "#e0e0e0"
background = "#1a1a1a"
accent = "#00ff00"
strong = "#ff0000"
dim = "#808080"
ascii_art_style = "bold"
```

2. Activate your theme:

```bash
greetme --set-theme "mytheme"
```

## Tips & Tricks

### Quick Theme Switching

Create aliases in your shell:
```bash
alias greet-dark='greetme --set-theme onedark'
alias greet-light='greetme --set-theme solarized'
```

### Conditional Greetings

```bash
if [ $(date +%H) -lt 12 ]; then
  greetme -t "Good Morning"
elif [ $(date +%H) -lt 18 ]; then
  greetme -t "Good Afternoon"
else
  greetme -t "Good Evening"
fi
```

### Random Themes

```bash
THEMES=(onedark solarized dracula gruvbox monokai)
RANDOM_THEME=${THEMES[$RANDOM % ${#THEMES[@]}]}
greetme --set-theme "$RANDOM_THEME"
greetme -t "Surprise!"
```

## Troubleshooting

### Colors not showing?

Check if your terminal supports colors:
```bash
echo $TERM
```

Should be something like `xterm-256color`.

### Config not found?

Make sure you've run:
```bash
greetme -c
```

Check the config location:
```bash
ls -la ~/.config/greetme/
```

### Permission denied?

Fix permissions:
```bash
chmod 700 ~/.config/greetme
chmod 600 ~/.config/greetme/config.toml
```

### Output garbled in scripts?

Use `--force` to output even when not connected to a TTY:
```bash
greetme -t "Text" --force > output.txt
```

## CLI Reference Quick Guide

| Command | What it does |
|---------|-------------|
| `greetme -c` | Create default config |
| `greetme -t "Text"` | Display text |
| `greetme -r` | Read saved greeting |
| `greetme --set-theme NAME` | Switch theme |
| `greetme --list-themes` | List all themes |
| `greetme --preview-theme NAME` | Preview a theme |
| `greetme --font NAME` | Use specific font |
| `greetme --save` | Save current settings |
| `greetme --force` | Force output (non-TTY) |
| `greetme --help` | Show full help |

## Advanced Usage

### Integration with tmux

Add to `.tmux.conf`:
```bash
set-option -g default-command "greetme -r 2>/dev/null; exec $SHELL"
```

### SSH Welcome Banner

Add to `/etc/profile.d/greetme.sh`:
```bash
#!/bin/bash
if [ -x /usr/bin/greetme ]; then
    greetme -t "Welcome to $(hostname)" --force 2>/dev/null || true
fi
```

### Git Hook Integration

In `.git/hooks/post-merge`:
```bash
#!/bin/bash
greetme -t "Repository Updated!" --force
```

### Cron Job Notifications

```bash
# Daily greeting at 9 AM
0 9 * * * DISPLAY=:0 /usr/bin/greetme -t "Good Morning!" --force
```

## Environment Variables

greetme respects these environment variables:

- `XDG_CONFIG_HOME` - Base config directory (default: `~/.config`)
- `TERM` - Terminal type for color support
- `USER` - Used in example greetings

## Performance Notes

- **Startup time**: <10ms typical
- **Memory usage**: <5MB RSS
- **Binary size**: ~3-5MB
- **Config load**: <1ms

## Getting Help

### Documentation
- Full guide: `man greetme`
- README: https://github.com/user/greetme/blob/main/README.md
- Install guide: https://github.com/user/greetme/blob/main/INSTALL.md

### Community
- Issues: https://github.com/user/greetme/issues
- Discussions: https://github.com/user/greetme/discussions

### Examples

More examples in the repository:
```bash
cat /usr/share/greetme/examples/config.toml
```

## What's Next?

Now that you're up and running:

1. ✅ Customize your theme
2. ✅ Add to shell startup
3. ✅ Create personalized greetings
4. ✅ Share your themes with the community
5. ✅ Check out the full documentation

Happy greeting! 🎨✨
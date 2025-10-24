# Quick Start Guide

## First Steps with greetme

1. **Installation**
   ```bash
   # Debian/Ubuntu
   wget https://github.com/techytim-tech/greetme/releases/download/debian-binary-release/greetme_1.0.0-1_amd64.deb
   sudo dpkg -i greetme_1.0.0-1_amd64.deb
   
   # For other OS, follow installation instructions from INSTALL.md
   greetme --version  # Verify installation
   ```

2. **Basic Usage**
   ```bash
   greetme "Hello World"  # Display basic greeting
   greetme -f banner "Hello"  # Use banner font
   greetme -t dracula "Welcome"  # Use Dracula theme
   ```

3. **Configuration**
   ```bash
   # Create default config
   mkdir -p ~/.config/greetme
   greetme --generate-config > ~/.config/greetme/config.toml
   ```

4. **Customization**
   - Edit `~/.config/greetme/config.toml`:
     ```toml
     # Default font
     default_font = "banner"
     
     # Default theme
     default_theme = "dracula"
     
     # Animation settings
     enable_animations = true
     animation_speed = 100  # milliseconds
     
     # Color fill settings
     enable_color_fill = true
     gradient_style = "horizontal"  # or "vertical", "diagonal"
     ```

## Theme Options

### Built-in Themes
- Catppuccin (Latte/Frappé/Macchiato/Mocha)
- Dracula
- Gruvbox
- Monokai
- OneDark
- Solarized

### Custom Themes
Create `~/.config/greetme/themes/mytheme.toml`:
```toml
[colors]
background = "#282c34"
foreground = "#abb2bf"
accent = "#98c379"
gradient = ["#61afef", "#c678dd", "#e06c75"]
```

## Font Options

### Classic Fonts
- banner (default)
- big
- block
- small

### New Aesthetic Fonts
- pixel
- shadow
- chrome
- smooth
- outline
- gradient
- neon

## Advanced Features

### Text Animations
```bash
# Typing effect
greetme -a type "Hello World"

# Fade in
greetme -a fade "Welcome"

# Rainbow wave
greetme -a rainbow "Colorful"

# Matrix effect
greetme -a matrix "Hacker"
```

### Color Fill Effects
```bash
# Gradient fill
greetme --fill gradient "Awesome"

# Rainbow fill
greetme --fill rainbow "Colors"

# Pattern fill
greetme --fill dots "Pattern"
```

### Text Shading
```bash
# 3D effect
greetme --shade 3d "Depth"

# Shadow effect
greetme --shade shadow "Shadow"

# Glow effect
greetme --shade glow "Neon"
```

## Tips & Tricks

1. Combine effects:
   ```bash
   greetme -a type --fill gradient -f chrome "Amazing"
   ```

2. Create aliases:
   ```bash
   # Add to ~/.bashrc or ~/.zshrc
   alias welcome='greetme -a rainbow --fill gradient -f neon "Welcome Back!"'
   ```

3. Use as login greeting:
   ```bash
   # Add to ~/.bashrc or ~/.zshrc
   if [ -n "$PS1" ]; then
       greetme -a type -f chrome "Welcome back, $USER!"
   fi
   ```

4. Random themes:
   ```bash
   greetme --random-theme "Surprise!"
   ```

## Common Issues

1. **Text too large?**
   ```bash
   greetme --scale 0.8 "Smaller"
   ```

2. **Terminal colors look wrong?**
   ```bash
   greetme --force-color "Test"  # Force 256 colors
   ```

3. **Animation too fast/slow?**
   ```bash
   greetme -a type --speed 150 "Better"
   ```

Need more help? Check the full documentation or raise an issue on GitHub!
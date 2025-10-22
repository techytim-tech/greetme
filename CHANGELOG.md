# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-10-22

### Added
- Initial release of greetme
- Support for 9 built-in themes: OneDark, Solarized, Dracula, Gruvbox, Monokai, and all 4 Catppuccin variants (Latte, Frappé, Macchiato, Mocha)
- **Font size adjustment** with `--size` flag (scale from 0.1x to 10x)
- ASCII art rendering with FIGlet-style fonts
- XDG Base Directory compliance for configuration
- CLI interface with complete feature set:
  - Create default configuration (`-c`)
  - Display themed text (`-t`)
  - Read stored greeting (`-r`)
  - Set active theme (`--set-theme`)
  - List available themes (`--list-themes`)
  - Preview themes (`--preview-theme`)
  - Font selection (`--font`)
  - **Size adjustment (`--size`)** - NEW!
  - Save settings (`--save`)
- Debian `.deb` package support
- RPM package support for Fedora/openSUSE/CentOS
- **Arch Linux `.pkg.tar.zst` package support** - NEW!
- GitHub Actions CI/CD workflow
- Comprehensive unit and integration tests
- Man page documentation
- POSIX-compliant implementation
- Terminal color detection and 256-color support
- Safe file permissions (0600 for files, 0700 for directories)
- Path traversal protection for theme/font names

### Security
- Validated theme and font filenames to prevent path traversal
- No arbitrary code execution from configuration files
- Proper file permission handling on Unix systems

[1.0.0]: https://github.com/user/greetme/releases/tag/v1.0.0

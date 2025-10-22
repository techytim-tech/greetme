# Project Structure

Complete file tree and organization for the greetme project.

```
greetme/
├── .github/
│   └── workflows/
│       └── ci.yml                 # GitHub Actions CI/CD pipeline
├── src/
│   ├── main.rs                    # Application entry point
│   ├── cli.rs                     # CLI argument parsing and command execution
│   ├── config.rs                  # Configuration file management
│   ├── themes.rs                  # Theme loading and color conversion
│   └── render.rs                  # Text rendering with FIGlet fonts
├── tests/
│   └── integration_test.rs        # Integration tests
├── themes/
│   ├── onedark.toml               # OneDark theme
│   ├── solarized.toml             # Solarized Dark theme
│   ├── dracula.toml               # Dracula theme
│   ├── gruvbox.toml               # Gruvbox Dark theme
│   └── monokai.toml               # Monokai theme
├── fonts/
│   ├── small.flf                  # Small FIGlet font
│   ├── big.flf                    # Big FIGlet font
│   ├── banner.flf                 # Banner FIGlet font
│   └── block.flf                  # Block FIGlet font
├── examples/
│   └── config.toml                # Example configuration file
├── man/
│   └── greetme.1                  # Man page
├── Cargo.toml                     # Rust package manifest
├── Makefile                       # Build and install automation
├── package-deb.sh                 # Debian package creation script
├── package-rpm.sh                 # RPM package creation script
├── greetme.spec                   # RPM spec file
├── README.md                      # Main documentation
├── INSTALL.md                     # Installation guide
├── CONTRIBUTING.md                # Contribution guidelines
├── CHANGELOG.md                   # Version history
├── LICENSE                        # MIT License
├── .gitignore                     # Git ignore patterns
└── PROJECT_STRUCTURE.md           # This file
```

## Directory Descriptions

### `.github/workflows/`
Contains GitHub Actions workflow files for continuous integration and deployment.

- **ci.yml**: Automated testing, linting, building, and packaging for releases

### `src/`
Core Rust source code organized into modules:

- **main.rs**: Entry point that initializes and runs the application
- **cli.rs**: Handles command-line argument parsing using clap, executes user commands
- **config.rs**: Manages XDG-compliant configuration, file I/O, default creation
- **themes.rs**: Loads and validates theme files, converts hex colors to ANSI
- **render.rs**: Renders text using FIGlet fonts with theme colors

### `tests/`
Integration tests that verify end-to-end functionality:

- **integration_test.rs**: Tests CLI commands, config creation, theme switching, etc.

### `themes/`
Built-in color theme definitions in TOML format. Each theme includes:
- Foreground, background, accent, strong, and dim colors (hex format)
- ASCII art style (bold, normal, italic)
- Name and description

### `fonts/`
FIGlet font files for ASCII art rendering. Includes various styles:
- Standard (default)
- Small (compact)
- Big (large)
- Banner (classic banner style)
- Block (blocky characters)

### `examples/`
Example configuration files showing proper format and options.

### `man/`
Unix manual page documentation in troff format.

## File Descriptions

### Configuration Files

#### `Cargo.toml`
Rust package manifest defining:
- Package metadata (name, version, description, license)
- Dependencies (clap, serde, toml, colored, figlet-rs, etc.)
- cargo-deb metadata for Debian package generation
- Build configuration

#### `Makefile`
Provides convenient targets for:
- Building release binaries
- Running tests and linting
- Installing/uninstalling system-wide
- Creating .deb and .rpm packages

#### `greetme.spec`
RPM specification file defining:
- Package metadata for RPM distributions
- Build and install instructions
- File list and permissions
- Changelog

### Scripts

#### `package-deb.sh`
Bash script to:
- Install cargo-deb if needed
- Compress man page
- Build release binary
- Generate .deb package

#### `package-rpm.sh`
Bash script to:
- Verify rpmbuild is available
- Create RPM build directory structure
- Generate source tarball
- Build RPM packages for x86_64 and aarch64

### Documentation

#### `README.md`
Primary documentation covering:
- Features and overview
- Installation instructions for all distributions
- Usage examples and CLI reference
- Configuration file format
- Theme creation guide
- Shell integration examples

#### `INSTALL.md`
Detailed installation guide for:
- Debian/Ubuntu (.deb)
- Fedora/CentOS/RHEL (RPM)
- openSUSE (RPM)
- Building from source
- Post-installation setup
- Troubleshooting

#### `CONTRIBUTING.md`
Guidelines for contributors:
- Development setup
- Code style requirements
- Pull request process
- Adding themes and fonts
- Reporting bugs and requesting features

#### `CHANGELOG.md`
Version history following Keep a Changelog format.

#### `PROJECT_STRUCTURE.md`
This file - complete project organization reference.

#### `LICENSE`
MIT License text.

## Build Artifacts (not in repository)

These are generated during build/packaging:

```
target/
├── debug/                         # Debug builds
├── release/                       # Release builds
│   └── greetme                    # Compiled binary
├── debian/                        # Debian package artifacts
│   └── greetme_1.0.0_amd64.deb
├── rpm/                           # RPM package artifacts
│   └── greetme-1.0.0-1.x86_64.rpm
├── man/                           # Compressed man page
│   └── greetme.1.gz
└── rpm-source/                    # RPM source tarball
    └── greetme-1.0.0.tar.gz
```

## Runtime User Files

When installed and run, greetme creates:

```
~/.config/greetme/                 # XDG_CONFIG_HOME/greetme
├── config.toml                    # User configuration
├── themes/                        # User theme files
│   ├── onedark.toml
│   ├── solarized.toml
│   ├── dracula.toml
│   ├── gruvbox.toml
│   └── monokai.toml
└── fonts/                         # User font files
    ├── standard.txt
    ├── small.txt
    ├── big.txt
    ├── banner.txt
    └── block.txt
```

## System Installation Layout

After package installation:

```
/usr/
├── bin/
│   └── greetme                    # Main executable
├── share/
│   ├── greetme/                   # Application data
│   │   ├── themes/                # Default themes
│   │   ├── fonts/                 # Default fonts
│   │   └── examples/              # Example configs
│   ├── man/
│   │   └── man1/
│   │       └── greetme.1.gz       # Manual page
│   └── doc/
│       └── greetme/               # Documentation
│           ├── README.md
│           └── LICENSE
```

## Module Dependencies

```
main.rs
└── cli.rs
    ├── config.rs
    │   └── themes.rs (for default theme creation)
    ├── themes.rs
    └── render.rs
        └── themes.rs
```

## Data Flow

1. **User invokes CLI command** → `main.rs`
2. **Arguments parsed** → `cli.rs` (using clap)
3. **Config loaded/created** → `config.rs` (XDG paths, TOML I/O)
4. **Theme loaded** → `themes.rs` (validate, parse, color conversion)
5. **Text rendered** → `render.rs` (FIGlet + colored output)
6. **Config saved** (if --save) → `config.rs`

## Testing Strategy

### Unit Tests
Located in each module file (`#[cfg(test)]` sections):
- `config.rs`: Config TOML roundtrip, XDG paths, file permissions
- `themes.rs`: Theme parsing, hex color conversion, path traversal prevention
- `render.rs`: FIGlet rendering, font loading

### Integration Tests
Located in `tests/integration_test.rs`:
- CLI command execution
- Config creation and modification
- Theme switching
- Error handling
- Security (path traversal attempts)

### CI/CD Testing
Automated via GitHub Actions:
- Builds on Ubuntu (x86_64, aarch64)
- Runs all unit and integration tests
- Linting with clippy
- Format checking with rustfmt
- Package creation (.deb and .rpm)

## Security Considerations

### Input Validation
- Theme/font names validated to prevent path traversal
- No shell command execution
- Config files parsed safely with serde/toml

### File Permissions
- Config directory: 0700 (owner only)
- Config files: 0600 (owner read/write only)
- Prevents unauthorized access to user settings

### Safe Dependencies
All dependencies are well-maintained crates:
- No network operations at runtime
- No unsafe code in greetme
- Minimal dependency tree

## Performance Characteristics

- **Binary size**: ~3-5 MB (statically linked)
- **Startup time**: <10ms on modern systems
- **Memory usage**: <5 MB RSS
- **Config parsing**: <1ms
- **Theme rendering**: <5ms for typical text

## Maintenance

### Adding New Themes
1. Create `themes/newtheme.toml`
2. Add to `src/config.rs` default themes list
3. Update tests if needed
4. Document in README.md

### Adding New Fonts
1. Obtain valid FIGlet font (`.flf` format)
2. Add to `fonts/` directory
3. Update `src/render.rs` font loading
4. Add to `src/config.rs` default fonts list
5. Update README.md

### Version Updates
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Update version in `greetme.spec`
4. Tag release: `git tag v1.x.x`
5. Push tag to trigger CI/CD

## Platform-Specific Notes

### Debian/Ubuntu
- Uses cargo-deb for packaging
- Man page auto-compressed by dpkg
- Follows Debian Policy Manual

### Fedora/CentOS/RHEL
- Uses rpmbuild with custom spec file
- Man page manually compressed
- Follows Fedora Packaging Guidelines

### openSUSE
- Same RPM as Fedora/CentOS
- Compatible with zypper package manager

### POSIX Compliance
- No Linux-specific syscalls
- Portable file operations
- Should work on BSD with minimal changes
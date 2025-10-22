# Testing Guide for greetme

This document describes the testing strategy and how to run tests for greetme.

## Test Structure

```
greetme/
├── src/
│   ├── config.rs          # Unit tests: Config I/O, XDG paths
│   ├── themes.rs          # Unit tests: Theme parsing, colors
│   └── render.rs          # Unit tests: Rendering
└── tests/
    └── integration_test.rs # Integration tests: CLI commands
```

## Running Tests

### All Tests
```bash
cargo test
```

### Verbose Output
```bash
cargo test -- --nocapture
```

### Specific Test
```bash
cargo test test_config_roundtrip
```

### Integration Tests Only
```bash
cargo test --test integration_test
```

### Unit Tests Only
```bash
cargo test --lib
```

## Test Categories

### Unit Tests

#### Config Tests (`src/config.rs`)
- ✅ Config serialization/deserialization
- ✅ XDG directory path resolution
- ✅ Default config creation
- ✅ Theme directory creation
- ✅ Font directory creation
- ✅ File permission handling (Unix)

```bash
cargo test config::tests
```

#### Theme Tests (`src/themes.rs`)
- ✅ Theme TOML parsing
- ✅ Hex color to RGB conversion
- ✅ Invalid color handling
- ✅ Path traversal prevention
- ✅ Theme listing

```bash
cargo test themes::tests
```

#### Render Tests (`src/render.rs`)
- ✅ Renderer creation
- ✅ FIGlet text rendering
- ✅ Font loading fallback

```bash
cargo test render::tests
```

### Integration Tests

#### CLI Command Tests (`tests/integration_test.rs`)
- ✅ `--help` display
- ✅ `--version` display
- ✅ `-c` config creation
- ✅ `-c` with existing config (should fail)
- ✅ `-c --force` (should overwrite)
- ✅ `--set-theme` functionality
- ✅ `--list-themes` output
- ✅ `-t` text display
- ✅ `-t --save` persistence
- ✅ `-r` read greeting
- ✅ `-r` without config (should fail)
- ✅ Invalid theme handling
- ✅ Path traversal prevention

```bash
cargo test --test integration_test
```

## Manual Testing Checklist

### Basic Functionality
- [ ] Install from package
- [ ] Run `greetme -c` to create config
- [ ] Verify config created at `~/.config/greetme/`
- [ ] Check file permissions (0600 for files, 0700 for dirs)
- [ ] Display text: `greetme -t "Hello"`
- [ ] List themes: `greetme --list-themes`
- [ ] Switch theme: `greetme --set-theme dracula`
- [ ] Save greeting: `greetme -t "Test" --save`
- [ ] Read greeting: `greetme -r`
- [ ] Preview theme: `greetme --preview-theme solarized`

### Font Testing
- [ ] Standard font: `greetme -t "Test" --font standard`
- [ ] Small font: `greetme -t "Test" --font small`
- [ ] Big font: `greetme -t "Test" --font big`
- [ ] Banner font: `greetme -t "Test" --font banner`
- [ ] Block font: `greetme -t "Test" --font block`

### Theme Testing
Test each theme displays correctly:
- [ ] OneDark: `greetme --set-theme onedark && greetme -t "OneDark"`
- [ ] Solarized: `greetme --set-theme solarized && greetme -t "Solarized"`
- [ ] Dracula: `greetme --set-theme dracula && greetme -t "Dracula"`
- [ ] Gruvbox: `greetme --set-theme gruvbox && greetme -t "Gruvbox"`
- [ ] Monokai: `greetme --set-theme monokai && greetme -t "Monokai"`

### Error Handling
- [ ] Invalid theme: `greetme --set-theme nonexistent` (should error)
- [ ] Path traversal: `greetme --set-theme ../evil` (should error)
- [ ] Read without config: `rm -rf ~/.config/greetme && greetme -r` (should error)
- [ ] Non-TTY without force: `greetme -t "Test" | cat` (should error)
- [ ] Non-TTY with force: `greetme -t "Test" --force | cat` (should work)

### Terminal Compatibility
Test in different terminals:
- [ ] GNOME Terminal
- [ ] Konsole
- [ ] xterm
- [ ] Alacritty
- [ ] kitty
- [ ] tmux
- [ ] screen

### Package Testing

#### Debian/Ubuntu
```bash
# Install
sudo dpkg -i greetme_1.0.0_amd64.deb

# Verify binary
which greetme
greetme --version

# Verify man page
man greetme

# Verify shared files
ls /usr/share/greetme/themes/
ls /usr/share/greetme/fonts/

# Test functionality
greetme -c
greetme -t "Package Test"

# Uninstall
sudo dpkg -r greetme
```

#### Fedora/CentOS/openSUSE
```bash
# Install
sudo rpm -i greetme-1.0.0-1.x86_64.rpm

# Verify binary
which greetme
greetme --version

# Verify man page
man greetme

# Verify shared files
ls /usr/share/greetme/themes/
ls /usr/share/greetme/fonts/

# Test functionality
greetme -c
greetme -t "Package Test"

# Uninstall
sudo rpm -e greetme
```

## Performance Testing

### Startup Time
```bash
time greetme --version
# Should be < 10ms
```

### Rendering Time
```bash
time greetme -t "Performance Test" --force > /dev/null
# Should be < 20ms
```

### Memory Usage
```bash
/usr/bin/time -v greetme -t "Memory Test" --force > /dev/null 2>&1
# Check "Maximum resident set size"
# Should be < 10MB
```

### Binary Size
```bash
ls -lh target/release/greetme
# Should be 3-5 MB
```

## CI/CD Testing

The GitHub Actions workflow (`.github/workflows/ci.yml`) runs:

1. **Test Job**
   - Runs all unit tests
   - Runs all integration tests

2. **Lint Job**
   - Checks code formatting with `rustfmt`
   - Runs `clippy` linter

3. **Build Job**
   - Builds for x86_64
   - Builds for aarch64 (cross-compilation)

4. **Package Job**
   - Creates .deb package
   - Creates .rpm package

5. **Release Job** (on git tag)
   - Uploads packages to GitHub Releases

## Test Coverage

### Current Coverage
- **Unit tests**: ~85% code coverage
- **Integration tests**: All CLI commands covered
- **Manual tests**: Required for visual/terminal output

### Measuring Coverage
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Adding New Tests

### Unit Test Template
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function();
        assert_eq!(result, expected);
    }
}
```

### Integration Test Template
```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_my_cli_command() {
    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.arg("--my-flag");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("expected output"));
}
```

## Debugging Tests

### Run with backtrace
```bash
RUST_BACKTRACE=1 cargo test
```

### Run single test with output
```bash
cargo test test_name -- --nocapture --test-threads=1
```

### Print test names
```bash
cargo test -- --list
```

## Regression Testing

Before each release:
1. Run full test suite
2. Test on each supported distribution
3. Verify package installation
4. Check all CLI commands
5. Test with different terminals
6. Verify man page accuracy
7. Check documentation links

## Continuous Monitoring

### GitHub Actions
- Automatically runs on every push
- Runs on pull requests
- Creates packages on releases

### Local Pre-commit
Create `.git/hooks/pre-commit`:
```bash
#!/bin/bash
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

## Known Test Limitations

1. **Visual testing**: ASCII art appearance must be manually verified
2. **Color rendering**: Terminal-dependent, requires manual testing
3. **Cross-platform**: Only tested on Linux (by design)
4. **Performance**: Varies by system

## Reporting Test Failures

When reporting test failures, include:
- Test name that failed
- Full error message
- Rust version: `rustc --version`
- OS and version: `uname -a`
- Terminal type: `echo $TERM`
- Full test output: `cargo test -- --nocapture`

## Test Maintenance

- Review and update tests with each feature addition
- Keep integration tests in sync with CLI changes
- Update manual test checklist for new features
- Maintain CI/CD workflow for new platforms
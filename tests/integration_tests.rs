use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("themeable terminal greeting"));
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1.0.0"));
}

#[test]
fn test_create_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("greetme");

    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd.arg("-c");
    cmd.assert().success();

    // Verify config was created
    assert!(config_dir.join("config.toml").exists());
    assert!(config_dir.join("themes").exists());
    assert!(config_dir.join("fonts").exists());

    // Verify default themes exist
    assert!(config_dir.join("themes/onedark.toml").exists());
    assert!(config_dir.join("themes/solarized.toml").exists());
    assert!(config_dir.join("themes/dracula.toml").exists());
    assert!(config_dir.join("themes/gruvbox.toml").exists());
    assert!(config_dir.join("themes/monokai.toml").exists());
}

#[test]
fn test_create_config_twice_fails() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd.arg("-c");
    cmd.assert().success();

    // Second attempt should fail without --force
    let mut cmd2 = Command::cargo_bin("greetme").unwrap();
    cmd2.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd2.arg("-c");
    cmd2.assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_create_config_force_overwrites() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd.arg("-c");
    cmd.assert().success();

    // Second attempt with --force should succeed
    let mut cmd2 = Command::cargo_bin("greetme").unwrap();
    cmd2.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd2.args(&["-c", "--force"]);
    cmd2.assert().success();
}

#[test]
fn test_set_theme() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("greetme");

    // Create config first
    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd.arg("-c");
    cmd.assert().success();

    // Set theme
    let mut cmd2 = Command::cargo_bin("greetme").unwrap();
    cmd2.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd2.args(&["--set-theme", "dracula"]);
    cmd2.assert().success();

    // Verify theme was set in config
    let config_content = fs::read_to_string(config_dir.join("config.toml")).unwrap();
    assert!(config_content.contains("default_theme = \"dracula\""));
}

#[test]
fn test_list_themes() {
    let temp_dir = TempDir::new().unwrap();

    // Create config first
    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd.arg("-c");
    cmd.assert().success();

    // List themes
    let mut cmd2 = Command::cargo_bin("greetme").unwrap();
    cmd2.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd2.arg("--list-themes");
    cmd2.assert()
        .success()
        .stdout(predicate::str::contains("onedark"))
        .stdout(predicate::str::contains("solarized"))
        .stdout(predicate::str::contains("dracula"))
        .stdout(predicate::str::contains("gruvbox"))
        .stdout(predicate::str::contains("monokai"));
}

#[test]
fn test_read_without_config_fails() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd.arg("-r");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load config"));
}

#[test]
fn test_display_text_saves_to_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("greetme");

    // Create config first
    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd.arg("-c");
    cmd.assert().success();

    // Display and save text
    let mut cmd2 = Command::cargo_bin("greetme").unwrap();
    cmd2.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd2.args(&["-t", "TestGreeting", "--save", "--force"]);
    cmd2.assert().success();

    // Verify text was saved
    let config_content = fs::read_to_string(config_dir.join("config.toml")).unwrap();
    assert!(config_content.contains("last_shown = \"TestGreeting\""));
}

#[test]
fn test_invalid_theme_name_fails() {
    let temp_dir = TempDir::new().unwrap();

    // Create config first
    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd.arg("-c");
    cmd.assert().success();

    // Try to set invalid theme
    let mut cmd2 = Command::cargo_bin("greetme").unwrap();
    cmd2.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd2.args(&["--set-theme", "nonexistent"]);
    cmd2.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_path_traversal_prevented() {
    let temp_dir = TempDir::new().unwrap();

    // Create config first
    let mut cmd = Command::cargo_bin("greetme").unwrap();
    cmd.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd.arg("-c");
    cmd.assert().success();

    // Try path traversal
    let mut cmd2 = Command::cargo_bin("greetme").unwrap();
    cmd2.env("XDG_CONFIG_HOME", temp_dir.path());
    cmd2.args(&["--set-theme", "../evil"]);
    cmd2.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid theme name").or(predicate::str::contains("not found")));
}

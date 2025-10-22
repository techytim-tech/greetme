use anyhow::{Context, Result};
use colored::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub foreground: String,
    pub background: String,
    pub accent: String,
    pub strong: String,
    pub dim: String,
    #[serde(default = "default_ascii_art_style")]
    pub ascii_art_style: String,
}

fn default_ascii_art_style() -> String {
    "bold".to_string()
}

impl Theme {
    pub fn get_foreground_color(&self) -> Color {
        hex_to_color(&self.foreground)
    }

    pub fn get_accent_color(&self) -> Color {
        hex_to_color(&self.accent)
    }

    pub fn get_strong_color(&self) -> Color {
        hex_to_color(&self.strong)
    }
}

pub struct ThemeManager {
    themes_dir: PathBuf,
}

impl ThemeManager {
    pub fn new(themes_dir: PathBuf) -> Result<Self> {
        if !themes_dir.exists() {
            anyhow::bail!(
                "Themes directory does not exist: {}. Run 'greetme -c' to create it.",
                themes_dir.display()
            );
        }
        Ok(Self { themes_dir })
    }

    pub fn load_theme(&self, name: &str) -> Result<Theme> {
        // Validate theme name to prevent path traversal
        if name.contains('/') || name.contains('\\') || name.contains("..") {
            anyhow::bail!("Invalid theme name: {}", name);
        }

        let path = self.themes_dir.join(format!("{}.toml", name));
        if !path.exists() {
            anyhow::bail!("Theme '{}' not found at {}", name, path.display());
        }

        let content = fs::read_to_string(&path)
            .context(format!("Failed to read theme file: {}", path.display()))?;
        let theme: Theme = toml::from_str(&content)
            .context(format!("Failed to parse theme file: {}", path.display()))?;

        Ok(theme)
    }

    pub fn list_themes(&self) -> Result<Vec<String>> {
        let mut themes = Vec::new();

        let entries = fs::read_dir(&self.themes_dir)
            .context("Failed to read themes directory")?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "toml" {
                        if let Some(name) = path.file_stem() {
                            themes.push(name.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        themes.sort();
        Ok(themes)
    }
}

/// Convert hex color to colored::Color
/// Supports #RRGGBB format and falls back to white on parse errors
fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        return Color::White;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);

    Color::TrueColor { r, g, b }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_hex_to_color() {
        let color = hex_to_color("#ff0000");
        match color {
            Color::TrueColor { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 0);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected TrueColor"),
        }
    }

    #[test]
    fn test_theme_parsing() {
        let toml = "
name = \"test\"
description = \"Test theme\"
foreground = \"#ffffff\"
background = \"#000000\"
accent = \"#ff0000\"
strong = \"#00ff00\"
dim = \"#0000ff\"
ascii_art_style = \"bold\"
";

        let theme: Theme = toml::from_str(toml).unwrap();
        assert_eq!(theme.name, "test");
        assert_eq!(theme.ascii_art_style, "bold");
    }

    #[test]
    fn test_path_traversal_prevention() {
        let temp_dir = TempDir::new().unwrap();
        let themes_dir = temp_dir.path().join("themes");
        fs::create_dir(&themes_dir).unwrap();

        let manager = ThemeManager::new(themes_dir).unwrap();
        
        // Should fail on path traversal attempts
        assert!(manager.load_theme("../evil").is_err());
        assert!(manager.load_theme("subdir/theme").is_err());
    }
}

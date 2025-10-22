use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub default_text: String,
    pub default_theme: String,
    pub default_font: String,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    pub last_shown: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
}

fn default_font_size() -> f32 {
    1.0
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            default_text: "Hello, world!".to_string(),
            default_theme: "onedark".to_string(),
            default_font: "standard".to_string(),
            font_size: 1.0,
            last_shown: Some("Hello, world!".to_string()),
            last_updated: Some(Utc::now()),
        }
    }
}

pub struct ConfigManager {
    config_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_dir = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config).join("greetme")
        } else {
            dirs::home_dir()
                .context("Could not determine home directory")?
                .join(".config")
                .join("greetme")
        };

        Ok(Self { config_dir })
    }

    pub fn config_path(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    pub fn themes_dir(&self) -> PathBuf {
        self.config_dir.join("themes")
    }

    pub fn fonts_dir(&self) -> PathBuf {
        self.config_dir.join("fonts")
    }

    pub fn config_exists(&self) -> bool {
        self.config_path().exists()
    }

    pub fn load_config(&self) -> Result<Config> {
        let content = fs::read_to_string(self.config_path())
            .context("Failed to read config file")?;
        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn save_config(&self, config: &Config) -> Result<()> {
        let content = toml::to_string_pretty(config)
            .context("Failed to serialize config")?;
        fs::write(self.config_path(), content)
            .context("Failed to write config file")?;
        Ok(())
    }

    pub fn create_default_config(&self) -> Result<()> {
        // Create config directory
        create_dir_with_perms(&self.config_dir)?;
        create_dir_with_perms(&self.themes_dir())?;
        create_dir_with_perms(&self.fonts_dir())?;

        // Create default config
        let config = Config::default();
        self.save_config(&config)?;

        // Create default themes
        self.create_default_themes()?;

        // Create default fonts
        self.create_default_fonts()?;

        Ok(())
    }

    fn create_default_themes(&self) -> Result<()> {
        let themes = vec![
            ("onedark", include_str!("../themes/onedark.toml")),
            ("solarized", include_str!("../themes/solarized.toml")),
            ("dracula", include_str!("../themes/dracula.toml")),
            ("gruvbox", include_str!("../themes/gruvbox.toml")),
            ("monokai", include_str!("../themes/monokai.toml")),
            ("catppuccin-latte", include_str!("../themes/catppuccin-latte.toml")),
            ("catppuccin-frappe", include_str!("../themes/catppuccin-frappe.toml")),
            ("catppuccin-macchiato", include_str!("../themes/catppuccin-macchiato.toml")),
            ("catppuccin-mocha", include_str!("../themes/catppuccin-mocha.toml")),
        ];

        for (name, content) in themes {
            let path = self.themes_dir().join(format!("{}.toml", name));
            fs::write(&path, content)
                .context(format!("Failed to create theme: {}", name))?;
            set_file_perms(&path)?;
        }

        Ok(())
    }

    fn create_default_fonts(&self) -> Result<()> {
        let fonts = vec![
            ("standard", "Standard FIGlet font"),
            ("big", "Big FIGlet font"),
            ("small", "Small FIGlet font"),
            ("banner", "Banner FIGlet font"),
            ("block", "Block FIGlet font"),
        ];

        for (name, description) in fonts {
            let path = self.fonts_dir().join(format!("{}.txt", name));
            fs::write(&path, description)
                .context(format!("Failed to create font: {}", name))?;
            set_file_perms(&path)?;
        }

        Ok(())
    }
}

fn create_dir_with_perms(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::Permissions::from_mode(0o700);
            fs::set_permissions(path, perms)?;
        }
    }
    Ok(())
}

fn set_file_perms(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o600);
        fs::set_permissions(path, perms)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_config_manager() -> (ConfigManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("greetme");
        let manager = ConfigManager { config_dir };
        (manager, temp_dir)
    }

    #[test]
    fn test_config_roundtrip() {
        let (manager, _temp) = create_test_config_manager();
        manager.create_default_config().unwrap();

        let config = manager.load_config().unwrap();
        assert_eq!(config.version, "1.0");
        assert_eq!(config.default_theme, "onedark");

        let mut config = config;
        config.default_theme = "dracula".to_string();
        manager.save_config(&config).unwrap();

        let loaded = manager.load_config().unwrap();
        assert_eq!(loaded.default_theme, "dracula");
    }

    #[test]
    fn test_default_themes_created() {
        let (manager, _temp) = create_test_config_manager();
        manager.create_default_config().unwrap();

        let themes = ["onedark", "solarized", "dracula", "gruvbox", "monokai"];
        for theme in &themes {
            let path = manager.themes_dir().join(format!("{}.toml", theme));
            assert!(path.exists(), "Theme {} should exist", theme);
        }
    }
}

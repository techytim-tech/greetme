use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::io::{self, IsTerminal};

use crate::config::ConfigManager;
use crate::render::Renderer;
use crate::themes::ThemeManager;

#[derive(Parser, Debug)]
#[command(name = "greetme")]
#[command(version = "1.0.0")]
#[command(about = "A fast, themeable terminal greeting application", long_about = None)]
pub struct Args {
    /// Text to display as greeting
    #[arg(short = 't', long = "text")]
    pub text: Option<String>,

    /// Read and display stored greeting from config
    #[arg(short = 'r', long = "read")]
    pub read: bool,

    /// Create default config and themes
    #[arg(short = 'c', long = "create-config")]
    pub create_config: bool,

    /// Set the active theme
    #[arg(short = 's', long = "set-theme")]
    pub set_theme: Option<String>,

    /// List available themes
    #[arg(long = "list-themes")]
    pub list_themes: bool,

    /// Use specified font for this run
    #[arg(long = "font")]
    pub font: Option<String>,

    /// Set font size multiplier (1.0 = normal, 2.0 = double, etc.)
    #[arg(long = "size", default_value = "1.0")]
    pub size: f32,

    /// Save current settings to config (use with -t)
    #[arg(long = "save")]
    pub save: bool,

    /// Preview a theme
    #[arg(long = "preview-theme")]
    pub preview_theme: Option<String>,

    /// Force operation even if not a TTY
    #[arg(long = "force")]
    pub force: bool,

    /// Enable verbose output
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}

pub fn execute(args: Args) -> Result<()> {
    let config_manager = ConfigManager::new()?;

    // Handle create-config
    if args.create_config {
        return handle_create_config(&config_manager, args.force);
    }

    // Handle list-themes
    if args.list_themes {
        return handle_list_themes(&config_manager);
    }

    // Handle preview-theme
    if let Some(theme_name) = args.preview_theme {
        return handle_preview_theme(&config_manager, &theme_name);
    }

    // Handle set-theme
    if let Some(theme_name) = args.set_theme {
        return handle_set_theme(&config_manager, &theme_name);
    }

    // Handle text display
    if let Some(text) = args.text {
        return handle_display_text(&config_manager, &text, args.font, args.size, args.save, args.force);
    }

    // Handle read
    if args.read {
        return handle_read(&config_manager, args.size, args.force);
    }

    // No command specified
    println!("No command specified. Use --help for usage information.");
    Ok(())
}

fn handle_create_config(config_manager: &ConfigManager, force: bool) -> Result<()> {
    if config_manager.config_exists() && !force {
        return Err(anyhow!(
            "Config already exists at {}. Use --force to overwrite.",
            config_manager.config_path().display()
        ));
    }

    config_manager.create_default_config()?;
    println!(
        "✓ Created default configuration at {}",
        config_manager.config_path().display()
    );
    println!(
        "✓ Created themes directory at {}",
        config_manager.themes_dir().display()
    );
    println!(
        "✓ Created fonts directory at {}",
        config_manager.fonts_dir().display()
    );
    Ok(())
}

fn handle_list_themes(config_manager: &ConfigManager) -> Result<()> {
    let config = config_manager
        .load_config()
        .context("Failed to load config. Run 'greetme -c' to create default config.")?;
    let theme_manager = ThemeManager::new(config_manager.themes_dir())?;
    let themes = theme_manager.list_themes()?;

    if themes.is_empty() {
        println!("No themes found. Run 'greetme -c' to create default themes.");
        return Ok(());
    }

    println!("Available themes:");
    for theme_name in themes {
        if theme_name == config.default_theme {
            println!("  * {} (active)", theme_name);
        } else {
            println!("    {}", theme_name);
        }
    }
    Ok(())
}

fn handle_preview_theme(config_manager: &ConfigManager, theme_name: &str) -> Result<()> {
    let theme_manager = ThemeManager::new(config_manager.themes_dir())?;
    let theme = theme_manager
        .load_theme(theme_name)
        .context(format!("Failed to load theme '{}'", theme_name))?;

    let config = config_manager.load_config().unwrap_or_default();
    let renderer = Renderer::new(theme, &config.default_font, config.font_size);

    println!("Preview of theme '{}':", theme_name);
    renderer.render("PREVIEW")?;
    Ok(())
}

fn handle_set_theme(config_manager: &ConfigManager, theme_name: &str) -> Result<()> {
    // Verify theme exists
    let theme_manager = ThemeManager::new(config_manager.themes_dir())?;
    theme_manager
        .load_theme(theme_name)
        .context(format!("Theme '{}' not found", theme_name))?;

    // Load and update config
    let mut config = config_manager
        .load_config()
        .context("Failed to load config. Run 'greetme -c' to create default config.")?;
    config.default_theme = theme_name.to_string();
    config_manager.save_config(&config)?;

    println!("✓ Set active theme to '{}'", theme_name);
    Ok(())
}

fn handle_display_text(
    config_manager: &ConfigManager,
    text: &str,
    font: Option<String>,
    size: f32,
    save: bool,
    force: bool,
) -> Result<()> {
    // Check if stdout is a TTY
    if !force && !io::stdout().is_terminal() {
        return Err(anyhow!(
            "Output is not a terminal. Use --force to print anyway."
        ));
    }

    // Validate size
    if size <= 0.0 || size > 10.0 {
        return Err(anyhow!("Font size must be between 0.1 and 10.0"));
    }

    let mut config = config_manager
        .load_config()
        .context("Failed to load config. Run 'greetme -c' to create default config.")?;

    let theme_manager = ThemeManager::new(config_manager.themes_dir())?;
    let theme = theme_manager.load_theme(&config.default_theme)?;

    let font_name = font.as_ref().unwrap_or(&config.default_font);
    let renderer = Renderer::new(theme, font_name, size);

    renderer.render(text)?;

    // Save if requested
    if save {
        config.last_shown = Some(text.to_string());
        config.last_updated = Some(chrono::Utc::now());
        config.font_size = size;
        if let Some(f) = font {
            config.default_font = f;
        }
        config_manager.save_config(&config)?;
    }

    Ok(())
}

fn handle_read(config_manager: &ConfigManager, size: f32, force: bool) -> Result<()> {
    // Check if stdout is a TTY
    if !force && !io::stdout().is_terminal() {
        return Err(anyhow!(
            "Output is not a terminal. Use --force to print anyway."
        ));
    }

    let config = config_manager
        .load_config()
        .context("Failed to load config. Run 'greetme -c' to create default config.")?;

    let text = config.last_shown.as_ref().ok_or_else(|| {
        anyhow!("No greeting stored in config. Use 'greetme -t \"Your text\" --save' to store one.")
    })?;

    let theme_manager = ThemeManager::new(config_manager.themes_dir())?;
    let theme = theme_manager.load_theme(&config.default_theme)?;
    
    // Use provided size or config size
    let font_size = if size != 1.0 { size } else { config.font_size };
    let renderer = Renderer::new(theme, &config.default_font, font_size);

    renderer.render(text)?;
    Ok(())
}

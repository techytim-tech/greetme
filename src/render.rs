use anyhow::Result;
use colored::Colorize;
use figlet_rs::FIGfont;

use crate::themes::Theme;

pub struct Renderer {
    theme: Theme,
    font_name: String,
    size: f32,
}

impl Renderer {
    pub fn new(theme: Theme, font_name: &str, size: f32) -> Self {
        Self {
            theme,
            font_name: font_name.to_string(),
            size,
        }
    }

    pub fn render(&self, text: &str) -> Result<()> {
        let output = self.render_figlet(text)?;
        
        // Apply size scaling if needed
        let scaled_output = if self.size != 1.0 {
            self.scale_text(&output, self.size)
        } else {
            output
        };
        
        // Apply colors based on theme
        let colored_output = match self.theme.ascii_art_style.as_str() {
            "bold" => scaled_output.color(self.theme.get_strong_color()).bold().to_string(),
            "italic" => scaled_output.color(self.theme.get_accent_color()).italic().to_string(),
            "normal" | _ => scaled_output.color(self.theme.get_foreground_color()).to_string(),
        };

        println!("{}", colored_output);
        Ok(())
    }

    fn scale_text(&self, text: &str, scale: f32) -> String {
        if scale <= 1.0 {
            return text.to_string();
        }
        
        let lines: Vec<&str> = text.lines().collect();
        let mut scaled_lines = Vec::new();
        
        // Vertical scaling - repeat each line
        let vertical_repeat = scale.round() as usize;
        for line in lines {
            // Horizontal scaling - repeat each character
            let scaled_line = if scale >= 2.0 {
                line.chars()
                    .map(|c| c.to_string().repeat(scale.round() as usize))
                    .collect::<String>()
            } else {
                line.to_string()
            };
            
            // Repeat line vertically
            for _ in 0..vertical_repeat {
                scaled_lines.push(scaled_line.clone());
            }
        }
        
        scaled_lines.join("\n")
    }

    fn render_figlet(&self, text: &str) -> Result<String> {
        // Try to load the specified font, fall back to standard
        let font_result = match self.font_name.as_str() {
            "standard" => FIGfont::standard(),
            "small" => FIGfont::from_content(include_str!("../fonts/small.flf")),
            "big" => FIGfont::from_content(include_str!("../fonts/big.flf")),
            "banner" => FIGfont::from_content(include_str!("../fonts/banner.flf")),
            "block" => FIGfont::from_content(include_str!("../fonts/block.flf")),
            _ => FIGfont::standard(),
        };

        let font = font_result.unwrap_or_else(|_| {
            // If all else fails, try to create a minimal font
            FIGfont::from_content("").unwrap_or_else(|_| {
                // This should never happen, but if it does, we'll panic
                panic!("Failed to load any font")
            })
        });

        let figure = font.convert(text);
        match figure {
            Some(fig) => Ok(fig.to_string()),
            None => Ok(text.to_string()), // Fallback to plain text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_theme() -> Theme {
        Theme {
            name: "test".to_string(),
            description: "Test theme".to_string(),
            foreground: "#ffffff".to_string(),
            background: "#000000".to_string(),
            accent: "#ff0000".to_string(),
            strong: "#00ff00".to_string(),
            dim: "#0000ff".to_string(),
            ascii_art_style: "bold".to_string(),
        }
    }

    #[test]
    fn test_renderer_creation() {
        let theme = create_test_theme();
        let renderer = Renderer::new(theme, "standard", 1.0);
        assert_eq!(renderer.font_name, "standard");
    }

    #[test]
    fn test_render_figlet() {
        let theme = create_test_theme();
        let renderer = Renderer::new(theme, "standard", 1.0);
        let result = renderer.render_figlet("TEST");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }
}

use std::{thread, time::Duration};
use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    ExecutableCommand,
    QueueableCommand,
    style::{Color, SetForegroundColor, Print, ResetColor},
};
use std::io::{stdout, Write};
use rand::Rng;

pub struct AnimationConfig {
    pub speed: u64,
    pub style: AnimationStyle,
    pub color_mode: ColorMode,
}

pub enum AnimationStyle {
    Type,
    Fade,
    Rainbow,
    Matrix,
    None,
}

pub enum ColorMode {
    Gradient(Vec<Color>),
    Rainbow,
    Static(Color),
    None,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        AnimationConfig {
            speed: 100,
            style: AnimationStyle::None,
            color_mode: ColorMode::None,
        }
    }
}

pub struct TextAnimator {
    config: AnimationConfig,
}

impl TextAnimator {
    pub fn new(config: AnimationConfig) -> Self {
        TextAnimator { config }
    }

    pub fn animate(&self, text: &str) -> std::io::Result<()> {
        match self.config.style {
            AnimationStyle::Type => self.type_animation(text),
            AnimationStyle::Fade => self.fade_animation(text),
            AnimationStyle::Rainbow => self.rainbow_animation(text),
            AnimationStyle::Matrix => self.matrix_animation(text),
            AnimationStyle::None => {
                print!("{}", text);
                Ok(())
            }
        }
    }

    fn type_animation(&self, text: &str) -> std::io::Result<()> {
        let mut stdout = stdout();
        for c in text.chars() {
            stdout.queue(Print(c))?;
            stdout.flush()?;
            thread::sleep(Duration::from_millis(self.config.speed));
        }
        Ok(())
    }

    fn fade_animation(&self, text: &str) -> std::io::Result<()> {
        let mut stdout = stdout();
        let colors = [
            Color::Rgb { r: 128, g: 128, b: 128 },
            Color::Rgb { r: 160, g: 160, b: 160 },
            Color::Rgb { r: 192, g: 192, b: 192 },
            Color::Rgb { r: 224, g: 224, b: 224 },
            Color::Rgb { r: 255, g: 255, b: 255 },
        ];

        for color in colors.iter() {
            stdout.queue(cursor::SavePosition)?
                .queue(SetForegroundColor(*color))?
                .queue(Print(text))?
                .queue(cursor::RestorePosition)?;
            stdout.flush()?;
            thread::sleep(Duration::from_millis(self.config.speed));
        }
        
        stdout.queue(ResetColor)?;
        stdout.queue(Print(text))?;
        stdout.flush()?;
        Ok(())
    }

    fn rainbow_animation(&self, text: &str) -> std::io::Result<()> {
        let mut stdout = stdout();
        let colors = [
            Color::Rgb { r: 255, g: 0, b: 0 },    // Red
            Color::Rgb { r: 255, g: 127, b: 0 },  // Orange
            Color::Rgb { r: 255, g: 255, b: 0 },  // Yellow
            Color::Rgb { r: 0, g: 255, b: 0 },    // Green
            Color::Rgb { r: 0, g: 0, b: 255 },    // Blue
            Color::Rgb { r: 75, g: 0, b: 130 },   // Indigo
            Color::Rgb { r: 148, g: 0, b: 211 },  // Violet
        ];

        for _ in 0..50 {
            for (i, c) in text.chars().enumerate() {
                let color = colors[i % colors.len()];
                stdout.queue(cursor::SavePosition)?
                    .queue(SetForegroundColor(color))?
                    .queue(Print(c))?;
            }
            stdout.flush()?;
            thread::sleep(Duration::from_millis(self.config.speed));
            stdout.queue(cursor::RestorePosition)?;
            
            // Rotate colors
            let first = colors[0];
            for i in 0..colors.len()-1 {
                colors[i] = colors[i+1];
            }
            colors[colors.len()-1] = first;
        }
        
        stdout.queue(ResetColor)?;
        stdout.queue(Print(text))?;
        stdout.flush()?;
        Ok(())
    }

    fn matrix_animation(&self, text: &str) -> std::io::Result<()> {
        let mut stdout = stdout();
        let mut rng = rand::thread_rng();
        let chars = "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ".chars().collect::<Vec<_>>();
        
        for _ in 0..15 {
            stdout.queue(cursor::SavePosition)?;
            for c in text.chars() {
                if rng.gen::<f32>() < 0.5 {
                    stdout.queue(SetForegroundColor(Color::Rgb { r: 0, g: 255, b: 0 }))?
                        .queue(Print(chars[rng.gen_range(0..chars.len())]))?;
                } else {
                    stdout.queue(SetForegroundColor(Color::Rgb { r: 0, g: 200, b: 0 }))?
                        .queue(Print(c))?;
                }
            }
            stdout.flush()?;
            thread::sleep(Duration::from_millis(self.config.speed));
            stdout.queue(cursor::RestorePosition)?;
        }
        
        stdout.queue(SetForegroundColor(Color::Rgb { r: 0, g: 255, b: 0 }))?
            .queue(Print(text))?
            .queue(ResetColor)?;
        stdout.flush()?;
        Ok(())
    }
}
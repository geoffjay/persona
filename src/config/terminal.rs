use gpui::{px, Edges};
use gpui_terminal::{ColorPalette, TerminalConfig as GpuiTerminalConfig};
use serde::{Deserialize, Serialize};

/// Application terminal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TerminalConfig {
    /// Font family name
    pub font_family: String,

    /// Font size in points
    pub font_size: f32,

    /// Maximum scrollback lines
    pub scrollback: usize,

    /// Line height multiplier
    pub line_height: f32,

    /// Padding around terminal content
    pub padding: f32,

    /// Color theme name or inline theme
    #[serde(default)]
    pub theme: TerminalThemeConfig,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            font_family: "monospace".into(),
            font_size: 14.0,
            scrollback: 10000,
            line_height: 1.2,
            padding: 8.0,
            theme: TerminalThemeConfig::default(),
        }
    }
}

impl TerminalConfig {
    /// Convert to gpui-terminal's TerminalConfig
    pub fn to_terminal_config(&self, cols: usize, rows: usize) -> GpuiTerminalConfig {
        GpuiTerminalConfig {
            cols,
            rows,
            font_family: self.font_family.clone(),
            font_size: px(self.font_size),
            scrollback: self.scrollback,
            line_height_multiplier: self.line_height,
            padding: Edges::all(px(self.padding)),
            colors: self.theme.to_color_palette(),
        }
    }
}

/// Terminal color theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TerminalThemeConfig {
    /// Use a named built-in theme
    Named(String),
    /// Use a custom inline theme
    Custom(TerminalTheme),
}

impl Default for TerminalThemeConfig {
    fn default() -> Self {
        Self::Named("tokyo-night".into())
    }
}

impl TerminalThemeConfig {
    pub fn to_color_palette(&self) -> ColorPalette {
        match self {
            Self::Named(name) => TerminalTheme::from_name(name).to_color_palette(),
            Self::Custom(theme) => theme.to_color_palette(),
        }
    }
}

/// RGB color as hex string or array
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Hex(String),
    Rgb([u8; 3]),
}

impl Color {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Self::Hex(hex) => {
                let hex = hex.trim_start_matches('#');
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                (r, g, b)
            }
            Self::Rgb([r, g, b]) => (*r, *g, *b),
        }
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Self::Hex(s.to_string())
    }
}

/// Terminal color theme
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TerminalTheme {
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,

    // Normal colors
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,

    // Bright colors
    pub bright_black: Color,
    pub bright_red: Color,
    pub bright_green: Color,
    pub bright_yellow: Color,
    pub bright_blue: Color,
    pub bright_magenta: Color,
    pub bright_cyan: Color,
    pub bright_white: Color,
}

impl Default for TerminalTheme {
    fn default() -> Self {
        Self::tokyo_night()
    }
}

impl TerminalTheme {
    /// Get a theme by name, falling back to Tokyo Night
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "tokyo-night" | "tokyo_night" | "tokyonight" => Self::tokyo_night(),
            "gruvbox" | "gruvbox-dark" => Self::gruvbox_dark(),
            "catppuccin" | "catppuccin-mocha" => Self::catppuccin_mocha(),
            _ => Self::tokyo_night(),
        }
    }

    /// Tokyo Night dark theme
    pub fn tokyo_night() -> Self {
        Self {
            background: "#1a1b26".into(),
            foreground: "#c0caf5".into(),
            cursor: "#c0caf5".into(),

            // Normal colors
            black: "#15161e".into(),
            red: "#f7768e".into(),
            green: "#9ece6a".into(),
            yellow: "#e0af68".into(),
            blue: "#7aa2f7".into(),
            magenta: "#bb9af7".into(),
            cyan: "#7dcfff".into(),
            white: "#a9b1d6".into(),

            // Bright colors
            bright_black: "#414868".into(),
            bright_red: "#f7768e".into(),
            bright_green: "#9ece6a".into(),
            bright_yellow: "#e0af68".into(),
            bright_blue: "#7aa2f7".into(),
            bright_magenta: "#bb9af7".into(),
            bright_cyan: "#7dcfff".into(),
            bright_white: "#c0caf5".into(),
        }
    }

    /// Gruvbox dark theme
    pub fn gruvbox_dark() -> Self {
        Self {
            background: "#282828".into(),
            foreground: "#ebdbb2".into(),
            cursor: "#ebdbb2".into(),

            black: "#282828".into(),
            red: "#cc241d".into(),
            green: "#98971a".into(),
            yellow: "#d79921".into(),
            blue: "#458588".into(),
            magenta: "#b16286".into(),
            cyan: "#689d6a".into(),
            white: "#a89984".into(),

            bright_black: "#928374".into(),
            bright_red: "#fb4934".into(),
            bright_green: "#b8bb26".into(),
            bright_yellow: "#fabd2f".into(),
            bright_blue: "#83a598".into(),
            bright_magenta: "#d3869b".into(),
            bright_cyan: "#8ec07c".into(),
            bright_white: "#ebdbb2".into(),
        }
    }

    /// Catppuccin Mocha theme
    pub fn catppuccin_mocha() -> Self {
        Self {
            background: "#1e1e2e".into(),
            foreground: "#cdd6f4".into(),
            cursor: "#f5e0dc".into(),

            black: "#45475a".into(),
            red: "#f38ba8".into(),
            green: "#a6e3a1".into(),
            yellow: "#f9e2af".into(),
            blue: "#89b4fa".into(),
            magenta: "#f5c2e7".into(),
            cyan: "#94e2d5".into(),
            white: "#bac2de".into(),

            bright_black: "#585b70".into(),
            bright_red: "#f38ba8".into(),
            bright_green: "#a6e3a1".into(),
            bright_yellow: "#f9e2af".into(),
            bright_blue: "#89b4fa".into(),
            bright_magenta: "#f5c2e7".into(),
            bright_cyan: "#94e2d5".into(),
            bright_white: "#a6adc8".into(),
        }
    }

    /// Convert to gpui-terminal's ColorPalette
    pub fn to_color_palette(&self) -> ColorPalette {
        let (bg_r, bg_g, bg_b) = self.background.to_rgb();
        let (fg_r, fg_g, fg_b) = self.foreground.to_rgb();
        let (cur_r, cur_g, cur_b) = self.cursor.to_rgb();

        let (black_r, black_g, black_b) = self.black.to_rgb();
        let (red_r, red_g, red_b) = self.red.to_rgb();
        let (green_r, green_g, green_b) = self.green.to_rgb();
        let (yellow_r, yellow_g, yellow_b) = self.yellow.to_rgb();
        let (blue_r, blue_g, blue_b) = self.blue.to_rgb();
        let (magenta_r, magenta_g, magenta_b) = self.magenta.to_rgb();
        let (cyan_r, cyan_g, cyan_b) = self.cyan.to_rgb();
        let (white_r, white_g, white_b) = self.white.to_rgb();

        let (br_black_r, br_black_g, br_black_b) = self.bright_black.to_rgb();
        let (br_red_r, br_red_g, br_red_b) = self.bright_red.to_rgb();
        let (br_green_r, br_green_g, br_green_b) = self.bright_green.to_rgb();
        let (br_yellow_r, br_yellow_g, br_yellow_b) = self.bright_yellow.to_rgb();
        let (br_blue_r, br_blue_g, br_blue_b) = self.bright_blue.to_rgb();
        let (br_magenta_r, br_magenta_g, br_magenta_b) = self.bright_magenta.to_rgb();
        let (br_cyan_r, br_cyan_g, br_cyan_b) = self.bright_cyan.to_rgb();
        let (br_white_r, br_white_g, br_white_b) = self.bright_white.to_rgb();

        ColorPalette::builder()
            .background(bg_r, bg_g, bg_b)
            .foreground(fg_r, fg_g, fg_b)
            .cursor(cur_r, cur_g, cur_b)
            .black(black_r, black_g, black_b)
            .red(red_r, red_g, red_b)
            .green(green_r, green_g, green_b)
            .yellow(yellow_r, yellow_g, yellow_b)
            .blue(blue_r, blue_g, blue_b)
            .magenta(magenta_r, magenta_g, magenta_b)
            .cyan(cyan_r, cyan_g, cyan_b)
            .white(white_r, white_g, white_b)
            .bright_black(br_black_r, br_black_g, br_black_b)
            .bright_red(br_red_r, br_red_g, br_red_b)
            .bright_green(br_green_r, br_green_g, br_green_b)
            .bright_yellow(br_yellow_r, br_yellow_g, br_yellow_b)
            .bright_blue(br_blue_r, br_blue_g, br_blue_b)
            .bright_magenta(br_magenta_r, br_magenta_g, br_magenta_b)
            .bright_cyan(br_cyan_r, br_cyan_g, br_cyan_b)
            .bright_white(br_white_r, br_white_g, br_white_b)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod color_tests {
        use super::*;

        #[test]
        fn test_hex_color_parsing() {
            let color = Color::Hex("#ff5500".to_string());
            assert_eq!(color.to_rgb(), (255, 85, 0));
        }

        #[test]
        fn test_hex_color_without_hash() {
            let color = Color::Hex("1a1b26".to_string());
            assert_eq!(color.to_rgb(), (26, 27, 38));
        }

        #[test]
        fn test_rgb_array_color() {
            let color = Color::Rgb([128, 64, 32]);
            assert_eq!(color.to_rgb(), (128, 64, 32));
        }

        #[test]
        fn test_color_from_str() {
            let color: Color = "#c0caf5".into();
            assert_eq!(color.to_rgb(), (192, 202, 245));
        }

        #[test]
        fn test_color_serialization_in_theme() {
            // Colors serialize correctly when inside a struct (not standalone)
            let theme = TerminalTheme::tokyo_night();
            let toml_str = toml::to_string(&theme).expect("Failed to serialize");
            // Background color should be present
            assert!(toml_str.contains("1a1b26"));
        }

        #[test]
        fn test_color_deserialization_hex() {
            // Test that hex colors deserialize correctly
            let toml_str = r##"background = "#ff0000""##;

            #[derive(Deserialize)]
            struct TestStruct {
                background: Color,
            }

            let parsed: TestStruct = toml::from_str(toml_str).expect("Failed to deserialize");
            assert_eq!(parsed.background.to_rgb(), (255, 0, 0));
        }

        #[test]
        fn test_color_deserialization_rgb() {
            // Test that RGB arrays deserialize correctly
            let toml_str = "background = [255, 128, 64]";

            #[derive(Deserialize)]
            struct TestStruct {
                background: Color,
            }

            let parsed: TestStruct = toml::from_str(toml_str).expect("Failed to deserialize");
            assert_eq!(parsed.background.to_rgb(), (255, 128, 64));
        }
    }

    mod terminal_config_tests {
        use super::*;

        #[test]
        fn test_default_values() {
            let config = TerminalConfig::default();
            assert_eq!(config.font_family, "monospace");
            assert_eq!(config.font_size, 14.0);
            assert_eq!(config.scrollback, 10000);
            assert_eq!(config.line_height, 1.2);
            assert_eq!(config.padding, 8.0);
        }

        #[test]
        fn test_serialization_roundtrip() {
            let config = TerminalConfig {
                font_family: "JetBrains Mono".to_string(),
                font_size: 16.0,
                scrollback: 5000,
                line_height: 1.5,
                padding: 12.0,
                theme: TerminalThemeConfig::Named("gruvbox".to_string()),
            };

            let toml_str = toml::to_string(&config).expect("Failed to serialize");
            let parsed: TerminalConfig = toml::from_str(&toml_str).expect("Failed to deserialize");

            assert_eq!(parsed.font_family, config.font_family);
            assert_eq!(parsed.font_size, config.font_size);
            assert_eq!(parsed.scrollback, config.scrollback);
            assert_eq!(parsed.line_height, config.line_height);
            assert_eq!(parsed.padding, config.padding);
        }

        #[test]
        fn test_deserialize_from_toml() {
            let toml_str = r#"
                font_family = "Fira Code"
                font_size = 18.0
                scrollback = 20000
                line_height = 1.3
                padding = 10.0
                theme = "catppuccin"
            "#;

            let config: TerminalConfig = toml::from_str(toml_str).expect("Failed to deserialize");
            assert_eq!(config.font_family, "Fira Code");
            assert_eq!(config.font_size, 18.0);
            assert_eq!(config.scrollback, 20000);
            assert_eq!(config.line_height, 1.3);
            assert_eq!(config.padding, 10.0);
        }

        #[test]
        fn test_partial_config_uses_defaults() {
            let toml_str = r#"
                font_size = 20.0
            "#;

            let config: TerminalConfig = toml::from_str(toml_str).expect("Failed to deserialize");
            assert_eq!(config.font_size, 20.0);
            // Other fields should use defaults
            assert_eq!(config.font_family, "monospace");
            assert_eq!(config.scrollback, 10000);
        }
    }

    mod theme_config_tests {
        use super::*;

        #[test]
        fn test_default_is_tokyo_night() {
            let config = TerminalThemeConfig::default();
            match config {
                TerminalThemeConfig::Named(name) => assert_eq!(name, "tokyo-night"),
                _ => panic!("Expected Named variant"),
            }
        }

        #[test]
        fn test_named_theme_in_terminal_config() {
            // Theme serializes correctly when inside TerminalConfig
            let config = TerminalConfig {
                theme: TerminalThemeConfig::Named("gruvbox".to_string()),
                ..Default::default()
            };
            let toml_str = toml::to_string(&config).expect("Failed to serialize");
            // Named themes serialize as simple strings
            assert!(toml_str.contains("gruvbox"));
        }

        #[test]
        fn test_named_theme_deserialization() {
            let toml_str = r#"theme = "catppuccin""#;

            #[derive(Deserialize)]
            struct TestStruct {
                theme: TerminalThemeConfig,
            }

            let parsed: TestStruct = toml::from_str(toml_str).expect("Failed to deserialize");
            match parsed.theme {
                TerminalThemeConfig::Named(name) => assert_eq!(name, "catppuccin"),
                _ => panic!("Expected Named variant"),
            }
        }
    }

    mod terminal_theme_tests {
        use super::*;

        #[test]
        fn test_tokyo_night_colors() {
            let theme = TerminalTheme::tokyo_night();
            assert_eq!(theme.background.to_rgb(), (26, 27, 38)); // #1a1b26
            assert_eq!(theme.foreground.to_rgb(), (192, 202, 245)); // #c0caf5
        }

        #[test]
        fn test_gruvbox_colors() {
            let theme = TerminalTheme::gruvbox_dark();
            assert_eq!(theme.background.to_rgb(), (40, 40, 40)); // #282828
            assert_eq!(theme.foreground.to_rgb(), (235, 219, 178)); // #ebdbb2
        }

        #[test]
        fn test_catppuccin_colors() {
            let theme = TerminalTheme::catppuccin_mocha();
            assert_eq!(theme.background.to_rgb(), (30, 30, 46)); // #1e1e2e
            assert_eq!(theme.foreground.to_rgb(), (205, 214, 244)); // #cdd6f4
        }

        #[test]
        fn test_from_name_variations() {
            // Tokyo Night variations
            let _ = TerminalTheme::from_name("tokyo-night");
            let _ = TerminalTheme::from_name("tokyo_night");
            let _ = TerminalTheme::from_name("tokyonight");
            let _ = TerminalTheme::from_name("TOKYO-NIGHT"); // case insensitive

            // Gruvbox variations
            let _ = TerminalTheme::from_name("gruvbox");
            let _ = TerminalTheme::from_name("gruvbox-dark");

            // Catppuccin variations
            let _ = TerminalTheme::from_name("catppuccin");
            let _ = TerminalTheme::from_name("catppuccin-mocha");
        }

        #[test]
        fn test_unknown_theme_falls_back_to_tokyo_night() {
            let theme = TerminalTheme::from_name("unknown-theme");
            let tokyo = TerminalTheme::tokyo_night();
            assert_eq!(theme.background.to_rgb(), tokyo.background.to_rgb());
        }

        #[test]
        fn test_to_color_palette_does_not_panic() {
            // Just ensure the conversion doesn't panic
            let _ = TerminalTheme::tokyo_night().to_color_palette();
            let _ = TerminalTheme::gruvbox_dark().to_color_palette();
            let _ = TerminalTheme::catppuccin_mocha().to_color_palette();
        }
    }
}

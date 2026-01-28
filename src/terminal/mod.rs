mod theme;

pub use theme::TerminalThemeConfig;

use gpui::{px, Edges};
use gpui_terminal::TerminalConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application terminal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppTerminalConfig {
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

impl Default for AppTerminalConfig {
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

impl AppTerminalConfig {
    /// Load configuration from the default config file location
    pub fn load() -> Self {
        Self::config_path()
            .and_then(|path| {
                if path.exists() {
                    std::fs::read_to_string(&path).ok()
                } else {
                    None
                }
            })
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_default()
    }

    /// Get the default config file path
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("persona-ui").join("terminal.toml"))
    }

    /// Convert to gpui-terminal's TerminalConfig
    pub fn to_terminal_config(&self, cols: usize, rows: usize) -> TerminalConfig {
        TerminalConfig {
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

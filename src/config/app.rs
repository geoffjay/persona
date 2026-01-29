use super::terminal::TerminalConfig;
use super::{BerryConfig, PersonasConfig};
use crate::persona::Persona;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main application configuration loaded from TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct AppConfig {
    /// Terminal configuration
    pub terminal: TerminalConfig,

    /// Berry server configuration
    pub berry: BerryConfig,

    /// Personas configuration
    pub personas: PersonasConfig,
}

impl AppConfig {
    /// Load configuration from the default config file location.
    /// Environment variables override file values for specific settings.
    pub fn load() -> Self {
        // First load from TOML file
        let mut config: Self = Self::config_path()
            .and_then(|path| {
                if path.exists() {
                    std::fs::read_to_string(&path).ok()
                } else {
                    None
                }
            })
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_default();

        // Apply environment variable overrides
        if let Ok(url) = std::env::var("BERRY_SERVER_URL") {
            config.berry.server_url = url;
        }

        if let Ok(dir) = std::env::var("PERSONAS_DIR") {
            config.personas.directory = PathBuf::from(dir);
        }

        config
    }

    /// Save the configuration to the default config file location
    pub fn save(&self) -> Result<(), std::io::Error> {
        let Some(config_path) = Self::config_path() else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine config directory",
            ));
        };

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        std::fs::write(&config_path, toml_str)
    }

    /// Get the default config file path
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("persona").join("config.toml"))
    }

    /// Convenience accessor for berry server URL
    pub fn berry_server_url(&self) -> &str {
        &self.berry.server_url
    }

    /// Convenience accessor for personas directory
    pub fn personas_dir(&self) -> &PathBuf {
        &self.personas.directory
    }

    /// Load personas from the configured directory
    pub fn load_personas(&self) -> Vec<Persona> {
        let mut personas = Vec::new();

        if !self.personas.directory.exists() {
            eprintln!(
                "Personas directory not found: {:?}",
                self.personas.directory
            );
            return personas;
        }

        let entries = match std::fs::read_dir(&self.personas.directory) {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Failed to read personas directory: {}", err);
                return personas;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                match Persona::from_file(path.clone()) {
                    Ok(persona) => personas.push(persona),
                    Err(err) => eprintln!("Failed to load persona from {:?}: {}", path, err),
                }
            }
        }

        // Sort by name
        personas.sort_by(|a, b| a.name.cmp(&b.name));
        personas
    }

    /// Load configuration from a specific TOML string (for testing)
    #[cfg(test)]
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// Load configuration from a specific file path (for testing)
    #[cfg(test)]
    pub fn load_from_path(path: &PathBuf) -> Self {
        if path.exists() {
            std::fs::read_to_string(path)
                .ok()
                .and_then(|content| toml::from_str(&content).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    /// Save configuration to a specific file path (for testing)
    #[cfg(test)]
    pub fn save_to_path(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        std::fs::write(path, toml_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    mod default_tests {
        use super::*;

        #[test]
        fn test_default_values() {
            let config = AppConfig::default();

            // Terminal defaults
            assert_eq!(config.terminal.font_family, "monospace");
            assert_eq!(config.terminal.font_size, 14.0);

            // Berry defaults
            assert_eq!(config.berry.server_url, "http://localhost:4114");

            // Personas directory should be set
            assert!(!config.personas.directory.as_os_str().is_empty());
        }

        #[test]
        fn test_convenience_accessors() {
            let config = AppConfig::default();
            assert_eq!(config.berry_server_url(), "http://localhost:4114");
            assert!(!config.personas_dir().as_os_str().is_empty());
        }
    }

    mod serialization_tests {
        use super::*;

        #[test]
        fn test_serialization_roundtrip() {
            let config = AppConfig {
                terminal: TerminalConfig {
                    font_family: "Hack".to_string(),
                    font_size: 16.0,
                    ..Default::default()
                },
                berry: BerryConfig {
                    server_url: "http://custom:8080".to_string(),
                },
                personas: PersonasConfig {
                    directory: PathBuf::from("/custom/path"),
                },
            };

            let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize");
            let parsed: AppConfig = toml::from_str(&toml_str).expect("Failed to deserialize");

            assert_eq!(parsed.terminal.font_family, "Hack");
            assert_eq!(parsed.terminal.font_size, 16.0);
            assert_eq!(parsed.berry.server_url, "http://custom:8080");
            assert_eq!(parsed.personas.directory, PathBuf::from("/custom/path"));
        }

        #[test]
        fn test_deserialize_full_config() {
            let toml_str = r#"
                [terminal]
                font_family = "JetBrains Mono"
                font_size = 18.0
                scrollback = 5000
                line_height = 1.4
                padding = 10.0
                theme = "gruvbox"

                [berry]
                server_url = "https://berry.example.com"

                [personas]
                directory = "/home/user/personas"
            "#;

            let config = AppConfig::from_toml(toml_str).expect("Failed to deserialize");

            assert_eq!(config.terminal.font_family, "JetBrains Mono");
            assert_eq!(config.terminal.font_size, 18.0);
            assert_eq!(config.terminal.scrollback, 5000);
            assert_eq!(config.berry.server_url, "https://berry.example.com");
            assert_eq!(
                config.personas.directory,
                PathBuf::from("/home/user/personas")
            );
        }

        #[test]
        fn test_deserialize_partial_config_uses_defaults() {
            let toml_str = r#"
                [berry]
                server_url = "http://custom:9999"
            "#;

            let config = AppConfig::from_toml(toml_str).expect("Failed to deserialize");

            // Berry should be custom
            assert_eq!(config.berry.server_url, "http://custom:9999");

            // Terminal should use defaults
            assert_eq!(config.terminal.font_family, "monospace");
            assert_eq!(config.terminal.font_size, 14.0);
        }

        #[test]
        fn test_deserialize_empty_uses_all_defaults() {
            let toml_str = "";
            let config = AppConfig::from_toml(toml_str).expect("Failed to deserialize");

            assert_eq!(config.terminal.font_family, "monospace");
            assert_eq!(config.berry.server_url, "http://localhost:4114");
        }
    }

    mod file_io_tests {
        use super::*;

        #[test]
        fn test_save_and_load_from_path() {
            let temp_dir = TempDir::new().expect("Failed to create temp dir");
            let config_path = temp_dir.path().join("config.toml");

            let config = AppConfig {
                terminal: TerminalConfig {
                    font_family: "Test Font".to_string(),
                    font_size: 20.0,
                    ..Default::default()
                },
                berry: BerryConfig {
                    server_url: "http://test:1234".to_string(),
                },
                personas: PersonasConfig {
                    directory: PathBuf::from("/test/personas"),
                },
            };

            config
                .save_to_path(&config_path)
                .expect("Failed to save config");

            let loaded = AppConfig::load_from_path(&config_path);

            assert_eq!(loaded.terminal.font_family, "Test Font");
            assert_eq!(loaded.terminal.font_size, 20.0);
            assert_eq!(loaded.berry.server_url, "http://test:1234");
            assert_eq!(loaded.personas.directory, PathBuf::from("/test/personas"));
        }

        #[test]
        fn test_load_from_nonexistent_path_returns_defaults() {
            let config_path = PathBuf::from("/nonexistent/path/config.toml");
            let config = AppConfig::load_from_path(&config_path);

            assert_eq!(config.terminal.font_family, "monospace");
            assert_eq!(config.berry.server_url, "http://localhost:4114");
        }

        #[test]
        fn test_save_creates_parent_directories() {
            let temp_dir = TempDir::new().expect("Failed to create temp dir");
            let config_path = temp_dir
                .path()
                .join("nested")
                .join("dir")
                .join("config.toml");

            let config = AppConfig::default();
            config
                .save_to_path(&config_path)
                .expect("Failed to save config");

            assert!(config_path.exists());
        }
    }

    mod toml_format_tests {
        use super::*;

        #[test]
        fn test_generated_toml_has_expected_sections() {
            let config = AppConfig::default();
            let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize");

            assert!(toml_str.contains("[terminal]"));
            assert!(toml_str.contains("[berry]"));
            assert!(toml_str.contains("[personas]"));
        }

        #[test]
        fn test_generated_toml_has_expected_keys() {
            let config = AppConfig::default();
            let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize");

            // Terminal keys
            assert!(toml_str.contains("font_family"));
            assert!(toml_str.contains("font_size"));
            assert!(toml_str.contains("scrollback"));
            assert!(toml_str.contains("line_height"));
            assert!(toml_str.contains("padding"));
            assert!(toml_str.contains("theme"));

            // Berry keys
            assert!(toml_str.contains("server_url"));

            // Personas keys
            assert!(toml_str.contains("directory"));
        }
    }
}

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Personas configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PersonasConfig {
    /// Directory containing persona files
    pub directory: PathBuf,
}

impl Default for PersonasConfig {
    fn default() -> Self {
        Self {
            directory: dirs::config_dir()
                .map(|p| p.join("persona").join("personas"))
                .unwrap_or_else(|| PathBuf::from("personas")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_uses_config_dir() {
        let config = PersonasConfig::default();
        // Should end with persona/personas regardless of the base config dir
        let path_str = config.directory.to_string_lossy();
        assert!(
            path_str.ends_with("persona/personas") || path_str.ends_with("persona\\personas"),
            "Expected path to end with persona/personas, got: {}",
            path_str
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let config = PersonasConfig {
            directory: PathBuf::from("/custom/personas/path"),
        };

        let toml_str = toml::to_string(&config).expect("Failed to serialize");
        let parsed: PersonasConfig = toml::from_str(&toml_str).expect("Failed to deserialize");

        assert_eq!(parsed.directory, config.directory);
    }

    #[test]
    fn test_deserialize_from_toml() {
        let toml_str = r#"
            directory = "/my/custom/personas"
        "#;

        let config: PersonasConfig = toml::from_str(toml_str).expect("Failed to deserialize");
        assert_eq!(config.directory, PathBuf::from("/my/custom/personas"));
    }

    #[test]
    fn test_deserialize_empty_uses_defaults() {
        let toml_str = "";
        let config: PersonasConfig = toml::from_str(toml_str).expect("Failed to deserialize");
        // Should use the default path
        let path_str = config.directory.to_string_lossy();
        assert!(
            path_str.ends_with("persona/personas")
                || path_str.ends_with("persona\\personas")
                || path_str == "personas"
        );
    }
}

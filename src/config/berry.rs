use serde::{Deserialize, Serialize};

/// Berry server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BerryConfig {
    /// URL of the Berry server
    pub server_url: String,
}

impl Default for BerryConfig {
    fn default() -> Self {
        Self {
            server_url: "http://localhost:4114".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let config = BerryConfig::default();
        assert_eq!(config.server_url, "http://localhost:4114");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let config = BerryConfig {
            server_url: "http://example.com:8080".to_string(),
        };

        let toml_str = toml::to_string(&config).expect("Failed to serialize");
        let parsed: BerryConfig = toml::from_str(&toml_str).expect("Failed to deserialize");

        assert_eq!(parsed.server_url, config.server_url);
    }

    #[test]
    fn test_deserialize_from_toml() {
        let toml_str = r#"
            server_url = "https://berry.example.com"
        "#;

        let config: BerryConfig = toml::from_str(toml_str).expect("Failed to deserialize");
        assert_eq!(config.server_url, "https://berry.example.com");
    }

    #[test]
    fn test_deserialize_empty_uses_defaults() {
        let toml_str = "";
        let config: BerryConfig = toml::from_str(toml_str).expect("Failed to deserialize");
        assert_eq!(config.server_url, "http://localhost:4114");
    }
}

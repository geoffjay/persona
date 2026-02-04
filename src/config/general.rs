use serde::{Deserialize, Serialize};

/// General application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    /// The name of the UI theme to use
    pub theme: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            theme: "Kanagawa Wave".to_string(),
        }
    }
}

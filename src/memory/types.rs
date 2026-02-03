use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MemoryType {
    Question,
    Request,
    Information,
}

impl std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryType::Question => write!(f, "Question"),
            MemoryType::Request => write!(f, "Request"),
            MemoryType::Information => write!(f, "Information"),
        }
    }
}

fn default_memory_type() -> MemoryType {
    MemoryType::Information
}

/// Raw memory as returned by berry-rs API (flat structure, snake_case)
#[derive(Debug, Clone, Deserialize)]
pub struct RawMemory {
    pub id: String,
    pub content: String,
    #[serde(rename = "type", default = "default_memory_type")]
    pub memory_type: MemoryType,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub visibility: String,
    #[serde(default)]
    pub shared_with: Vec<String>,
}

/// Flattened memory for easier use in the UI
#[derive(Debug, Clone)]
pub struct Memory {
    pub id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub tags: Vec<String>,
    pub memory_type: MemoryType,
}

impl From<RawMemory> for Memory {
    fn from(raw: RawMemory) -> Self {
        Self {
            id: raw.id,
            content: raw.content,
            memory_type: raw.memory_type,
            created_at: raw.created_at,
            created_by: raw.created_by,
            tags: raw.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub query: String,
    pub as_actor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub memory_type: Option<MemoryType>,
}

/// Response from berry-rs search endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    pub success: bool,
    #[serde(default)]
    pub memories: Vec<RawMemory>,
    pub total: usize,
    #[serde(default)]
    pub error: Option<String>,
}

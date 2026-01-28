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

/// Metadata nested inside memory response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryMetadata {
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Raw memory as returned by the API
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawMemory {
    pub id: String,
    pub content: String,
    #[serde(rename = "type", default = "default_memory_type")]
    pub memory_type: MemoryType,
    pub metadata: MemoryMetadata,
}

fn default_memory_type() -> MemoryType {
    MemoryType::Information
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
            created_at: raw.metadata.created_at,
            created_by: raw.metadata.created_by,
            tags: raw.metadata.tags,
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

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    pub success: bool,
    pub data: Vec<RawMemory>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetMemoryResponse {
    pub memory: RawMemory,
}

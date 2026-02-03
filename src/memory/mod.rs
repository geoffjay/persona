mod types;

pub use types::*;

use thiserror::Error;
use tracing::{debug, error};

#[derive(Error, Debug)]
pub enum BerryError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Server error: {status} - {message}")]
    Server { status: u16, message: String },
    #[error("Parse error: {0}")]
    Parse(String),
}

#[derive(Clone)]
pub struct BerryClient {
    base_url: String,
    client: reqwest::Client,
}

impl BerryClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_memory(&self, id: &str, as_actor: &str) -> Result<Memory, BerryError> {
        let url = format!("{}/v1/memory/{}", self.base_url, id);
        debug!("GET {}", url);

        let response = self
            .client
            .get(&url)
            .query(&[("asActor", as_actor)])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            error!("Server error: {} - {}", status, message);
            return Err(BerryError::Server { status, message });
        }

        let body = response.text().await?;
        debug!("Response body: {}", &body[..body.len().min(500)]);

        let result: RawMemory = serde_json::from_str(&body).map_err(|e| {
            error!("Failed to parse response: {}", e);
            error!("Response body was: {}", &body[..body.len().min(1000)]);
            BerryError::Parse(e.to_string())
        })?;
        Ok(result.into())
    }

    pub async fn search(&self, request: SearchRequest) -> Result<Vec<Memory>, BerryError> {
        let url = format!("{}/v1/search", self.base_url);
        debug!("POST {} with request: {:?}", url, request);

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            error!("Server error: {} - {}", status, message);
            return Err(BerryError::Server { status, message });
        }

        let body = response.text().await?;
        debug!("Response body: {}", &body[..body.len().min(500)]);

        let result: SearchResponse = serde_json::from_str(&body).map_err(|e| {
            error!("Failed to parse response: {}", e);
            error!("Response body was: {}", &body[..body.len().min(1000)]);
            BerryError::Parse(e.to_string())
        })?;

        Ok(result.memories.into_iter().map(Memory::from).collect())
    }
}

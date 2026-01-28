mod types;

pub use types::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BerryError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Server error: {status} - {message}")]
    Server { status: u16, message: String },
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
        let response = self
            .client
            .get(&url)
            .query(&[("asActor", as_actor)])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            return Err(BerryError::Server { status, message });
        }

        let result: GetMemoryResponse = response.json().await?;
        Ok(result.memory.into())
    }

    pub async fn search(&self, request: SearchRequest) -> Result<Vec<Memory>, BerryError> {
        let url = format!("{}/v1/search", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            return Err(BerryError::Server { status, message });
        }

        let result: SearchResponse = response.json().await?;
        // Convert RawMemory to Memory
        let memories = result.data.into_iter().map(Memory::from).collect();
        Ok(memories)
    }
}

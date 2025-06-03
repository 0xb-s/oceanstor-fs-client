use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorSection {
    /// Numeric error code (0 == success).
    #[serde(rename = "code")]
    pub code: i32,
    /// Human-readable description ( just `"0"` on success).
    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Debug, Error)]
pub enum OceanStorError {
    /// Non-zero `"error.code"` returned by the array.
    #[error("OceanStor error {code}: {description}")]
    Api { code: i32, description: String },

    /// HTTP transport failure (network, TLS, …).
    #[error(transparent)]
    Transport(#[from] reqwest::Error),

    /// Unexpected HTTP status (non-2xx that did *not* carry a JSON body).
    #[error("unexpected HTTP status {0}")]
    Status(StatusCode),

    /// JSON decode/encode error.
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

impl From<ApiErrorSection> for OceanStorError {
    fn from(sec: ApiErrorSection) -> Self {
        OceanStorError::Api {
            code: sec.code,
            description: sec.description,
        }
    }
}

use crate::error::{ApiErrorSection, OceanStorError};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(Debug, serde::Deserialize)]
pub struct Envelope<T> {
    pub data: T,
    pub error: ApiErrorSection,
}

impl<T> Envelope<T> {
    pub fn into_result(self) -> Result<T, OceanStorError> {
        if self.error.code == 0 {
            Ok(self.data)
        } else {
            Err(self.error.into())
        }
    }
}

pub async fn parse<T: DeserializeOwned>(resp: reqwest::Response) -> Result<T, OceanStorError> {
    let status = resp.status();
    if !status.is_success() {
        return Err(OceanStorError::Status(status));
    }
    let env: Envelope<T> = resp.json().await?;
    env.into_result()
}

pub type Extra = std::collections::HashMap<String, Value>;

use crate::util::Extra;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FileShareCfg {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "FILESHARINGPROTOCOL")]
    pub protocols: String,
    #[serde(rename = "INITIALENABLEDSTATE")]
    pub enabled: String,
    #[serde(flatten)]
    pub extra: Extra,
}

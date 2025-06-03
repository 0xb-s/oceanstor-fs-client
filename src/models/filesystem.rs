use crate::util::Extra;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct FileSystemSummary {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "HEALTHSTATUS")]
    pub health: String,
    #[serde(rename = "RUNNINGSTATUS")]
    pub running: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileSystem {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "CAPACITY")]
    pub capacity: String,
    #[serde(rename = "IOPRIORITY")]
    pub io_priority: String,
    // todo this type is to review
    #[serde(flatten)]
    pub extra: Extra,
}

use crate::util::Extra;
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ShareProtocol {
    Cifs = 2,
    Nfs = 3,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileShareTypeCfg {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "FILESHARINGPROTOCOL")]
    pub protocol: String,
    #[serde(rename = "WRITEPOLICY")]
    pub write_policy: String,
    // … dozens of other fields …
    #[serde(flatten)]
    pub extra: Extra,
}

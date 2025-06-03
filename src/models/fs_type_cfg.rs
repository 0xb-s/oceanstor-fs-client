use crate::util::Extra;
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FsType {
    Wushan = 1,
    Nofs = 2,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FsTypeCfg {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ACTUALFILESYSTEMTYPE")]
    pub fs_type: String,
    // … all other fields …
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FsCfgAbility {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "TYPE")]
    pub typ: u32,
    #[serde(rename = "SUPPORTEDACTUALFILESYSTEMTYPES")]
    pub supported_types: String,
    // … etc …
    #[serde(flatten)]
    pub extra: Extra,
}

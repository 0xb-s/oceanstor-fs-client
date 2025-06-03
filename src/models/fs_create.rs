use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct CreateFsReq<'a> {
    #[serde(rename = "NAME")]
    pub name: &'a str,
    #[serde(rename = "PARENTID")]
    pub pool_id: &'a str,
    #[serde(rename = "PARENTTYPE", default = "default_parent_type")]
    pub parent_type: u16,
    #[serde(rename = "ALLOCTYPE", default = "default_alloc")]
    pub alloc: u8,
    #[serde(rename = "SNAPSHOTRESERVEPER", default = "default_snap_per")]
    pub snap_percent: u8,
    #[serde(rename = "SECTORSIZE")]
    pub sector_size: u32,
    #[serde(rename = "APPLICATIONSCENARIO", default = "default_app")]
    pub app: u8,
    #[serde(rename = "DESCRIPTION", skip_serializing_if = "Option::is_none")]
    pub desc: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCloneReq<'a> {
    #[serde(rename = "NAME")]
    pub name: &'a str,
    #[serde(rename = "PARENTFILESYSTEMID")]
    pub parent_fs: &'a str,
    #[serde(rename = "ALLOCTYPE", default = "default_alloc")]
    pub alloc: u8,
    #[serde(rename = "DESCRIPTION", skip_serializing_if = "Option::is_none")]
    pub desc: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFsResp(pub crate::models::filesystem::FileSystem);

impl From<CreateFsResp> for crate::models::filesystem::FileSystem {
    fn from(r: CreateFsResp) -> Self {
        r.0
    }
}

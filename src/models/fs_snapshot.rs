use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotToggleReq<'a> {
    #[serde(rename = "ID")]
    pub id: &'a str,
    #[serde(rename = "ENABLETIMINGSNAPSHOT")]
    pub enable: bool,

    #[serde(rename = "vstoreId", skip_serializing_if = "Option::is_none")]
    pub vstore_id: Option<&'a str>,
}

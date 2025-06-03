use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SplitSpeed {
    Low = 1,
    Medium = 2,
    High = 3,
    Highest = 4,
}

#[derive(Debug, Clone, Serialize)]
pub struct CloneSplitReq<'a> {
    /// File-system clone ID.
    #[serde(rename = "ID")]
    pub id: &'a str,
    /// `true` to start, `false` to stop.
    #[serde(rename = "SPLITENABLE")]
    pub enable: bool,
    /// Optional split speed.
    #[serde(rename = "SPLITSPEED", skip_serializing_if = "Option::is_none")]
    pub speed: Option<SplitSpeed>,
    /// Delete parent snapshots once splitting is done?
    #[serde(
        rename = "ISDELETEPARENTSNAPSHOT",
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_parent_snaps: Option<bool>,
}

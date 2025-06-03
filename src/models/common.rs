use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Deserialize)]
pub struct Count {
    #[serde(rename = "COUNT")]
    pub count: u64,
}

pub type BoolString = String;

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IoPriority {
    Low = 1,
    Middle = 2,
    High = 3,
}

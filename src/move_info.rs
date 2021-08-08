use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct MoveInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nps: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

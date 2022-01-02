gen_type! {
    pub struct MoveInfo {
        #[serde(skip_serializing_if = "Option::is_none")]
        nodes: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        depth: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        extra: Option<String>,
    }
}

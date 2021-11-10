pub mod bot_msg;
pub mod data;
pub mod frontend_msg;

pub use bot_msg::BotMessage;
pub use frontend_msg::FrontendMessage;

#[cfg(feature = "move_info")]
pub mod move_info;
#[cfg(feature = "randomizer")]
pub mod randomizer;

pub type Rest = std::collections::HashMap<String, serde_json::Value>;

serde_big_array::big_array!(BigArray; 40);

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Feature {
    #[serde(other)]
    Unknown,
}

impl Feature {
    pub fn enabled() -> Vec<Feature> {
        #[allow(unused_mut)]
        let mut features = vec![];
        features
    }
}

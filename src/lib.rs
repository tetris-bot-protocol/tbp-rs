pub mod bot_msg;
pub mod frontend_msg;
pub mod data;

pub use bot_msg::BotMessage;
pub use frontend_msg::FrontendMessage;

#[cfg(feature = "randomizer")]
pub mod randomizer;
#[cfg(feature = "move_info")]
pub mod move_info;

pub type Rest = std::collections::HashMap<String, serde_json::Value>;

serde_big_array::big_array!(BigArray; 40);

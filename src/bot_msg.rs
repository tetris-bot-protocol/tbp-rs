use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::data::{ErrorCause, Move};

#[derive(Serialize, Deserialize, Clone, Debug, From)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BotMessage {
    Error(Error),
    Ready(Ready),
    Info(Info),
    Suggestion(Suggestion),
}

gen_type! {
    pub struct Error {
        required reason: ErrorCause,
    }

    pub struct Info {
        required name: String,
        required version: String,
        required author: String,
        features: Vec<String>,
    }

    pub struct Ready {}

    pub struct Suggestion {
        moves: Vec<Move>,

        #[serde(default)]
        required move_info: crate::move_info::MoveInfo,
    }
}

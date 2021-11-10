use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::data::{ErrorCause, Move};
use crate::{Feature, Rest};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, From)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BotMessage {
    Error(Error),
    Ready(Ready),
    Info(Info),
    Suggestion(Suggestion),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Error {
    pub reason: ErrorCause,

    #[serde(flatten)]
    pub rest: Rest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Ready {
    #[serde(flatten)]
    pub rest: Rest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Info {
    pub name: String,
    pub version: String,
    pub author: String,
    pub features: Vec<Feature>,

    #[serde(flatten)]
    pub rest: Rest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Suggestion {
    pub moves: Vec<Move>,

    #[cfg(feature = "move_info")]
    #[serde(default)]
    pub move_info: crate::move_info::MoveInfo,

    #[serde(flatten)]
    pub rest: Rest,
}

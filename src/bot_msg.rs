use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::{MaybeUnknown, Feature};
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
        required reason: MaybeUnknown<ErrorCause>,
    }

    pub struct Info {
        required name: String,
        required version: String,
        required author: String,
        required features: Vec<MaybeUnknown<Feature>>,
    }

    pub struct Ready {}

    pub struct Suggestion {
        required moves: Vec<Move>,

        move_info: crate::move_info::MoveInfo,
    }
}

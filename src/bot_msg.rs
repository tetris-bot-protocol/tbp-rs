use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::data::{ErrorCause, Move};
use crate::Feature;

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
        reason: ErrorCause,
    }

    impl Error {
        pub fn new(reason: ErrorCause) -> Self {
            Self {
                reason,
                rest: Default::default()
            }
        }
    }

    pub struct Info {
        name: String,
        version: String,
        author: String,
        features: Vec<Feature>,
    }

    impl Info {
        pub fn new(
            name: String,
            version: String,
            author: String,
            features: Vec<Feature>,
        ) -> Self {
            Self {
                name,
                version,
                author,
                features,
                rest: Default::default(),
            }
        }
    }

    #[derive(Default)]
    pub struct Ready {}

    pub struct Suggestion {
        moves: Vec<Move>,

        #[serde(default)]
        move_info: crate::move_info::MoveInfo,
    }

    impl Suggestion {
        pub fn new(moves: Vec<Move>) -> Self {
            Self {
                moves,
                move_info: Default::default(),
                rest: Default::default(),
            }
        }
    }
}

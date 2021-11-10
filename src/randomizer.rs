use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use crate::data::Piece;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum RandomizerRule {
    Uniform,
    SevenBag,
    GeneralBag,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RandomizerState {
    Uniform,
    SevenBag {
        bag_state: Vec<Piece>,
    },
    GeneralBag {
        current_bag: EnumMap<Piece, u32>,
        filled_bag: EnumMap<Piece, u32>,
    },
    #[serde(other)]
    Unknown,
}

impl Default for RandomizerRule {
    fn default() -> Self {
        RandomizerRule::Unknown
    }
}

impl Default for RandomizerState {
    fn default() -> Self {
        RandomizerState::Unknown
    }
}

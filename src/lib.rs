use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[cfg(feature = "randomizer")]
pub mod randomizer;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FrontendMessage {
    Start {
        hold: Option<Piece>,
        queue: Vec<Piece>,
        combo: u32,
        back_to_back: bool,
        #[serde(with = "BigArray")]
        board: [[Option<char>; 10]; 40],
        #[cfg(feature = "randomizer")]
        #[serde(default)]
        randomizer: randomizer::RandomizerState,
    },
    Stop,
    Suggest,
    Play {
        #[serde(rename = "move")]
        mv: Move,
    },
    NewPiece {
        piece: Piece,
    },
    Rules {
        #[cfg(feature = "randomizer")]
        #[serde(default)]
        randomizer: randomizer::RandomizerRule,
    },
    Quit,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BotMessage {
    Error {
        reason: ErrorCause,
    },
    Ready,
    Info {
        name: String,
        version: String,
        author: String,
        features: Vec<Feature>,
    },
    Suggestion {
        moves: Vec<Move>,
    },
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, Enum)]
pub enum Piece {
    I,
    O,
    T,
    L,
    J,
    S,
    Z,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    pub location: PieceLocation,
    pub spin: Spin,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PieceLocation {
    #[serde(rename = "type")]
    pub kind: Piece,
    pub orientation: Orientation,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Spin {
    None,
    Mini,
    Full,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCause {
    UnsupportedRules,

    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

serde_big_array::big_array!(BigArray; 40);

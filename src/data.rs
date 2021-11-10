use serde::{Serialize, Deserialize};
use enum_map::Enum;

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
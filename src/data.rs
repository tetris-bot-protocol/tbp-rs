use serde::{Deserialize, Serialize};

use crate::MaybeUnknown;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
pub enum Piece {
    I,
    O,
    T,
    L,
    J,
    S,
    Z,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Spin {
    None,
    Mini,
    Full,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ErrorCause {
    UnsupportedRules,
}

gen_type! {
    pub struct Move {
        required location: PieceLocation,
        required spin: MaybeUnknown<Spin>,
    }

    pub struct PieceLocation {
        #[serde(rename = "type")]
        required kind: MaybeUnknown<Piece>,
        required orientation: MaybeUnknown<Orientation>,
        required x: i32,
        required y: i32,
    }
}

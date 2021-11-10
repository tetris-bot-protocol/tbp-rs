use serde::{Serialize, Deserialize};
use derive_more::From;

use crate::Rest;
use crate::data::{Move, Piece};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, From)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FrontendMessage {
    Start(Start),
    Stop(Stop),
    Suggest(Suggest),
    Play(Play),
    NewPiece(NewPiece),
    Rules(Rules),
    Quit(Quit),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Start {
    pub hold: Option<Piece>,
    pub queue: Vec<Piece>,
    pub combo: u32,
    pub back_to_back: bool,
    #[serde(with = "crate::BigArray")]
    pub board: [[Option<char>; 10]; 40],

    #[cfg(feature = "randomizer")]
    #[serde(default)]
    pub randomizer: crate::randomizer::RandomizerState,

    #[serde(flatten)]
    pub rest: Rest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Stop {
    #[serde(flatten)]
    pub rest: Rest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Suggest {
    #[serde(flatten)]
    pub rest: Rest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Play {
    #[serde(rename = "move")]
    pub mv: Move,

    #[serde(flatten)]
    pub rest: Rest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct NewPiece {
    pub piece: Piece,

    #[serde(flatten)]
    pub rest: Rest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Rules {
    #[cfg(feature = "randomizer")]
    #[serde(default)]
    pub randomizer: crate::randomizer::RandomizerRule,

    #[serde(flatten)]
    pub rest: Rest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Quit {
    #[serde(flatten)]
    pub rest: Rest,
}

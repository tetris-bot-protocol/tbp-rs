use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::MaybeUnknown;
use crate::data::{Move, Piece};

#[derive(Serialize, Deserialize, Clone, Debug, From)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FrontendMessage {
    Start(Start),
    Stop(Stop),
    Suggest(Suggest),
    Play(Play),
    NewPiece(NewPiece),
    Rules(Rules),
    Quit(Quit),
}

gen_type! {
    pub struct Stop {}

    pub struct Quit {}

    pub struct Start {
        required hold: Option<MaybeUnknown<Piece>>,
        required queue: Vec<MaybeUnknown<Piece>>,
        required combo: u32,
        required back_to_back: bool,
        required board: Vec<Vec<Option<char>>>,
    
        randomizer: crate::randomizer::RandomizerState,
    }

    pub struct Suggest {}

    pub struct Play {
        #[serde(rename = "move")]
        required mv: Move,
    }

    pub struct NewPiece {
        required piece: MaybeUnknown<Piece>,
    }

    pub struct Rules {
        randomizer: crate::randomizer::RandomizerRule,
    }
}

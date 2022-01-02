use derive_more::From;
use serde::{Deserialize, Serialize};

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
    #[derive(Default)]
    pub struct Stop {}

    #[derive(Default)]
    pub struct Quit {}

    pub struct Start {
        hold: Option<Piece>,
        queue: Vec<Piece>,
        combo: u32,
        back_to_back: bool,
        #[serde(with = "crate::BigArray")]
        board: [[Option<char>; 10]; 40],
    
        #[serde(default)]
        randomizer: crate::randomizer::RandomizerState,
    }

    impl Start {
        pub fn new(
            hold: Option<Piece>,
            queue: Vec<Piece>,
            combo: u32,
            back_to_back: bool,
            board: [[Option<char>; 10]; 40],
        ) -> Self {
            Start {
                hold,
                queue,
                combo,
                back_to_back,
                board,
                randomizer: Default::default(),
                rest: Default::default(),
            }
        }
    }

    #[derive(Default)]
    pub struct Suggest {}

    pub struct Play {
        #[serde(rename = "move")]
        mv: Move,
    }

    impl Play {
        pub fn new(mv: Move) -> Self {
            Self {
                mv,
                rest: Default::default()
            }
        }
    }

    pub struct NewPiece {
        piece: Piece,
    }

    impl NewPiece {
        pub fn new(piece: Piece) -> Self {
            Self {
                piece,
                rest: Default::default()
            }
        }
    }

    #[derive(Default)]
    pub struct Rules {
        randomizer: crate::randomizer::RandomizerRule,
    }
}

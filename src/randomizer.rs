use serde::{Deserialize, Serialize};

use crate::data::Piece;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RandomizerRule {
    Uniform,
    SevenBag,
    GeneralBag,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RandomizerState {
    Uniform,
    SevenBag {
        bag_state: Vec<Piece>,
    },
    GeneralBag {
        current_bag: Bag,
        filled_bag: Bag,
    },
    #[serde(other)]
    Unknown,
}

gen_type! {
    pub struct Bag {
        #[serde(rename = "I")] i: u32,
        #[serde(rename = "O")] o: u32,
        #[serde(rename = "T")] t: u32,
        #[serde(rename = "L")] l: u32,
        #[serde(rename = "J")] j: u32,
        #[serde(rename = "S")] s: u32,
        #[serde(rename = "Z")] z: u32,
    }
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

impl std::ops::Index<Piece> for Bag {
    type Output = u32;
    fn index(&self, piece: Piece) -> &u32 {
        match piece {
            Piece::I => &self.i,
            Piece::O => &self.o,
            Piece::T => &self.t,
            Piece::L => &self.l,
            Piece::J => &self.j,
            Piece::S => &self.s,
            Piece::Z => &self.z,
        }
    }
}

impl std::ops::IndexMut<Piece> for Bag {
    fn index_mut(&mut self, piece: Piece) -> &mut u32 {
        match piece {
            Piece::I => &mut self.i,
            Piece::O => &mut self.o,
            Piece::T => &mut self.t,
            Piece::L => &mut self.l,
            Piece::J => &mut self.j,
            Piece::S => &mut self.s,
            Piece::Z => &mut self.z,
        }
    }
}

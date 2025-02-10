#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod utils;
use utils::fen::*;

mod board;
pub use board::{BitBoard, Board, ChessMove, FinishedState, GameState, MoveType};
pub use utils::{
    extract_squares as wrap_extract_squares,
    fen::{
        CASTLE_KINGSIDE_POSITION, CASTLE_QUEENSIDE_POSITION, CHECK_POSITION, CMK_POSITION,
        KILLER_POSITION, PROMOTION_POSITION, START_POSITION, TRICKY_POSITION,
    },
    san::SanMove,
    squares::*,
    ChessError, Colour, Piece, Pieces,
};

// pub use {bishopTargets, knightTargets, queenTargets, rookTargets};

mod tests;

pub fn init() {
    unsafe {
        init_targets();
    }
}

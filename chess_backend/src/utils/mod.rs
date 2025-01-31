pub mod fen;
pub mod san;
pub mod squares;

use core::panic;
use std::{error::Error, fmt::Display};

use crate::{piece_map_bitboards, squares_to_bitboard};

#[derive(Debug)]
pub enum ChessError {
    InputError,
}
impl Display for ChessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChessError")
    }
}
impl Error for ChessError {}

/// Enum representation of the colour of pieces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}
impl Colour {
    pub fn as_int(&self) -> i32 {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }
    pub fn other(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn(Colour),
    King(Colour),
    Queen(Colour),
    Bishop(Colour),
    Knight(Colour),
    Rook(Colour),
}
impl Piece {
    pub fn symbol(&self) -> Option<&str> {
        match self {
            Self::Pawn(Colour::Black) => Some("♙"),
            Self::Pawn(Colour::White) => Some("♟︎"),
            Self::King(Colour::Black) => Some("♔"),
            Self::King(Colour::White) => Some("♚"),
            Self::Queen(Colour::Black) => Some("♕"),
            Self::Queen(Colour::White) => Some("♛"),
            Self::Bishop(Colour::Black) => Some("♗"),
            Self::Bishop(Colour::White) => Some("♝"),
            Self::Knight(Colour::Black) => Some("♘"),
            Self::Knight(Colour::White) => Some("♞"),
            Self::Rook(Colour::Black) => Some("♖"),
            Self::Rook(Colour::White) => Some("♜"),
        }
    }
    pub fn letter(&self) -> &str {
        match self {
            Self::Pawn(_) => "",
            Self::King(_) => "K",
            Self::Queen(_) => "Q",
            Self::Bishop(_) => "B",
            Self::Knight(_) => "N",
            Self::Rook(_) => "R",
        }
    }
    pub fn from_letter(l: &str, colour: Colour) -> Piece {
        match l {
            "" => Self::Pawn(colour),
            "K" => Self::King(colour),
            "Q" => Self::Queen(colour),
            "B" => Self::Bishop(colour),
            "N" => Self::Knight(colour),
            "R" => Self::Rook(colour),
            _ => panic!("Invalid piece string"),
        }
    }
}

pub struct Pieces {
    pub king: Vec<i32>,
    pub queens: Vec<i32>,
    pub bishops: Vec<i32>,
    pub knights: Vec<i32>,
    pub rooks: Vec<i32>,
    pub pawns: Vec<i32>,
}
impl Default for Pieces {
    fn default() -> Self {
        Self {
            king: vec![],
            queens: vec![],
            bishops: vec![],
            knights: vec![],
            rooks: vec![],
            pawns: vec![],
        }
    }
}
impl From<piece_map_bitboards> for Pieces {
    fn from(value: piece_map_bitboards) -> Self {
        let king = extract_squares(value.king);
        let queens = extract_squares(value.queens);
        let bishops = extract_squares(value.bishops);
        let knights = extract_squares(value.knights);
        let rooks = extract_squares(value.rooks);
        let pawns = extract_squares(value.pawns);
        Self {
            king,
            queens,
            bishops,
            knights,
            rooks,
            pawns,
        }
    }
}

pub fn extract_squares(bitboard: u64) -> Vec<i32> {
    let mut res = Vec::new();
    let mut i = 1;
    for iter in 0..64 {
        if bitboard & i != 0 {
            res.push(iter);
        }
        i <<= 1;
    }

    res
}

unsafe fn convert_piece_map_to_bitboards(colour: &mut Pieces) -> piece_map_bitboards {
    piece_map_bitboards {
        pawns: squares_to_bitboard(colour.pawns.as_mut_ptr(), colour.pawns.len() as i32),
        king: squares_to_bitboard(colour.king.as_mut_ptr(), colour.king.len() as i32),
        queens: squares_to_bitboard(colour.queens.as_mut_ptr(), colour.queens.len() as i32),
        bishops: squares_to_bitboard(colour.bishops.as_mut_ptr(), colour.bishops.len() as i32),
        knights: squares_to_bitboard(colour.knights.as_mut_ptr(), colour.knights.len() as i32),
        rooks: squares_to_bitboard(colour.rooks.as_mut_ptr(), colour.rooks.len() as i32),
    }
}

impl From<&mut Pieces> for piece_map_bitboards {
    fn from(value: &mut Pieces) -> Self {
        unsafe { convert_piece_map_to_bitboards(value) }
    }
}

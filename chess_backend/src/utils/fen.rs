use super::{Colour, Pieces};
use crate::{board::Board, castling_rights, createBase, piece_map_bitboards, utils::squares};
use core::panic;
use std::convert::From;

pub const EMPTY_BOARD: &str = "8/8/8/8/8/8/8/8 w - - 0 1";
pub const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";
pub const CASTLE_KINGSIDE_POSITION: &str =
    "rnbqk2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1 ";
pub const CASTLE_QUEENSIDE_POSITION: &str = "r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1 ";

pub const PROMOTION_POSITION: &str = "4k3/1P6/2P5/8/8/8/8/4K3 w - - 0 1";

pub const CHECK_POSITION: &str = "4k3/1P6/2P5/8/8/8/5p2/4K3 w - - 0 1";

pub const TRICKY_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
pub const KILLER_POSITION: &str =
    "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
pub const CMK_POSITION: &str =
    "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 ";

/// Create a chess board instance from fen
impl<T> From<T> for Board
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        let val = value.into();
        let s = val.split_at((&val).find(" ").unwrap());
        let ranks = s.0.split("/");
        let mut args = s.1.split(" ");

        let mut white = Pieces::default();
        let mut black = Pieces::default();
        let mut i: i32 = 63;
        for rank in ranks {
            for c in rank.chars().rev() {
                if let Ok(n) = c.to_string().parse::<i32>() {
                    i -= n;
                } else {
                    match c {
                        'p' => black.pawns.push(i),
                        'P' => white.pawns.push(i),
                        'k' => black.king.push(i),
                        'K' => white.king.push(i),
                        'q' => black.queens.push(i),
                        'Q' => white.queens.push(i),
                        'b' => black.bishops.push(i),
                        'B' => white.bishops.push(i),
                        'n' => black.knights.push(i),
                        'N' => white.knights.push(i),
                        'r' => black.rooks.push(i),
                        'R' => white.rooks.push(i),
                        _ => panic!("Invalid symbol '{c}'"),
                    };
                    i -= 1;
                }
            }
        }

        args.next();
        let side_to_move = match args.next().unwrap() {
            "w" => Colour::White,
            "b" => Colour::Black,
            _ => panic!("Invalid side to move"),
        };

        let castling_rights = castling_rights::from(args.next().unwrap());
        let killer_square = squares::from_str(args.next().unwrap());
        let halfmove = args.next().unwrap().parse::<i32>().unwrap();
        let fullmove = args.next().unwrap().parse::<i32>().unwrap();

        unsafe {
            let base = createBase(
                piece_map_bitboards::from(&mut white),
                piece_map_bitboards::from(&mut black),
            );
            Self::new(
                base,
                killer_square,
                castling_rights,
                side_to_move,
                halfmove,
                fullmove,
            )
        }
    }
}

impl From<&str> for castling_rights {
    fn from(value: &str) -> Self {
        let white_king = value.contains("K");
        let white_queen = value.contains("Q");
        let black_king = value.contains("k");
        let black_queen = value.contains("q");

        Self {
            white_king,
            white_queen,
            black_king,
            black_queen,
        }
    }
}

macro_rules! convert_into_character {
    ($set:ident set $board:ident, $param:ident = $char:expr) => {
        for i in $set.$param {
            $board[i as usize] = $char;
        }
    };
}
impl Board {
    pub fn into_fen(&self) -> String {
        let mut board = ['-'; 64];
        let white_pieces = Pieces::from(self.base.white);
        let black_pieces = Pieces::from(self.base.black);

        convert_into_character!(white_pieces set board, king = 'K');
        convert_into_character!(white_pieces set board, queens = 'Q');
        convert_into_character!(white_pieces set board, bishops = 'B');
        convert_into_character!(white_pieces set board, knights = 'N');
        convert_into_character!(white_pieces set board, rooks = 'R');
        convert_into_character!(white_pieces set board, pawns = 'P');

        convert_into_character!(black_pieces set board, king = 'k');
        convert_into_character!(black_pieces set board, queens = 'q');
        convert_into_character!(black_pieces set board, bishops = 'b');
        convert_into_character!(black_pieces set board, knights = 'n');
        convert_into_character!(black_pieces set board, rooks = 'r');
        convert_into_character!(black_pieces set board, pawns = 'p');

        let mut rows = Vec::with_capacity(8);
        for row in board.chunks(8).rev() {
            let mut empty_squares = 0;
            let mut row_str = String::new();
            for (i, p) in row.iter().enumerate() {
                if p == &'-' {
                    empty_squares += 1;
                }
                if (p != &'-' || i == 7) && empty_squares > 0 {
                    row_str += &empty_squares.to_string();
                    empty_squares = 0;
                }
                if p != &'-' {
                    row_str.push(*p)
                }
            }
            rows.push(row_str);
        }
        let mut res = rows.join("/");

        // Trailers
        if self.side_to_move() == Colour::White {
            res += " w ";
        } else {
            res += " b ";
        }

        let castling_rights = self.castling_rights_as_arr();
        if castling_rights.iter().all(|r| !r) {
            res += "-";
        } else {
            if castling_rights[0] {
                res += "K";
            }
            if castling_rights[1] {
                res += "Q";
            }
            if castling_rights[2] {
                res += "k";
            }
            if castling_rights[3] {
                res += "q";
            }
        }

        if let Some(killer_square) = squares::to_str(self.killer_square()) {
            res += " ";
            res += &killer_square;
        } else {
            res += " -";
        }

        res += " ";
        res += &self.halfmove().to_string();
        res += " ";
        res += &self.fullmove().to_string();
        res
    }
}

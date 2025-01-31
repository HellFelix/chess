use std::fmt::Display;

// Standard Algebraic Notation SAN
use crate::{
    board::ReasonWin, generateAttackTargets, piece_map_bitboards, Board, ChessMove, Colour,
    FinishedState, GameState, Piece,
};

use super::{squares, ChessError};

// Special moves
const CASTLE_KING: &str = "O-O";
const CASTLE_QUEEN: &str = "O-O-O";

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SanMove {
    piece: Piece,
    capture: bool,
    check: bool,
    checkmate: bool,
    origin: i32,
    disambiguation: (bool, bool),
    dest: i32,
    promotion: Option<Piece>,
    castling: Option<(bool, bool)>,
}
impl SanMove {
    pub fn new(
        piece: Piece,
        capture: bool,
        check: bool,
        checkmate: bool,
        origin: i32,
        disambiguation: (bool, bool),
        dest: i32,
        promotion: Option<Piece>,
        castling: Option<(bool, bool)>,
    ) -> Self {
        Self {
            piece,
            capture,
            check,
            checkmate,
            origin,
            disambiguation,
            dest,
            promotion,
            castling,
        }
    }

    pub fn kingside_castle(colour: Colour) -> Self {
        let (origin, dest) = if colour == Colour::White {
            (4, 6)
        } else {
            (60, 62)
        };
        Self {
            piece: Piece::King(colour),
            capture: false,
            check: false,
            checkmate: false,
            origin,
            disambiguation: (false, false),
            dest,
            promotion: None,
            castling: Some((true, false)),
        }
    }

    pub fn queenside_castle(colour: Colour) -> Self {
        let (origin, dest) = if colour == Colour::White {
            (4, 2)
        } else {
            (60, 58)
        };
        Self {
            piece: Piece::King(colour),
            capture: false,
            check: false,
            checkmate: false,
            origin,
            disambiguation: (false, false),
            dest,
            promotion: None,
            castling: Some((false, true)),
        }
    }
    pub fn from_string(value: impl Into<String>, origin_board: Board) -> Result<Self, ChessError> {
        let value: String = value.into();
        for m in origin_board.generate_legal_moves() {
            let san = origin_board.get_san(&m.board);
            if <SanMove as Into<String>>::into(san) == value {
                return Ok(san);
            }
        }
        Err(ChessError::InputError)
    }
}
impl Into<String> for SanMove {
    fn into(self) -> String {
        if let Some(sides) = self.castling {
            if sides.0 {
                return String::from(CASTLE_KING);
            }
            if sides.1 {
                return String::from(CASTLE_QUEEN);
            }
        }

        let mut res = String::new();
        res += &self.piece.letter();

        let origin_square = squares::to_str(self.origin).expect("Invalid origin number");
        if self.disambiguation.0 {
            res += &origin_square[0..1];
        }
        if self.disambiguation.1 {
            res += &origin_square[1..2];
        }

        if self.capture {
            res += "x";
        }

        res += &squares::to_str(self.dest).expect("Invalid destination number");

        if let Some(res_piece) = self.promotion {
            res += "=";
            res += res_piece.letter();
        }

        if self.check {
            res += "+";
        } else if self.checkmate {
            res += "#";
        }

        res
    }
}
impl Display for SanMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <SanMove as Into<String>>::into(*self))
    }
}

macro_rules! find_descrepency {
    ($origin_pieces:ident, $dest_pieces:ident, $possible_boards:ident, $pieces:ident, $origin_square:ident, $dest_square:ident, $disambiguation:ident, $($param:ident -> $val:expr,)*) => {
        $(
            let mut discrepency = false;
            for i in 0..64 {
                let placement = 1 << i;
                if ($origin_pieces.$param & placement) ^ ($dest_pieces.$param & placement) != 0 {
                    discrepency = true;
                    if !$pieces.contains(&$val) {
                        $pieces.push($val);
                    }
                    if ($origin_pieces.$param & placement != 0) {
                        if $origin_square == None {
                            $origin_square = Some(i);
                        }
                    } else {
                        if $dest_square == None {
                            $dest_square = Some(i);
                        }
                    }
                }
            }
            if discrepency && $dest_square != None {
                let possible_moves = $possible_boards
                    .iter()
                    .filter(|m| $pieces.contains(&m.base.piece) && m.base.destination_square.unwrap_or_default() == $dest_square.unwrap() as i32)
                    .map(|m| m.base.starting_square.unwrap())
                    .collect::<Vec<i32>>();
                $disambiguation = if possible_moves.len() > 1 {
                    let origin = $origin_square.unwrap() as i32;
                    if possible_moves.iter().all(|s| (s % 8 != origin % 8) || *s == origin) {
                        // Different file (same remainder)
                        Some((true, false))
                    } else if possible_moves.iter().all(|s| (s / 8 != origin / 8) || *s == origin) {
                        // Different rank (same integer division)
                        Some((false, true))
                    } else {
                        // By process of elimination, if none of the above applies, we must disambiguate twice
                        Some((true, true))
                    }
                } else {
                    Some((false, false))
                }
            }
        )*
    };
}

impl Board {
    fn get_move_specs(
        origin_pieces: piece_map_bitboards,
        dest_pieces: piece_map_bitboards,
        colour: Colour,
        possible_boards: &Vec<ChessMove>,
    ) -> (Vec<Piece>, usize, usize, (bool, bool)) {
        let mut pieces: Vec<Piece> = Vec::new();
        let mut origin_square: Option<usize> = None;
        let mut dest_square: Option<usize> = None;
        let mut disambiguation: Option<(bool, bool)> = None;

        find_descrepency!(
            origin_pieces, dest_pieces, possible_boards,
            pieces, origin_square, dest_square, disambiguation,
            // king and pawn being checked first ensures that "multimoves" are not incorrectly interpreted
            king -> Piece::King(colour),
            pawns -> Piece::Pawn(colour),
            queens -> Piece::Queen(colour),
            bishops -> Piece::Bishop(colour),
            knights -> Piece::Knight(colour),
            rooks -> Piece::Rook(colour),
        );

        (
            pieces,
            origin_square.unwrap(),
            dest_square.unwrap(),
            disambiguation.unwrap(),
        )
    }
    /// Does no error checking, simply assuming the move was legal
    pub fn get_san(&self, res_board: &Self) -> SanMove {
        let possible_boards = self.generate_legal_moves();
        let colour = self.side_to_move();
        let (origin_pieces, dest_pieces, other_base, res_other_base, other_king) =
            if colour == Colour::White {
                (
                    self.base.white,
                    res_board.base.white,
                    self.base.black,
                    res_board.base.black,
                    res_board.base.black.king,
                )
            } else {
                (
                    self.base.black,
                    res_board.base.black,
                    self.base.white,
                    res_board.base.white,
                    res_board.base.white.king,
                )
            };

        let res_attack = unsafe {
            generateAttackTargets(
                res_board.base.get_side(colour),
                colour.as_int(),
                res_board.base.black_occupied + res_board.base.white_occupied,
            )
        };

        let (pieces, origin, dest, mut disambiguation) =
            Self::get_move_specs(origin_pieces, dest_pieces, colour, &possible_boards);

        // Check for castling
        if pieces.contains(&Piece::King(colour)) {
            if origin == 4 || origin == 60 {
                if dest == 6 || dest == 62 {
                    return SanMove::kingside_castle(colour);
                } else if dest == 2 || dest == 58 {
                    return SanMove::queenside_castle(colour);
                }
            }
        }
        // if we have not returned a value, castling must be None
        let castling = None;

        let capture = other_base != res_other_base;
        // check for promotion
        // If more than two bitboards have changed, and there was no castling, it must mean that a
        // pawn has promoted
        let (piece, promotion) = if pieces.len() > 1 {
            // Just take the first non-pawn entry

            if (disambiguation.0 || disambiguation.1) && !capture {
                disambiguation = (false, false)
            }
            if pieces[0] != Piece::Pawn(colour) {
                (Piece::Pawn(colour), Some(pieces[0]))
            } else {
                (Piece::Pawn(colour), Some(pieces[1]))
            }
        } else {
            (pieces[0], None)
        };

        if piece == Piece::Pawn(colour) && capture {
            disambiguation.0 = true;
        }

        let (check, checkmate) = if res_board.get_game_state()
            == GameState::Finished(FinishedState::Win(colour, ReasonWin::Checkmate))
        {
            (false, true)
        } else {
            (other_king & res_attack != 0, false)
        };

        SanMove {
            piece,
            capture,
            check,
            checkmate,
            origin: origin as i32,
            disambiguation,
            dest: dest as i32,
            promotion,
            castling,
        }
    }

    pub fn make_san_move(&mut self, m: SanMove) -> Result<(), ChessError> {
        // check and pick against the list of legal moves
        for next in self.generate_legal_moves().iter() {
            let found_san = self.get_san(&next.board);
            if found_san == m {
                *self = next.board;
                return Ok(());
            }
        }

        Err(ChessError::InputError)
    }
}

use chess_backend::*;
use log::debug;

use super::modifiers::{BISHOP_VAL, KNIGHT_VAL, PAWN_VAL, QUEEN_VAL, ROOK_VAL};
use crate::engine::{tree::Branch, utils::eval::Eval};

// priority modifiers
const KNIGHT_ATTACK: f32 = 60.;
const BISHIP_ATTACK: f32 = 500.;
const ROOK_ATTACK: f32 = 100.;
const QUEEN_ATTACK: f32 = 150.;

const CAPTURE_BONUS: f32 = 50.;

const DEPTH_PENALTY: f32 = 10.;

impl Branch {
    // Preliminary evaluation to find how promising the move is
    pub fn calc_priority(
        orig_board: Board,
        res_board: Board,
        depth: usize,
        heuristic: Eval,
    ) -> Eval {
        if let Eval::Numeric(mut res) = heuristic {
            res = if res_board.side_to_move() == Colour::White {
                res
            } else {
                -res
            };
            // let white_pieces = Pieces::from(res_board.base.white);
            // let black_pieces = Pieces::from(res_board.base.black);

            // // This acts weird possibly because placing a piece where it can be taken provokes
            // // further investigation
            // res += Self::pawn_piece_attacks(res_board, Colour::White, &white_pieces, &black_pieces);
            // res += Self::pawn_piece_attacks(res_board, Colour::Black, &white_pieces, &black_pieces);

            // // if let Some(capture_investigation) = Self::is_capture(orig_board, res_board) {
            // //     res += capture_investigation;
            // // }

            // res -= depth as f32 * DEPTH_PENALTY;

            Eval::Numeric(res)
        } else {
            heuristic
        }
    }

    fn pawn_piece_attacks(
        res_board: Board,
        colour: Colour,
        white_pieces: &Pieces,
        black_pieces: &Pieces,
    ) -> f32 {
        let mut res = 0.;
        let (pieces, other_pieces) = if colour == Colour::White {
            (white_pieces, black_pieces)
        } else {
            (black_pieces, white_pieces)
        };
        let occupancy = res_board.base.black_occupied + res_board.base.white_occupied;

        for square in &pieces.pawns {
            let attacked =
                unsafe { wrap_extract_squares(pawnTargets(*square, colour.as_int(), occupancy)) };
            for square in &other_pieces.knights {
                if attacked.contains(square) {
                    res += KNIGHT_ATTACK;
                }
            }
            for square in &other_pieces.bishops {
                if attacked.contains(square) {
                    res += BISHIP_ATTACK;
                }
            }
            for square in &other_pieces.rooks {
                if attacked.contains(square) {
                    res += ROOK_ATTACK;
                }
            }
            for square in &other_pieces.queens {
                if attacked.contains(square) {
                    res += QUEEN_ATTACK;
                }
            }
        }
        res
    }

    fn is_capture(orig_board: Board, res_board: Board) -> Option<f32> {
        let side = res_board.side_to_move();
        let orig_side = orig_board.base.get_side(side);
        let res_side = res_board.base.get_side(side);

        if orig_side.pawns != res_side.pawns {
            Some(PAWN_VAL as f32)
        } else if orig_side.queens != res_side.queens {
            Some(QUEEN_VAL as f32)
        } else if orig_side.rooks != res_side.rooks {
            Some(ROOK_VAL as f32)
        } else if orig_side.bishops != res_side.bishops {
            Some(BISHOP_VAL as f32)
        } else if orig_side.knights != res_side.knights {
            Some(KNIGHT_VAL as f32)
        } else {
            None
        }
    }
}

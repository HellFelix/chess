use chess_backend::*;
use log::debug;

use super::modifiers::{BISHOP_VAL, KNIGHT_VAL, PAWN_VAL, QUEEN_VAL, ROOK_VAL};
use crate::engine::{tree::Branch, utils::eval::Eval};

// priority modifiers

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

            res -= depth as f32 * DEPTH_PENALTY;

            if let Some(capture_val) = Self::is_capture(orig_board, res_board) {
                res += capture_val * (CAPTURE_BONUS - DEPTH_PENALTY * depth as f32);
            }

            Eval::Numeric(res)
        } else {
            heuristic
        }
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

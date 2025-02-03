use chess_backend::{Board, ChessMove, Colour, FinishedState, GameState, MoveType, Piece, Pieces};
use log::debug;

mod evaluation;
pub mod modifiers;
use super::{tree::Branch, utils::eval::Eval};
use modifiers::{BISHOP_VAL, KING_VAL, KNIGHT_VAL, PAWN_VAL, QUEEN_VAL, ROOK_VAL};
mod priority;

impl Branch {
    pub fn eval_position(&mut self, mobility: usize, depth: usize) -> Eval {
        match self.board.get_unchecked_game_state(mobility) {
            GameState::Ongoing => self.eval_heuristic(),
            GameState::Finished(state) => {
                self.game_over = true;
                match state {
                    // With a finished state, the evaluation is absolute.
                    FinishedState::Win(c, _) => match c {
                        Colour::White => Eval::Mate(depth, Colour::White),
                        Colour::Black => Eval::Mate(depth, Colour::Black),
                    },
                    FinishedState::Draw(_) => {
                        debug!("Draw");
                        Eval::Numeric(0.)
                    }
                }
            }
        }
    }
}

pub fn piece_val(piece: Piece) -> f32 {
    match piece {
        Piece::Pawn(_) => PAWN_VAL as f32,
        Piece::King(_) => KING_VAL as f32,
        Piece::Queen(_) => QUEEN_VAL as f32,
        Piece::Rook(_) => ROOK_VAL as f32,
        Piece::Bishop(_) => BISHOP_VAL as f32,
        Piece::Knight(_) => KNIGHT_VAL as f32,
    }
}

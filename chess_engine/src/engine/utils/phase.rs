use chess_backend::{Board, Pieces};
use sqlite::Connection;

#[derive(Clone, Copy, Debug)]
pub enum GamePhase {
    Opening(i64),
    MiddleGame,
    EndGame,
}
impl GamePhase {
    pub fn determine_phase(
        current_phase: Option<GamePhase>,
        white_pieces: &Pieces,
        black_pieces: &Pieces,
    ) -> Self {
        if let Some(phase) = current_phase {
            match phase {
                GamePhase::Opening(id) => {
                    unimplemented!()
                    // Check if opening database can still be used
                }
                GamePhase::MiddleGame => Self::determine_middle_or_end(white_pieces, black_pieces),
                GamePhase::EndGame => GamePhase::EndGame,
            }
        } else {
            // Start by trying to find the position in the opening database. If it cannot be found,
            // determine middle vs endgame (Same as in middlegame)
            Self::determine_middle_or_end(white_pieces, black_pieces)
        }
    }

    fn determine_middle_or_end(white_pieces: &Pieces, black_pieces: &Pieces) -> GamePhase {
        if piece_count(white_pieces) + piece_count(black_pieces) <= 14 {
            GamePhase::EndGame
        } else {
            GamePhase::MiddleGame
        }
    }
}

macro_rules! gen_piece_count {
    ($($param:ident),*) => {
        fn piece_count(pieces: &Pieces) -> usize {
            let mut res = 0;
            $(
                res += pieces.$param.len();
            )*
            res
        }
    };
}
gen_piece_count!(king, queens, bishops, knights, rooks, pawns);

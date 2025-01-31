use log::{debug, error, info};
use std::cmp::max_by;

use sqlite::{Connection, State};

use chess_backend::Board;

use crate::engine::utils::{error::EngineError, phase::GamePhase};

#[derive(Debug, Clone)]
struct BookMove {
    pub id: i64,
    pub parent_move: i64,
    pub san: String,
    pub eval: f64,
    pub freq: i64,
    pub terminal: bool,
}

const CONSENSUS_THRESHOLD: i64 = 3;

pub fn play_bookmove(
    db_conn: &Connection,
    id: i64,
) -> Result<(Option<Board>, Option<GamePhase>), EngineError> {
    debug!("Attempting to find book move");
    if let Some(chosen) = find_best_by_parent(db_conn, id) {
        let mut stm_req = db_conn
            .prepare("SELECT * FROM boards WHERE id = :id")
            .unwrap();
        stm_req.bind((":id", chosen.id)).unwrap();
        if let Ok(State::Row) = stm_req.next() {
            let res_board = Board::from(stm_req.read::<String, _>("fen").unwrap());
            let phase = if chosen.terminal {
                Some(GamePhase::MiddleGame)
            } else {
                Some(GamePhase::Opening(chosen.id))
            };
            Ok((Some(res_board), phase))
        } else {
            error!("Failed to find board from chosen book move");
            Err(EngineError::DatabaseError)
        }
    } else {
        Ok((None, Some(GamePhase::MiddleGame)))
    }
}

fn find_best_by_parent(db_conn: &Connection, id: i64) -> Option<BookMove> {
    let mut stm = db_conn
        .prepare("SELECT * FROM moves WHERE parent_move = :id AND frequency >= :ct")
        .unwrap();
    stm.bind((":id", id)).unwrap();
    stm.bind((":ct", CONSENSUS_THRESHOLD)).unwrap();

    let mut children = Vec::new();
    while let Ok(State::Row) = stm.next() {
        children.push(BookMove {
            id: stm.read::<i64, _>("id").unwrap(),
            parent_move: stm.read::<i64, _>("parent_move").unwrap(),
            san: stm.read::<String, _>("san").unwrap(),
            eval: stm.read::<f64, _>("eval").unwrap(),
            freq: stm.read::<i64, _>("frequency").unwrap(),
            terminal: stm.read::<i64, _>("terminal").unwrap() == 1,
        });
    }

    if let Some(res) = children.iter().max_by(|bm1, bm2| bm1.freq.cmp(&bm2.freq)) {
        Some(res.clone())
    } else {
        None
    }
}

pub fn search_manual(db_conn: &Connection, parent_id: i64, san: String) -> Option<GamePhase> {
    let mut stm = db_conn
        .prepare("SELECT * FROM moves WHERE san = :san AND parent_move = :parent_id")
        .unwrap();
    stm.bind((":san", &san[..])).unwrap();
    stm.bind((":parent_id", parent_id)).unwrap();

    if let Ok(State::Row) = stm.next() {
        let id = stm.read::<i64, _>("id").unwrap();
        let terminal = stm.read::<i64, _>("terminal").unwrap() == 1;
        if terminal {
            None
        } else {
            Some(GamePhase::Opening(id))
        }
    } else {
        None
    }
}

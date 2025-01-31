use core::panic;
use std::{fs, path::Path};

use pgn_parser::{parse_pgn, Move, ParsedGame};
use sqlite::{self, Connection, Value};

use chess_backend::{Board, ChessError, SanMove, START_POSITION};

const INIT_COMMAND: &str = "
    CREATE TABLE moves (
        id INTEGER PRIMARY KEY AUTOINCREMENT,   -- Unique identifier
        parent_move INTEGER NOT NULL,           -- Shared by all children of the parent node
        children INTEGER DEFAULT 0,             -- Number of children form this node
        san TEXT NOT NULL,                      -- San for the move (For the origin node, this is simply 'Origin')
        eval REAL NOT NULL,                     -- Predetermined evaluation to determine what move to play
        frequency INTEGER,                      -- The number of games this move was made
        terminal BOOL NOT NULL DEFAULT TRUE     -- False if there are further children node, else true
    );
    CREATE TABLE boards (
        id INTEGER NOT NULL,
        fen TEXT NOT NULL
    );

    INSERT INTO moves (parent_move, san, eval) VALUES (0, 'Origin', 1);
    INSERT INTO boards (id, fen) VALUES (1, 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1');
    ";

fn main() {
    let db_name = "openings.db";
    let pre_existing_db = Path::new(db_name).exists();

    let conn = sqlite::open(db_name).unwrap();

    if !pre_existing_db {
        conn.execute(INIT_COMMAND)
            .expect("Failed to initialize table");
    }

    chess_backend::init();
    add_from_file(&conn, "games.pgn");
    println!("Done");
}

fn add_from_file(conn: &Connection, file_name: &str) {
    let pgn_data = fs::read_to_string(file_name).unwrap();
    let games = pgn_data.split("[Event");

    let mut game_count = 0;
    for game_pgn in games {
        if let Some(start_index) = game_pgn.find("\n") {
            if let Ok(game) = parse_pgn(&game_pgn[start_index..]) {
                add_game(&conn, game);

                println!("Added game {game_count}");
                game_count += 1;
            }
        }
    }

    println!("Finishing up...");
    conn.execute("UPDATE moves SET terminal = TRUE WHERE children < 3")
        .unwrap();
}

fn add_game(conn: &Connection, game: ParsedGame) {
    let mut metadata = game.metadata.iter();
    let white_elo = metadata
        .find(|v| v.0 == "WhiteElo")
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();

    let black_elo = metadata
        .find(|v| v.0 == "BlackElo")
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();

    let res: Vec<&str> = game.game_result.split("-").collect();
    let white_score = result_conv(res[0]);
    let black_score = result_conv(res[1]);

    let mut parent_move = find_move(&conn, "Origin", 0).unwrap();
    let mut move_number = 1;
    let mut board = Board::from(START_POSITION);
    for m in game.moves {
        let white_move = fix_move_format(m.0);
        parent_move = if let Ok(id) = add_move(
            conn,
            &white_move,
            parent_move,
            eval_modifier(white_score, white_elo),
            &mut board,
        ) {
            id
        } else {
            break;
        };

        if let Some(m1) = m.1 {
            let black_move = fix_move_format(m1);

            parent_move = if let Ok(id) = add_move(
                conn,
                &black_move,
                parent_move,
                eval_modifier(black_score, black_elo),
                &mut board,
            ) {
                id
            } else {
                break;
            }
        } else {
            break;
        }

        move_number += 1;
        // Go no deeper than 12 moves
        if move_number == 12 {
            break;
        }
    }
}

fn result_conv(res: &str) -> f32 {
    match res {
        "1" => 1.,
        "1/2" => 0.5,
        "0" => 0.2,
        _ => 0.,
    }
}

fn fix_move_format(m: Move) -> String {
    let s = m.to_string();
    let end_index = s.find("{").unwrap();
    s[..end_index].trim().to_string()
}

fn eval_modifier(score: f32, elo: usize) -> f32 {
    score * (elo as f32) / 1000.
}

fn add_move(
    conn: &Connection,
    san: &str,
    parent_move: i64,
    eval: f32,
    board: &mut Board,
) -> Result<i64, ChessError> {
    if let Some(id) = find_move(conn, san, parent_move) {
        conn.execute(format!(
            "UPDATE moves SET eval = eval + {eval}, frequency = frequency + 1 WHERE id = {id}"
        ))
        .unwrap();
        board.make_san_move(SanMove::from_string(san, *board)?)?;
        Ok(id)
    } else {
        conn.execute(format!(
            "INSERT INTO moves (parent_move, san, eval, frequency) VALUES ({parent_move}, '{san}', {eval}, 1)"
        ))
        .expect(&format!(
            "Failed to insert move with san '{san}' and parent {parent_move}"
        ));

        let id = find_move(conn, san, parent_move).unwrap();
        board.make_san_move(SanMove::from_string(san, *board)?)?;
        let fen = board.into_fen();
        conn.execute(format!(
            "INSERT INTO boards (id, fen) VALUES ({id}, '{fen}')"
        ))
        .expect(&format!(
            "Failed to insert board fen with san '{san}' and parent {parent_move}"
        ));

        conn.execute(format!(
            "UPDATE moves SET terminal = FALSE WHERE id = {parent_move}"
        ))
        .expect(&format!(
            "Failed to update parent move with id {parent_move}"
        ));
        conn.execute(format!(
            "UPDATE moves SET children = children + 1 WHERE id = {parent_move}"
        ))
        .expect(&format!(
            "Failed to update children count for parent move with id {parent_move}"
        ));
        Ok(id)
    }
}

fn find_move(conn: &Connection, san: &str, parent_move: i64) -> Option<i64> {
    let mut stm = conn
        .prepare(format!(
            "SELECT id FROM moves WHERE san = '{san}' AND parent_move = {parent_move}"
        ))
        .unwrap();

    if let Some(row) = stm.iter().next() {
        if let Value::Integer(res) = row.unwrap().take(0) {
            Some(res)
        } else {
            panic!("Retreived non-integer id");
        }
    } else {
        None
    }
}

mod tests;

use env_logger::{self};
use log::debug;
use std::{thread, time::Duration};

mod engine;
use chess_backend::{init, Board, START_POSITION};
use engine::{tree::Branch, EngineController};

fn main() {
    env_logger::init();

    //test_position_eval();
    test_move_pick();

    // let mut controller = EngineController::default();
    // controller.set_black(engine::Player::Manual);

    // controller.play().unwrap();
}

fn test_position_eval() {
    init();

    let board = Board::from("rnb1kb1r/pp2qppp/2pp4/1B2N3/5P2/P1P5/1PP3PP/R1BQK2R b KQkq - 0 8");
    //let board = Board::from("rn1q1rk1/ppp1ppb1/3p1n1p/3N4/2BPP1bp/5N2/PPP2PPP/2RQK2R w K - 0 10");

    let eval = Branch::test_eval(board);
    println!("{eval:?}");
}

fn test_move_pick() {
    let board =
        Board::from("rn1qkb1r/ppp1pp1p/3p1np1/8/3PP1b1/2N2Q2/PPP2PPP/R1B1KBNR w KQkq - 2 5");

    let mut controller = EngineController::new(
        engine::Player::Engine,
        engine::Player::Manual,
        board,
        num_cpus::get(),
        Duration::from_secs(2),
    );
    controller.play();
}

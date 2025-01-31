use std::time::SystemTime;

use chess_backend::{init, Board, START_POSITION};

#[test]
fn bench_eval() {
    // init();
    // let board = Board::from(START_POSITION);
    // let start = SystemTime::now();
    // let val = eval_position(
    //     &board,
    //     board.generate_legal_moves().iter().map(|m| m.board).len(),
    // );
    // let stop = start.elapsed();

    // println!(
    //     "Evaluation took {}us and got value {}",
    //     stop.unwrap().as_micros(),
    //     val
    // );
}

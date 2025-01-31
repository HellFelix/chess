use crate::board::Board;
use crate::utils::extract_squares;
use crate::utils::squares::*;
use crate::START_POSITION;

#[test]
fn test_squares_parse() {
    assert_eq!(from_str(&to_str(a1).unwrap()), a1);
}

#[test]
fn test_extraction() {
    assert_eq!(vec![a1], extract_squares(1_u64 << a1));
    assert_eq!(vec![b5], extract_squares(1_u64 << b5));
    assert_eq!(vec![g6], extract_squares(1_u64 << g6));
    assert_eq!(vec![h7], extract_squares(1_u64 << h7));
    assert_eq!(vec![d8], extract_squares(1_u64 << d8));

    assert_eq!(
        vec![a1, b5, g6, h7, d8],
        extract_squares(
            (1_u64 << a1) + (1_u64 << g6) + (1_u64 << d8) + (1_u64 << b5) + (1_u64 << h7)
        )
    );
}

#[test]
fn test_fen() {
    let board = Board::from(START_POSITION);
    assert_eq!(vec![a1, h1], extract_squares(board.base.white.rooks));
    assert_eq!(vec![b1, g1], extract_squares(board.base.white.knights));
    assert_eq!(vec![c1, f1], extract_squares(board.base.white.bishops));
    assert_eq!(vec![d1], extract_squares(board.base.white.queens));
    assert_eq!(vec![e1], extract_squares(board.base.white.king));
    assert_eq!(
        vec![a2, b2, c2, d2, e2, f2, g2, h2],
        extract_squares(board.base.white.pawns)
    );

    assert_eq!(vec![a8, h8], extract_squares(board.base.black.rooks));
    assert_eq!(vec![b8, g8], extract_squares(board.base.black.knights));
    assert_eq!(vec![c8, f8], extract_squares(board.base.black.bishops));
    assert_eq!(vec![d8], extract_squares(board.base.black.queens));
    assert_eq!(vec![e8], extract_squares(board.base.black.king));
    assert_eq!(
        vec![a7, b7, c7, d7, e7, f7, g7, h7],
        extract_squares(board.base.black.pawns)
    );
}

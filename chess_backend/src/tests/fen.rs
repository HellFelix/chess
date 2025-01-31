use crate::{
    Board, CASTLE_KINGSIDE_POSITION, CASTLE_QUEENSIDE_POSITION, CHECK_POSITION, CMK_POSITION,
    EMPTY_BOARD, KILLER_POSITION, PROMOTION_POSITION, START_POSITION, TRICKY_POSITION,
};

#[test]
fn fen_conversion() {
    for pos in [
        EMPTY_BOARD,
        START_POSITION,
        CASTLE_KINGSIDE_POSITION,
        CASTLE_QUEENSIDE_POSITION,
        PROMOTION_POSITION,
        CHECK_POSITION,
        TRICKY_POSITION,
        KILLER_POSITION,
        CMK_POSITION,
    ] {
        let board = Board::from(pos);

        let first_iter_fen = board.into_fen();
        assert_eq!(first_iter_fen.trim(), pos.trim());

        let second_iter_fen = Board::from(&first_iter_fen).into_fen();
        assert_eq!(first_iter_fen, second_iter_fen);
    }
}

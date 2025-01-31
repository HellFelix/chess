use chess_backend::{init, Board, Colour, Piece, SanMove};
use std::{fmt::Display, time::SystemTime};

#[test]
fn basic_notation() {
    init();
    let mut board = Board::default();

    let m = SanMove::new(
        Piece::Pawn(Colour::White),
        false,
        false,
        false,
        12,
        (false, false),
        28,
        None,
        None,
    );
    board.make_san_move(m);

    println!("{board}");
    println!("{m}");
}

#[test]
fn file_disambiguation() {
    init();
    let fen = "1k6/ppp5/8/7R/8/8/PPP5/1K2R3 w - - 0 1";

    let board = Board::from(fen);

    for m in board
        .generate_legal_moves()
        .iter()
        .filter(|m| m.base.destination_square.unwrap() == 36)
    {
        println!("{}", board.get_san(&m.board));
    }
}

#[test]
fn rank_disambiguation() {
    init();
    let fen = "1k6/ppp5/8/7R/8/8/PPP5/1K5R w - - 0 1";

    let board = Board::from(fen);

    for m in board
        .generate_legal_moves()
        .iter()
        .filter(|m| m.base.destination_square.unwrap() == 15)
    {
        println!("{}", board.get_san(&m.board));
    }
}

#[test]
fn double_disambiguation() {
    init();

    let fen = "1k6/ppp5/5N1N/8/8/8/PPP4N/1K6 w - - 0 1";

    let board = Board::from(fen);

    for m in board
        .generate_legal_moves()
        .iter()
        .filter(|m| m.base.destination_square.unwrap() == 30)
    {
        println!("{}", board.get_san(&m.board));
    }
}

#[test]
fn kingside_castle() {
    init();

    let fen = "1k6/ppp5/8/8/8/8/8/4K2R w K - 0 1";

    let board = Board::from(fen);

    for m in board.generate_legal_moves() {
        println!("{}", board.get_san(&m.board));
    }
}

#[test]
fn queenside_castle() {
    init();

    let fen = "1k6/ppp5/8/8/8/8/P7/R3K3 w Q - 0 1";

    let board = Board::from(fen);

    for m in board.generate_legal_moves() {
        println!("{}", board.get_san(&m.board));
    }
}

#[test]
fn promotion_san() {
    init();
    let fen = "1k6/ppp3P1/8/8/8/8/8/4K3 w - - 0 1";

    let board = Board::from(fen);

    for m in board.generate_legal_moves() {
        println!("{}", board.get_san(&m.board));
    }
}

#[test]
fn promotion_disambiguation() {
    init();
    let fen = "1k4rb/ppp2P2/8/8/8/8/8/5K2 w - - 0 1";

    let board = Board::from(fen);

    for m in board.generate_legal_moves() {
        println!("{}", board.get_san(&m.board));
    }
}

#[test]
fn pawn_captures() {
    init();
    let fen = "rn1q1rk1/pp2bpp1/2p1pn1p/3p1b2/2PPP3/2N3P1/PP1N1PBP/R1BQ1RK1 b - e3 17 9";
    let board = Board::from(fen);

    for m in board.generate_legal_moves() {
        println!("{}", board.get_san(&m.board));
    }
}

#[test]
fn check() {
    init();
    let fen = "r1b1kb1r/pppn1ppp/3q1n2/3Np1B1/4P3/5N2/PPP2PPP/R2QKB1R w KQkq - 14 8";
    let board = Board::from(fen);

    for m in board.generate_legal_moves() {
        println!("{}", board.get_san(&m.board));
    }
}

#[test]
fn test_parsing() {
    init();
    let mut original_board =
        Board::from("rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq e6 0 3");
    println!("{original_board}");
    let res_board = Board::from("rnbqkbnr/pppp1ppp/8/4N3/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 1 3");
    if let Ok(m) = SanMove::from_string("Nxe5", original_board) {
        original_board.make_san_move(m);
        assert_eq!(original_board, res_board);
    }
}

#[test]
fn test_en_passent() {
    init();
    let fen = "r1bqkb1r/3pnppp/ppn1p3/8/B1pPP3/2P2N2/PP3PPP/RNBQR1K1 b kq d3 0 8";
    let board = Board::from(fen);

    for m in board.generate_legal_moves() {
        println!("{}", board.get_san(&m.board));
    }
}

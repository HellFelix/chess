#[cfg(test)]
mod move_gen {
    use std::time::SystemTime;

    use chess_backend::{
        init, Board, CASTLE_KINGSIDE_POSITION, CASTLE_QUEENSIDE_POSITION, CHECK_POSITION,
        PROMOTION_POSITION, START_POSITION,
    };

    #[test]
    fn standard_position() {
        init();
        let board = Board::from(START_POSITION);
        let start = SystemTime::now();
        let moves = board.generate_legal_moves();
        let stop = start.elapsed();
        for m in moves {
            println!("{}", m.board);
        }
        println!("It took {}us to generate moves", stop.unwrap().as_micros());
    }

    #[test]
    fn castle_kingside_position() {
        init();
        let board = Board::from(CASTLE_KINGSIDE_POSITION);
        for m in board.generate_legal_moves() {
            println!("{}", m.board);
        }
    }

    #[test]
    fn castle_queenside_position() {
        init();
        let board = Board::from(CASTLE_QUEENSIDE_POSITION);
        for m in board.generate_legal_moves() {
            println!("{}", m.board);
        }
    }

    #[test]
    fn promotion_position() {
        init();
        let board = Board::from(PROMOTION_POSITION);
        for m in board.generate_legal_moves() {
            println!("{}", m.board);
        }
    }

    #[test]
    fn check_position() {
        init();
        let board = Board::from(CHECK_POSITION);
        for m in board.generate_legal_moves() {
            println!("{}", m.board);
        }
    }
}

#[cfg(test)]
mod eval {
    use crate::engine::utils::eval::Eval;
    use chess_backend::Colour;

    #[test]
    fn test_paritalord() {
        // Checkmate eval for white
        assert!(Eval::Mate(1, Colour::White) > Eval::Mate(3, Colour::White));
        assert!(Eval::Mate(2, Colour::White) >= Eval::Mate(2, Colour::White));
        assert!(Eval::Mate(1, Colour::White) >= Eval::Mate(3, Colour::White));

        assert!(Eval::Mate(6, Colour::White) < Eval::Mate(3, Colour::White));
        assert!(Eval::Mate(5, Colour::White) <= Eval::Mate(5, Colour::White));
        assert!(Eval::Mate(6, Colour::White) <= Eval::Mate(3, Colour::White));

        // Checkmate eval for black
        assert!(Eval::Mate(1, Colour::Black) < Eval::Mate(3, Colour::Black));
        assert!(Eval::Mate(2, Colour::Black) <= Eval::Mate(2, Colour::Black));
        assert!(Eval::Mate(1, Colour::Black) <= Eval::Mate(3, Colour::Black));

        assert!(Eval::Mate(6, Colour::Black) > Eval::Mate(3, Colour::Black));
        assert!(Eval::Mate(5, Colour::Black) >= Eval::Mate(5, Colour::Black));
        assert!(Eval::Mate(6, Colour::Black) >= Eval::Mate(3, Colour::Black));

        // Checkmate white vs black
        assert!(Eval::Mate(5, Colour::White) > Eval::Mate(2, Colour::Black));

        // Numerics
        assert!(Eval::Numeric(7.) > Eval::Numeric(4.));
        assert!(Eval::Numeric(4.) >= Eval::Numeric(4.));
        assert!(Eval::Numeric(7.) >= Eval::Numeric(4.));

        assert!(Eval::Numeric(4.) < Eval::Numeric(7.));
        assert!(Eval::Numeric(4.) <= Eval::Numeric(4.));
        assert!(Eval::Numeric(4.) <= Eval::Numeric(7.));

        // Numerics vs checkmate
        assert!(Eval::Mate(9, Colour::White) > Eval::Numeric(100.));
        assert!(Eval::Numeric(100.) < Eval::Mate(9, Colour::White));
        assert!(Eval::Mate(1, Colour::Black) < Eval::Numeric(-100.));
        assert!(Eval::Numeric(-100.) > Eval::Mate(1, Colour::Black));

        // Numerics vs infinity
        assert!(Eval::Numeric(0.) < Eval::Infinity);
        assert!(Eval::Infinity > Eval::Numeric(0.));
        assert!(Eval::Numeric(-1.) > Eval::NegInfinity);
        assert!(Eval::NegInfinity < Eval::Numeric(-1.));

        // checkmate vs infinity
        assert!(Eval::Mate(1, Colour::White) < Eval::Infinity);
        assert!(Eval::Infinity > Eval::Mate(1, Colour::White));
        assert!(Eval::Mate(1, Colour::Black) > Eval::NegInfinity);
        assert!(Eval::NegInfinity < Eval::Mate(1, Colour::Black));

        // maximizing
        assert_eq!(
            Eval::Mate(1, Colour::White).max(Eval::Infinity),
            Eval::Infinity
        );

        assert_eq!(Eval::Numeric(8.).min(Eval::Infinity), Eval::Numeric(8.));
    }
}

#[cfg(test)]
mod engine;

#[cfg(test)]
mod san;

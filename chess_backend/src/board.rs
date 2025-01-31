use crate::{
    bishopTargets, bitboard_base, castling_rights, createBase, generateAttackTargets, kingTargets,
    knightTargets, pawnAttackTargets, pawnTargets, piece_map_bitboards, queenTargets, rookTargets,
    utils::{extract_squares, squares::*, Colour, Piece, Pieces},
    START_POSITION,
};
use core::panic;
use std::fmt::Display;

type TargetFunction = Box<dyn Fn(i32) -> u64>;
type MutateFunction = Box<dyn Fn(&mut Board)>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    King,
    Queen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    Standard,
    Castling(Side),
    EnPassent,
    Promotion(Piece),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    // in order to avoid generating moves multiple times, the moves are inherited
    // with the "Ongoing" state.
    Ongoing,
    Finished(FinishedState),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinishedState {
    Win(Colour, ReasonWin),
    Draw(ReasonDraw),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonWin {
    Checkmate,
    Resignation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonDraw {
    Stalemate,
    InsufficientMaterial,
    HalfmoveLimit,
    Agreement,
}

impl bitboard_base {
    pub fn get_side(&self, colour: Colour) -> piece_map_bitboards {
        match colour {
            Colour::White => self.white,
            Colour::Black => self.black,
        }
    }

    fn get_side_occupied(&self, colour: Colour) -> u64 {
        match colour {
            Colour::White => self.white_occupied,
            Colour::Black => self.black_occupied,
        }
    }

    pub fn make_move(
        &self,
        piece: Piece,
        starting_square: u64,
        destination_square: u64,
        colour: Colour,
        prom_piece: Option<Piece>,
    ) -> Self {
        let mut side = self.get_side(colour);
        let mut other = self.get_side(colour.other());
        if let Some(res_piece) = prom_piece {
            let side_bitboard = match res_piece {
                Piece::Queen(_c) => &mut side.queens,
                Piece::Bishop(_c) => &mut side.bishops,
                Piece::Knight(_c) => &mut side.knights,
                Piece::Rook(_c) => &mut side.rooks,
                // Pawn and King is not allowed as a promotion piece
                _ => panic!("Invalid promotion Piece"),
            };
            side.pawns ^= starting_square;
            *side_bitboard ^= destination_square;
        } else {
            let side_bitboard = match piece {
                Piece::Pawn(_c) => &mut side.pawns,
                Piece::King(_c) => &mut side.king,
                Piece::Queen(_c) => &mut side.queens,
                Piece::Bishop(_c) => &mut side.bishops,
                Piece::Knight(_c) => &mut side.knights,
                Piece::Rook(_c) => &mut side.rooks,
            };

            *side_bitboard ^= starting_square | destination_square;
        }

        for other_bitboard in vec![
            &mut other.pawns,
            &mut other.king,
            &mut other.queens,
            &mut other.bishops,
            &mut other.knights,
            &mut other.rooks,
        ] {
            *other_bitboard ^= destination_square;
            *other_bitboard &= !destination_square;
        }

        if colour == Colour::White {
            let white = side;
            let black = other;
            unsafe {
                return createBase(white, black);
            }
        } else {
            let white = other;
            let black = side;
            unsafe {
                return createBase(white, black);
            }
        }
    }

    fn get_pseudo_legal_piece_moves(
        &self,
        colour: Colour,
        piece: Piece,
        bitboard: u64,
        target_function: TargetFunction,
    ) -> Vec<(ChessMoveBase, Option<MutateFunction>)> {
        let mut res = Vec::new();
        for square in extract_squares(bitboard) {
            let side_occupied = self.get_side_occupied(colour);
            for target in
                extract_squares((target_function(square) ^ side_occupied) & !side_occupied)
            {
                let res_board = self.make_move(piece, 1 << square, 1 << target, colour, None);
                let f =
                    if piece == Piece::Pawn(Colour::White) && target - square == 16 {
                        Some(Box::new(move |b: &mut Board| b.killer_square = square + 8)
                            as MutateFunction)
                    } else if piece == Piece::Pawn(Colour::Black) && square - target == 16 {
                        Some(Box::new(move |b: &mut Board| b.killer_square = square - 8)
                            as MutateFunction)
                    } else if let Piece::King(c) = piece {
                        match c {
                            Colour::White => Some(Box::new(|b: &mut Board| {
                                b.castling_rights.white_king = false;
                                b.castling_rights.white_queen = false;
                            }) as MutateFunction),
                            Colour::Black => Some(Box::new(|b: &mut Board| {
                                b.castling_rights.black_king = false;
                                b.castling_rights.black_queen = false;
                            }) as MutateFunction),
                        }
                    } else if let Piece::Rook(c) = piece {
                        match c {
                            Colour::White => {
                                if square == a1 {
                                    Some(Box::new(|b: &mut Board| {
                                        b.castling_rights.white_queen = false
                                    }) as MutateFunction)
                                } else if square == h1 {
                                    Some(Box::new(|b: &mut Board| {
                                        b.castling_rights.white_king = false
                                    }) as MutateFunction)
                                } else {
                                    None
                                }
                            }
                            Colour::Black => {
                                if square == a8 {
                                    Some(Box::new(|b: &mut Board| {
                                        b.castling_rights.black_queen = false
                                    }) as MutateFunction)
                                } else if square == h8 {
                                    Some(Box::new(|b: &mut Board| {
                                        b.castling_rights.black_king = false
                                    }) as MutateFunction)
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        None
                    };
                res.push((
                    ChessMoveBase {
                        starting_square: Some(square),
                        destination_square: Some(target),
                        piece,
                        move_type: MoveType::Standard,
                        colour,
                        res_board,
                    },
                    f,
                ))
            }
        }
        res
    }

    fn get_pseudo_legal_castling(
        &self,
        colour: Colour,
    ) -> Vec<(ChessMoveBase, Option<MutateFunction>)> {
        let (king_side, queen_side, mutate): (
            bitboard_base,
            bitboard_base,
            (MutateFunction, MutateFunction),
        ) = match colour {
            Colour::White => {
                let mut king_side = self.white;
                let mut queen_side = self.white;
                king_side.king ^= 80;
                king_side.rooks ^= 160;

                queen_side.king ^= 20;
                queen_side.rooks ^= 9;

                let mutate = (
                    Box::new(|b: &mut Board| {
                        b.castling_rights.white_king = false;
                        b.castling_rights.white_queen = false;
                    }) as MutateFunction,
                    Box::new(|b: &mut Board| {
                        b.castling_rights.white_king = false;
                        b.castling_rights.white_queen = false;
                    }) as MutateFunction,
                );
                unsafe {
                    (
                        createBase(king_side, self.black),
                        createBase(queen_side, self.black),
                        mutate,
                    )
                }
            }
            Colour::Black => {
                let mut king_side = self.black;
                let mut queen_side = self.black;
                king_side.king ^= 5764607523034234880;
                king_side.rooks ^= 11529215046068469760;

                queen_side.king ^= 1441151880758558720;
                queen_side.rooks ^= 648518346341351424;

                let mutate = (
                    Box::new(|b: &mut Board| {
                        b.castling_rights.black_king = false;
                        b.castling_rights.black_queen = false;
                    }) as MutateFunction,
                    Box::new(|b: &mut Board| {
                        b.castling_rights.black_king = false;
                        b.castling_rights.black_queen = false;
                    }) as MutateFunction,
                );
                unsafe {
                    (
                        createBase(self.white, king_side),
                        createBase(self.white, queen_side),
                        mutate,
                    )
                }
            }
        };

        vec![
            (
                ChessMoveBase {
                    starting_square: None,
                    destination_square: None,
                    piece: Piece::King(colour),
                    move_type: MoveType::Castling(Side::King),
                    colour,
                    res_board: king_side,
                },
                Some(mutate.0),
            ),
            (
                ChessMoveBase {
                    starting_square: None,
                    destination_square: None,
                    piece: Piece::King(colour),
                    move_type: MoveType::Castling(Side::Queen),
                    colour,
                    res_board: queen_side,
                },
                Some(mutate.1),
            ),
        ]
    }

    fn get_enpassent_move(
        &self,
        start_square: i32,
        killer_square: i32,
        colour: Colour,
    ) -> (ChessMoveBase, Option<MutateFunction>) {
        let mut white = self.white;
        let mut black = self.black;
        match colour {
            Colour::White => {
                white.pawns ^= (1 << start_square) + (1 << killer_square);
                black.pawns ^= 1 << (killer_square - 8);
            }
            Colour::Black => {
                black.pawns ^= (1 << start_square) + (1 << killer_square);
                white.pawns ^= 1 << (killer_square + 8);
            }
        };
        unsafe {
            let res_board = createBase(white, black);
            (
                ChessMoveBase {
                    starting_square: Some(start_square),
                    destination_square: Some(killer_square),
                    piece: Piece::Pawn(colour),
                    move_type: MoveType::EnPassent,
                    colour,
                    res_board,
                },
                None,
            )
        }
    }

    fn add_promotion_moves(
        &self,
        back_rank: u64,
        colour: Colour,
        pawns: u64,
        occupancy: u64,
        res: &mut Vec<(ChessMoveBase, Option<MutateFunction>)>,
    ) {
        for starting_square in extract_squares(pawns) {
            unsafe {
                for target in
                    extract_squares(pawnTargets(starting_square, colour.as_int(), occupancy))
                {
                    if 1u64 << target & back_rank != 0 {
                        for p in [
                            Piece::Queen(colour),
                            Piece::Rook(colour),
                            Piece::Knight(colour),
                            Piece::Bishop(colour),
                        ] {
                            res.push((
                                ChessMoveBase {
                                    starting_square: Some(starting_square),
                                    destination_square: Some(target),
                                    piece: Piece::Pawn(colour),
                                    move_type: MoveType::Promotion(p),
                                    colour,
                                    res_board: self.make_move(
                                        Piece::Pawn(colour),
                                        1u64 << starting_square,
                                        1u64 << target,
                                        colour,
                                        Some(p),
                                    ),
                                },
                                None,
                            ))
                        }
                    }
                }
            }
        }
    }

    fn get_promotion_moves(
        &self,
        colour: Colour,
        pawns: u64,
        occupancy: u64,
    ) -> Vec<(ChessMoveBase, Option<MutateFunction>)> {
        let mut res = Vec::new();
        if colour == Colour::White && pawns & 71776119061217280 != 0 {
            self.add_promotion_moves(18374686479671623680, colour, pawns, occupancy, &mut res);
        } else if colour == Colour::Black && pawns & 65280 != 0 {
            self.add_promotion_moves(255, colour, pawns, occupancy, &mut res);
        }
        res
    }

    pub fn get_pseudo_legal_moves(
        &self,
        colour: Colour,
        killer_square: i32,
    ) -> Vec<(ChessMoveBase, Option<MutateFunction>)> {
        let occupancy = self.white_occupied + self.black_occupied;
        let mut res = Vec::new();

        // --- Pawn Moves ---
        res.append(&mut self.get_promotion_moves(colour, self.get_side(colour).pawns, occupancy));
        res.append(&mut self.get_pseudo_legal_piece_moves(
            colour,
            Piece::Pawn(colour),
            self.get_side(colour).pawns,
            Self::pawn_target_function(colour, occupancy),
        ));
        // En passent is only handled if the board has a killer square
        if killer_square >= 0 {
            unsafe {
                for square in extract_squares(self.get_side(colour).pawns) {
                    if 1 << killer_square as u32 & pawnAttackTargets(square, colour.as_int()) != 0 {
                        res.push(self.get_enpassent_move(square, killer_square, colour));
                    }
                }
            }
        }

        // --- King Moves ---
        res.append(&mut self.get_pseudo_legal_piece_moves(
            colour,
            Piece::King(colour),
            self.get_side(colour).king,
            Self::king_target_function(),
        ));
        // castling is always generated as pseudo-legal and might be removed when checking legality
        res.append(&mut self.get_pseudo_legal_castling(colour));

        // --- Queen Moves ---
        res.append(&mut self.get_pseudo_legal_piece_moves(
            colour,
            Piece::Queen(colour),
            self.get_side(colour).queens,
            Self::queen_target_function(occupancy),
        ));

        // --- Bishop Moves ---
        res.append(&mut self.get_pseudo_legal_piece_moves(
            colour,
            Piece::Bishop(colour),
            self.get_side(colour).bishops,
            Self::bishop_target_function(occupancy),
        ));

        // --- Knight Moves ---
        res.append(&mut self.get_pseudo_legal_piece_moves(
            colour,
            Piece::Knight(colour),
            self.get_side(colour).knights,
            Self::knight_target_function(),
        ));

        // --- Rook Moves ---
        res.append(&mut self.get_pseudo_legal_piece_moves(
            colour,
            Piece::Rook(colour),
            self.get_side(colour).rooks,
            Self::rook_target_function(occupancy),
        ));

        res
    }

    fn pawn_target_function(colour: Colour, occupancy: u64) -> TargetFunction {
        Box::new(move |s| unsafe { pawnTargets(s, colour.as_int(), occupancy) })
    }
    fn king_target_function() -> TargetFunction {
        Box::new(|s| unsafe { kingTargets(s) })
    }
    fn queen_target_function(occupancy: u64) -> TargetFunction {
        Box::new(move |s| unsafe { queenTargets(s, occupancy) })
    }
    fn bishop_target_function(occupancy: u64) -> TargetFunction {
        Box::new(move |s| unsafe { bishopTargets(s, occupancy) })
    }
    fn knight_target_function() -> TargetFunction {
        Box::new(|s| unsafe { knightTargets(s) })
    }
    fn rook_target_function(occupancy: u64) -> TargetFunction {
        Box::new(move |s| unsafe { rookTargets(s, occupancy) })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChessMoveBase {
    pub starting_square: Option<i32>,
    pub destination_square: Option<i32>,
    pub piece: Piece,
    pub move_type: MoveType,
    pub colour: Colour,
    pub res_board: bitboard_base,
}
impl ChessMoveBase {
    pub fn is_legal(
        &self,
        castling_rights: castling_rights,
        occupancy: u64,
        rooks: (u64, u64),
    ) -> bool {
        let side = self.res_board.get_side(self.colour);
        let other_side = self.res_board.get_side(self.colour.other());

        // Avoid addition overflow
        if self.res_board.white_occupied > u64::MAX - self.res_board.black_occupied {
            return false;
        }

        unsafe {
            let other_attacks = generateAttackTargets(
                other_side,
                self.colour.other().as_int(),
                self.res_board.white_occupied + self.res_board.black_occupied,
            );
            // king should not be in check after the move has been made
            if side.king & other_attacks != 0 {
                return false;
            }
            // pawns should not be on any back rank
            if side.pawns + other_side.pawns & 18374686479671623935 != 0 {
                return false;
            }
            if let MoveType::Castling(castling_side) = self.move_type {
                // When castling, the coresponding castling right must be true, there must be no
                // pieces between the king and rook, the rooks must not have been captured, and the
                // squares over which the king passes must not be attacked by enemy pieces
                if !((castling_side == Side::King
                    && self.colour == Colour::White
                    && castling_rights.white_king
                    && (96 & occupancy == 0)
                    && (128 & rooks.0 != 0)
                    && (112 & other_attacks == 0))
                    || (castling_side == Side::Queen
                        && self.colour == Colour::White
                        && castling_rights.white_queen
                        && (14 & occupancy == 0)
                        && (1 & rooks.0 != 0)
                        && (30 & other_attacks == 0))
                    || (castling_side == Side::King
                        && self.colour == Colour::Black
                        && castling_rights.black_king
                        && (6917529027641081856 & occupancy == 0)
                        && (9223372036854775808 & rooks.1 != 0)
                        && (8070450532247928832 & other_attacks == 0))
                    || (castling_side == Side::Queen
                        && self.colour == Colour::Black
                        && castling_rights.black_queen
                        && (1008806316530991104 & occupancy == 0)
                        && (72057594037927936 & rooks.1 != 0)
                        && (2017612633061982208 & other_attacks == 0)))
                {
                    return false;
                }
            }
            true
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChessMove {
    pub base: ChessMoveBase,
    pub board: Board,
}
impl ChessMove {
    pub fn get_base(&self) -> ChessMoveBase {
        self.base
    }
    pub fn from_base(
        base: ChessMoveBase,
        board: &Board,
        castling_rights: castling_rights,
        occupancy: u64,
        rooks: (u64, u64),
        mutate_function: Option<MutateFunction>,
    ) -> Option<Self> {
        if base.is_legal(castling_rights, occupancy, rooks) {
            let mut new_board = *board;
            new_board.base = base.res_board;
            new_board.side_to_move = board.side_to_move.other();
            new_board.halfmove += 1;
            if base.colour == Colour::Black {
                new_board.fullmove += 1;
            }

            // Reset killer square
            new_board.killer_square = -1;

            if let Some(f) = mutate_function {
                f(&mut new_board);
            }

            Some(Self {
                base,
                board: new_board,
            })
        } else {
            None
        }
    }
}

impl PartialEq for piece_map_bitboards {
    fn eq(&self, other: &Self) -> bool {
        self.king == other.king
            && self.queens == other.queens
            && self.bishops == other.bishops
            && self.knights == other.knights
            && self.rooks == other.rooks
            && self.pawns == other.pawns
    }
}

impl PartialEq for bitboard_base {
    fn eq(&self, other: &Self) -> bool {
        self.white == other.white
            && self.black == other.black
            && self.white_occupied == other.white_occupied
            && self.black_occupied == other.black_occupied
    }
}
impl PartialEq for castling_rights {
    fn eq(&self, other: &Self) -> bool {
        self.white_king == other.white_king
            && self.black_king == other.black_king
            && self.white_queen == other.white_queen
            && self.black_queen == other.black_queen
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    pub base: bitboard_base,
    killer_square: i32,
    castling_rights: castling_rights,
    side_to_move: Colour,
    halfmove: i32,
    fullmove: i32,
}
impl Board {
    pub fn new(
        base: bitboard_base,
        killer_square: i32,
        castling_rights: castling_rights,
        side_to_move: Colour,
        halfmove: i32,
        fullmove: i32,
    ) -> Self {
        Self {
            base,
            killer_square,
            castling_rights,
            side_to_move,
            halfmove,
            fullmove,
        }
    }
    pub fn side_to_move(&self) -> Colour {
        self.side_to_move
    }
    pub fn castling_rights_as_arr(&self) -> [bool; 4] {
        [
            self.castling_rights.white_king,
            self.castling_rights.white_queen,
            self.castling_rights.black_king,
            self.castling_rights.black_queen,
        ]
    }
    pub fn fullmove(&self) -> i32 {
        self.fullmove
    }
    pub fn halfmove(&self) -> i32 {
        self.halfmove
    }
    pub fn killer_square(&self) -> i32 {
        self.killer_square
    }
    pub fn view_bitboard(&self, colour: Colour, piece: Piece) {
        let bitboard_maps = match colour {
            Colour::White => self.base.white,
            Colour::Black => self.base.black,
        };

        let map = match piece {
            Piece::Pawn(_) => bitboard_maps.pawns,
            Piece::King(_) => bitboard_maps.king,
            Piece::Queen(_) => bitboard_maps.queens,
            Piece::Bishop(_) => bitboard_maps.bishops,
            Piece::Knight(_) => bitboard_maps.knights,
            Piece::Rook(_) => bitboard_maps.rooks,
        };

        println!("{}", BitBoard(map));
    }

    pub fn generate_legal_moves(&self) -> Vec<ChessMove> {
        let mut res = Vec::new();
        let occupancy = self.base.black_occupied + self.base.white_occupied;

        for (m, f) in self
            .base
            .get_pseudo_legal_moves(self.side_to_move, self.killer_square)
        {
            if let Some(legal_move) = ChessMove::from_base(
                m,
                self,
                self.castling_rights,
                occupancy,
                (self.base.white.rooks, self.base.black.rooks),
                f,
            ) {
                res.push(legal_move);
            }
        }
        res
    }

    pub fn get_game_state(&self) -> GameState {
        self.get_unchecked_game_state(self.generate_legal_moves().len())
    }

    pub fn get_unchecked_game_state(&self, mobility: usize) -> GameState {
        // Even though most positions should return Ongoing, there are some states that
        // should be checked before generating moves since they could yield a finished state
        // regardless of if there are "legal" moves.

        // --- Insufficient Material ---
        // Easiest way to skip further calculations is to check if there are pawns
        // on either side. This should skip further calculations in most positions.
        if self.base.white.pawns + self.base.black.pawns == 0 {
            if self.base.white.queens
                + self.base.black.queens
                + self.base.white.rooks
                + self.base.black.rooks
                == 0
            {
                // If at least one side has at least one queen or one rook, there is sufficient
                // material
                // There should be more checks here to allow for bishop and knights
                // TEMPORARY FOR TESTING
                return GameState::Finished(FinishedState::Draw(ReasonDraw::InsufficientMaterial));
            }
        }

        // --- Halfmove limit (yet to be implemented) ---

        // --- Resignation & Agreement (yet to be implemented) ---

        // From here on out, we require legal moves of this position
        if mobility != 0 {
            // --- Ongoing ---
            // Easiest way to check if the game is still ongoing is to check if there
            // are any legal moves in the current position.
            // If we have gotten this far in the game state checking and there are
            // legal moves, the game is still ongoing.
            GameState::Ongoing
        } else {
            // The game is over. Now it's a matter of figuring out why
            // The game has ended in either checkmate or stalemate

            GameState::Finished(
                // is the king in check?
                if self.is_check() {
                    // --- Checkmate ---
                    // If there are no legal moves and the king is in check, the game ends in
                    // checkmate
                    FinishedState::Win(self.side_to_move.other(), ReasonWin::Checkmate)
                } else {
                    // --- Stalemate ---
                    // If there are no legal moves and the king is not in check, the game ends in
                    // stalemate
                    FinishedState::Draw(ReasonDraw::Stalemate)
                },
            )
        }
    }
    pub fn is_check(&self) -> bool {
        let side = self.base.get_side(self.side_to_move);
        let other_side = self.base.get_side(self.side_to_move.other());

        let other_attacks = unsafe {
            generateAttackTargets(
                other_side,
                self.side_to_move.other().as_int(),
                self.base.white_occupied + self.base.black_occupied,
            )
        };

        side.king & other_attacks != 0
    }
}
impl Default for Board {
    fn default() -> Self {
        Self::from(START_POSITION)
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let white = Pieces::from(self.base.white);
        let black = Pieces::from(self.base.black);
        for i in 0..8 {
            // reversed because writing goes from right to left in order of write! macro
            for j in (0..8).rev() {
                if let Some(piece) = find_on_square(63 - (i * 8 + j), &white, &black) {
                    if let Some(symbol) = piece.symbol() {
                        write!(f, "{symbol} ")?;
                    } else {
                        return Err(std::fmt::Error);
                    }
                } else {
                    // Empty
                    write!(f, ". ")?;
                }
            }
            write!(f, "\n")?;
        }

        match self.side_to_move {
            Colour::White => writeln!(f, "White to move")?,
            Colour::Black => writeln!(f, "Black to move")?,
        }

        if let Some(square) = to_str(self.killer_square) {
            writeln!(f, "Board has killer square {square}")?;
        } else {
            writeln!(f, "Board has no killer square")?;
        }
        writeln!(f, "Board halfmove clock is {}", self.halfmove)?;
        writeln!(f, "Board fullmove is {}", self.fullmove)?;

        Ok(())
    }
}

fn find_on_square(square: i32, white: &Pieces, black: &Pieces) -> Option<Piece> {
    if white.pawns.contains(&square) {
        Some(Piece::Pawn(Colour::White))
    } else if black.pawns.contains(&square) {
        Some(Piece::Pawn(Colour::Black))
    } else if white.king.contains(&square) {
        Some(Piece::King(Colour::White))
    } else if black.king.contains(&square) {
        Some(Piece::King(Colour::Black))
    } else if white.queens.contains(&square) {
        Some(Piece::Queen(Colour::White))
    } else if black.queens.contains(&square) {
        Some(Piece::Queen(Colour::Black))
    } else if white.bishops.contains(&square) {
        Some(Piece::Bishop(Colour::White))
    } else if black.bishops.contains(&square) {
        Some(Piece::Bishop(Colour::Black))
    } else if white.knights.contains(&square) {
        Some(Piece::Knight(Colour::White))
    } else if black.knights.contains(&square) {
        Some(Piece::Knight(Colour::Black))
    } else if white.rooks.contains(&square) {
        Some(Piece::Rook(Colour::White))
    } else if black.rooks.contains(&square) {
        Some(Piece::Rook(Colour::Black))
    } else {
        None
    }
}

#[derive(Debug)]
pub struct BitBoard(pub u64);
impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bits: Vec<char> = format!("{:#0b}", self.0)[2..].chars().collect();
        for _ in 0..(64 - bits.len()) {
            bits.insert(0, '0');
        }
        let mut ranks: Vec<Vec<char>> = bits.chunks(8).map(|s| s.into()).collect();
        for rank in &mut ranks {
            rank.reverse();
            for square in rank {
                if *square == '0' {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{square} ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
impl BitBoard {
    // Ranks should be ordered 8-1 and squares in rank should be in order a-h
    pub fn from_ranks(mut ranks: [[u8; 8]; 8]) -> Self {
        let mut res = 0;
        let mut exp = 0;
        ranks.reverse();
        for rank in ranks {
            for square in rank {
                if square == 1 {
                    // only calculated if sqare is not 0
                    res += 2u64.pow(exp);
                }
                exp += 1;
            }
        }

        Self(res)
    }
}

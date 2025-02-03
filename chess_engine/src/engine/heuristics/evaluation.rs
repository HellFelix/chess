use chess_backend::*;

use super::modifiers::*;
use crate::engine::tree::Branch;
use crate::engine::utils::eval::Eval;
use crate::engine::utils::phase::GamePhase;

struct EvalData {
    pub game_phase: i32,
    pub mg_mobility: [i32; 2],
    pub eg_mobility: [i32; 2],
    pub attack_count: [i32; 2],
    pub attack_weight: [i32; 2],
    pub king_shield: [i32; 2],
    pub material_adjustement: [i32; 2],
    pub blockages: [i32; 2],
    pub positional_themes: [i32; 2],
}
impl Default for EvalData {
    fn default() -> Self {
        Self {
            game_phase: 0,
            mg_mobility: [0; 2],
            eg_mobility: [0; 2],
            attack_count: [0; 2],
            attack_weight: [0; 2],
            king_shield: [0; 2],
            material_adjustement: [0; 2],
            blockages: [0; 2],
            positional_themes: [0; 2],
        }
    }
}

fn to_index(colour: Colour) -> usize {
    match colour {
        Colour::White => 0,
        Colour::Black => 1,
    }
}

impl Branch {
    pub fn eval_heuristic(&self) -> Eval {
        let mut res = 0;
        let mut eval_data = EvalData::default();

        let white_pieces = Pieces::from(self.board.base.white);
        let black_pieces = Pieces::from(self.board.base.black);
        let side = self.board.side_to_move();

        let (mut eval_mg, mut eval_eg) = Self::eval_material(&white_pieces, &black_pieces);
        Self::eval_shield(&white_pieces, &black_pieces, &mut eval_data);
        eval_mg += eval_data.king_shield[0] - eval_data.king_shield[1];

        Self::eval_blocked_pieces(&white_pieces, &black_pieces, &mut eval_data);

        res += self.tempo_bonus(side);
        res += self.combination_adjustment(&white_pieces, &black_pieces);
        res += self.eval_structure(&white_pieces, &black_pieces);

        self.eval_pieces(&white_pieces, &black_pieces, &mut eval_data);

        res += Self::eval_pawn_atttacks(self.board, &white_pieces, &black_pieces);

        eval_mg += eval_data.mg_mobility[0] - eval_data.mg_mobility[1];
        eval_eg += eval_data.eg_mobility[0] - eval_data.eg_mobility[1];

        eval_data.game_phase = if eval_data.game_phase > 24 {
            24
        } else {
            eval_data.game_phase
        };

        let mg_weight = eval_data.game_phase;
        let eg_weight = 24 - eval_data.game_phase;

        res += ((eval_mg * mg_weight) + (eval_eg * eg_weight)) / 24;

        res += eval_data.blockages[0] - eval_data.blockages[1];
        res += eval_data.positional_themes[0] - eval_data.positional_themes[1];
        res += eval_data.material_adjustement[0] - eval_data.material_adjustement[1];

        if eval_data.attack_count[0] < 2 || white_pieces.queens.len() == 0 {
            eval_data.attack_weight[0] = 0;
        }
        if eval_data.attack_count[1] < 2 || black_pieces.queens.len() == 0 {
            eval_data.attack_weight[1] = 0;
        }

        res += SAFETY_TABLE[eval_data.attack_weight[0] as usize]
            - SAFETY_TABLE[eval_data.attack_weight[1] as usize];

        Eval::Numeric(res as f32)
    }

    fn eval_pawn_atttacks(res_board: Board, white_pieces: &Pieces, black_pieces: &Pieces) -> i32 {
        let mut res = 0;

        let (mod_white, mod_black) = if res_board.side_to_move() == Colour::White {
            (ACTIVE_ATTACK_MOD, PASSIVE_ATTACK_MOD)
        } else {
            (PASSIVE_ATTACK_MOD, ACTIVE_ATTACK_MOD)
        };

        res += mod_white
            * Self::pawn_piece_attacks(res_board, Colour::White, white_pieces, black_pieces);
        res -= mod_black
            * Self::pawn_piece_attacks(res_board, Colour::Black, white_pieces, black_pieces);

        res
    }

    fn pawn_piece_attacks(
        res_board: Board,
        colour: Colour,
        white_pieces: &Pieces,
        black_pieces: &Pieces,
    ) -> i32 {
        let mut res = 0;
        let (pieces, other_pieces) = if colour == Colour::White {
            (white_pieces, black_pieces)
        } else {
            (black_pieces, white_pieces)
        };
        let occupancy = res_board.base.black_occupied + res_board.base.white_occupied;

        for square in &pieces.pawns {
            let attacked =
                unsafe { wrap_extract_squares(pawnTargets(*square, colour.as_int(), occupancy)) };
            for square in &other_pieces.knights {
                if attacked.contains(square) {
                    res += KNIGHT_ATTACK;
                }
            }
            for square in &other_pieces.bishops {
                if attacked.contains(square) {
                    res += BISHIP_ATTACK;
                }
            }
            for square in &other_pieces.rooks {
                if attacked.contains(square) {
                    res += ROOK_ATTACK;
                }
            }
            for square in &other_pieces.queens {
                if attacked.contains(square) {
                    res += QUEEN_ATTACK;
                }
            }
        }
        res
    }

    pub fn test_eval(board: Board) -> Eval {
        let branch = Self::from(board);
        branch.eval_heuristic()
    }

    fn eval_side_material(pieces: &Pieces, side: Colour) -> (i32, i32) {
        let (mut mg_res, mut eg_res) = (0, 0);

        for square in &pieces.king {
            mg_res += KING_VAL;
            eg_res += KING_VAL;

            mg_res += MG_KING[to_index(side)][*square as usize];
            eg_res += EG_KING[to_index(side)][*square as usize];
        }
        for square in &pieces.pawns {
            mg_res += PAWN_VAL;
            eg_res += PAWN_VAL;

            mg_res += MG_PAWN[to_index(side)][*square as usize];
            eg_res += EG_PAWN[to_index(side)][*square as usize];
        }
        for square in &pieces.knights {
            mg_res += KNIGHT_VAL;
            eg_res += KNIGHT_VAL;

            mg_res += MG_KNIGHT[to_index(side)][*square as usize];
            eg_res += EG_KNIGHT[to_index(side)][*square as usize];
        }
        for square in &pieces.bishops {
            mg_res += BISHOP_VAL;
            eg_res += BISHOP_VAL;

            mg_res += MG_BISHOP[to_index(side)][*square as usize];
            eg_res += EG_BISHOP[to_index(side)][*square as usize];
        }
        for square in &pieces.rooks {
            mg_res += ROOK_VAL;
            eg_res += ROOK_VAL;

            mg_res += MG_ROOK[to_index(side)][*square as usize];
            eg_res += EG_ROOK[to_index(side)][*square as usize];
        }
        for square in &pieces.queens {
            mg_res += QUEEN_VAL;
            eg_res += QUEEN_VAL;

            mg_res += MG_QUEEN[to_index(side)][*square as usize];
            eg_res += EG_QUEEN[to_index(side)][*square as usize];
        }

        (mg_res, eg_res)
    }

    fn eval_material(white_pieces: &Pieces, black_pieces: &Pieces) -> (i32, i32) {
        let (white_mg, white_eg) = Self::eval_side_material(white_pieces, Colour::White);
        let (black_mg, black_eg) = Self::eval_side_material(black_pieces, Colour::Black);

        (white_mg - black_mg, white_eg - black_eg)
    }

    fn eval_shield(white_pieces: &Pieces, black_pieces: &Pieces, eval_data: &mut EvalData) {
        eval_data.king_shield[0] = Self::eval_shield_white(white_pieces);
        eval_data.king_shield[1] = Self::eval_shield_black(black_pieces);
    }

    fn eval_shield_white(pieces: &Pieces) -> i32 {
        let mut res = 0;
        if pieces.king[0] % 8 > 4 {
            // kingside
            if pieces.pawns.contains(&f2) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&f3) {
                res += SHIELD_2;
            }
            if pieces.pawns.contains(&g2) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&g3) {
                res += SHIELD_2;
            }
            if pieces.pawns.contains(&h2) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&h3) {
                res += SHIELD_2;
            }
        } else if pieces.king[0] % 8 < 3 {
            if pieces.pawns.contains(&a2) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&a3) {
                res += SHIELD_2;
            }
            if pieces.pawns.contains(&b2) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&b3) {
                res += SHIELD_2;
            }
            if pieces.pawns.contains(&c2) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&c3) {
                res += SHIELD_2;
            }
        }

        res
    }

    fn eval_shield_black(pieces: &Pieces) -> i32 {
        let mut res = 0;
        if pieces.king[0] % 8 > 4 {
            // kingside
            if pieces.pawns.contains(&f7) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&f6) {
                res += SHIELD_2;
            }
            if pieces.pawns.contains(&g7) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&g6) {
                res += SHIELD_2;
            }
            if pieces.pawns.contains(&h7) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&h6) {
                res += SHIELD_2;
            }
        } else if pieces.king[0] % 8 < 3 {
            if pieces.pawns.contains(&a7) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&a6) {
                res += SHIELD_2;
            }
            if pieces.pawns.contains(&b7) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&b6) {
                res += SHIELD_2;
            }
            if pieces.pawns.contains(&c7) {
                res += SHIELD_1;
            } else if pieces.pawns.contains(&c6) {
                res += SHIELD_2;
            }
        }

        res
    }

    fn eval_blocked_pieces(white_pieces: &Pieces, black_pieces: &Pieces, eval_data: &mut EvalData) {
        if (white_pieces.king[0] == f1 || white_pieces.king[0] == g1)
            && (white_pieces.rooks.contains(&h1) || white_pieces.rooks.contains(&g1))
        {
            eval_data.blockages[0] -= P_KING_BLOCKS_ROOK;
        }
        if (white_pieces.king[0] == c1 || white_pieces.king[0] == b1)
            && (white_pieces.rooks.contains(&a1) || white_pieces.rooks.contains(&b1))
        {
            eval_data.blockages[0] -= P_KING_BLOCKS_ROOK;
        }

        if (black_pieces.king[0] == f8 || black_pieces.king[0] == g8)
            && (black_pieces.rooks.contains(&h8) || black_pieces.rooks.contains(&g8))
        {
            eval_data.blockages[1] -= P_KING_BLOCKS_ROOK;
        }
        if (black_pieces.king[0] == c8 || black_pieces.king[0] == b8)
            && (black_pieces.rooks.contains(&a8) || black_pieces.rooks.contains(&b8))
        {
            eval_data.blockages[0] -= P_KING_BLOCKS_ROOK;
        }
    }

    fn tempo_bonus(&self, side: Colour) -> i32 {
        if side == Colour::White {
            TEMPO
        } else {
            -TEMPO
        }
    }

    fn combination_adjustment(&self, white_pieces: &Pieces, black_pieces: &Pieces) -> i32 {
        let mut res = 0;

        if white_pieces.bishops.len() > 1 {
            res += BISHOP_PAIR;
        }
        if black_pieces.bishops.len() > 1 {
            res -= BISHOP_PAIR;
        }
        if white_pieces.knights.len() > 1 {
            res -= P_KNIGHT_PAIR;
        }
        if black_pieces.knights.len() > 1 {
            res += P_KNIGHT_PAIR;
        }
        if white_pieces.rooks.len() > 1 {
            res -= P_ROOK_PAIR;
        }
        if black_pieces.rooks.len() > 1 {
            res += P_ROOK_PAIR;
        }

        res
    }

    fn eval_structure(&self, white_pieces: &Pieces, black_pieces: &Pieces) -> i32 {
        // TODO! wrap hashing

        let mut res = 0;

        for square in &white_pieces.pawns {
            res += self.eval_pawn(*square, Colour::White, white_pieces, black_pieces);
        }
        for square in &black_pieces.pawns {
            res -= self.eval_pawn(*square, Colour::Black, black_pieces, white_pieces);
        }

        res
    }

    fn eval_pawn(
        &self,
        square: i32,
        side: Colour,
        side_pieces: &Pieces,
        other_pieces: &Pieces,
    ) -> i32 {
        let mut res = 0;
        let mut flag_passed = true;
        let mut flag_weak = true;
        let mut flag_opposed = false;

        let (step_foreward, step_backward): (fn(i32) -> Option<i32>, fn(i32) -> Option<i32>) =
            if side == Colour::White {
                (NORTH_OF, SOUTH_OF)
            } else {
                (SOUTH_OF, NORTH_OF)
            };

        let mut current_square = square;
        while let Some(next_square) = step_foreward(current_square) {
            if side_pieces.pawns.contains(&next_square) {
                flag_passed = false;
                res -= P_DOUBLED_PAWN;
            } else if other_pieces.pawns.contains(&next_square) {
                flag_passed = false;
                flag_opposed = true;
            }

            if let Some(west_square) = WEST_OF(next_square) {
                if other_pieces.pawns.contains(&west_square) {
                    flag_passed = false;
                }
            }
            if let Some(east_square) = EAST_OF(next_square) {
                if other_pieces.pawns.contains(&east_square) {
                    flag_passed = false;
                }
            }

            current_square = next_square;
        }

        current_square = square;
        while let Some(next_square) = step_backward(current_square) {
            if let Some(west_square) = WEST_OF(next_square) {
                if side_pieces.pawns.contains(&west_square) {
                    flag_weak = false;
                    break;
                }
            }
            if let Some(east_square) = EAST_OF(next_square) {
                if side_pieces.pawns.contains(&east_square) {
                    flag_weak = false;
                    break;
                }
            }

            current_square = next_square;
        }

        if flag_passed {
            if self.pawn_supported(square, side, side_pieces) {
                res += PROTECTED_PASSED_PAWN[to_index(side)][square as usize]
            } else {
                res += PASSED_PAWN[to_index(side)][square as usize]
            }
        }

        if flag_weak {
            res += WEAK_PAWN[to_index(side)][square as usize];
            if !flag_opposed {
                res -= P_OPPONENT_FLAG
            }
        }

        res
    }

    fn pawn_supported(&self, square: i32, side: Colour, pieces: &Pieces) -> bool {
        let step = if side == Colour::White {
            SOUTH_OF
        } else {
            NORTH_OF
        };

        if let Some(west_square) = WEST_OF(square) {
            if pieces.pawns.contains(&west_square) {
                return true;
            } else if let Some(step_west_square) = step(west_square) {
                if pieces.pawns.contains(&step_west_square) {
                    return true;
                }
            }
        }
        if let Some(east_square) = EAST_OF(square) {
            if pieces.pawns.contains(&east_square) {
                return true;
            } else if let Some(step_east_square) = step(east_square) {
                if pieces.pawns.contains(&step_east_square) {
                    return true;
                }
            }
        }

        false
    }

    fn eval_pieces(&self, white_pieces: &Pieces, black_pieces: &Pieces, eval_data: &mut EvalData) {
        let occupancy = self.board.base.white_occupied + self.board.base.black_occupied;
        for square in &white_pieces.knights {
            self.eval_knight(
                *square,
                Colour::White,
                white_pieces,
                black_pieces,
                black_pieces.king[0],
                eval_data,
            );
        }
        for square in &black_pieces.knights {
            self.eval_knight(
                *square,
                Colour::Black,
                white_pieces,
                black_pieces,
                white_pieces.king[0],
                eval_data,
            );
        }

        for square in &white_pieces.bishops {
            self.eval_bishop(
                *square,
                Colour::White,
                white_pieces,
                black_pieces,
                black_pieces.king[0],
                occupancy,
                eval_data,
            );
        }
        for square in &black_pieces.bishops {
            self.eval_bishop(
                *square,
                Colour::Black,
                white_pieces,
                black_pieces,
                white_pieces.king[0],
                occupancy,
                eval_data,
            );
        }

        for square in &white_pieces.rooks {
            self.eval_rook(
                *square,
                Colour::White,
                white_pieces,
                black_pieces,
                black_pieces.king[0],
                occupancy,
                eval_data,
            );
        }
        for square in &black_pieces.rooks {
            self.eval_rook(
                *square,
                Colour::Black,
                black_pieces,
                white_pieces,
                white_pieces.king[0],
                occupancy,
                eval_data,
            );
        }

        for square in &white_pieces.queens {
            self.eval_queen(
                *square,
                Colour::White,
                white_pieces,
                black_pieces,
                black_pieces.king[0],
                occupancy,
                eval_data,
            );
        }
        for square in &black_pieces.queens {
            self.eval_queen(
                *square,
                Colour::Black,
                white_pieces,
                black_pieces,
                white_pieces.king[0],
                occupancy,
                eval_data,
            );
        }
    }

    fn eval_knight(
        &self,
        square: i32,
        side: Colour,
        white_pieces: &Pieces,
        black_pieces: &Pieces,
        opponent_king_pos: i32,
        eval_data: &mut EvalData,
    ) {
        let mut attacks = 0;
        let mut mobility = 0;
        eval_data.game_phase += 1;
        if side == Colour::White {
            match square {
                a8 => {
                    if black_pieces.pawns.contains(&a7) || black_pieces.pawns.contains(&c7) {
                        eval_data.blockages[0] -= P_KNIGHT_TRAPPED_A8;
                    }
                }
                h8 => {
                    if black_pieces.pawns.contains(&h7) || black_pieces.pawns.contains(&f7) {
                        eval_data.blockages[0] -= P_KNIGHT_TRAPPED_A8;
                    }
                }
                a7 => {
                    if black_pieces.pawns.contains(&a6) && black_pieces.pawns.contains(&b7) {
                        eval_data.blockages[0] -= P_KNIGHT_TRAPPED_A7;
                    }
                }
                h7 => {
                    if black_pieces.pawns.contains(&h6) && black_pieces.pawns.contains(&g7) {
                        eval_data.blockages[0] -= P_KNIGHT_TRAPPED_A7;
                    }
                }
                c3 => {
                    if white_pieces.pawns.contains(&c2)
                        && white_pieces.pawns.contains(&d4)
                        && !white_pieces.pawns.contains(&e4)
                    {
                        eval_data.blockages[0] -= P_C3_KNIGHT;
                    }
                }
                _ => {}
            }

            eval_data.material_adjustement[0] += KNIGHT_ADJ[white_pieces.knights.len()];
        } else {
            match square {
                a1 => {
                    if white_pieces.pawns.contains(&a2) || white_pieces.pawns.contains(&c2) {
                        eval_data.blockages[1] -= P_KNIGHT_TRAPPED_A8;
                    }
                }
                h1 => {
                    if white_pieces.pawns.contains(&h2) || white_pieces.pawns.contains(&f2) {
                        eval_data.blockages[1] -= P_KNIGHT_TRAPPED_A8;
                    }
                }
                a2 => {
                    if white_pieces.pawns.contains(&a3) && white_pieces.pawns.contains(&b2) {
                        eval_data.blockages[1] -= P_KNIGHT_TRAPPED_A7;
                    }
                }
                h2 => {
                    if white_pieces.pawns.contains(&h3) && black_pieces.pawns.contains(&g2) {
                        eval_data.blockages[1] -= P_KNIGHT_TRAPPED_A7;
                    }
                }
                c6 => {
                    if black_pieces.pawns.contains(&c7)
                        && black_pieces.pawns.contains(&d5)
                        && !black_pieces.pawns.contains(&e5)
                    {
                        eval_data.blockages[1] -= P_C3_KNIGHT;
                    }
                }
                _ => {}
            }
            eval_data.material_adjustement[1] += KNIGHT_ADJ[black_pieces.knights.len()];
        }

        unsafe {
            for s in wrap_extract_squares(knightTargets(square)) {
                mobility += 1;
                if SQ_NEAR_KING[to_index(side)][opponent_king_pos as usize][s as usize] == 1 {
                    attacks += 1;
                }
            }
        }

        eval_data.mg_mobility[to_index(side)] += 4 * (mobility - 4);
        eval_data.eg_mobility[to_index(side)] += 4 * (mobility - 4);

        if attacks != 0 {
            eval_data.attack_count[to_index(side)] += 1;
            eval_data.attack_weight[to_index(side)] += 2 * attacks;
        }
    }

    fn eval_bishop(
        &self,
        square: i32,
        side: Colour,
        white_pieces: &Pieces,
        black_pieces: &Pieces,
        opponent_king_pos: i32,
        occupancy: u64,
        eval_data: &mut EvalData,
    ) {
        let mut mobility = 0;
        let mut attacks = 0;
        eval_data.game_phase += 1;

        if side == Colour::White {
            match square {
                a7 => {
                    if black_pieces.pawns.contains(&b6) {
                        eval_data.blockages[0] -= P_BISHOP_TRAPPED_A7;
                    }
                }
                h7 => {
                    if black_pieces.pawns.contains(&g6) {
                        eval_data.blockages[0] -= P_BISHOP_TRAPPED_A7;
                    }
                }
                b8 => {
                    if black_pieces.pawns.contains(&c7) {
                        eval_data.blockages[0] -= P_BISHOP_TRAPPED_A7;
                    }
                }
                g8 => {
                    if black_pieces.pawns.contains(&f7) {
                        eval_data.blockages[0] -= P_BISHOP_TRAPPED_A7;
                    }
                }
                a6 => {
                    if black_pieces.pawns.contains(&b5) {
                        eval_data.blockages[0] -= P_BISHOP_TRAPPED_A6;
                    }
                }
                h6 => {
                    if black_pieces.pawns.contains(&g5) {
                        eval_data.blockages[0] -= P_BISHOP_TRAPPED_A7;
                    }
                }
                f1 => {
                    if white_pieces.king[0] == g1 {
                        eval_data.positional_themes[0] += RETURNING_BISHOP;
                    }
                }
                c1 => {
                    if white_pieces.king[0] == b1 {
                        eval_data.positional_themes[0] += RETURNING_BISHOP;
                    }
                }
                _ => {}
            }
        } else {
            match square {
                a2 => {
                    if white_pieces.pawns.contains(&b3) {
                        eval_data.blockages[1] -= P_BISHOP_TRAPPED_A7;
                    }
                }
                h2 => {
                    if white_pieces.pawns.contains(&g3) {
                        eval_data.blockages[1] -= P_BISHOP_TRAPPED_A7;
                    }
                }
                b1 => {
                    if white_pieces.pawns.contains(&c2) {
                        eval_data.blockages[1] -= P_BISHOP_TRAPPED_A7;
                    }
                }
                g1 => {
                    if white_pieces.pawns.contains(&f2) {
                        eval_data.blockages[1] -= P_BISHOP_TRAPPED_A7;
                    }
                }
                a3 => {
                    if white_pieces.pawns.contains(&b4) {
                        eval_data.blockages[1] -= P_BISHOP_TRAPPED_A6;
                    }
                }
                h3 => {
                    if black_pieces.pawns.contains(&g4) {
                        eval_data.blockages[1] -= P_BISHOP_TRAPPED_A7;
                    }
                }
                f8 => {
                    if white_pieces.king[0] == g8 {
                        eval_data.positional_themes[1] += RETURNING_BISHOP;
                    }
                }
                c8 => {
                    if white_pieces.king[0] == b8 {
                        eval_data.positional_themes[1] += RETURNING_BISHOP;
                    }
                }
                _ => {}
            }
        }

        unsafe {
            for s in wrap_extract_squares(bishopTargets(square, occupancy)) {
                mobility += 1;
                if SQ_NEAR_KING[to_index(side)][opponent_king_pos as usize][s as usize] == 1 {
                    attacks += 1;
                }
            }
        }

        eval_data.mg_mobility[to_index(side)] += 3 * (mobility - 7);
        eval_data.eg_mobility[to_index(side)] += 3 * (mobility - 7);

        if attacks != 0 {
            eval_data.attack_count[to_index(side)] += 1;
            eval_data.attack_weight[to_index(side)] += 2 * attacks;
        }
    }

    fn eval_rook(
        &self,
        square: i32,
        side: Colour,
        side_pieces: &Pieces,
        other_pieces: &Pieces,
        opponent_king_pos: i32,
        occupancy: u64,
        eval_data: &mut EvalData,
    ) {
        let mut mobility = 0;
        let mut attacks = 0;
        let mut own_pawns_blocking = 0;
        let mut opponent_pawns_blocking = 0;

        eval_data.game_phase += 2;

        eval_data.material_adjustement[to_index(side)] += ROOK_ADJ[side_pieces.pawns.len()];

        let step_foreward = if side == Colour::White {
            NORTH_OF
        } else {
            SOUTH_OF
        };

        let mut current_square = square;
        while let Some(next_square) = step_foreward(current_square) {
            if side_pieces.pawns.contains(&next_square) {
                own_pawns_blocking += 1;
            } else if other_pieces.pawns.contains(&next_square) {
                opponent_pawns_blocking += 1;
            }

            current_square = next_square;
        }

        if own_pawns_blocking == 0 {
            if opponent_pawns_blocking == 0 {
                eval_data.mg_mobility[to_index(side)] += ROOK_OPEN;
                eval_data.eg_mobility[to_index(side)] += ROOK_OPEN;
            } else {
                eval_data.mg_mobility[to_index(side)] += ROOK_HALF;
                eval_data.eg_mobility[to_index(side)] += ROOK_HALF;
            }
        }

        unsafe {
            for s in wrap_extract_squares(rookTargets(square, occupancy)) {
                mobility += 1;
                if SQ_NEAR_KING[to_index(side)][opponent_king_pos as usize][s as usize] == 1 {
                    attacks += 1;
                }
            }
        }

        eval_data.mg_mobility[to_index(side)] += 2 * (mobility - 7);
        eval_data.eg_mobility[to_index(side)] += 4 * (mobility - 7);

        if attacks != 0 {
            eval_data.attack_count[to_index(side)] += 1;
            eval_data.attack_weight[to_index(side)] += 3 * attacks;
        }
    }

    fn eval_queen(
        &self,
        square: i32,
        side: Colour,
        white_pieces: &Pieces,
        black_pieces: &Pieces,
        opponent_king_pos: i32,
        occupancy: u64,
        eval_data: &mut EvalData,
    ) {
        let mut mobility = 0;
        let mut attacks = 0;
        eval_data.game_phase += 4;

        if side == Colour::White && square > h2 {
            if white_pieces.knights.contains(&b1) {
                eval_data.positional_themes[0] -= P_QUEEN_DEVELOPED_EARLY;
            }
            if white_pieces.knights.contains(&g1) {
                eval_data.positional_themes[0] -= P_QUEEN_DEVELOPED_EARLY;
            }
            if white_pieces.bishops.contains(&c1) {
                eval_data.positional_themes[0] -= P_QUEEN_DEVELOPED_EARLY;
            }
            if white_pieces.bishops.contains(&f1) {
                eval_data.positional_themes[0] -= P_QUEEN_DEVELOPED_EARLY;
            }
        } else if side == Colour::Black && square < a7 {
            if black_pieces.knights.contains(&b8) {
                eval_data.positional_themes[1] -= P_QUEEN_DEVELOPED_EARLY;
            }
            if white_pieces.knights.contains(&g8) {
                eval_data.positional_themes[1] -= P_QUEEN_DEVELOPED_EARLY;
            }
            if white_pieces.bishops.contains(&c8) {
                eval_data.positional_themes[1] -= P_QUEEN_DEVELOPED_EARLY;
            }
            if white_pieces.bishops.contains(&f8) {
                eval_data.positional_themes[1] -= P_QUEEN_DEVELOPED_EARLY;
            }
        }

        unsafe {
            for s in wrap_extract_squares(queenTargets(square, occupancy)) {
                mobility += 1;
                if SQ_NEAR_KING[to_index(side)][opponent_king_pos as usize][s as usize] == 1 {
                    attacks += 1;
                }
            }
        }

        eval_data.mg_mobility[to_index(side)] += 1 * (mobility - 14);
        eval_data.eg_mobility[to_index(side)] += 2 * (mobility - 14);

        if attacks != 0 {
            eval_data.attack_count[to_index(side)] += 1;
            eval_data.attack_weight[to_index(side)] += 4 * attacks;
        }
    }

    // fn low_material_correction()
}

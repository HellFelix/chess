use const_for::const_for;

use chess_backend::*;

pub const KNIGHT_ATTACK: i32 = 40;
pub const BISHIP_ATTACK: i32 = 45;
pub const ROOK_ATTACK: i32 = 50;
pub const QUEEN_ATTACK: i32 = 60;
pub const ACTIVE_ATTACK_MOD: i32 = 3;
pub const PASSIVE_ATTACK_MOD: i32 = 1;

const fn CONVERT_TO_USIZE(input: [i32; 64]) -> [usize; 64] {
    let mut res = [0; 64];
    const_for!(i in 0..64 => {
        res[i] = input[i] as usize;
    });

    res
}

// constants based on the CPW stdafx library

const INDEX_WHITE_INT: [i32; 64] = [
    a8, b8, c8, d8, e8, f8, g8, h8, //
    a7, b7, c7, d7, e7, f7, g7, h7, //
    a6, b6, c6, d6, e6, f6, g6, h6, //
    a5, b5, c5, d5, e5, f5, g5, h5, //
    a4, b4, c4, d4, e4, f4, g4, h4, //
    a3, b3, c3, d3, e3, f3, g3, h3, //
    a2, b2, c2, d2, e2, f2, g2, h2, //
    a1, b1, c1, d1, e1, f1, g1, h1, //
];

const INDEX_WHITE: [usize; 64] = CONVERT_TO_USIZE(INDEX_WHITE_INT);

const INDEX_BLACK_INT: [i32; 64] = [
    a1, b1, c1, d1, e1, f1, g1, h1, //
    a2, b2, c2, d2, e2, f2, g2, h2, //
    a3, b3, c3, d3, e3, f3, g3, h3, //
    a4, b4, c4, d4, e4, f4, g4, h4, //
    a5, b5, c5, d5, e5, f5, g5, h5, //
    a6, b6, c6, d6, e6, f6, g6, h6, //
    a7, b7, c7, d7, e7, f7, g7, h7, //
    a8, b8, c8, d8, e8, f8, g8, h8, //
];

const INDEX_BLACK: [usize; 64] = CONVERT_TO_USIZE(INDEX_BLACK_INT);

// ----- Pawns -----
const PAWN_PCSQ_MG: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, //
    -6, -4, 1, 1, 1, 1, -4, -6, //
    -6, -4, 1, 2, 2, 1, -4, -6, //
    -6, -4, 2, 8, 8, 2, -4, -6, //
    -6, -4, 5, 10, 10, 5, -4, -6, //
    -4, -4, 1, 5, 5, 1, -4, -4, //
    -6, -4, 1, -24, -24, 1, -4, -6, //
    0, 0, 0, 0, 0, 0, 0, 0, //
];

const PAWN_PCSQ_EG: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, //
    -6, -4, 1, 1, 1, 1, -4, -6, //
    -6, -4, 1, 2, 2, 1, -4, -6, //
    -6, -4, 2, 8, 8, 2, -4, -6, //
    -6, -4, 5, 10, 10, 5, -4, -6, //
    -4, -4, 1, 5, 5, 1, -4, -4, //
    -6, -4, 1, -24, -24, 1, -4, -6, //
    0, 0, 0, 0, 0, 0, 0, 0, //
];

// ----- Knights -----
const KNIGHT_PCSQ_MG: [i32; 64] = [
    -8, -8, -8, -8, -8, -8, -8, -8, //
    -8, 0, 0, 0, 0, 0, 0, -8, //
    -8, 0, 4, 4, 4, 4, 0, -8, //
    -8, 0, 4, 8, 8, 4, 0, -8, //
    -8, 0, 4, 8, 8, 4, 0, -8, //
    -8, 0, 4, 4, 4, 4, 0, -8, //
    -8, 0, 1, 2, 2, 1, 0, -8, //
    -8, -12, -8, -8, -8, -8, -12, -8, //
];

const KNIGHT_PCSQ_EG: [i32; 64] = [
    -8, -8, -8, -8, -8, -8, -8, -8, //
    -8, 0, 0, 0, 0, 0, 0, -8, //
    -8, 0, 4, 4, 4, 4, 0, -8, //
    -8, 0, 4, 8, 8, 4, 0, -8, //
    -8, 0, 4, 8, 8, 4, 0, -8, //
    -8, 0, 4, 4, 4, 4, 0, -8, //
    -8, 0, 1, 2, 2, 1, 0, -8, //
    -8, -12, -8, -8, -8, -8, -12, -8, //
];

pub const KNIGHT_ADJ: [i32; 9] = [-20, -16, -12, -8, -4, 0, 4, 8, 12];

// ----- Bishop -----
const BISHOP_PCSQ_MG: [i32; 64] = [
    -4, -4, -4, -4, -4, -4, -4, -4, //
    -4, 0, 0, 0, 0, 0, 0, -4, //
    -4, 0, 2, 4, 4, 2, 0, -4, //
    -4, 0, 4, 6, 6, 4, 0, -4, //
    -4, 0, 4, 6, 6, 4, 0, -4, //
    -4, 1, 2, 4, 4, 2, 1, -4, //
    -4, 2, 1, 1, 1, 1, 2, -4, //
    -4, -4, -12, -4, -4, -12, -4, -4, //
];

const BISHOP_PCSQ_EG: [i32; 64] = [
    -4, -4, -4, -4, -4, -4, -4, -4, //
    -4, 0, 0, 0, 0, 0, 0, -4, //
    -4, 0, 2, 4, 4, 2, 0, -4, //
    -4, 0, 4, 6, 6, 4, 0, -4, //
    -4, 0, 4, 6, 6, 4, 0, -4, //
    -4, 1, 2, 4, 4, 2, 1, -4, //
    -4, 2, 1, 1, 1, 1, 2, -4, //
    -4, -4, -12, -4, -4, -12, -4, -4, //
];

// ----- Rook -----
const ROOK_PCSQ_MG: [i32; 64] = [
    5, 5, 5, 5, 5, 5, 5, 5, //
    20, 20, 20, 20, 20, 20, 20, 20, //
    -5, 0, 0, 0, 0, 0, 0, -5, //
    -5, 0, 0, 0, 0, 0, 0, -5, //
    -5, 0, 0, 0, 0, 0, 0, -5, //
    -5, 0, 0, 0, 0, 0, 0, -5, //
    -5, 0, 0, 0, 0, 0, 0, -5, //
    0, 0, 0, 2, 2, 0, 0, 0, //
];

const ROOK_PCSQ_EG: [i32; 64] = [
    5, 5, 5, 5, 5, 5, 5, 5, //
    20, 20, 20, 20, 20, 20, 20, 20, //
    -5, 0, 0, 0, 0, 0, 0, -5, //
    -5, 0, 0, 0, 0, 0, 0, -5, //
    -5, 0, 0, 0, 0, 0, 0, -5, //
    -5, 0, 0, 0, 0, 0, 0, -5, //
    -5, 0, 0, 0, 0, 0, 0, -5, //
    0, 0, 0, 2, 2, 0, 0, 0, //
];

pub const ROOK_ADJ: [i32; 9] = [15, 12, 9, 6, 3, 0, -3, -6, -9];

// ----- Queen -----
const QUEEN_PCSQ_MG: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 1, 1, 1, 1, 0, 0, //
    0, 0, 1, 2, 2, 1, 0, 0, //
    0, 0, 2, 3, 3, 2, 0, 0, //
    0, 0, 2, 3, 3, 2, 0, 0, //
    0, 0, 1, 2, 2, 1, 0, 0, //
    0, 0, 1, 1, 1, 1, 0, 0, //
    -5, -5, -5, -5, -5, -5, -5, -5, //
];

const QUEEN_PCSQ_EG: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 1, 1, 1, 1, 0, 0, //
    0, 0, 1, 2, 2, 1, 0, 0, //
    0, 0, 2, 3, 3, 2, 0, 0, //
    0, 0, 2, 3, 3, 2, 0, 0, //
    0, 0, 1, 2, 2, 1, 0, 0, //
    0, 0, 1, 1, 1, 1, 0, 0, //
    -5, -5, -5, -5, -5, -5, -5, -5, //
];

// ----- King -----
const KING_PCSQ_MG: [i32; 64] = [
    -40, -30, -50, -70, -70, -50, -30, -40, //
    -30, -20, -40, -60, -60, -40, -20, -30, //
    -20, -10, -30, -50, -50, -30, -10, -20, //
    -10, 0, -20, -40, -40, -20, 0, -10, //
    0, 10, -10, -30, -30, -10, 10, 0, //
    10, 20, 0, -20, -20, 0, 20, 10, //
    30, 40, 20, 0, 0, 20, 40, 30, //
    40, 50, 30, 10, 10, 30, 50, 40, //
];

const KING_PCSQ_EG: [i32; 64] = [
    -72, -48, -36, -24, -24, -36, -48, -72, //
    -48, -24, -12, 0, 0, -12, -24, -48, //
    -36, -12, 0, 12, 12, 0, -12, -36, //
    -24, 0, 12, 24, 24, 12, 0, -24, //
    -24, 0, 12, 24, 24, 12, 0, -24, //
    -36, -12, 0, 12, 12, 0, -12, -36, //
    -48, -24, -12, 0, 0, -12, -24, -48, //
    -72, -48, -36, -24, -24, -36, -48, -72, //
];

const WEAK_PAWN_PCSQ: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, //
    -10, -12, -14, -16, -16, -14, -12, -10, //
    -10, -12, -14, -16, -16, -14, -12, -10, //
    -10, -12, -14, -16, -16, -14, -12, -10, //
    -10, -12, -14, -16, -16, -14, -12, -10, //
    -8, -12, -14, -16, -16, -14, -12, -10, //
    -8, -12, -14, -16, -16, -14, -12, -10, //
    0, 0, 0, 0, 0, 0, 0, 0, //
];

const PASSED_PAWN_PCSQ: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, //
    100, 100, 100, 100, 100, 100, 100, 100, //
    80, 80, 80, 80, 80, 80, 80, 80, //
    60, 60, 60, 60, 60, 60, 60, 60, //
    40, 40, 40, 40, 40, 40, 40, 40, //
    20, 20, 20, 20, 20, 20, 20, 20, //
    20, 20, 20, 20, 20, 20, 20, 20, //
    0, 0, 0, 0, 0, 0, 0, 0, //
];

pub const SAFETY_TABLE: [i32; 100] = [
    0, 0, 1, 2, 3, 5, 7, 9, 12, 15, //
    18, 22, 26, 30, 35, 39, 44, 50, 56, 62, //
    68, 75, 82, 85, 89, 97, 105, 113, 122, 131, //
    140, 150, 169, 180, 191, 202, 213, 225, 237, 248, //
    260, 272, 283, 295, 307, 319, 330, 342, 354, 366, //
    377, 389, 401, 412, 424, 436, 448, 459, 471, 483, //
    494, 500, 500, 500, 500, 500, 500, 500, 500, 500, //
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, //
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, //
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, //
];

const fn SET_TABLES() -> [[i32; 64]; 30] {
    let mut res = [[0; 64]; 30];
    const_for!(i in 0..64 => {
        res[0][INDEX_WHITE[i]] = WEAK_PAWN_PCSQ[i];
        res[1][INDEX_BLACK[i]] = WEAK_PAWN_PCSQ[i];
        res[2][INDEX_WHITE[i]] = PASSED_PAWN_PCSQ[i];
        res[3][INDEX_BLACK[i]] = PASSED_PAWN_PCSQ[i];

        /* protected passers are considered slightly stronger
        than ordinary passed pawns */

        res[4][INDEX_WHITE[i]] = (PASSED_PAWN_PCSQ[i] * 10) / 8;
        res[5][INDEX_BLACK[i]] = (PASSED_PAWN_PCSQ[i] * 10) / 8;

        /* now set the piece/square tables for each color and piece type */

        res[6][INDEX_WHITE[i]] = PAWN_PCSQ_MG[i];
        res[7][INDEX_BLACK[i]] = PAWN_PCSQ_MG[i];
        res[8][INDEX_WHITE[i]] = KNIGHT_PCSQ_MG[i];
        res[9][INDEX_BLACK[i]] = KNIGHT_PCSQ_MG[i];
        res[10][INDEX_WHITE[i]] = BISHOP_PCSQ_MG[i];
        res[11][INDEX_BLACK[i]] = BISHOP_PCSQ_MG[i];
        res[12][INDEX_WHITE[i]] = ROOK_PCSQ_MG[i];
        res[13][INDEX_BLACK[i]] = ROOK_PCSQ_MG[i];
        res[14][INDEX_WHITE[i]] = QUEEN_PCSQ_MG[i];
        res[15][INDEX_BLACK[i]] = QUEEN_PCSQ_MG[i];
        res[16][INDEX_WHITE[i]] = KING_PCSQ_MG[i];
        res[17][INDEX_BLACK[i]] = KING_PCSQ_MG[i];

        res[18][INDEX_WHITE[i]] = PAWN_PCSQ_EG[i];
        res[19][INDEX_BLACK[i]] = PAWN_PCSQ_EG[i];
        res[20][INDEX_WHITE[i]] = KNIGHT_PCSQ_EG[i];
        res[21][INDEX_BLACK[i]] = KNIGHT_PCSQ_EG[i];
        res[22][INDEX_WHITE[i]] = BISHOP_PCSQ_EG[i];
        res[23][INDEX_BLACK[i]] = BISHOP_PCSQ_EG[i];
        res[24][INDEX_WHITE[i]] = ROOK_PCSQ_EG[i];
        res[25][INDEX_BLACK[i]] = ROOK_PCSQ_EG[i];
        res[26][INDEX_WHITE[i]] = QUEEN_PCSQ_EG[i];
        res[27][INDEX_BLACK[i]] = QUEEN_PCSQ_EG[i];
        res[28][INDEX_WHITE[i]] = KING_PCSQ_EG[i];
        res[29][INDEX_BLACK[i]] = KING_PCSQ_EG[i];
    });

    res
}

const TABLES: [[i32; 64]; 30] = SET_TABLES();

const WEAK_PAWN_WHITE: [i32; 64] = TABLES[0];
const WEAK_PAWN_BLACK: [i32; 64] = TABLES[1];
pub const WEAK_PAWN: [[i32; 64]; 2] = [WEAK_PAWN_WHITE, WEAK_PAWN_BLACK];
const PASSED_PAWN_WHITE: [i32; 64] = TABLES[2];
const PASSED_PAWN_BLACK: [i32; 64] = TABLES[3];
pub const PASSED_PAWN: [[i32; 64]; 2] = [PASSED_PAWN_WHITE, PASSED_PAWN_BLACK];
const PROTECTED_PASSER_WHITE: [i32; 64] = TABLES[4];
const PROTECTED_PASSER_BLACK: [i32; 64] = TABLES[5];
pub const PROTECTED_PASSED_PAWN: [[i32; 64]; 2] = [PROTECTED_PASSER_WHITE, PROTECTED_PASSER_BLACK];

const MG_PAWN_WHITE: [i32; 64] = TABLES[6];
const MG_PAWN_BLACK: [i32; 64] = TABLES[7];
pub const MG_PAWN: [[i32; 64]; 2] = [MG_PAWN_WHITE, MG_PAWN_BLACK];
const MG_KNIGHT_WHITE: [i32; 64] = TABLES[8];
const MG_KNIGHT_BLACK: [i32; 64] = TABLES[9];
pub const MG_KNIGHT: [[i32; 64]; 2] = [MG_KNIGHT_WHITE, MG_KNIGHT_BLACK];
const MG_BISHOP_WHITE: [i32; 64] = TABLES[10];
const MG_BISHOP_BLACK: [i32; 64] = TABLES[11];
pub const MG_BISHOP: [[i32; 64]; 2] = [MG_BISHOP_WHITE, MG_BISHOP_BLACK];
const MG_ROOK_WHITE: [i32; 64] = TABLES[12];
const MG_ROOK_BLACK: [i32; 64] = TABLES[13];
pub const MG_ROOK: [[i32; 64]; 2] = [MG_ROOK_WHITE, MG_ROOK_BLACK];
const MG_QUEEN_WHITE: [i32; 64] = TABLES[14];
const MG_QUEEN_BLACK: [i32; 64] = TABLES[15];
pub const MG_QUEEN: [[i32; 64]; 2] = [MG_QUEEN_WHITE, MG_QUEEN_BLACK];
const MG_KING_WHITE: [i32; 64] = TABLES[16];
const MG_KING_BLACK: [i32; 64] = TABLES[17];
pub const MG_KING: [[i32; 64]; 2] = [MG_KING_WHITE, MG_KING_BLACK];

const EG_PAWN_WHITE: [i32; 64] = TABLES[18];
const EG_PAWN_BLACK: [i32; 64] = TABLES[19];
pub const EG_PAWN: [[i32; 64]; 2] = [EG_PAWN_WHITE, EG_PAWN_BLACK];
const EG_KNIGHT_WHITE: [i32; 64] = TABLES[20];
const EG_KNIGHT_BLACK: [i32; 64] = TABLES[21];
pub const EG_KNIGHT: [[i32; 64]; 2] = [EG_KNIGHT_WHITE, EG_KNIGHT_BLACK];
const EG_BISHOP_WHITE: [i32; 64] = TABLES[22];
const EG_BISHOP_BLACK: [i32; 64] = TABLES[23];
pub const EG_BISHOP: [[i32; 64]; 2] = [EG_BISHOP_WHITE, EG_BISHOP_BLACK];
const EG_ROOK_WHITE: [i32; 64] = TABLES[24];
const EG_ROOK_BLACK: [i32; 64] = TABLES[25];
pub const EG_ROOK: [[i32; 64]; 2] = [EG_ROOK_WHITE, EG_ROOK_BLACK];
const EG_QUEEN_WHITE: [i32; 64] = TABLES[26];
const EG_QUEEN_BLACK: [i32; 64] = TABLES[27];
pub const EG_QUEEN: [[i32; 64]; 2] = [EG_QUEEN_WHITE, EG_QUEEN_BLACK];
const EG_KING_WHITE: [i32; 64] = TABLES[28];
const EG_KING_BLACK: [i32; 64] = TABLES[29];
pub const EG_KING: [[i32; 64]; 2] = [EG_KING_WHITE, EG_KING_BLACK];

// Piece values
pub const PAWN_VAL: i32 = 100;
pub const KNIGHT_VAL: i32 = 325;
pub const BISHOP_VAL: i32 = 335;
pub const ROOK_VAL: i32 = 500;
pub const QUEEN_VAL: i32 = 975;
pub const KING_VAL: i32 = 0;

pub const BISHOP_PAIR: i32 = 30;
pub const P_KNIGHT_PAIR: i32 = 8;
pub const P_ROOK_PAIR: i32 = 16;

pub const START_MATERIAL: i32 = QUEEN_VAL + 2 * ROOK_VAL + 2 * BISHOP_VAL + 2 * KNIGHT_VAL;

// Trapped pieces
pub const P_KING_BLOCKS_ROOK: i32 = 24;
pub const P_BLOCK_CENTRAL_PAWN: i32 = 24;
pub const P_BISHOP_TRAPPED_A7: i32 = 150;
pub const P_BISHOP_TRAPPED_A6: i32 = 50;
pub const P_KNIGHT_TRAPPED_A8: i32 = 150;
pub const P_KNIGHT_TRAPPED_A7: i32 = 100;

// Penalties
pub const P_C3_KNIGHT: i32 = 5;
pub const P_NO_FIANCHETTO: i32 = 4;
pub const P_DOUBLED_PAWN: i32 = 20;
pub const P_OPPONENT_FLAG: i32 = 4;
pub const P_QUEEN_DEVELOPED_EARLY: i32 = 2;

// King's defence
pub const SHIELD_1: i32 = 10;
pub const SHIELD_2: i32 = 5;
pub const P_NO_SHIELD: i32 = 10;

// Bonuses
pub const ROOK_OPEN: i32 = 10;
pub const ROOK_HALF: i32 = 5;
pub const RETURNING_BISHOP: i32 = 20;
pub const FIANCHETTO: i32 = 4;
pub const TEMPO: i32 = 10;

pub const ENDGAME_MAT: i32 = 1300;

const fn A_FILE(sq: i32) -> bool {
    sq % 8 == 0
}
const fn H_FILE(sq: i32) -> bool {
    sq % 8 == 7
}
const fn FIRST_RANK(sq: i32) -> bool {
    sq / 8 == 0
}
const fn EIHGHT_RANK(sq: i32) -> bool {
    sq / 8 == 7
}

pub const fn NORTH_OF(sq: i32) -> Option<i32> {
    if EIHGHT_RANK(sq) {
        None
    } else {
        Some(sq + 8)
    }
}
pub const fn NORTH_WEST_OF(sq: i32) -> Option<i32> {
    if EIHGHT_RANK(sq) || A_FILE(sq) {
        None
    } else {
        Some(sq + 7)
    }
}
pub const fn NORTH_EAST_OF(sq: i32) -> Option<i32> {
    if EIHGHT_RANK(sq) || H_FILE(sq) {
        None
    } else {
        Some(sq + 9)
    }
}
pub const fn SOUTH_OF(sq: i32) -> Option<i32> {
    if FIRST_RANK(sq) {
        None
    } else {
        Some(sq - 8)
    }
}
pub const fn SOUTH_WEST_OF(sq: i32) -> Option<i32> {
    if FIRST_RANK(sq) || A_FILE(sq) {
        None
    } else {
        Some(sq - 9)
    }
}
pub const fn SOUTH_EAST_OF(sq: i32) -> Option<i32> {
    if FIRST_RANK(sq) || H_FILE(sq) {
        None
    } else {
        Some(sq - 7)
    }
}
pub const fn EAST_OF(sq: i32) -> Option<i32> {
    if H_FILE(sq) {
        None
    } else {
        Some(sq + 1)
    }
}
pub const fn WEST_OF(sq: i32) -> Option<i32> {
    if A_FILE(sq) {
        None
    } else {
        Some(sq - 1)
    }
}
pub const fn GET_SQ(row: i32, col: i32) -> i32 {
    row * 8 + col
}

const fn contains_l11(array: [i32; 11], value: i32) -> bool {
    array[0] == value
        || array[1] == value
        || array[2] == value
        || array[3] == value
        || array[4] == value
        || array[5] == value
        || array[6] == value
        || array[7] == value
        || array[8] == value
        || array[9] == value
        || array[10] == value
}

const fn FIND_SQUARES_NEAR_KING() -> [[[i32; 64]; 64]; 2] {
    let mut res_white = [[0; 64]; 64];
    let mut res_black = [[0; 64]; 64];
    const_for!(i in 0..64 => {

        let mut near_white = [-1; 11];
        let mut near_black = [-1; 11];
        if let Some(sq) = NORTH_OF(i as i32) {
            near_white[0] = sq;
            near_black[0] = sq;

            if let Some(next) = NORTH_OF(sq) {
                near_white[8] = next;
            }
            if let Some(next) = EAST_OF(sq) {
                near_white[9] = next;
            }
            if let Some(next) = WEST_OF(sq) {
                near_white[10] = next;
            }
        }
        if let Some(sq) = SOUTH_OF(i as i32) {
            near_white[1] = sq;
            near_black[1] = sq;

            if let Some(next) = SOUTH_OF(sq) {
                near_black[8] = next;
            }
            if let Some(next) = EAST_OF(sq) {
                near_black[9] = next;
            }
            if let Some(next) = WEST_OF(sq) {
                near_black[10] = next;
            }
        }
        if let Some(sq) = WEST_OF(i as i32) {
            near_white[2] = sq;
            near_black[2] = sq;
        }
        if let Some(sq) = EAST_OF(i as i32) {
            near_white[3] = sq;
            near_black[3] = sq;
        }

        if let Some(sq) = NORTH_EAST_OF(i as i32) {
            near_white[4] = sq;
            near_black[4] = sq;
        }
        if let Some(sq) = NORTH_WEST_OF(i as i32) {
            near_white[5] = sq;
            near_black[5] = sq;
        }
        if let Some(sq) = SOUTH_EAST_OF(i as i32) {
            near_white[6] = sq;
            near_black[6] = sq;
        }
        if let Some(sq) = SOUTH_WEST_OF(i as i32) {
            near_white[7] = sq;
            near_black[7] = sq;
        }

        const_for!(j in 0..64 => {
            if contains_l11(near_white, j as i32) {
                res_white[i][j] = 1;
            }
            if contains_l11(near_black, j as i32) {
                res_black[i][j] = 1;
            }
        })
    });
    [res_white, res_black]
}

pub const SQ_NEAR_KING: [[[i32; 64]; 64]; 2] = FIND_SQUARES_NEAR_KING();

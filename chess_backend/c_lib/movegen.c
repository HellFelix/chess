#include "movegen.h"
#include "targets/king.h"
#include "targets/knight.h"
#include "targets/pawn.h"
#include "targets/sliders.h"

void init_targets() {
  init_king_attacks();
  init_knight_attacks();
  init_pawn_attacks();
  init_sliders();
}

U64 third_rank = 16711680ULL;
U64 sixth_rank = 280375465082880ULL;

U64 XOR_AND_NOT(U64 input, U64 other) { return (input ^ other) & ~other; }
// --- Pawn ---
U64 pawnPushTargets(int square, int colour, U64 occupancy) {
  // with regards to occupancy
  // XOR AND NOT
  U64 res = XOR_AND_NOT(get_pawn_targets(square, colour), occupancy);

  if (colour && (third_rank & res)) {
    if (XOR_AND_NOT(res << 8, occupancy)) {
      res |= (res << 8);
    }
  } else if (!colour && (sixth_rank & res)) {
    if (XOR_AND_NOT(res >> 8, occupancy)) {
      res |= (res >> 8);
    }
  }

  return res;
}

U64 pawnAttackTargets(int square, int colour) {
  return get_pawn_attacks(square, colour);
}

U64 pawnTargets(int square, int colour, U64 occupancy) {
  return (pawnAttackTargets(square, colour) & occupancy) |
         pawnPushTargets(square, colour, occupancy);
}

// --- King ---
U64 kingTargets(int square) { return get_king_attacks(square); }

// --- Knight ---
U64 knightTargets(int square) { return get_knight_attacks(square); }

// --- Bishop ---
U64 bishopTargets(int square, U64 occupancy) {
  return get_bishop_attacks(square, occupancy);
}

// --- Rook ---
U64 rookTargets(int square, U64 occupancy) {
  return get_rook_attacks(square, occupancy);
}

// --- Queen ---
U64 queenTargets(int square, U64 occupancy) {
  return get_rook_attacks(square, occupancy) +
         get_bishop_attacks(square, occupancy);
}

U64 generateAttackTargets(piece_map_bitboards pieces, int colour,
                          U64 occupancy) {
  U64 res = 0;

  square_array *king_arr = extract_squares(pieces.king);
  square_array *queen_arr = extract_squares(pieces.queens);
  square_array *bishop_arr = extract_squares(pieces.bishops);
  square_array *knight_arr = extract_squares(pieces.knights);
  square_array *rook_arr = extract_squares(pieces.rooks);
  square_array *pawn_arr = extract_squares(pieces.pawns);
  int i;

  // king targets
  for (i = 0; i < king_arr->size; i++) {
    res |= kingTargets(king_arr->squares[i]);
  }
  // queen targets
  for (i = 0; i < queen_arr->size; i++) {
    res |= queenTargets(queen_arr->squares[i], occupancy);
  }
  // bishop targets
  for (i = 0; i < bishop_arr->size; i++) {
    res |= bishopTargets(bishop_arr->squares[i], occupancy);
  }
  // knight targets
  for (i = 0; i < knight_arr->size; i++) {
    res |= knightTargets(knight_arr->squares[i]);
  }
  // rook targets
  for (i = 0; i < rook_arr->size; i++) {
    res |= rookTargets(rook_arr->squares[i], occupancy);
  }
  // pawn targets
  for (i = 0; i < pawn_arr->size; i++) {
    res |= pawnAttackTargets(pawn_arr->squares[i], colour);
  }

  return res;
}

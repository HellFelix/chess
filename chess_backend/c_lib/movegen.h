#include "bitboard.h"
typedef unsigned long long U64;

typedef struct chess_move {
  bitboard_base res_board;
  int source_square;
  int dest_square;
} chess_move;

typedef struct chess_moves {
  size_t size;
  chess_move moves[];
} chess_moves;

void init_targets();
chess_move generatePseudoLegal(bitboard_base board);
U64 generateAttackTargets(piece_map_bitboards pieces, int colour, U64 occupacy);

U64 pawnAttackTargets(int square, int colour);
U64 pawnTargets(int square, int colour, U64 occupancy);
U64 kingTargets(int square);
U64 knightTargets(int square);
U64 bishopTargets(int square, U64 occupancy);
U64 rookTargets(int square, U64 occupancy);
U64 queenTargets(int square, U64 occupancy);

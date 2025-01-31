#include "king.h"
#include <stdio.h>
/***************************************\
                Credit to
            Code Moneky King
\**************************************/

// define bitboard type
#define U64 unsigned long long

// bit manipulation macros
#define set_bit(bitboard, index) (bitboard |= (1ULL << index))

U64 not_a_file = 18374403900871474942ULL;
U64 not_h_file = 9187201950435737471ULL;
// king attacks array [square]
U64 king_attacks[64];

// mask king attacks
U64 mask_king_attacks(int square) {
  // attack bitboard
  U64 attacks = 0;

  // piece bitboard
  U64 bitboard = 0;

  // set piece on bitboard
  set_bit(bitboard, square);

  // generate king attacks
  if (bitboard >> 8)
    attacks |= (bitboard >> 8);
  if (bitboard << 8)
    attacks |= (bitboard << 8);
  if ((bitboard >> 1) & not_h_file)
    attacks |= (bitboard >> 1);
  if ((bitboard >> 9) & not_h_file)
    attacks |= (bitboard >> 9);
  if ((bitboard >> 7) & not_a_file)
    attacks |= (bitboard >> 7);
  if ((bitboard << 1) & not_a_file)
    attacks |= (bitboard << 1);
  if ((bitboard << 9) & not_a_file)
    attacks |= (bitboard << 9);
  if ((bitboard << 7) & not_h_file)
    attacks |= (bitboard << 7);

  // return attack map for king on a given square
  return attacks;
}

// init pre-calculated attack tables for leaper pieces (pawns, knights, kings)
void init_king_attacks() {
  printf("Generating King Targets\n");
  // loop over 64 board squares
  for (int square = 0; square < 64; square++) {
    king_attacks[square] = mask_king_attacks(square);
  }
}

U64 get_king_attacks(int square) { return king_attacks[square]; }

#include "knight.h"
/***************************************\
              Credits to
            Code Moneky King
\**************************************/

// headers
#include <stdio.h>

// define bitboard type
#define U64 unsigned long long

// bit manipulation macros
#define set_bit(bitboard, index) (bitboard |= (1ULL << index))

U64 not_a_file = 18374403900871474942ULL;
U64 not_h_file = 9187201950435737471ULL;
U64 not_hg_file = 4557430888798830399ULL;
U64 not_ab_file = 18229723555195321596ULL;
// knight attacks array [square]
U64 knight_attacks[64];
// mask pawn attacks
U64 mask_pawn_attacks(int side, int square) {
  // attack bitboard
  U64 attacks = 0;

  // piece bitboard
  U64 bitboard = 0;

  // set piece on bitboard
  set_bit(bitboard, square);

  // white pawn attacks
  if (!side) {
    // make sure attack is on board
    if ((bitboard >> 7) & not_a_file)
      attacks |= (bitboard >> 7);
    if ((bitboard >> 9) & not_h_file)
      attacks |= (bitboard >> 9);
  }

  // black pawn atacks
  else {
    // make sure attack is on board
    if ((bitboard << 7) & not_h_file)
      attacks |= (bitboard << 7);
    if ((bitboard << 9) & not_a_file)
      attacks |= (bitboard << 9);
  }

  // return attack map for pawn on a given square
  return attacks;
}

// mask knight attacks
U64 mask_knight_attacks(int square) {
  // attack bitboard
  U64 attacks = 0;

  // piece bitboard
  U64 bitboard = 0;

  // set piece on bitboard
  set_bit(bitboard, square);

  // generate knight
  if ((bitboard >> 17) & not_h_file)
    attacks |= (bitboard >> 17);
  if ((bitboard >> 15) & not_a_file)
    attacks |= (bitboard >> 15);
  if ((bitboard >> 10) & not_hg_file)
    attacks |= (bitboard >> 10);
  if ((bitboard >> 6) & not_ab_file)
    attacks |= (bitboard >> 6);
  if ((bitboard << 17) & not_a_file)
    attacks |= (bitboard << 17);
  if ((bitboard << 15) & not_h_file)
    attacks |= (bitboard << 15);
  if ((bitboard << 10) & not_ab_file)
    attacks |= (bitboard << 10);
  if ((bitboard << 6) & not_hg_file)
    attacks |= (bitboard << 6);

  // return attack map for knight on a given square
  return attacks;
}

// init pre-calculated attack tables for leaper pieces (pawns, knights, kings)
void init_knight_attacks() {
  printf("Generating Knight Targets\n");
  // loop over 64 board squares
  for (int square = 0; square < 64; square++) {
    // init leaper attacks
    knight_attacks[square] = mask_knight_attacks(square);
  }
}

U64 get_knight_attacks(int square) { return knight_attacks[square]; }

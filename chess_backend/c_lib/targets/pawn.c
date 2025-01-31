#include "pawn.h"
/***************************************\
              Credit to
            Code Moneky King
\**************************************/

// headers
#include <stdio.h>

// bit manipulation macros
#define set_bit(bitboard, index) (bitboard |= (1ULL << index))

// define sides
enum { white, black };
// not A file bitboard
U64 not_a_file = 18374403900871474942ULL;
U64 not_h_file = 9187201950435737471ULL;

// pawn atacks array [side][bitboard]
// Shouldn't actually need first or eighth rank, but it's more of a hassle
// removing them and it really doesn't matter
U64 pawn_attacks[2][64];
U64 pawn_targets[2][64];

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

    // make sure attack in on board
    if ((bitboard >> 9) & not_h_file)
      attacks |= (bitboard >> 9);
  }

  // black pawn atacks
  else {
    // make sure attack is on board
    if ((bitboard << 7) & not_h_file)
      attacks |= (bitboard << 7);

    // make sure attack in on board
    if ((bitboard << 9) & not_a_file)
      attacks |= (bitboard << 9);
  }

  // return attack map for pawn on a given square
  return attacks;
}

U64 mask_pawn_targets(int side, int square) {
  U64 targets = 0;
  U64 bitboard = 0;
  set_bit(bitboard, square);

  if (side) {
    targets |= (bitboard << 8);
  }

  else {
    targets |= (bitboard >> 8);
  }

  return targets;
}

// init pre-calculated attack tables for leaper pieces (pawns, knights, kings)
void init_pawn_attacks() {
  printf("Generating Pawn Attack Targets\n");
  // loop over 64 board squares
  for (int square = 0; square < 64; square++) {
    // init pawn attacks
    pawn_attacks[white][square] = mask_pawn_attacks(white, square);
    pawn_attacks[black][square] = mask_pawn_attacks(black, square);

    // init pawn targets
    pawn_targets[white][square] = mask_pawn_targets(white, square);
    pawn_targets[black][square] = mask_pawn_targets(black, square);
  }
}

U64 get_pawn_attacks(int square, int colour) {
  return pawn_attacks[colour][square];
}

U64 get_pawn_targets(int square, int colour) {
  return pawn_targets[colour][square];
}

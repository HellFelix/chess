#include "bitboard.h"
#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef unsigned long long U64; // 64 bit unsigned. Same as u64 in Rust
#define MAX_VALUE(a) (((U64)1 << (sizeof(a) * CHAR_BIT)) - 1)
#define C64(constantU64) constantU64##ULL

// helper functions
U64 rotateLeft(U64 x, int s) { return (x << s) | (x >> (64 - s)); }
U64 rotateRight(U64 x, int s) { return (x >> s) | (x << (64 - s)); }

int is_flagged(int square, U64 bitboard) {
  if (bitboard & (1 << square)) {
    return true;
  } else {
    return false;
  }
}

square_array *extract_squares(U64 bitboard) {
  square_array *res = malloc(sizeof(struct square_array) + 64 * sizeof(int));
  int position = 0;
  U64 i;
  int iter;
  for (i = 1ULL, iter = 0; iter < 64; i <<= 1, iter++) {
    if (bitboard & i) {
      res->squares[position] = iter;
      position++;
    }
  }

  res->size = position;

  return res;
}

U64 occupied(piece_map_bitboards boards) {
  return boards.king + boards.pawns + boards.rooks + boards.queens +
         boards.bishops + boards.knights;
}

bitboard_base createBase(piece_map_bitboards white, piece_map_bitboards black) {
  bitboard_base base;
  base.white = white;
  base.black = black;
  base.white_occupied = occupied(white);
  base.black_occupied = occupied(black);
  return base;
}

U64 squares_to_bitboard(int squares[], int size) {
  U64 res = 0;

  int i;
  for (i = 0; i < size; i++) {
    res |= (1ULL << squares[i]);
  }

  return res;
}

// // --- Flipping ---
// U64 flipVertical(U64 x) {
//   const U64 k1 = C64(0x00FF00FF00FF00FF);
//   const U64 k2 = C64(0x0000FFFF0000FFFF);
//   x = ((x >> 8) & k1) | ((x & k1) << 8);
//   x = ((x >> 16) & k2) | ((x & k2) << 16);
//   x = (x >> 32) | (x << 32);
//   return x;
// }
// U64 flipHorizontal(U64 x) {
//   const U64 k1 = C64(0x5555555555555555);
//   const U64 k2 = C64(0x3333333333333333);
//   const U64 k4 = C64(0x0f0f0f0f0f0f0f0f);
//   x = ((x >> 1) & k1) | ((x & k1) << 1);
//   x = ((x >> 2) & k2) | ((x & k2) << 2);
//   x = ((x >> 4) & k4) | ((x & k4) << 4);
//   return x;
// }
// U64 flipDiagA1H8(U64 x) {
//   U64 t;
//   const U64 k1 = C64(0x5500550055005500);
//   const U64 k2 = C64(0x3333000033330000);
//   const U64 k4 = C64(0x0f0f0f0f00000000);
//   t = k4 & (x ^ (x << 28));
//   x ^= t ^ (t >> 28);
//   t = k2 & (x ^ (x << 14));
//   x ^= t ^ (t >> 14);
//   t = k1 & (x ^ (x << 7));
//   x ^= t ^ (t >> 7);
//   return x;
// }
// U64 flipDiagA8H1(U64 x) {
//   U64 t;
//   const U64 k1 = C64(0xaa00aa00aa00aa00);
//   const U64 k2 = C64(0xcccc0000cccc0000);
//   const U64 k4 = C64(0xf0f0f0f00f0f0f0f);
//   t = x ^ (x << 36);
//   x ^= k4 & (t ^ (x >> 36));
//   t = k2 & (x ^ (x << 18));
//   x ^= t ^ (t >> 18);
//   t = k1 & (x ^ (x << 9));
//   x ^= t ^ (t >> 9);
//   return x;
// }

// // --- Rotating ---
// U64 rotate90Clockwise(U64 x) { return flipVertical(flipDiagA1H8(x)); }
// U64 rotate180Clockwise(U64 x) { return flipHorizontal(flipVertical(x)); }
// U64 rotate270Clockwise(U64 x) { return flipDiagA1H8(flipVertical(x)); }

// // --- Pseudo-Rotating ---

// U64 pseudoRotate45clockwise(U64 x) {
//   const U64 k1 = C64(0xAAAAAAAAAAAAAAAA);
//   const U64 k2 = C64(0xCCCCCCCCCCCCCCCC);
//   const U64 k4 = C64(0xF0F0F0F0F0F0F0F0);
//   x ^= k1 & (x ^ rotateRight(x, 8));
//   x ^= k2 & (x ^ rotateRight(x, 16));
//   x ^= k4 & (x ^ rotateRight(x, 32));
//   return x;
// }

// U64 pseudoRotate45antiClockwise(U64 x) {
//   const U64 k1 = C64(0x5555555555555555);
//   const U64 k2 = C64(0x3333333333333333);
//   const U64 k4 = C64(0x0f0f0f0f0f0f0f0f);
//   x ^= k1 & (x ^ rotateRight(x, 8));
//   x ^= k2 & (x ^ rotateRight(x, 16));
//   x ^= k4 & (x ^ rotateRight(x, 32));
//   return x;
// }

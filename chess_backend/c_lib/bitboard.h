#include <stdbool.h>
#include <stdlib.h>
typedef unsigned long long  U64;
#define C64(constantU64) constantU64##ULL

// Type definitions
typedef struct piece_map_bitboards
{
  U64 pawns;
  U64 king;
  U64 queens;
  U64 bishops;
  U64 knights;
  U64 rooks;
} piece_map_bitboards;

typedef struct castling_rights {
  bool white_king;
  bool white_queen;
  bool black_king;
  bool black_queen;
} castling_rights;

typedef struct bitboard_base
{
  piece_map_bitboards white;
  U64 white_occupied;
  piece_map_bitboards black;
  U64 black_occupied;
} bitboard_base;

typedef struct square_array {
  size_t size;
  int squares[];
} square_array;

square_array * extract_squares(U64 bitboard);
bitboard_base createBase(piece_map_bitboards white, piece_map_bitboards black);
U64 squares_to_bitboard(int squares[], int size);

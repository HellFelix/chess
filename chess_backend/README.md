# Chess Backend
Backend library for board representaton and move generation using standard bitboard logic.

This is a low level representation of chessboards that require minimal external libraries. The only dependencies for this crate are build-dependencies used for compiling C.

### Bindgen
Note that because the low level board representation is written in C (along with some of the target 
generation), This crate uses [bindgen](https://github.com/rust-lang/rust-bindgen) to generate bindings 
for Rust. See bindgen documentation for instructions on how to generate bindings.

## Board representation
The board representation is a simple bitboard (U64) where each bit refers to the position of one square, 
(first bit representing a1 and last bit representing h8. Note that the pattern goes a1, b1, c1, ..., a2, b2, c2, ... 
following rows rather than a snakelike pattern).

The bitboard base used for representation in the Rust wrapper consists of one piece_map_bitboards 
(struct with positional bitboards for each piece) along with one occupancy bitboard (bitboard representation of 
occupied squares) each. One should also note that the occupancy bitboards should always be exclusive 
(i.e ``white_occupied & black_occupied = 0`` and ``white_occupied ^ black_occupied = white_occupied + black_occupied``)

The C library also proviedes representations for castling rights and a way of extracting squares from a bitboard (U64)
into component squares (in a dynamic array).

The `Board` struct abstraction in the Rust wrapper contains one bitboard base (for board structure), along with a killer
square (note that the killer square is simply the position of the killer square in the bitboard. Logically, it can only
take values of [15, 22] (a3 to h3 for white pawns) and [47, 55] (a6 to h6 for black pawns)). The `Board` struct also 
contains the aforementioned castling rights, the side to move and the board halfmove clock and fullmove length.
This is all the information about a game of chess that the engine requires to figure out which move to play. 

Note that because `Board` implements `Copy`, the performance cost for copying boards between scopes is low which 
allows for move generation to copy boards and mutate them rather than building them from scratch.

`Board` implements `Display` which allows for printing the board from standard POV (first row down, first rank left).
Note that using this method of displaying the board will also show side to move, the board killer square (if there is one), the halfmove clock, and the fullmove counter.
For instance, the standard starting position will be displayed as:
```
♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎
♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
White to move
Board has no killer square
Board halfmove clock is 0
Board fullmove is 1
```
and after `1. e4` it will be displayed as:
```
♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
. . . . . . . .
. . . . . . . .
. . . . ♟︎ . . .
. . . . . . . .
♟︎ ♟︎ ♟︎ ♟︎ . ♟︎ ♟︎ ♟︎
♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
Black to move
Board has killer square e3
Board halfmove clock is 1
Board fullmove is 1
```

### BitBoard
There is also a public `BitBoard` abstraction struct used mostly for debugging that implements `Display`. 
Displaying the `BitBoard` instances for the white occupancy for the boards above will yield:
```
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1
```
and
```
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . 1 . . .
. . . . . . . .
1 1 1 1 . 1 1 1
1 1 1 1 1 1 1 1
```
respectively.

## Move generation

THe public method `generate_legal_moves` in the Board struct generates a vector of legal moves from the current 
Board instance. This works by first generating pseudo-legal moves (all possible moves regardless of whether the move
would put the king in check or leave the king in check)

Legality checking is applied first afterward, checking if king is attacked, castling is legal (i.e king's path is not 
attacked or blocked by any pieces. If castling rights are true, pseudo-legal move generation will assume that castling 
is allowed), and that pawns are not on any of the back ranks.

Some of the target generation within the [C library](./c_lib/targets/) is based on code from 
[Code Monkey King](https://github.com/maksimKorzh/chess_programming/).


## FEN

`Board` also implements `From<Into<String>>` which allows a `Board` instance to be created from any string-like instance.
The function `from` will assume that the input is in correct [FEN format](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
and will not check the legality of the position. 
Converting back into FEN is handled by the `into_fen` method from a `Board` instance.

This crate also contains some public constants for FEN positions such as
- `EMPTY_BOARD`= "8/8/8/8/8/8/8/8 w - - 0 1"
- `START_POSITION` = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

## SAN

There is functionality for creating and playing moves from [SAN](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)),
and though it's (as far testing goes) functional, the performance is unoptimal due to how the ´make_san_move´ function 
handles board mutations by first generating legal moves, converting them into `SanMove` instances and seeing if there's a 
match. The benefit of this is that a there is no way of making an illegal move, and disambiguation is already handled by the
`get_san` function, however, it does mean that this functionality is quite slow. Therefore, this functionality is only used
for user inputted moves (assuming that the user knows SAN) and creating the position tree in the [opening database](../chess_openings/)

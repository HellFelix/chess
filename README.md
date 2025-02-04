# Chess
This project covers the making of a chess engine all the way from low level bitboard representation and movegeneration all the way up to high level move searching and evaluation algorithms. 
See [engine documentation](./chess_engine/README.md) for usage.

Each of the three subdirectories in this repository covers one of the main modules in this project
### Backend (Representation and Game)
This is the backbone of the project, acting as the base layer upon which everything else is built.

For further information, see [backend documentation](./chess_backend/README.md)

### Openings (Database Generation and Management)
Using the [FICS games database](https://www.ficsgames.org/), this crate creates a database with a tree-like move structure that contains thousands of the most common opening lines, allowing for engine use 

For further information, see [openings documentation](./chess_openings/README.md)

### Engine (Main Engine Program)
The engine itself is a consists of...

## Further improvements
Having worked on this project for a while, I am not too concerned with further developing this project. There are, however, a couple of things I would like to think I will add in the future (no timeline currently)
* Transposition table
* Pawn structure lookup table
* Endgame Tablebase
* GUI to make playing the engine easier

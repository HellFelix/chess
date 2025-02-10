[![GitHub tag](https://img.shields.io/github/tag/HellFelix/chess?include_prereleases=&sort=semver&color=blue)](https://github.com/HellFelix/chess/releases/)
[![License](https://img.shields.io/badge/License-MIT-blue)](#license)

# Chess
This project covers the making of a chess engine all the way from low level bitboard representation and movegeneration all the way up to high level move searching and evaluation algorithms. 
See [engine documentation](./chess_engine/README.md) for usage.

Each of the three subdirectories in this repository covers one of the main modules in this project
### Backend (Representation and Game)
This is the backbone of the project, acting as the base layer upon which everything else is built.

For more information, see [backend documentation](./chess_backend/README.md).

### Openings (Database Generation and Management)
Using the [FICS games database](https://www.ficsgames.org/), this crate creates a database with a tree-like move structure that contains thousands of the most common opening lines, which can be used by the engine as a reference for how to play most of the common openings.

For more information, see [openings documentation](./chess_openings/README.md).

### Engine (Main Engine Program - BRAIN)
The engine itself splits the evaluation of a position into tow phases, first running preliminaty evaluation followed by a deep search. The search is split between the available threads, and thus it will probably perform better on machines with more cpus since more of the tree can be searched at the same time. Based on limited testing, the engine performs at a rating of about 2200.

For more information, see [engine documentation](./chess_engine/README.md).

## Further improvements
Having worked on this project for a while, I am not too concerned with further developing this project. There are, however, a couple of things I would like to think I will add in the future (no timeline currently)
* Transposition table
* Pawn structure lookup table
* Endgame Tablebase
* GUI to make playing the engine easier

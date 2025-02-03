# Chess Openings
This crate handles the creation of an opening database in the form of a SQLite file 
for the engine. This is done by iterating
though master-level games and adding the opening moves played to a tree that an engine instance can follow along (if the opponent's moves match).

Databases have been created using games downloaded from 
[FICS games database](https://www.ficsgames.org/), though as long as the games are in valid pgn 
format, these should be no issues. 

If the database doesn't yet exist, the program will create one, otherwise, it will assume that 
the database has already been initiated and add to it. 

Each entry in the moves table will have the following properties:
* id (A unique identifier for eacch move and it's position in the tree)
* parent_move (Shared by all children of the parent node so that children can be found)
* children (Number of children form this node)
* san (San for the move (For the origin node, this is simply 'Origin'))
* eval (Predetermined evaluation to determine what move to play)
* frequency (The number of games this move was made. Incremented every time its found)
* terminal (False if there are further children node, else true)

Because the Origin node has `id=1`, we can find all first moves with 

```sql
SELECT * FROM moves WHERE parent_move=1;
```

Which should yield something similar to the following table

|id  |parent_move |children |san |eval          |frequency |terminal|
|----|------------|---------|----|--------------|----------|--------|
|2   |1           |14       |d4  |2413.04410015 |1166      |0       |
|24  |1           |15       |e4  |2637.18820047 |1323      |0       |
|412 |1           |3        |g4  |54.5316       |29        |0       |
|560 |1           |10       |c4  |283.37740003  |138       |0       |
|582 |1           |2        |a3  |7.27760005    |4         |0       |      
|604 |1           |3        |Na3 |14.51939997   |7         |0       |
|815 |1           |10       |Nf3 |349.94720002  |173       |0       |
|... |

Along with just creating the tree, the program also keeps track of board position as it
iterates through the games, adding FEN strings for each move and adding them to another
table called `boards`. The board fen can be retrieved, knowing the id of the latest move made.
With the table above, retrieving the FEN for the board after "1. d4" (id=2) could be done with
```sql
SELECT fen FROM boards WHERE id=2;
```
Of course, the id could be replaced with any id of the ones in the `moves` table.

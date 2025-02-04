# Engine
This crate is the culmination of this project.

## Usage: Controllers and Engines
A game is played though a controller. The controller contains two players, which can either be `Manual` or
`Player` depending on how each side is managed. When a move is to be played, the controller will request a
move from the player whose side it is to move. If the player is manual, the controller will wait for an 
input (in SAN format) and execute the inputted move. If the player is engine, the controller will create
an `Engine` instance to handle the finding of the next move. No information is saved for the next `Engine`.

### Performance
From testing against some of the bots on chess.com, the engine performs at an accuracy between 75% and 90%
depending on the opponent's rating. The game review feature estimated the engines game rating at about 2200-2400.
It was able to beat all the bots up to a rating of 2300.


## Evaluation (Heuristics)
When evaluation a board position, the `Engine` will first check to see if the game is still ongoing. 
If not, then the evaluation is absolute. For a draw, evaluation is 0, and for checkmate, the evaluation
is simply `Mate`, see [Eval](#Eval). If the game is not over, evaluation comes from the `eval_heuristic`
function.

### Eval
The `Eval` enum is the value assigned to each position. There are four types of Eval with different purposes.
With the relative values describes below, positions can be ordered based on their attached evaluations.
| Type | Purpose | Value |
|------|---------|-------|
| Numeric | This is the normal evaluation that comes out of the heuristic evaluation of a position | The value is simply the attached number |
| Mate | This represents the value of a position where one side has won. It comes attached with a colour (winning colour) and the depth from the current position | White Mate can be thought of as a sort of infinity, whereas Black Mate can be thought of as a sort of negative infinity, with the added caveat that Mate at a lower depth is better than Mate at a higher depth |
| Infinity | Acts as an initial value within the minimax algorithm | This is always greater than any other Eval instance, even White Mate |
| NegInfinity | Acts as an initial value within the minimax algorithm, analogous to Infinity | This is always less than any other Eval instance, even Black Mate |

Examples:
* ``Numeric(7.) > Numeric(4.)``
* ``Mate(1, White) > Mate(3, White)``
* ``Mate(1, Black) < Mate(3, Black)``
* ``Mate(1, White) < Infinity``

## Tree Structure
The search tree has a simple [B-tree](https://en.wikipedia.org/wiki/B-tree) structure. Each node (`Branch` instance) 
in the tree contains all necessary data for the evaluation, expansion, and searching required to search the tree
from that `Branch`.

## Searching
The searching algorithm is largely "best first" exploration. Divided into two phases that have different
methods for finding the next node to be explored, the move search can be tailored to be more wide search 
or more deep search heavy by extending or shortening the time spent on preliminary search.

At the start of each phase, the tree is divided into roughly equal parts so that each thread has a separate
area of responsibility in order to prevent crossover in searching with multiple threads. 

### Phases
#### Preliminary Search Phase
The preliminary search prioritises a wide search, disregarding heuristics and simply exploring the "shallowest" 
unexpanded node. This ensures that the engine doesn't miss any crazy sacrifices or similar tactics that may, at
first glance only seem to lose material.

#### Main Search Phase
The main search prioritises the best moves first (i.e. the move most likely to be played based on a heuristic evaluation).
In order not to cut off capture chains or recaptures, and also prevent tunnelvision, there are some modifires added to the
heuristic evaluation to create a priority score which is what's actually used to find the most promising move. Of course,
when actually choosing a move, the priority is disregarded, only considering the heuristic evaluation of terminal nodes.


#### Dependencies
The main dependency of the crate is the [chess backend crate](../chess_backend/) which was built
for the purpose of compatible with this crate. Note that the [opening database](../chess_openings/)
is not a dependency, rather relying on the preexistence of a database file that can be used.

Other dependencies include some bare-bones essentials for logging, constant function looping, 
database handling, thread management, and multithreading.

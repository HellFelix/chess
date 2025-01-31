use std::marker::{Send, Sync};
use std::sync::{Arc, Mutex};

use chess_backend::{to_str, Board, ChessMove, Colour, MoveType, Piece};
use log::{debug, error};

use crate::engine::utils::eval::Eval;
use crate::engine::utils::phase::GamePhase;

#[derive(Debug, Clone)]
pub struct Branch {
    pub board: Board,
    pub _res_move: Option<ChessMove>, // debug purposes only
    pub eval: Option<Eval>,
    pub phase: Option<GamePhase>,
    pub children: Vec<Branch>,
    priority: Option<Eval>,
    pub is_terminal: bool,
}
unsafe impl Sync for Branch {}
unsafe impl Send for Branch {}

impl Branch {
    pub fn populate(&mut self, depth: usize) {
        self.children = self
            .board
            .generate_legal_moves()
            .iter()
            .map(|m| (self.from_parent(*m, depth)))
            .collect();
    }

    pub fn run_base_node(&mut self) {
        self.eval_node(None, 0);
        self.run_node(0);
    }

    pub fn run_node(&mut self, depth: usize) {
        debug!("Running node");
        self.is_terminal = false;
        for child in &mut self.children {
            child.eval_node(Some(self.board), depth + 1);
        }
    }

    fn eval_node(&mut self, parent_board: Option<Board>, depth: usize) {
        self.populate(depth);

        self.is_terminal = true;

        let heuristic = self.eval_position(self.children.len(), depth);
        self.eval = Some(heuristic);
        self.priority = if let Some(orig_board) = parent_board {
            Some(Self::calc_priority(
                orig_board, self.board, depth, heuristic,
            ))
        } else {
            None
        }
    }

    pub fn find_surface_terminal(&self, relative_location: &Vec<usize>) -> Option<Vec<usize>> {
        let mut depth = 0;
        if self.is_terminal {
            debug!("Found terminal");
            return Some(relative_location.clone());
        }
        loop {
            if let Some(res) = self.find_terminal_level(depth, relative_location) {
                return Some(res);
            } else if depth > 10 {
                return None;
            } else {
                depth += 1;
            }
        }
    }

    pub fn find_terminal_level(
        &self,
        level: usize,
        relative_location: &Vec<usize>,
    ) -> Option<Vec<usize>> {
        if level > 0 {
            for (i, child) in self.children.iter().enumerate() {
                let mut location = relative_location.clone();
                location.push(i);
                if let Some(res) = child.find_terminal_level(level - 1, &location) {
                    return Some(res);
                }
            }
        } else {
            // level == 0
            for (i, child) in self.children.iter().enumerate() {
                let mut location = relative_location.clone();
                location.push(i);
                if child.is_terminal {
                    return Some(location);
                }
            }
        }
        None
    }

    pub fn search_absolute_priority(&self, relative_location: &[usize]) -> (Eval, Vec<usize>) {
        if self.is_terminal {
            (self.priority.unwrap(), relative_location.into())
        } else {
            let mut highest_prio = Eval::NegInfinity;
            let mut highest_location = vec![];
            for (i, child) in self.children.iter().enumerate() {
                let child_location = [relative_location, &[i]].concat();
                let (prio, location) = child.search_absolute_priority(&child_location);
                if prio > highest_prio {
                    highest_prio = prio;
                    highest_location = location;
                }
            }
            (highest_prio, highest_location)
        }
    }

    pub fn alpha_beta_search_priority(
        &self,
        maximize: bool,
        relative_location: &[usize],
        alpha: Eval,
        beta: Eval,
        claimed: &Vec<Vec<usize>>,
    ) -> (Eval, Vec<usize>) {
        if self.is_terminal {
            // Unwrap should be safe. All terminal nodes have been evaluated
            (self.priority.unwrap(), relative_location.into())
        } else if maximize {
            let mut max_eval = Eval::NegInfinity;
            let mut max_location = vec![];
            let mut alpha = alpha;
            for (i, child) in self.children.iter().enumerate() {
                let child_location = [relative_location, &[i]].concat();
                if let Some(_prel_eval) = child.eval {
                    let (eval, location) = child.alpha_beta_search_priority(
                        false,
                        &child_location,
                        alpha,
                        beta,
                        claimed,
                    );
                    if eval > max_eval && !claimed.contains(&child_location) {
                        max_eval = eval;
                        max_location = location;
                    }

                    // beta cutoff
                    alpha = alpha.max(max_eval);
                    if max_eval >= beta {
                        break;
                    }
                }
            }

            max_eval = if max_eval == Eval::NegInfinity {
                Eval::Infinity
            } else {
                max_eval
            };
            (max_eval, max_location)
        } else {
            let mut min_eval = Eval::Infinity;
            let mut min_location = vec![];
            let mut beta = beta;
            for (i, child) in self.children.iter().enumerate() {
                let child_location = [relative_location, &[i]].concat();
                if let Some(_prel_eval) = child.eval {
                    let (eval, location) = child.alpha_beta_search_priority(
                        true,
                        &child_location,
                        alpha,
                        beta,
                        claimed,
                    );

                    if eval < min_eval && !claimed.contains(&child_location) {
                        min_eval = eval;
                        min_location = location;
                    }

                    // alpha cutoff
                    beta = beta.min(min_eval);
                    if min_eval <= alpha {
                        break;
                    }
                }
            }

            min_eval = if min_eval == Eval::Infinity {
                Eval::NegInfinity
            } else {
                min_eval
            };
            (min_eval, min_location)
        }
    }

    // Doesn't evaluate positions, simply rearanges with new information
    pub fn simple_minimax(
        &mut self,
        maximize: bool,
        relative_location: &Vec<usize>,
    ) -> (Eval, Vec<usize>) {
        if self.is_terminal {
            // Unwrap should be safe. All terminal nodes have been evaluated
            (self.eval.unwrap(), relative_location.clone())
        } else if maximize {
            let mut max_eval = Eval::NegInfinity;
            let mut max_location = vec![];
            for (i, child) in &mut self.children.iter_mut().enumerate() {
                let mut child_location = relative_location.clone();
                child_location.push(i);
                if let Some(_prel_eval) = child.eval {
                    let (eval, location) = child.simple_minimax(false, &child_location);
                    if eval > max_eval {
                        max_eval = eval;
                        max_location = location;
                    }
                }
            }
            self.eval = Some(max_eval);
            (max_eval, max_location)
        } else {
            let mut min_eval = Eval::Infinity;
            let mut min_location = vec![];
            for (i, child) in &mut self.children.iter_mut().enumerate() {
                let mut child_location = relative_location.clone();
                child_location.push(i);
                if let Some(_prel_eval) = child.eval {
                    let (eval, location) = child.simple_minimax(true, &child_location);

                    if eval < min_eval {
                        min_eval = eval;
                        min_location = location;
                    }
                }
            }
            self.eval = Some(min_eval);
            (min_eval, min_location)
        }
    }

    pub fn get_best(&mut self, maximize: bool, relative_location: &Vec<usize>) -> Option<&Branch> {
        debug!("Finding best");
        // fix tree after expanded search
        self.simple_minimax(maximize, relative_location);
        if maximize {
            self.children
                .iter()
                .max_by(|c1, c2| c1.eval.partial_cmp(&c2.eval).unwrap())
        } else {
            self.children
                .iter()
                .min_by(|c1, c2| c1.eval.partial_cmp(&c2.eval).unwrap())
        }
    }

    pub fn find_branch(&self, location: &[usize]) -> &Self {
        if location.len() == 0 {
            self
        } else {
            self.children[location[0]].find_branch(&location[1..])
        }
    }

    pub fn insert_branch(&mut self, input_branch: Branch, location: &[usize]) {
        if location.len() == 0 {
            *self = input_branch;
        } else {
            self.children[location[0]].insert_branch(input_branch, &location[1..]);
        }
    }

    pub fn show_branch(&self, depth: usize) {
        if self.eval != None {
            for _ in 0..depth {
                print!("|   ");
            }
            if depth == 0 {
                print!("=>");
            } else {
                print!("({depth})-");
            }
            if let Some(m) = self._res_move {
                if let Some(dest) = m.base.destination_square {
                    print!("{:?} -> {:?}, ", m.base.piece, to_str(dest));
                }
            }
            println!(
                "{} {:?} {:?} {}",
                self.is_terminal,
                self.eval,
                self.priority,
                self.board.side_to_move() == Colour::White
            );
            for child in &self.children {
                child.show_branch(depth + 1);
            }
        }
    }

    /// Creates a new branch that should inherit the game phase from its parent.
    /// If the phase is None, it will be determined at the next evaluation
    pub fn from_parent(&self, m: ChessMove, depth: usize) -> Self {
        Self {
            board: m.board,
            _res_move: Some(m),
            eval: None,
            phase: self.phase,
            priority: None,
            children: Vec::new(),
            is_terminal: false,
        }
    }
}
impl From<Board> for Branch {
    fn from(value: Board) -> Self {
        Self {
            board: value,
            _res_move: None,
            eval: None,
            phase: None,
            children: Vec::new(),
            priority: None,
            is_terminal: false,
        }
    }
}
impl Default for Branch {
    fn default() -> Self {
        Self {
            board: Board::default(),
            _res_move: None,
            eval: None,
            phase: Some(GamePhase::Opening(1)),
            children: Vec::new(),
            priority: None,
            is_terminal: false,
        }
    }
}

use std::{
    error::Error,
    io,
    slice::Iter,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread::sleep,
    time::{Duration, SystemTime},
    usize,
};

use log::{debug, error, info};
use opening_book::search_manual;
use sqlite::{self, Connection};
const DB_PATH: &str = "openings.db";

use chess_backend::{Board, Colour, GameState, SanMove};
use threadpool::ThreadPool;
use tree::Branch;
use utils::{
    error::EngineError,
    eval::Eval,
    phase::{self, GamePhase},
};

pub mod heuristics;
mod opening_book;
pub mod tree;
pub mod utils;

fn get_db_connection() -> Connection {
    Connection::open(DB_PATH).expect("Failed to connect to opening database")
}

#[derive(Debug, Clone, Copy)]
pub enum Player {
    Manual,
    Engine,
}

const PRELIMINARY_TIME_SHARE: f32 = 0.5;

pub struct EngineController {
    white: Player,
    black: Player,
    time_limit: Duration,
    board: Board,
    n_workers: usize,
    db_conn: Connection,
    phase: Option<GamePhase>,
}
impl EngineController {
    fn init() {
        chess_backend::init();
    }
    pub fn new(
        white: Player,
        black: Player,
        board: Board,
        n_workers: usize,
        time_limit: Duration,
        phase: Option<GamePhase>,
    ) -> Self {
        Self {
            white,
            black,
            time_limit,
            board,
            n_workers,
            db_conn: get_db_connection(),
            phase,
        }
    }
    pub fn pick_move(&mut self, time_limit: Duration) {
        let mut engine = Engine::new(self.board, self.n_workers, self.phase);
        (self.board, self.phase) = engine.begin_search(
            time_limit.mul_f32(PRELIMINARY_TIME_SHARE),
            time_limit.mul_f32(1. - PRELIMINARY_TIME_SHARE),
            self.phase,
            &self.db_conn,
        );
    }

    pub fn show_board(&self) {
        println!("{}", self.board);
    }

    pub fn get_game_state(&self) -> GameState {
        self.board.get_game_state()
    }

    pub fn is_over(&self) -> bool {
        self.get_game_state() != GameState::Ongoing
    }

    pub fn get_phase(&self) -> Option<GamePhase> {
        self.phase
    }

    fn request_move(&mut self, player: Player) -> Result<(), EngineError> {
        match player {
            Player::Engine => {
                self.pick_move(self.time_limit);
                Ok(())
            }
            Player::Manual => {
                let m = io::stdin().lines().next().unwrap().unwrap();
                self.manual_move(
                    SanMove::from_string(m.trim(), self.board).expect("Failed to parse san move"),
                )
            }
        }
    }

    pub fn manual_move(&mut self, m: impl Into<SanMove>) -> Result<(), EngineError> {
        let san_m: SanMove = m.into();
        if let Ok(_) = self.board.make_san_move(san_m) {
            if let Some(GamePhase::Opening(id)) = self.phase {
                if let Some(phase) = search_manual(&self.db_conn, id, san_m.into()) {
                    self.phase = Some(phase);
                } else {
                    self.phase = Some(GamePhase::MiddleGame)
                }
            }
            Ok(())
        } else {
            Err(EngineError::InputError)
        }
    }

    pub fn set_white(&mut self, player: Player) {
        self.white = player;
    }
    pub fn set_black(&mut self, player: Player) {
        self.black = player;
    }

    pub fn play(&mut self) -> Result<(), EngineError> {
        Self::init();

        println!("{}", self.board);
        while let GameState::Ongoing = self.board.get_game_state() {
            let colour = self.board.side_to_move();
            match colour {
                Colour::White => self.request_move(self.white)?,
                Colour::Black => self.request_move(self.black)?,
            }
            println!("{}", self.board);
        }

        Ok(())
    }
}
impl Default for EngineController {
    fn default() -> Self {
        Self {
            white: Player::Engine,
            black: Player::Engine,
            time_limit: Duration::from_secs(2),
            board: Board::default(),
            n_workers: num_cpus::get(),
            db_conn: get_db_connection(),
            phase: Some(GamePhase::Opening(1)),
        }
    }
}

#[derive(Debug, Clone)]
enum WorkerType {
    // Contains the base node that this worker covers
    DeepSearch((Vec<Vec<usize>>, usize)),
    // Contains the surface locations that are within the worker's responsibility
    WideSearch((Vec<Vec<usize>>, usize)),
}

type WorkerRes = (
    Option<(Branch, Vec<usize>, bool)>,
    (Option<Vec<usize>>, WorkerType),
);

#[derive(Debug)]
struct Engine {
    branch: Arc<RwLock<Branch>>,
    workers: ThreadPool,
    n_workers: usize,
    sender_model: Sender<WorkerRes>,
    receiver: Receiver<WorkerRes>,
}
impl Engine {
    pub fn new(board: Board, n_workers: usize, phase: Option<GamePhase>) -> Self {
        let (sender_model, receiver) = channel();
        Self {
            branch: Arc::new(RwLock::new(Branch::from(board))),
            workers: ThreadPool::new(n_workers),
            sender_model,
            n_workers,
            receiver,
        }
    }
    pub fn begin_search(
        &mut self,
        prel_search_limit: Duration,
        deep_search_limit: Duration,
        phase: Option<GamePhase>,
        db_conn: &Connection,
    ) -> (Board, Option<GamePhase>) {
        if let Some(p) = phase {
            match p {
                GamePhase::Opening(id) => {
                    if let Ok((board, phase)) = opening_book::play_bookmove(db_conn, id) {
                        if let Some(res_board) = board {
                            (res_board, phase)
                        } else {
                            self.search(prel_search_limit, deep_search_limit)
                        }
                    } else {
                        info!("Falling back to tree search");
                        self.search(prel_search_limit, deep_search_limit)
                    }
                }
                _ => self.search(prel_search_limit, deep_search_limit),
            }
        } else {
            // Likely the first search, meaning the phase has yet to be determined
            info!("Likely first");
            self.search(prel_search_limit, deep_search_limit)
        }
    }

    fn search(
        &mut self,
        prel_search_limit: Duration,
        deep_search_limit: Duration,
    ) -> (Board, Option<GamePhase>) {
        self.preliminary_search(prel_search_limit);
        self.main_search(deep_search_limit);

        // Then choose the best branch from the explored tree
        let colour = if let Ok(branch) = self.branch.read() {
            branch.board.side_to_move()
        } else {
            panic!("Failed to read branch");
        };
        if let Ok(mut branch) = self.branch.write() {
            let best = branch.get_best(colour == Colour::White, &vec![]);
            info!("Best eval is {:?}", best.unwrap().eval);

            let res = if let Some(chosen) = best {
                (chosen.board, chosen.phase)
            } else {
                debug!("Incomplete");
                panic!("Failed to analyze position");
            };
            //branch.show_branch(0);
            res
        } else {
            panic!("Failed to read branch");
        }
    }

    fn divide_base(&self) -> Vec<Vec<Vec<usize>>> {
        debug!("dividing");
        if let Ok(tree) = self.branch.read() {
            debug!("Found tree");
            let mut children = tree.children.iter().enumerate();

            let mut res = vec![vec![]; self.n_workers.min(children.len())];
            let mut current = 0;
            while let Some((i, _branch)) = children.next() {
                res[current].push(vec![i]);
                if current + 1 == res.len() {
                    current = 0;
                } else {
                    current += 1;
                }
            }
            res
        } else {
            panic!("Could not read tree");
        }
    }

    // conducts preliminary search (only surface level)
    fn preliminary_search(&mut self, time_limit: Duration) {
        debug!("Running preliminary");
        let start_time = SystemTime::now();
        self.add_base_job();
        'search_loop: loop {
            for (res, worker_res) in self.receiver.try_iter() {
                // set subbranch if such is returned
                if let Some((res_branch, location, first)) = res {
                    if let Ok(mut branch) = self.branch.write() {
                        branch.insert_branch(res_branch, location.as_slice());
                    }
                    if first {
                        debug!("Caught first");
                        let worker_covers = self.divide_base();
                        debug!("Covers are {worker_covers:?}");
                        for locations in worker_covers {
                            debug!("Sending worker with {locations:?}");
                            self.add_find_only(WorkerType::WideSearch((locations, 0)));
                        }
                        continue;
                    }
                }
                debug!("Caught not first");
                // setup next job
                let (next, _worker) = worker_res;
                let next_worker = if let WorkerType::WideSearch((locations, i)) = _worker {
                    debug!("found locations {locations:?}");
                    let next_i = if i + 1 == locations.len() { 0 } else { i + 1 };
                    WorkerType::WideSearch((locations, next_i))
                } else {
                    continue;
                };
                let requested = if let Some(_req) = next {
                    debug!("requested is {_req:?}");
                    _req
                } else {
                    self.add_find_only(next_worker);
                    continue;
                };

                self.add_job(requested, next_worker);

                if start_time.elapsed().unwrap() > time_limit {
                    break 'search_loop;
                }
            }
        }
    }

    fn main_search(&mut self, time_limit: Duration) {
        let start_time = SystemTime::now();
        let bases = self.divide_base();
        for locations in bases {
            self.add_find_only(WorkerType::DeepSearch((locations, 0)));
        }
        'search_loop: loop {
            for (res, worker_res) in self.receiver.try_iter() {
                if let Some((res_branch, location, _first)) = res {
                    if let Ok(mut branch) = self.branch.write() {
                        branch.insert_branch(res_branch, location.as_slice());
                    }
                }
                let (next, _worker) = worker_res;
                let next_worker = if let WorkerType::DeepSearch((locations, i)) = _worker {
                    if next == Some(vec![]) {
                        continue;
                    }
                    let next_i = if i + 1 == locations.len() { 0 } else { i + 1 };
                    WorkerType::DeepSearch((locations, next_i))
                } else {
                    continue;
                };
                let requested = if let Some(_req) = next {
                    _req
                } else {
                    self.add_find_only(next_worker);
                    continue;
                };

                self.add_job(requested, next_worker);

                if start_time.elapsed().unwrap() > time_limit {
                    // When an answer has been demanded, let the current threads finish
                    break 'search_loop;
                }
            }
        }
        info!("Joining workers");
        self.workers.join();
    }

    // // divides the main tree into the branches that each deep searcher will cover
    // fn divide_tree(&self) -> Vec<Vec<usize>> {
    //     let mut claimed = Vec::new();
    //     if let Ok(branch) = self.branch.read() {
    //         for _ in 0..SEARCHERS {
    //             claimed.push(vec![
    //                 branch
    //                     .alpha_beta_search_priority(
    //                         branch.board.side_to_move() == Colour::White,
    //                         &vec![],
    //                         Eval::NegInfinity,
    //                         Eval::Infinity,
    //                         &claimed,
    //                     )
    //                     .1[0],
    //             ])
    //         }
    //     }
    //     info!("{claimed:?}");
    //     claimed
    // }

    fn add_find_only(&self, worker_type: WorkerType) {
        let tx = self.sender_model.clone();
        let tree_ref = Arc::clone(&self.branch);
        self.workers.execute(move || {
            if let Ok(tree) = tree_ref.read() {
                let res = match worker_type {
                    WorkerType::DeepSearch((locations, i)) => (
                        Self::get_next_deep(&tree, &locations[i]),
                        WorkerType::DeepSearch((locations, i)),
                    ),
                    WorkerType::WideSearch((locations, i)) => (
                        Self::get_next_wide(&tree, &locations[i]),
                        WorkerType::WideSearch((locations, i)),
                    ),
                };
                tx.send((None, res)).expect("Failed to send waiting worker");
            } else {
                panic!("Failed to read branch ref");
            }
        });
    }

    fn add_job(&self, location: Vec<usize>, worker_type: WorkerType) {
        let tx = self.sender_model.clone();
        let tree_wrapper = Arc::clone(&self.branch);
        let mut node = self.find_node(&location.as_slice());
        self.workers.execute(move || {
            let start_time = SystemTime::now();
            debug!("Evaluating {location:?}");
            node.run_node(location.len());

            // determine next node to be evaluated
            let next_worker = if let Ok(tree) = tree_wrapper.read() {
                match worker_type {
                    WorkerType::DeepSearch((locations, i)) => (
                        Self::get_next_deep(&tree, &locations[i]),
                        WorkerType::DeepSearch((locations, i)),
                    ),
                    WorkerType::WideSearch((locations, i)) => (
                        Self::get_next_wide(&tree, &locations[i]),
                        WorkerType::WideSearch((locations, i)),
                    ),
                }
            } else {
                panic!("Failed to read tree");
            };

            tx.send((Some((node, location.clone(), false)), next_worker))
                .expect("Failed to send finished branch");
            debug!("job took {}us", start_time.elapsed().unwrap().as_micros());
        });
    }

    fn get_next_deep(tree: &Branch, worker_base_location: &Vec<usize>) -> Option<Vec<usize>> {
        debug!("Getting next deep");
        let branch = tree.find_branch(&worker_base_location);
        let found = branch.search_absolute_priority(&worker_base_location);

        // .alpha_beta_search_priority(
        //     branch.board.side_to_move() == Colour::White,
        //     &worker_base_location,
        //     Eval::NegInfinity,
        //     Eval::Infinity,
        //     &vec![],
        // )
        // .1;
        debug!("Branch {worker_base_location:?} found {found:?}");
        Some(found.1)
    }
    fn get_next_wide(tree: &Branch, relative_location: &Vec<usize>) -> Option<Vec<usize>> {
        debug!("Finding from {relative_location:?}");
        let branch = tree.find_branch(&relative_location);
        branch.find_surface_terminal(relative_location)
    }

    fn add_base_job(&self) {
        let tx = self.sender_model.clone();

        let mut node = self.find_node(&vec![].as_slice()).clone();
        self.workers.execute(move || {
            debug!("Initializing base node");
            node.run_base_node();
            tx.send((
                Some((node, vec![], true)),
                (None, WorkerType::WideSearch((vec![], 0))),
            ))
            .expect("Failed to send finished branch");
        });
    }

    fn find_node(&self, location: &[usize]) -> Branch {
        if let Ok(branch) = self.branch.write() {
            branch.find_branch(&location).clone()
        } else {
            panic!("Unable to read branch");
        }
    }
}
impl Default for Engine {
    fn default() -> Self {
        let (sender_model, receiver) = channel();
        Self {
            branch: Arc::new(RwLock::new(Branch::default())),
            workers: ThreadPool::default(),
            sender_model,
            n_workers: num_cpus::get(),
            receiver,
        }
    }
}

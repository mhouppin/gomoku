use crate::core::{board::Board, types::Square};
use std::io::{self, Write};
use std::sync::{atomic::AtomicBool, Arc};
use std::time::{Duration, Instant};

use super::movegen::Movegen;

lazy_static! {
    pub static ref STOP: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
}

pub struct Params {
    searchmoves: Option<Vec<Square>>,
    wtime: Option<Duration>,
    btime: Option<Duration>,
    winc: Option<Duration>,
    binc: Option<Duration>,
    depth: Option<u16>,
    nodes: Option<u64>,
    mate: Option<u16>,
    movetime: Option<Duration>,
}

impl Params {
    pub fn new() -> Self {
        Self {
            searchmoves: None,
            wtime: None,
            btime: None,
            winc: None,
            binc: None,
            depth: None,
            nodes: None,
            mate: None,
            movetime: None,
        }
    }

    pub fn set_searchmoves(&mut self, searchmoves: &[Square]) {
        self.searchmoves = Some(Vec::from(searchmoves));
    }

    pub fn set_wtime(&mut self, wtime: Duration) {
        self.wtime = Some(wtime);
    }

    pub fn set_btime(&mut self, btime: Duration) {
        self.btime = Some(btime);
    }

    pub fn set_winc(&mut self, winc: Duration) {
        self.winc = Some(winc);
    }

    pub fn set_binc(&mut self, binc: Duration) {
        self.binc = Some(binc);
    }

    pub fn set_depth(&mut self, depth: u16) {
        self.depth = Some(depth);
    }

    pub fn set_nodes(&mut self, nodes: u64) {
        self.nodes = Some(nodes);
    }

    pub fn set_mate(&mut self, mate: u16) {
        self.mate = Some(mate);
    }

    pub fn set_movetime(&mut self, movetime: Duration) {
        self.movetime = Some(movetime);
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SearchData {
    start: Instant,
    node_count: u64,
    iter_depth: u16,
    seldepth: u16,
}

impl SearchData {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            node_count: 0,
            iter_depth: 0,
            seldepth: 0,
        }
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.start
    }

    pub fn node_count(&self) -> u64 {
        self.node_count
    }

    pub fn inc_node_count(&mut self) {
        self.node_count += 1;
    }

    pub fn iter_depth(&self) -> u16 {
        self.iter_depth
    }

    pub fn inc_iter_depth(&mut self) {
        self.iter_depth += 1;
    }

    pub fn seldepth(&self) -> u16 {
        self.seldepth
    }

    pub fn set_seldepth(&mut self, depth: u16) {
        self.seldepth = self.seldepth.max(depth);
    }
}

impl Default for SearchData {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run_search(board: &Board, _params: &Params) -> io::Result<()> {
    let mut data = SearchData::new();
    let mut movegen = Movegen::new();

    data.inc_iter_depth();
    data.inc_node_count();
    data.set_seldepth(1);
    movegen.generate_near(board);

    let bestmove = movegen.select_random_move();

    println!(
        "info depth {} seldepth {} score cp 0 nodes {} time {} pv {}",
        data.iter_depth(),
        data.seldepth(),
        data.node_count(),
        data.elapsed().as_millis(),
        bestmove
    );
    println!("bestmove {}", bestmove);
    io::stdout().flush()
}

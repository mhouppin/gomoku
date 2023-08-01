use crate::core::{board::Board, types::Square};
use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use super::eval;
use super::movegen::Movegen;
use super::score::{Score, ScoreKind};

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

    pub fn depth(&self) -> u16 {
        self.depth.unwrap_or(u16::MAX)
    }

    pub fn set_nodes(&mut self, nodes: u64) {
        self.nodes = Some(nodes);
    }

    pub fn nodes(&self) -> u64 {
        self.nodes.unwrap_or(u64::MAX)
    }

    pub fn set_mate(&mut self, mate: u16) {
        self.mate = Some(mate);
    }

    pub fn set_movetime(&mut self, movetime: Duration) {
        self.movetime = Some(movetime);
    }

    pub fn movetime(&self) -> Duration {
        self.movetime.unwrap_or(Duration::MAX)
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
    check_in: u16,
    bestmove: Square,
}

impl SearchData {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            node_count: 0,
            iter_depth: 0,
            seldepth: 0,
            check_in: 0,
            bestmove: Square::new(0),
        }
    }

    pub fn check_time(&mut self, params: &Params) {
        if self.check_in != 0 {
            self.check_in -= 1;
        } else {
            self.check_in = 256;

            if self.node_count >= params.nodes() || self.elapsed() >= params.movetime() {
                STOP.store(true, Ordering::Relaxed);
            }
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

    pub fn bestmove(&self) -> Square {
        self.bestmove
    }

    pub fn set_bestmove(&mut self, mv: Square) {
        self.bestmove = mv;
    }
}

impl Default for SearchData {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run_search(board: &Board, params: &Params) -> io::Result<()> {
    let mut data = SearchData::new();
    let mut board = board.clone();

    STOP.store(false, Ordering::Relaxed);

    while data.iter_depth() < params.depth() {
        data.inc_iter_depth();
        data.set_seldepth(0);
        let iter_depth = data.iter_depth();

        let score = search(
            &mut board,
            &mut data,
            params,
            iter_depth,
            Score::MIN,
            Score::MAX,
            0,
        );

        let score_str = match score.kind() {
            ScoreKind::Centipoint(v) => format!("cp {}", v),
            ScoreKind::MateIn(v) => format!("mate {}", (v + 1) / 2),
            ScoreKind::MatedIn(v) => format!("mate -{}", (v + 1) / 2),
        };

        if STOP.load(Ordering::Relaxed) {
            break;
        }

        let elapsed = data.elapsed();

        println!(
            "info depth {} seldepth {} score {} nodes {} nps {} time {} pv {}",
            data.iter_depth(),
            data.seldepth(),
            score_str,
            data.node_count(),
            (data.node_count() as f64 / elapsed.as_secs_f64()).round() as u64,
            elapsed.as_millis(),
            data.bestmove(),
        );
        io::stdout().flush()?;
    }

    println!("bestmove {}", data.bestmove());
    io::stdout().flush()
}

pub fn search(
    board: &mut Board,
    data: &mut SearchData,
    params: &Params,
    depth: u16,
    mut alpha: Score,
    beta: Score,
    ply: u16,
) -> Score {
    data.check_time(params);

    if STOP.load(Ordering::Relaxed) {
        return Score::cp(0);
    }

    data.set_seldepth(ply);

    if depth == 0 {
        return Score::cp(0);
    }

    let mut movegen = Movegen::new();

    if depth >= 5 {
        movegen.generate_far(board);
    } else {
        movegen.generate_near(board);
    }

    let mut bestscore = Score::MIN;

    for mv in movegen {
        board.push(mv);
        data.inc_node_count();

        if eval::is_victory(board, mv) {
            board.pop(mv);
            return Score::mate_in(ply as u8 + 1);
        }

        let score = -search(board, data, params, depth - 1, -beta, -alpha, ply + 1);

        board.pop(mv);

        if STOP.load(Ordering::Relaxed) {
            return Score::cp(0);
        }

        if score > bestscore {
            bestscore = score;

            if score > alpha {
                alpha = score;

                if ply == 0 {
                    data.set_bestmove(mv);
                }

                if score > beta {
                    break;
                }
            }
        }
    }

    bestscore
}

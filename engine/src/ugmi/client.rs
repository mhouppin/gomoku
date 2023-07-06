use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

use crate::core::{board::Board, types::Square};
use crate::engine::search::{self, Params};

pub struct Client {}

lazy_static! {
    pub static ref DEBUG: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> io::Result<()> {
        let mut board = Board::new();
        let stdin = io::stdin();
        let mut buffer = String::new();

        while stdin.read_line(&mut buffer)? != 0 {
            let mut tokens = buffer.split_ascii_whitespace();
            let command = tokens.next().unwrap_or("");

            match command {
                "ugmi" => self.display_ugmi()?,
                "d" => println!("{}", board),
                "debug" => self.select_debug(tokens)?,
                "isready" => self.display_readyok()?,
                "setoption" => (),
                "ugminewgame" => (),
                "position" => self.set_position(&mut board, tokens)?,
                "go" => self.new_search(tokens, &board)?,
                "stop" => self.stop_search(),
                "quit" => break,
                _ => println!("info string Error: unknown command '{}'", command),
            }

            buffer.clear();
        }

        Ok(())
    }

    fn display_ugmi(&self) -> io::Result<()> {
        println!("id name Kinko v{}", env!("CARGO_PKG_VERSION"));
        println!("id author Morgan Houppin");
        println!("ugmiok");
        io::stdout().flush()
    }

    fn display_readyok(&self) -> io::Result<()> {
        println!("readyok");
        io::stdout().flush()
    }

    fn select_debug<'a, I>(&self, mut tokens: I) -> io::Result<()>
    where
        I: Iterator<Item = &'a str>,
    {
        let debug_mode = tokens.next().unwrap_or("");

        match debug_mode {
            "on" => DEBUG.store(true, Ordering::Relaxed),
            "off" => DEBUG.store(false, Ordering::Relaxed),
            _ => {
                println!("info string Error: Unknown debug mode '{}'", debug_mode);
                io::stdout().flush()?;
            }
        }

        Ok(())
    }

    fn set_position<'a, I>(&self, board: &mut Board, mut tokens: I) -> io::Result<()>
    where
        I: Iterator<Item = &'a str>,
    {
        match tokens.next() {
            Some("startpos") => board.reset(),
            Some("board") => {
                board.setup_position(
                    tokens
                        .next()
                        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?,
                    tokens
                        .next()
                        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?,
                );
            }
            _ => (),
        }

        let _ = tokens.next();

        for sq in tokens {
            board.push(
                sq.parse()
                    .map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))?,
            )
        }

        Ok(())
    }

    fn parse_millis(&self, token: Option<&str>) -> Option<Duration> {
        if let Some(token_str) = token {
            token_str.parse::<u64>().ok().map(Duration::from_millis)
        } else {
            None
        }
    }

    fn new_search<'a, I>(&self, tokens: I, board: &Board) -> io::Result<()>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut params = Params::new();

        let tokens = tokens.collect::<Vec<_>>();
        let mut i = 0;

        while i < tokens.len() {
            match tokens[i] {
                "wtime" => {
                    if let Some(value) = self.parse_millis(tokens.get(i + 1).cloned()) {
                        params.set_wtime(value);
                        i += 1;
                    }
                }
                "btime" => {
                    if let Some(value) = self.parse_millis(tokens.get(i + 1).cloned()) {
                        params.set_btime(value);
                        i += 1;
                    }
                }
                "winc" => {
                    if let Some(value) = self.parse_millis(tokens.get(i + 1).cloned()) {
                        params.set_winc(value);
                        i += 1;
                    }
                }
                "binc" => {
                    if let Some(value) = self.parse_millis(tokens.get(i + 1).cloned()) {
                        params.set_binc(value);
                        i += 1;
                    }
                }
                "movetime" => {
                    if let Some(value) = self.parse_millis(tokens.get(i + 1).cloned()) {
                        params.set_movetime(value);
                        i += 1;
                    }
                }
                "depth" => {
                    if let Ok(value) = tokens.get(i + 1).cloned().unwrap_or("").parse::<u16>() {
                        params.set_depth(value);
                        i += 1;
                    }
                }
                "nodes" => {
                    if let Ok(value) = tokens.get(i + 1).cloned().unwrap_or("").parse::<u64>() {
                        params.set_nodes(value);
                        i += 1;
                    }
                }
                "mate" => {
                    if let Ok(value) = tokens.get(i + 1).cloned().unwrap_or("").parse::<u16>() {
                        params.set_mate(value);
                        i += 1;
                    }
                }
                "searchmoves" => {
                    let mut moves = Vec::new();
                    while i < tokens.len() {
                        if let Ok(value) = tokens[i].parse::<Square>() {
                            moves.push(value);
                        }
                        i += 1;
                    }
                    params.set_searchmoves(moves.as_slice());
                    break;
                }
                _ => break,
            }

            i += 1;
        }

        search::run_search(board, &params)
    }

    fn stop_search(&self) {}
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

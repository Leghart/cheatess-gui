pub mod http;
pub mod ws;
use cheatess_core::core::stockfish::Stockfish;
use std::sync::Arc;

use crate::wrappers::args;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
pub const ENGINE_PATH: &str = "/home/leghart/projects/cheatess/stockfish-ubuntu-x86-64-avx2";

#[derive(Clone)]
pub struct AppState {
    pub stockfish: Arc<Mutex<Option<Stockfish>>>,
    pub config: Arc<Mutex<args::CheatessArgsDto>>,
}

#[derive(Serialize, Deserialize)]
pub struct BoardGrid {
    data: [[String; 8]; 8],
}

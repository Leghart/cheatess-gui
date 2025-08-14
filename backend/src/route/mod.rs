pub mod http;
pub mod ws;
use cheatess_core::{core::stockfish::Stockfish, engine::AnyBoard};
use std::sync::Arc;

use crate::wrappers::args;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
pub const ENGINE_PATH: &str = "/home/leghart/projects/cheatess/stockfish-ubuntu-x86-64-avx2";

#[derive(Clone)]
pub struct AppState {
    pub stockfish: Arc<Mutex<Option<Stockfish>>>,
    pub config: Arc<Mutex<args::CheatessArgsDto>>,
    pub int_config: Arc<Mutex<IntConfig>>,
}

#[derive(Serialize, Deserialize)]
pub struct IntConfig {
    coords: Option<(u32, u32, u32, u32)>,
    #[serde(skip)]
    current_board: Option<Box<dyn AnyBoard + Send + Sync>>,
}

impl IntConfig {
    pub fn new() -> Self {
        IntConfig {
            coords: None,
            current_board: None,
        }
    }
}

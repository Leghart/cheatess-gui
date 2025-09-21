pub mod http;
pub mod ws;
use cheatess_core::{core::engine::Color, core::stockfish::Stockfish, procimg::Mat};

use std::collections::HashMap;
use std::sync::Arc;

use crate::wrappers::{
    args::CheatessArgsDto,
    func::{FuncWrapper, StockfishLike},
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub type ExtConfig = CheatessArgsDto;

#[derive(Clone)]
pub struct AppState {
    pub stockfish: Arc<Mutex<Option<Box<dyn StockfishLike>>>>,
    pub ext_config: Arc<Mutex<ExtConfig>>,
    pub int_config: Arc<Mutex<IntConfig>>,
    pub funcs: Arc<dyn FuncWrapper + Send + Sync>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct IntConfig {
    coords: Option<(u32, u32, u32, u32)>,
    prev_board: Option<[[char; 8]; 8]>,
    color: Option<Color>,
    #[serde(skip)]
    prev_board_mat: Option<Mat>,
    #[serde(skip)]
    pieces: Option<HashMap<char, Arc<Mat>>>,
}

impl IntConfig {
    pub fn new() -> Self {
        IntConfig {
            coords: None,
            prev_board: None,
            color: None,
            prev_board_mat: None,
            pieces: None,
        }
    }
}

// Set None to fields with Mat (it won't be serialized anyway)
impl Clone for IntConfig {
    fn clone(&self) -> Self {
        IntConfig {
            coords: self.coords,
            prev_board: self.prev_board,
            color: self.color,
            prev_board_mat: None,
            pieces: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StockfishSummary {
    main_line: Vec<String>,
    evaluation: String,
}

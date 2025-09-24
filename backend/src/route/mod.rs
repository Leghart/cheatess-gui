pub mod http;
pub mod ws;

use cheatess_core::{core::engine::Color, procimg::Mat};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::interfaces::{FuncWrapper, StockfishLike};
use crate::wrappers::args::CheatessArgsDto;

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
    pub coords: Option<(u32, u32, u32, u32)>,
    pub prev_board: Option<[[char; 8]; 8]>,
    pub color: Option<Color>,
    #[serde(skip)]
    pub prev_board_mat: Option<Mat>,
    #[serde(skip)]
    pub pieces: Option<HashMap<char, Arc<Mat>>>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct StockfishSummary {
    main_line: Vec<String>,
    evaluation: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StockfishResponse {
    pub version: String,
    pub summary: Option<Vec<StockfishSummary>>,
}

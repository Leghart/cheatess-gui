pub mod http;
pub mod ws;
use cheatess_core::{core::stockfish::Stockfish, procimg::Mat};
use std::sync::Arc;

use crate::wrappers::{args, enums::ColorDto};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub stockfish: Arc<Mutex<Option<Stockfish>>>,
    pub ext_config: Arc<Mutex<args::CheatessArgsDto>>,
    pub int_config: Arc<Mutex<IntConfig>>,
}

#[derive(Serialize, Deserialize)]
pub struct IntConfig {
    coords: Option<(u32, u32, u32, u32)>,
    prev_board: Option<[[char; 8]; 8]>,
    color: Option<ColorDto>,
    #[serde(skip)]
    prev_board_mat: Option<Mat>,
    #[serde(skip)]
    pieces: Option<std::collections::HashMap<char, Arc<Mat>>>,
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

use cheatess_core::engine::Color;
use cheatess_core::procimg::Mat;
use cheatess_core::stockfish::Summary;
use cheatess_core::utils::error::CheatessResult;
use cheatess_core::utils::monitor::Monitor;

use async_trait::async_trait;
use axum::extract::State;
use std::collections::HashMap;

use crate::AppState;
use crate::route::StockfishResponse;

pub trait StockfishLike: Send + Sync {
    fn get_version(&self) -> String;

    fn set_config(
        &mut self,
        elo: &str,
        skill: &str,
        hash: &str,
        multi_lines: &str,
    ) -> CheatessResult<()>;

    fn get_summary(&mut self, search_lines: usize) -> CheatessResult<Vec<Summary>>;

    fn make_move(&mut self, moves: Vec<String>) -> CheatessResult<()>;
}

#[async_trait]
pub trait FuncWrapper {
    async fn capture_screen_as_mat(&self, monitor_name: Option<String>) -> CheatessResult<Mat>;
    async fn crop_board(&self, screen: &Mat, coords: (u32, u32, u32, u32)) -> CheatessResult<Mat>;
    async fn get_coords(&self, board: &Mat) -> CheatessResult<(u32, u32, u32, u32)>;
    async fn get_monitor(&self, monitor_name: Option<String>) -> CheatessResult<Monitor>;
    async fn get_pieces(
        &self,
        board: &Mat,
        margin: u8,
        threshold: f64,
        color: &Color,
    ) -> CheatessResult<HashMap<char, Mat>>;

    async fn init_stockfish(
        &self,
        State(state): State<AppState>,
    ) -> CheatessResult<StockfishResponse>;

    fn detect_player_color(&self, gray_board: &Mat) -> CheatessResult<Color>;
    fn _create_stockfish(
        &self,
        path: &std::path::Path,
        depth: u8,
    ) -> Option<Box<dyn StockfishLike>>;
}

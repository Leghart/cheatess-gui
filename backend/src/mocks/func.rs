use cheatess_core::engine::Color;
use cheatess_core::procimg::Mat;
use cheatess_core::utils::error::CheatessError;
use cheatess_core::utils::error::CheatessResult;
use cheatess_core::utils::monitor::Monitor;

use async_trait::async_trait;
use axum::extract::State;
use std::collections::HashMap;

use crate::AppState;
use crate::interfaces::{FuncWrapper, StockfishLike};
use crate::mocks::stockfish::MockStockfish;
use crate::route::StockfishResponse;

#[derive(Default)]
pub struct MockFunc {
    pub mat: Option<Mat>,
    pub crop_mat: Option<Mat>,
    pub monitor: Option<Monitor>,
    pub map: Option<HashMap<char, Mat>>,
    pub coords: Option<(u32, u32, u32, u32)>,
    pub stockfish_ptr: Option<Box<dyn StockfishLike>>,
    pub init_stockfish_ok: bool,
    pub detect_color_ok: bool,
}

#[allow(dead_code)]
impl MockFunc {
    fn default() -> MockFunc {
        MockFunc {
            mat: None,
            crop_mat: None,
            monitor: None,
            map: None,
            coords: None,
            stockfish_ptr: None,
            init_stockfish_ok: true,
            detect_color_ok: true,
        }
    }
}

#[async_trait]
impl FuncWrapper for MockFunc {
    async fn capture_screen_as_mat(&self, _: Option<String>) -> CheatessResult<Mat> {
        match &self.mat {
            Some(m) => Ok(m.to_owned()),
            None => Err(CheatessError::MonitorNotFound), // TODO: temporary mapped to another error
        }
    }

    async fn crop_board(&self, _: &Mat, _: (u32, u32, u32, u32)) -> CheatessResult<Mat> {
        match &self.crop_mat {
            Some(m) => Ok(m.to_owned()),
            None => Err(CheatessError::MonitorNotFound), // TODO: temporary mapped to another error
        }
    }

    async fn get_monitor(&self, _: Option<String>) -> CheatessResult<Monitor> {
        match &self.monitor {
            Some(m) => Ok(m.to_owned()),
            None => Err(CheatessError::MonitorNotFound), // TODO: temporary mapped to another error
        }
    }

    async fn get_pieces(
        &self,
        _: &Mat,
        _: u8,
        _: f64,
        _: &Color,
    ) -> CheatessResult<HashMap<char, Mat>> {
        match &self.map {
            Some(m) => Ok(m.to_owned()),
            None => Err(CheatessError::MonitorNotFound), // TODO: temporary mapped to another error
        }
    }

    async fn get_coords(&self, _: &Mat) -> CheatessResult<(u32, u32, u32, u32)> {
        match &self.coords {
            Some(m) => Ok(m.to_owned()),
            None => Err(CheatessError::MonitorNotFound), // TODO: temporary mapped to another error
        }
    }

    async fn init_stockfish(&self, State(_): State<AppState>) -> CheatessResult<StockfishResponse> {
        match self.init_stockfish_ok {
            true => Ok(StockfishResponse {
                version: "1".to_string(),
                summary: None,
            }),
            false => Err(CheatessError::MonitorNotFound), //TODO
        }
    }

    fn detect_player_color(&self, _: &Mat) -> CheatessResult<Color> {
        match self.detect_color_ok {
            true => Ok(Color::White),
            false => Err(CheatessError::MonitorNotFound), // TODO: temporary mapped to another error
        }
    }

    fn _create_stockfish(
        &self,
        _path: &std::path::Path,
        _depth: u8,
    ) -> Option<Box<dyn StockfishLike>> {
        self.stockfish_ptr
            .as_ref()
            .map(|_| Box::new(MockStockfish::default()) as Box<dyn StockfishLike>)
    }
}

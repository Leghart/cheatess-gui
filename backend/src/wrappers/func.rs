use cheatess_core::engine::Color;
use cheatess_core::procimg::{
    Mat, crop_mat, extract_pieces, get_board_region, image_buffer_to_gray_mat,
};
use cheatess_core::stockfish::{Stockfish, Summary};
use cheatess_core::utils::error::CheatessResult;
use cheatess_core::utils::monitor::{self, Monitor};

use async_trait::async_trait;
use std::collections::HashMap;

#[cfg(test)]
use cheatess_core::utils::error::CheatessError;

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
    fn init_stockfish(
        &self,
        path: &std::path::PathBuf,
        depth: u8,
    ) -> Option<Box<dyn StockfishLike>>;
}

pub struct ProdFunc;

unsafe impl Send for ProdFunc {}
unsafe impl Sync for ProdFunc {}

#[async_trait]
impl FuncWrapper for ProdFunc {
    async fn capture_screen_as_mat(&self, monitor_name: Option<String>) -> CheatessResult<Mat> {
        let monitor = monitor::select_monitor(monitor_name)?;
        let screen = monitor::capture_entire_screen(&monitor)?;
        image_buffer_to_gray_mat(screen)
    }

    async fn crop_board(&self, screen: &Mat, coords: (u32, u32, u32, u32)) -> CheatessResult<Mat> {
        crop_mat(screen, &coords)
    }

    async fn get_monitor(&self, monitor_name: Option<String>) -> CheatessResult<Monitor> {
        monitor::select_monitor(monitor_name)
    }

    async fn get_pieces(
        &self,
        board: &Mat,
        margin: u8,
        threshold: f64,
        color: &Color,
    ) -> CheatessResult<std::collections::HashMap<char, Mat>> {
        extract_pieces(board, margin, threshold, color)
    }

    async fn get_coords(&self, board: &Mat) -> CheatessResult<(u32, u32, u32, u32)> {
        get_board_region(board)
    }

    fn init_stockfish(
        &self,
        path: &std::path::PathBuf,
        depth: u8,
    ) -> Option<Box<dyn StockfishLike>> {
        let sf = Stockfish::new(path, depth);
        Some(Box::new(ProdStockfish { inner: sf }))
    }
}

pub struct ProdStockfish {
    inner: Stockfish,
}

unsafe impl Send for ProdStockfish {}
unsafe impl Sync for ProdStockfish {}

impl StockfishLike for ProdStockfish {
    fn set_config(
        &mut self,
        elo: &str,
        skill: &str,
        hash: &str,
        multi_lines: &str,
    ) -> CheatessResult<()> {
        self.inner.set_config(elo, skill, hash, multi_lines)
    }

    fn get_summary(&mut self, search_lines: usize) -> CheatessResult<Vec<Summary>> {
        self.inner.summary(search_lines)
    }

    fn get_version(&self) -> String {
        self.inner.version.clone()
    }

    fn make_move(&mut self, moves: Vec<String>) -> CheatessResult<()> {
        self.inner.make_move(moves)
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct MockFunc {
    pub mat: Option<Mat>,
    pub crop_mat: Option<Mat>,
    pub monitor: Option<Monitor>,
    pub map: Option<HashMap<char, Mat>>,
    pub coords: Option<(u32, u32, u32, u32)>,
    pub stockfish_ptr: Option<Box<dyn StockfishLike>>,
}

#[cfg(test)]
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
        }
    }
}

#[cfg(test)]
pub struct MockStockfish {
    pub version: String,
    pub summary: Option<Vec<Summary>>,
}

#[cfg(test)]
impl StockfishLike for MockStockfish {
    fn get_summary(&mut self, _: usize) -> CheatessResult<Vec<Summary>> {
        match self.summary.take() {
            Some(s) => Ok(s),
            None => unreachable!("temporary"),
        }
    }

    fn get_version(&self) -> String {
        self.version.clone()
    }

    fn make_move(&mut self, _: Vec<String>) -> CheatessResult<()> {
        // TODO: temporary
        CheatessResult::Ok(())
    }

    fn set_config(&mut self, _: &str, _: &str, _: &str, _: &str) -> CheatessResult<()> {
        // TODO: temporary
        CheatessResult::Ok(())
    }
}

#[cfg(test)]
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

    fn init_stockfish(
        &self,
        _path: &std::path::PathBuf,
        _depth: u8,
    ) -> Option<Box<dyn StockfishLike>> {
        self.stockfish_ptr.as_ref().map(|_| {
            Box::new(MockStockfish {
                version: "mock".to_string(),
                summary: None,
            }) as Box<dyn StockfishLike>
        })
    }
}

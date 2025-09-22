use cheatess_core::engine::Color;
use cheatess_core::procimg::{
    Mat, crop_mat, extract_pieces, get_board_region, image_buffer_to_gray_mat,
};
use cheatess_core::stockfish::{Stockfish, Summary};
use cheatess_core::utils::error::CheatessResult;
use cheatess_core::utils::monitor::{self, Monitor};

use crate::AppState;
use crate::route::StockfishSummary;

use axum::extract::State;
use serde::{Deserialize, Serialize};

use async_trait::async_trait;
use std::collections::HashMap;

#[cfg(test)]
use cheatess_core::utils::error::CheatessError;

#[derive(Deserialize, Serialize)]
pub struct StockfishResponse {
    pub version: String,
    pub summary: Option<Vec<StockfishSummary>>,
}

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

    fn _create_stockfish(
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

    async fn init_stockfish(
        &self,
        State(state): State<AppState>,
    ) -> CheatessResult<StockfishResponse> {
        let mut sf_guard = state.stockfish.lock().await;
        let ext_config_guard = state.ext_config.lock().await;

        let version: String;
        if sf_guard.is_none() {
            let path = std::path::PathBuf::from(std::env::var("ENGINE_PATH").unwrap());
            let depth = 5;

            let sf = state.funcs._create_stockfish(&path, depth).unwrap();
            version = sf.get_version();
            *sf_guard = Some(sf);
        } else {
            version = sf_guard.as_ref().unwrap().get_version();
        }

        let elo = ext_config_guard.stockfish.as_ref().unwrap().elo.unwrap();
        let skill = ext_config_guard.stockfish.as_ref().unwrap().skill.unwrap();
        let hash = ext_config_guard.stockfish.as_ref().unwrap().hash.unwrap();
        let multi_lines = ext_config_guard.stockfish.as_ref().unwrap().pv.unwrap();

        sf_guard.as_mut().unwrap().set_config(
            &elo.to_string(),
            &skill.to_string(),
            &hash.to_string(),
            &multi_lines.to_string(),
        )?;

        return Ok(StockfishResponse {
            version,
            summary: None,
        });
    }

    fn _create_stockfish(
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
    pub init_stockfish_ok: bool,
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
            init_stockfish_ok: true,
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

    async fn init_stockfish(&self, State(_): State<AppState>) -> CheatessResult<StockfishResponse> {
        match self.init_stockfish_ok {
            true => Ok(StockfishResponse {
                version: "1".to_string(),
                summary: None,
            }),
            false => Err(CheatessError::MonitorNotFound), //TODO
        }
    }

    fn _create_stockfish(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::route::{AppState, IntConfig};
    use crate::wrappers::args::CheatessArgsDto;
    use crate::wrappers::{
        args::{ImgProcArgsDto, MonitorArgsDto, StockfishArgsDto},
        func::{MockFunc, MockStockfish},
    };
    use axum::extract::State;

    use cheatess_core::procimg::Mat;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn init_stockfish_create_first_time() {
        unsafe {
            std::env::set_var("ENGINE_PATH", "./engine_path");
        }

        let funcs = MockFunc {
            mat: Some(Mat::default()),
            coords: Some((1, 1, 1, 1)),
            stockfish_ptr: Some(Box::new(MockStockfish {
                version: "0.0".to_string(),
                summary: None,
            })),
            ..Default::default()
        };

        let state = AppState {
            int_config: Arc::new(Mutex::new(IntConfig {
                ..Default::default()
            })),
            ext_config: Arc::new(Mutex::new(CheatessArgsDto {
                monitor: Some(MonitorArgsDto {
                    name: Some("abc".to_string()),
                }),
                stockfish: Some(StockfishArgsDto {
                    pv: Some(3),
                    elo: Some(10),
                    skill: Some(5),
                    hash: Some(1),
                    ..Default::default()
                }),
                proc_image: Some(ImgProcArgsDto {
                    ..Default::default()
                }),
                ..Default::default()
            })),
            stockfish: Arc::new(Mutex::new(Default::default())),
            funcs: Arc::new(funcs),
        };

        assert!(state.stockfish.lock().await.is_none());
        let result = ProdFunc.init_stockfish(State(state.clone())).await;

        assert!(result.is_ok());
        assert!(state.stockfish.lock().await.is_some())
    }

    #[tokio::test]
    async fn init_stockfish_take_from_state() {
        unsafe {
            std::env::set_var("ENGINE_PATH", "./engine_path");
        }

        let funcs = MockFunc {
            mat: Some(Mat::default()),
            coords: Some((1, 1, 1, 1)),
            ..Default::default()
        };

        let state = AppState {
            int_config: Arc::new(Mutex::new(IntConfig {
                ..Default::default()
            })),
            ext_config: Arc::new(Mutex::new(CheatessArgsDto {
                monitor: Some(MonitorArgsDto {
                    name: Some("abc".to_string()),
                }),
                stockfish: Some(StockfishArgsDto {
                    pv: Some(3),
                    elo: Some(10),
                    skill: Some(5),
                    hash: Some(1),
                    ..Default::default()
                }),
                proc_image: Some(ImgProcArgsDto {
                    ..Default::default()
                }),
                ..Default::default()
            })),
            stockfish: Arc::new(Mutex::new(Some(Box::new(MockStockfish {
                version: "1".to_string(),
                summary: None,
            })))),
            funcs: Arc::new(funcs),
        };

        assert!(state.stockfish.lock().await.is_some());
        let before = state
            .stockfish
            .lock()
            .await
            .as_ref()
            .map(|b| &**b as *const _);

        let result = ProdFunc.init_stockfish(State(state.clone())).await;
        let after = state
            .stockfish
            .lock()
            .await
            .as_ref()
            .map(|b| &**b as *const _);

        assert!(result.is_ok());
        assert!(state.stockfish.lock().await.is_some());
        assert_eq!(before, after);
    }

    #[tokio::test]
    async fn init_stockfish_set_config_failed() {
        //TODO
    }
}

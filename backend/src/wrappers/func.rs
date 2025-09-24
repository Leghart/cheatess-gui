use cheatess_core::engine::Color;
use cheatess_core::procimg::{
    Mat, crop_mat, detect_player_color, extract_pieces, get_board_region, image_buffer_to_gray_mat,
};
use cheatess_core::stockfish::Stockfish;
use cheatess_core::utils::error::CheatessResult;
use cheatess_core::utils::monitor::{self, Monitor};

use super::stockfish::ProdStockfish;
use crate::AppState;
use crate::interfaces::{FuncWrapper, StockfishLike};
use crate::route::StockfishResponse;

use async_trait::async_trait;
use axum::extract::State;

pub struct ProdFunc;

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

        // TODO: add GeneralError in core to change these unwraps to return Err...
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

    fn detect_player_color(&self, gray_board: &Mat) -> CheatessResult<Color> {
        detect_player_color(gray_board)
    }

    fn _create_stockfish(
        &self,
        path: &std::path::Path,
        depth: u8,
    ) -> Option<Box<dyn StockfishLike>> {
        let sf = Stockfish::new(&std::path::PathBuf::from(path), depth);
        Some(Box::new(ProdStockfish { inner: sf }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::{func::MockFunc, stockfish::MockStockfish};
    use crate::route::{AppState, IntConfig};
    use crate::wrappers::args::{
        CheatessArgsDto, ImgProcArgsDto, MonitorArgsDto, StockfishArgsDto,
    };
    use axum::extract::State;
    use cheatess_core::utils::error::CheatessError;

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
            stockfish_ptr: Some(Box::new(MockStockfish::default())),
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
            stockfish: Arc::new(Mutex::new(Some(Box::new(MockStockfish::default())))),
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
                set_config_failed: true,
                ..Default::default()
            })))),
            funcs: Arc::new(funcs),
        };

        let result = ProdFunc.init_stockfish(State(state.clone())).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CheatessError::NoMoveDetected)); // TODO: add PartialEq to CheatessError
    }
}

use cheatess_core::engine::Color;
use cheatess_core::procimg::{
    Mat, crop_mat, extract_pieces, get_board_region, image_buffer_to_gray_mat,
};
use cheatess_core::utils::error::CheatessResult;
use cheatess_core::utils::monitor::{self, Monitor};

use async_trait::async_trait;
use std::collections::HashMap;

#[cfg(test)]
use cheatess_core::utils::error::CheatessError;

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
}

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
}

#[cfg(test)]
#[derive(Default)]
pub struct MockFunc {
    pub mat: Option<Mat>,
    pub monitor: Option<Monitor>,
    pub map: Option<HashMap<char, Mat>>,
    pub coords: Option<(u32, u32, u32, u32)>,
}

#[cfg(test)]
#[allow(dead_code)]
impl MockFunc {
    fn default() -> MockFunc {
        MockFunc {
            mat: None,
            monitor: None,
            map: None,
            coords: None,
        }
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
        match &self.mat {
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
}

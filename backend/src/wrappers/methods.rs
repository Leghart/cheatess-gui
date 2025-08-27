use cheatess_core::engine::Color;
use cheatess_core::procimg::{
    Mat, crop_mat, extract_pieces, get_board_region, image_buffer_to_gray_mat,
};
use cheatess_core::utils::error::CheatessResult;
use cheatess_core::utils::monitor::{self, Monitor};

pub async fn capture_screen_as_mat(monitor_name: Option<String>) -> CheatessResult<Mat> {
    let monitor = monitor::select_monitor(monitor_name)?;
    let screen = monitor::capture_entire_screen(&monitor)?;
    image_buffer_to_gray_mat(screen)
}

pub async fn crop_board(screen: &Mat, coords: (u32, u32, u32, u32)) -> CheatessResult<Mat> {
    crop_mat(screen, &coords)
}

pub async fn get_monitor(monitor_name: Option<String>) -> CheatessResult<Monitor> {
    monitor::select_monitor(monitor_name)
}

pub async fn get_pieces(
    board: &Mat,
    margin: u8,
    threshold: f64,
    color: &Color,
) -> CheatessResult<std::collections::HashMap<char, Mat>> {
    extract_pieces(board, margin, threshold, color)
}

pub async fn get_coords(board: &Mat) -> CheatessResult<(u32, u32, u32, u32)> {
    get_board_region(board)
}

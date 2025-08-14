use cheatess_core::engine::Color;
use cheatess_core::procimg::{
    Mat, crop_mat, extract_pieces, get_board_region, image_buffer_to_gray_mat,
};
use cheatess_core::utils::monitor;

pub async fn capture_screen_as_mat(monitor_number: u8) -> Mat {
    let monitor = monitor::select_monitor(monitor_number).expect("Requested monitor not found");
    let screen = monitor::capture_entire_screen(&monitor);
    image_buffer_to_gray_mat(screen).expect("ImageBuffer couldn't be transformed to Mat.")
}

pub async fn crop_board(screen: &Mat, coords: (u32, u32, u32, u32)) -> Mat {
    crop_mat(screen, &coords)
}

pub async fn get_pieces(
    board: &Mat,
    margin: u8,
    threshold: f64,
    color: Color,
) -> std::collections::HashMap<char, Mat> {
    extract_pieces(board, margin, threshold, &color).unwrap()
}

pub async fn get_coords(board: &Mat) -> (u32, u32, u32, u32) {
    get_board_region(board)
}

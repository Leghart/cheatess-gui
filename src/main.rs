use cheatess_core::core::engine::{Color, DefaultPrinter, create_board_default};
use std::io;

fn main() {
    let board = create_board_default::<DefaultPrinter>(&Color::Black);
    board.print(&mut io::stdout());
}

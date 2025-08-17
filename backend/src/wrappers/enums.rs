use cheatess_core::engine::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ColorDto {
    White,
    Black,
}

impl From<Color> for ColorDto {
    fn from(color: Color) -> Self {
        match color {
            Color::White => ColorDto::White,
            Color::Black => ColorDto::Black,
        }
    }
}

impl From<ColorDto> for Color {
    fn from(color: ColorDto) -> Self {
        match color {
            ColorDto::White => Color::White,
            ColorDto::Black => Color::Black,
        }
    }
}

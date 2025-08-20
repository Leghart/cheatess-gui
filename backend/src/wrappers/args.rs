use cheatess_core::utils::parser::CheatessArgs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CheatessArgsDto {
    pub verbose: Option<String>,
    pub mode: Option<String>,
    pub monitor: Option<MonitorArgsDto>,
    pub stockfish: Option<StockfishArgsDto>,
    pub proc_image: Option<ImgProcArgsDto>,
    pub engine: Option<EngineArgsDto>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MonitorArgsDto {
    pub number: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct StockfishArgsDto {
    pub path: Option<String>,
    pub elo: Option<usize>,
    pub skill: Option<u8>,
    pub depth: Option<u8>,
    pub hash: Option<usize>,
    pub pv: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ImgProcArgsDto {
    pub margin: Option<u8>,
    pub piece_threshold: Option<f64>,
    pub extract_piece_threshold: Option<f64>,
    pub board_threshold: Option<f64>,
    pub difference_level: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EngineArgsDto {
    pub pretty: Option<bool>,
}

impl From<&CheatessArgs> for CheatessArgsDto {
    fn from(c: &CheatessArgs) -> Self {
        Self {
            verbose: Some(format!("{:?}", c.verbose)),
            mode: Some(c.mode.to_string()),
            monitor: Some(MonitorArgsDto {
                number: Some(c.monitor.number),
            }),
            stockfish: Some(StockfishArgsDto {
                path: Some(c.stockfish.path.display().to_string()),
                elo: Some(c.stockfish.elo),
                skill: Some(c.stockfish.skill),
                depth: Some(c.stockfish.depth),
                hash: Some(c.stockfish.hash),
                pv: Some(c.stockfish.pv),
            }),
            proc_image: Some(ImgProcArgsDto {
                margin: Some(c.proc_image.margin),
                piece_threshold: Some(c.proc_image.piece_threshold),
                extract_piece_threshold: Some(c.proc_image.extract_piece_threshold),
                board_threshold: Some(c.proc_image.board_threshold),
                difference_level: Some(c.proc_image.difference_level),
            }),
            engine: Some(EngineArgsDto {
                pretty: Some(c.engine.pretty),
            }),
        }
    }
}

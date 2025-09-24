use cheatess_core::stockfish::Summary;
use cheatess_core::utils::error::{CheatessError, CheatessResult};

use crate::interfaces::StockfishLike;

#[derive(Default)]
pub struct MockStockfish {
    pub version: String,
    pub summary: Option<Vec<Summary>>,

    pub set_config_failed: bool,
    pub make_move_failed: bool,
}

#[allow(dead_code)]
impl MockStockfish {
    fn default() -> MockStockfish {
        MockStockfish {
            version: "mock".to_string(),
            summary: None,
            set_config_failed: false,
            make_move_failed: false,
        }
    }
}

impl StockfishLike for MockStockfish {
    fn get_summary(&mut self, _: usize) -> CheatessResult<Vec<Summary>> {
        match self.summary.take() {
            Some(s) => Ok(s),
            None => Err(CheatessError::NoMoveDetected), // TODO: temporary
        }
    }

    fn get_version(&self) -> String {
        self.version.clone()
    }

    fn make_move(&mut self, _: Vec<String>) -> CheatessResult<()> {
        match self.make_move_failed {
            true => Err(CheatessError::NoMoveDetected), // TODO: temporary
            false => Ok(()),
        }
    }

    fn set_config(&mut self, _: &str, _: &str, _: &str, _: &str) -> CheatessResult<()> {
        match self.set_config_failed {
            true => Err(CheatessError::NoMoveDetected), // TODO: temporary
            false => Ok(()),
        }
    }
}

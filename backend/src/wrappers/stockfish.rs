use cheatess_core::stockfish::{Stockfish, Summary};
use cheatess_core::utils::error::CheatessResult;

use crate::interfaces::StockfishLike;

pub struct ProdStockfish {
    pub inner: Stockfish,
}

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

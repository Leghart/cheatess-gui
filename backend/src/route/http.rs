use crate::route::{AppState, IntConfig};
use crate::wrappers::{self, args::CheatessArgsDto};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch, post},
};
use cheatess_core::core::{
    engine::{DefaultPrinter, create_board_default},
    procimg,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::sync::Arc;

use super::StockfishSummary;

#[derive(Deserialize, Serialize)]
pub struct RawBoardResponse {
    pub raw_data: [[char; 8]; 8],
}

#[derive(Deserialize, Serialize)]
pub struct StockfishResponse {
    pub version: String,
    pub summary: Option<Vec<StockfishSummary>>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/init", post(init))
        .route("/int_config", get(get_int_config))
        .route("/ext_config", get(get_ext_config))
        .route("/ext_config", patch(update_ext_config))
        .route("/board", get(get_prev_board))
}

async fn get_int_config(State(state): State<AppState>) -> (StatusCode, Json<IntConfig>) {
    (StatusCode::OK, Json(state.int_config.lock().await.clone()))
}

async fn get_ext_config(State(state): State<AppState>) -> (StatusCode, Json<CheatessArgsDto>) {
    (StatusCode::OK, Json(state.ext_config.lock().await.clone()))
}

async fn get_prev_board(State(state): State<AppState>) -> Response {
    let int_config_guard = state.int_config.lock().await;

    match int_config_guard.prev_board {
        Some(raw_data) => (StatusCode::OK, Json(RawBoardResponse { raw_data })).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn update_ext_config(
    State(state): State<AppState>,
    Json(partial): Json<wrappers::args::CheatessArgsDto>,
) -> (StatusCode, Json<CheatessArgsDto>) {
    let mut ext_config = state.ext_config.lock().await;

    if let Some(verbose) = partial.verbose {
        ext_config.verbose = Some(verbose);
    }
    if let Some(mode) = partial.mode {
        ext_config.mode = Some(mode);
    }
    if let Some(name) = partial.monitor.and_then(|m| m.name) {
        ext_config.monitor.get_or_insert(Default::default()).name = Some(name);
    }

    if let Some(stockfish) = partial.stockfish {
        let s = ext_config.stockfish.get_or_insert(Default::default());
        if let Some(path) = stockfish.path {
            s.path = Some(path);
        }
        if let Some(elo) = stockfish.elo {
            s.elo = Some(elo);
        }
        if let Some(skill) = stockfish.skill {
            s.skill = Some(skill);
        }
        if let Some(depth) = stockfish.depth {
            s.depth = Some(depth);
        }
        if let Some(hash) = stockfish.hash {
            s.hash = Some(hash);
        }
    }
    if let Some(proc_image) = partial.proc_image {
        let p = ext_config.proc_image.get_or_insert(Default::default());
        if let Some(margin) = proc_image.margin {
            p.margin = Some(margin);
        }
        if let Some(piece_threshold) = proc_image.piece_threshold {
            p.piece_threshold = Some(piece_threshold);
        }
        if let Some(extract_piece_threshold) = proc_image.extract_piece_threshold {
            p.extract_piece_threshold = Some(extract_piece_threshold);
        }
        if let Some(board_threshold) = proc_image.board_threshold {
            p.board_threshold = Some(board_threshold);
        }
        if let Some(difference_level) = proc_image.difference_level {
            p.difference_level = Some(difference_level);
        }
    }
    if let Some(engine) = partial.engine {
        let e = ext_config.engine.get_or_insert(Default::default());
        if let Some(pretty) = engine.pretty {
            e.pretty = Some(pretty);
        }
    }

    (StatusCode::OK, Json(ext_config.clone()))
}

async fn init(State(state): State<AppState>) -> impl IntoResponse {
    let monitor_name: Option<String>;
    let proc_img_args: wrappers::args::ImgProcArgsDto;
    let pv: usize;
    {
        let ext_config_guard = state.ext_config.lock().await;
        if ext_config_guard.monitor.is_none() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "MonitorArgsDto is None"})),
            );
        }

        if ext_config_guard.proc_image.is_none() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "ImgProcArgsDto is None"})),
            );
        }

        if ext_config_guard.stockfish.is_none() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Stockfish is None"})),
            );
        }

        monitor_name = ext_config_guard.monitor.as_ref().unwrap().name.clone();
        proc_img_args = ext_config_guard.proc_image.as_ref().unwrap().clone();
        pv = ext_config_guard.stockfish.as_ref().unwrap().pv.unwrap();
    }
    let screen = match state.funcs.capture_screen_as_mat(monitor_name).await {
        Ok(s) => s,
        Err(e) => {
            let msg = format!("Failed to capture screen: {e}");
            log::error!("{msg}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": msg})),
            );
        }
    };
    let coords = match state.funcs.get_coords(&screen).await {
        Ok(c) => c,
        Err(e) => {
            let msg = format!("Failed to get board coordinates: {e}");
            log::error!("{msg}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": msg })),
            );
        }
    };

    let (sf_status, Json(sf_data)) = init_stockfish(State(state.clone())).await;

    if sf_status != 200 {
        let msg = format!("Init stockfish failed");
        log::error!("{msg}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": msg})),
        );
    }

    let board = match state.funcs.crop_board(&screen, coords).await {
        Ok(b) => b,
        Err(e) => {
            let msg = format!("Failed to crop board: {e}");
            log::error!("{msg}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": msg})),
            );
        }
    };
    let color = match procimg::detect_player_color(&board) {
        Ok(c) => c,
        Err(e) => {
            let msg = format!("Failed to detect player color: {e}");
            log::error!("{msg}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": msg })),
            );
        }
    };

    {
        let mut int_config = state.int_config.lock().await;
        int_config.coords = Some(coords);

        let board = create_board_default::<DefaultPrinter>(&color);
        int_config.prev_board = Some(*board.raw());
    }

    let pieces = match state
        .funcs
        .get_pieces(
            &board,
            proc_img_args.margin.unwrap(),
            proc_img_args.extract_piece_threshold.unwrap(),
            &color,
        )
        .await
    {
        Ok(p) => p,
        Err(e) => {
            let msg = format!("Failed to extract pieces: {e}");
            log::error!("{msg}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": msg })),
            );
        }
    };

    let pieces = pieces
        .into_iter()
        .map(|(c, mat)| (c, Arc::new(mat)))
        .collect();

    let mut int_config_guard = state.int_config.lock().await;
    int_config_guard.pieces = Some(pieces);
    int_config_guard.prev_board_mat = Some(board);
    int_config_guard.color = Some(color);

    let mut sf_guard = state.stockfish.lock().await;

    let mut sf_data = sf_data;

    let summary = match sf_guard.as_mut().unwrap().get_summary(pv) {
        Ok(s) => s,
        Err(e) => {
            let msg = format!("Failed to get stockfish summary: {e}");
            log::error!("{msg}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": msg })),
            );
        }
    };

    sf_data.summary = Some(
        summary
            .into_iter()
            .map(|sum| StockfishSummary {
                main_line: sum.main_line,
                evaluation: sum.eval,
            })
            .collect(),
    );

    if sf_status != StatusCode::OK {
        let msg = format!("Stockfish initialization failed: {sf_status:?}");
        log::error!("{msg}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": msg })),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "stockfish": sf_data,
            "int_config":int_config_guard.clone()
        })),
    )
}

// TODO: move to FuncWrapper
async fn init_stockfish(State(state): State<AppState>) -> (StatusCode, Json<StockfishResponse>) {
    let mut sf_guard = state.stockfish.lock().await;
    let ext_config_guard = state.ext_config.lock().await;

    let version: String;
    if sf_guard.is_none() {
        let path = std::path::PathBuf::from(std::env::var("ENGINE_PATH").unwrap());
        let depth = 5;

        let sf = state.funcs.init_stockfish(&path, depth).unwrap();
        version = sf.get_version();
        *sf_guard = Some(sf);
    } else {
        version = sf_guard.as_ref().unwrap().get_version();
    }

    let elo = ext_config_guard.stockfish.as_ref().unwrap().elo.unwrap();
    let skill = ext_config_guard.stockfish.as_ref().unwrap().skill.unwrap();
    let hash = ext_config_guard.stockfish.as_ref().unwrap().hash.unwrap();
    let multi_lines = ext_config_guard.stockfish.as_ref().unwrap().pv.unwrap();

    match sf_guard.as_mut().unwrap().set_config(
        &elo.to_string(),
        &skill.to_string(),
        &hash.to_string(),
        &multi_lines.to_string(),
    ) {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(StockfishResponse {
                    version: "".to_string(),
                    summary: None,
                }),
            );
        }
    }

    (
        StatusCode::OK,
        Json(StockfishResponse {
            version,
            summary: None,
        }),
    )
}

#[cfg(test)]
mod tests {
    use crate::wrappers::{
        args::{ImgProcArgsDto, MonitorArgsDto, StockfishArgsDto},
        func::{MockFunc, MockStockfish},
    };

    use super::*;
    use axum_test::TestServer;
    use cheatess_core::{
        core::engine::{Color, DefaultPrinter, create_board_default},
        procimg::Mat,
    };
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn get_board_before_init() {
        let state = AppState {
            int_config: Arc::new(Mutex::new(IntConfig {
                prev_board: None,
                color: None,
                coords: None,
                prev_board_mat: None,
                pieces: None,
            })),
            ext_config: Arc::new(Mutex::new(Default::default())),
            stockfish: Arc::new(Mutex::new(Default::default())),
            funcs: Arc::new(MockFunc::default()),
        };

        let app = router().with_state(state);

        let server = TestServer::new(app).unwrap();

        let response = server.get("/board").await;

        response.assert_status_not_found();
        response.assert_text("");
    }

    #[tokio::test]
    async fn get_board_after_init() {
        let board = create_board_default::<DefaultPrinter>(&Color::White);
        let state = AppState {
            int_config: Arc::new(Mutex::new(IntConfig {
                prev_board: Some(*board.as_ref().raw()),
                color: None,
                coords: None,
                prev_board_mat: None,
                pieces: None,
            })),
            ext_config: Arc::new(Mutex::new(Default::default())),
            stockfish: Arc::new(Mutex::new(Default::default())),
            funcs: Arc::new(MockFunc::default()),
        };

        let app = router().with_state(state);

        let server = TestServer::new(app).unwrap();

        let response = server.get("/board").await;

        let output = [
            ["r", "n", "b", "q", "k", "b", "n", "r"],
            ["p", "p", "p", "p", "p", "p", "p", "p"],
            [" ", " ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " ", " "],
            ["P", "P", "P", "P", "P", "P", "P", "P"],
            ["R", "N", "B", "Q", "K", "B", "N", "R"],
        ];
        response.assert_status_ok();
        response.assert_json(&serde_json::json!({"raw_data":output}));
    }

    #[tokio::test]
    async fn init_failed_with_capture_screen() {
        let state = AppState {
            int_config: Arc::new(Mutex::new(Default::default())),
            ext_config: Arc::new(Mutex::new(CheatessArgsDto {
                monitor: Some(MonitorArgsDto {
                    name: Some("abc".to_string()),
                }),
                stockfish: Some(StockfishArgsDto {
                    pv: Some(3),
                    ..Default::default()
                }),
                proc_image: Some(ImgProcArgsDto {
                    ..Default::default()
                }),
                ..Default::default()
            })),
            stockfish: Arc::new(Mutex::new(Default::default())),
            funcs: Arc::new(MockFunc::default()),
        };

        let app = router().with_state(state);

        let server = TestServer::new(app).unwrap();

        let response = server.post("/init").await;

        response.assert_status_internal_server_error();
        response.assert_json(
            &serde_json::json!({"error":"Failed to capture screen: Monitor not found"}),
        );
    }

    #[tokio::test]
    async fn init_failed_with_get_coords() {
        let funcs = MockFunc {
            mat: Some(Mat::default()),
            ..Default::default()
        };
        let state = AppState {
            int_config: Arc::new(Mutex::new(Default::default())),
            ext_config: Arc::new(Mutex::new(CheatessArgsDto {
                monitor: Some(MonitorArgsDto {
                    name: Some("abc".to_string()),
                }),
                stockfish: Some(StockfishArgsDto {
                    pv: Some(3),
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

        let app = router().with_state(state);

        let server = TestServer::new(app).unwrap();

        let response = server.post("/init").await;

        response.assert_status_internal_server_error();
        response.assert_json(
            &serde_json::json!({"error":"Failed to get board coordinates: Monitor not found"}),
        );
    }

    #[tokio::test]
    async fn init_failed_with_crop_board() {
        unsafe {
            // TODO: unnecessary if async init_stockfish will be mocked
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

        let app = router().with_state(state);
        let server = TestServer::new(app).unwrap();

        let response = server.post("/init").await;

        response.assert_status_internal_server_error();
        response
            .assert_json(&serde_json::json!({"error":"Failed to crop board: Monitor not found"}));
    }

    // TODO!: add tests for init_stockfish
}

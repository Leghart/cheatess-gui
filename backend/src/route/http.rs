use crate::route::{AppState, IntConfig};
use crate::wrappers;
use crate::wrappers::args::CheatessArgsDto;
use crate::wrappers::enums::ColorDto;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, response::IntoResponse};
use cheatess_core::core::engine::{DefaultPrinter, create_board_default};
use cheatess_core::core::procimg;
use cheatess_core::core::stockfish;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::sync::Arc;
#[derive(Deserialize, Serialize)]
pub struct InitBoardRequest {
    color: wrappers::enums::ColorDto,
}

#[derive(Deserialize, Serialize)]
pub struct RawBoardResponse {
    pub raw_data: [[char; 8]; 8],
}

#[derive(Deserialize, Serialize)]
pub struct StockfishResponse {
    pub version: String,
    pub best_move: Option<String>,
    pub eval: Option<String>,
}

pub async fn get_int_config(State(state): State<AppState>) -> (StatusCode, Json<IntConfig>) {
    (StatusCode::OK, Json(state.int_config.lock().await.clone()))
}

pub async fn get_ext_config(State(state): State<AppState>) -> (StatusCode, Json<CheatessArgsDto>) {
    (StatusCode::OK, Json(state.ext_config.lock().await.clone()))
}

pub async fn update_ext_config(
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
    if let Some(monitor) = partial.monitor {
        if let Some(number) = monitor.number {
            ext_config.monitor.get_or_insert(Default::default()).number = Some(number);
        }
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

// TODO: PoC: opt required
pub async fn init(State(state): State<AppState>) -> impl IntoResponse {
    let monitor_number: u8;
    let proc_img_args: wrappers::args::ImgProcArgsDto;
    {
        let ext_config_guard = state.ext_config.lock().await;
        monitor_number = ext_config_guard.monitor.as_ref().unwrap().number.unwrap();
        proc_img_args = ext_config_guard.proc_image.as_ref().unwrap().clone();
    }
    let screen = wrappers::methods::capture_screen_as_mat(monitor_number).await;
    let coords = wrappers::methods::get_coords(&screen).await;

    let (sf_status, Json(sf_data)) = init_stockfish(State(state.clone())).await;

    let board = wrappers::methods::crop_board(&screen, coords).await;
    let color = procimg::detect_player_color(&board);

    {
        let mut int_config = state.int_config.lock().await;
        int_config.coords = Some(coords);

        let board = create_board_default::<DefaultPrinter>(&color);
        int_config.prev_board = Some(*board.raw());
    }

    let pieces = wrappers::methods::get_pieces(
        &board,
        proc_img_args.margin.unwrap(),
        proc_img_args.extract_piece_threshold.unwrap(),
        &color,
    )
    .await;

    let pieces = pieces
        .into_iter()
        .map(|(c, mat)| (c, Arc::new(mat)))
        .collect();

    let mut int_config_guard = state.int_config.lock().await;
    int_config_guard.pieces = Some(pieces);
    int_config_guard.prev_board_mat = Some(board);
    int_config_guard.color = Some(ColorDto::from(color));

    let mut sf_guard = state.stockfish.lock().await;
    let best_move = sf_guard.as_mut().unwrap().get_best_move().unwrap();

    let mut sf_data = sf_data;
    sf_data.best_move = Some(best_move);
    sf_data.eval = Some(sf_guard.as_mut().unwrap().get_evaluation());

    if sf_status != StatusCode::OK {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Stockfish initialization failed:{:?}",sf_status) })),
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

pub async fn get_prev_board(State(state): State<AppState>) -> (StatusCode, Json<RawBoardResponse>) {
    let int_config_guard = state.int_config.lock().await;
    let raw_data = int_config_guard
        .prev_board
        .clone()
        .expect("Board hasn't been created yet.");

    (StatusCode::OK, Json(RawBoardResponse { raw_data }))
}

pub async fn init_stockfish(
    State(state): State<AppState>,
) -> (StatusCode, Json<StockfishResponse>) {
    let mut sf_guard = state.stockfish.lock().await;
    let ext_config_guard = state.ext_config.lock().await;

    let version: String;
    if sf_guard.is_none() {
        let path = std::path::PathBuf::from(crate::route::ENGINE_PATH);
        let depth = 5;

        let sf = stockfish::Stockfish::new(&path, depth);
        version = sf.version.clone();
        *sf_guard = Some(sf);
    } else {
        version = sf_guard.as_ref().unwrap().version.clone();
    }

    let elo = ext_config_guard.stockfish.as_ref().unwrap().elo.unwrap();
    let skill = ext_config_guard.stockfish.as_ref().unwrap().skill.unwrap();
    let hash = ext_config_guard.stockfish.as_ref().unwrap().hash.unwrap();

    sf_guard
        .as_mut()
        .unwrap()
        .set_config(&elo.to_string(), &skill.to_string(), &hash.to_string());

    (
        StatusCode::OK,
        Json(StockfishResponse {
            version,
            best_move: None,
            eval: None,
        }),
    )
}

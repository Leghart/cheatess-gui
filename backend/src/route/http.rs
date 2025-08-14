use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, response::IntoResponse};
use cheatess_core::core::engine::{Color, DefaultPrinter, create_board_default};
use cheatess_core::core::procimg;
use cheatess_core::core::stockfish;
use cheatess_core::utils::monitor;
use serde_json;
use serde_json::json;

use crate::route::{AppState, BoardGrid};
use crate::wrappers;

pub async fn get_config_handler(State(state): State<AppState>) -> impl IntoResponse {
    let config_guard = state.config.lock().await;

    (StatusCode::OK, Json(config_guard.clone()))
}

pub async fn patch_config_handler(
    State(state): State<AppState>,
    Json(partial): Json<wrappers::args::CheatessArgsDto>,
) -> impl IntoResponse {
    let mut config = state.config.lock().await;

    if let Some(verbose) = partial.verbose {
        config.verbose = Some(verbose);
    }
    if let Some(mode) = partial.mode {
        config.mode = Some(mode);
    }
    if let Some(monitor) = partial.monitor {
        if let Some(number) = monitor.number {
            config.monitor.get_or_insert(Default::default()).number = Some(number);
        }
    }
    if let Some(stockfish) = partial.stockfish {
        let s = config.stockfish.get_or_insert(Default::default());
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
        let p = config.proc_image.get_or_insert(Default::default());
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
        let e = config.engine.get_or_insert(Default::default());
        if let Some(pretty) = engine.pretty {
            e.pretty = Some(pretty);
        }
    }

    (StatusCode::OK, Json(config.clone()))
}

pub async fn get_board_handler(State(_state): State<AppState>) -> impl IntoResponse {
    let board = create_board_default::<DefaultPrinter>(&Color::Black);
    let data = board.raw().map(|row| row.map(|c| c.to_string()));
    Json(json!(BoardGrid { data }))
}

pub async fn init_stockfish(State(state): State<AppState>) -> impl IntoResponse {
    let mut sf_guard = state.stockfish.lock().await;
    let config_guard = state.config.lock().await;

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

    let elo = config_guard.stockfish.as_ref().unwrap().elo.unwrap();
    let skill = config_guard.stockfish.as_ref().unwrap().skill.unwrap();
    let hash = config_guard.stockfish.as_ref().unwrap().hash.unwrap();

    sf_guard
        .as_mut()
        .unwrap()
        .set_config(&elo.to_string(), &skill.to_string(), &hash.to_string());

    (StatusCode::OK, Json(json!({"version":version})))
}

pub async fn detect_player_color(State(state): State<AppState>) -> impl IntoResponse {
    let config_guard = state.config.lock().await;
    let monitor_number = config_guard.monitor.as_ref().unwrap().number.unwrap();

    let monitor = monitor::select_monitor(monitor_number).expect("Requested monitor not found");
    let raw = monitor::capture_entire_screen(&monitor);
    let entire_screen_gray = procimg::image_buffer_to_gray_mat(&raw).unwrap();
    let coords = procimg::get_board_region(&entire_screen_gray);

    let cropped = procimg::crop_image(&raw, &coords);
    let board = procimg::image_buffer_to_gray_mat(&cropped).unwrap();

    let color = procimg::detect_player_color(&board);
    (
        StatusCode::OK,
        Json(json!({"color":wrappers::enums::ColorDto::from(color)})),
    )
}

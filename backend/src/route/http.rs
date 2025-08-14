use crate::route::AppState;
use crate::wrappers;
use crate::wrappers::args::CheatessArgsDto;
use crate::wrappers::enums::ColorDto;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, response::IntoResponse};
use cheatess_core::core::engine::{Color, DefaultPrinter, create_board_default};
use cheatess_core::core::procimg;
use cheatess_core::core::stockfish;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
#[derive(Deserialize, Serialize)]
pub struct InitBoardRequest {
    color: wrappers::enums::ColorDto,
}

#[derive(Deserialize, Serialize)]
pub struct BoardResponse {
    pub raw_data: [[char; 8]; 8],
}

#[derive(Deserialize, Serialize)]
pub struct StockfishResponse {
    pub version: String,
    pub best_move: Option<String>,
    pub eval: Option<String>,
}

pub async fn get_config_handler(
    State(state): State<AppState>,
) -> (StatusCode, Json<CheatessArgsDto>) {
    let config_guard = state.config.lock().await;

    (StatusCode::OK, Json(config_guard.clone()))
}

pub async fn patch_config_handler(
    State(state): State<AppState>,
    Json(partial): Json<wrappers::args::CheatessArgsDto>,
) -> (StatusCode, Json<CheatessArgsDto>) {
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

// TODO: PoC: opt required
// TODO: player color taken from payload, not auto detection
pub async fn init(
    State(state): State<AppState>,
    Json(payload): Json<InitBoardRequest>,
) -> impl IntoResponse {
    let monitor_number: u8;
    let proc_img_args: wrappers::args::ImgProcArgsDto;
    {
        let config_guard = state.config.lock().await;
        monitor_number = config_guard.monitor.as_ref().unwrap().number.unwrap();
        proc_img_args = config_guard.proc_image.as_ref().unwrap().clone();
    }
    let screen = wrappers::methods::capture_screen_as_mat(monitor_number).await;
    let coords = wrappers::methods::get_coords(&screen).await;

    {
        let mut int_config = state.int_config.lock().await;
        int_config.coords = Some(coords);
    }

    let sf = init_stockfish(State(state.clone()));
    let brd = init_board(State(state.clone()), Json(payload));
    // let pc = detect_player_color(State(state.clone()));

    let (sf_result, brd_result) = tokio::join!(sf, brd);

    let (sf_status, Json(sf_data)) = sf_result;
    let (brd_status, Json(brd_data)) = brd_result;
    // let (pc_status, Json(pc_data)) = pc_result;
    let board = wrappers::methods::crop_board(&screen, coords).await;

    let pieces = wrappers::methods::get_pieces(
        &board,
        proc_img_args.margin.unwrap(),
        proc_img_args.extract_piece_threshold.unwrap(),
        // pc_data.into(),
        Color::White,
    )
    .await;

    let mut sf_guard = state.stockfish.lock().await;
    let best_move = sf_guard.as_mut().unwrap().get_best_move().unwrap();

    let mut sf_data = sf_data;
    sf_data.best_move = Some(best_move);
    sf_data.eval = Some(sf_guard.as_mut().unwrap().get_evaluation());
    //TODO
    // save pieces
    // init prev_board

    if sf_status != StatusCode::OK || brd_status != StatusCode::CREATED {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                json!({ "error": format!("Initialization failed: stockfish status ={:?} board status ={:?}",sf_status,brd_status) }),
            ),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "stockfish": sf_data,
            "board": brd_data,
        })),
    )
}

pub async fn init_board(
    State(state): State<AppState>,
    Json(payload): Json<InitBoardRequest>,
) -> (StatusCode, Json<BoardResponse>) {
    let mut int_config_guard = state.int_config.lock().await;
    let color: Color = payload.color.into();

    let board = create_board_default::<DefaultPrinter>(&color);
    let raw_data = board.raw().clone();
    int_config_guard.current_board = Some(board);

    (StatusCode::CREATED, Json(BoardResponse { raw_data }))
}

pub async fn get_current_board(State(state): State<AppState>) -> impl IntoResponse {
    let int_config_guard = state.int_config.lock().await;
    let board = int_config_guard.current_board.as_ref().unwrap();
    let data: [[String; 8]; 8] = board.raw().map(|row| row.map(|c| c.to_string()));

    (StatusCode::OK, Json(json!({"data":data})))
}

pub async fn init_stockfish(
    State(state): State<AppState>,
) -> (StatusCode, Json<StockfishResponse>) {
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

    (
        StatusCode::OK,
        Json(StockfishResponse {
            version,
            best_move: None,
            eval: None,
        }),
    )
}

pub async fn detect_player_color(State(state): State<AppState>) -> (StatusCode, Json<ColorDto>) {
    let config_guard = state.config.lock().await;
    let int_config_guard = state.int_config.lock().await;

    let monitor_number = config_guard.monitor.as_ref().unwrap().number.unwrap();
    let coords = int_config_guard.coords.unwrap();

    let screen = wrappers::methods::capture_screen_as_mat(monitor_number).await;
    let board = wrappers::methods::crop_board(&screen, coords).await;
    let color = procimg::detect_player_color(&board);

    (StatusCode::OK, Json(ColorDto::from(color)))
}

use axum::extract::State;
use axum::extract::ws::CloseFrame;
use axum::extract::ws::Utf8Bytes;
use axum::http::StatusCode;
use axum::{
    Json, Router,
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::{any, get, patch},
};
use cheatess_core::core::engine::{Color, DefaultPrinter, create_board_default};
use cheatess_core::core::stockfish::Stockfish;
use cheatess_core::utils::parser::parse_args_from;
use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::sync::Arc;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tokio::sync::Mutex;

mod wrappers;
use wrappers::args;

const ENGINE_PATH: &str = "/home/leghart/projects/cheatess/stockfish-ubuntu-x86-64-avx2";

#[derive(Clone)]
struct AppState {
    stockfish: Arc<Mutex<Option<Stockfish>>>,
    config: Arc<Mutex<args::CheatessArgsDto>>,
}

#[derive(Serialize, Deserialize)]
struct BoardGrid {
    data: [[String; 8]; 8],
}

#[tokio::main]
async fn main() {
    let env_args = vec!["target/debug/backend", "stockfish", "-p", ENGINE_PATH];
    let args = parse_args_from(env_args);
    let state = AppState {
        stockfish: Arc::new(Mutex::new(None)),
        config: Arc::new(Mutex::new(args::CheatessArgsDto::from(&args))),
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/game", any(game_handler))
        .route("/config", get(get_config_handler))
        .route("/config", patch(patch_config_handler))
        .route("/board", get(get_board_handler))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn game_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| start_game(socket, axum::extract::State(state)))
}

async fn get_config_handler(State(state): State<AppState>) -> impl IntoResponse {
    let config_guard = state.config.lock().await;

    Json(config_guard.clone())
}

async fn patch_config_handler(
    State(state): State<AppState>,
    Json(partial): Json<args::CheatessArgsDto>,
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

async fn get_board_handler(State(_state): State<AppState>) -> impl IntoResponse {
    let board = create_board_default::<DefaultPrinter>(&Color::Black);
    let data = board.raw().map(|row| row.map(|c| c.to_string()));
    Json(json!(BoardGrid { data }))
}

async fn start_game(mut socket: WebSocket, State(state): State<AppState>) {
    if socket
        .send(Message::Ping(Bytes::from_static(&[1])))
        .await
        .is_ok()
    {
        println!("Connected...");
    } else {
        println!("Could not send ping!");
        return;
    }

    if let Some(msg) = socket.recv().await {
        if let Err(e) = msg {
            println!("client abruptly disconnected {}", e);
            return;
        }
    }

    let mut sf_guard = state.stockfish.lock().await;

    if sf_guard.is_none() {
        let path = PathBuf::from(ENGINE_PATH);
        let depth = 5;

        let sf = Stockfish::new(&path, depth);
        println!("version {}", sf.version);
        *sf_guard = Some(sf);
    }

    let sf = sf_guard.as_mut().unwrap();
    println!("Stockfish initialized: {}", sf.version);

    if socket.send(Message::Text("ok".into())).await.is_err() {
        println!("Could not send board to client!");
        return;
    };

    let (mut sender, _receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        for mv in [
            "e2e4", "e7e5", "g1f3", "b8c6", "d2d3", "b1c3", "c8g4", "f1e2", "g4f3", "e2f3", "d8g5",
            "e1g1",
        ] {
            if sender.send(Message::Text(mv.into())).await.is_err() {
                return mv.to_string();
            }

            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
        }

        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Utf8Bytes::from_static("Goodbye"),
            })))
            .await
        {
            println!("Could not send Close due to {e}, probably it is ok?");
        }
        "Finished".to_string()
    });

    if let Err(e) = send_task.await {
        eprintln!("Error in send task: {e}");
    }

    println!("Websocket context destroyed");
}

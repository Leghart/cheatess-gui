use axum::{
    Router,
    routing::{any, get, patch, post},
};
use cheatess_core::utils::parser::parse_args_from;

use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tokio::sync::Mutex;

mod route;
mod wrappers;

use route::{AppState, ENGINE_PATH, IntConfig};
use wrappers::args;

#[tokio::main]
async fn main() {
    let env_args = vec!["target/debug/backend", "stockfish", "-p", ENGINE_PATH];
    let args = parse_args_from(env_args);
    let state = AppState {
        stockfish: Arc::new(Mutex::new(None)),
        config: Arc::new(Mutex::new(args::CheatessArgsDto::from(&args))),
        int_config: Arc::new(Mutex::new(IntConfig::new())),
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
        .route("/game", any(route::ws::game_handler))
        .route("/init", post(route::http::init))
        .route("/init_board", post(route::http::init_board))
        .route("/init_stockfish", post(route::http::init_stockfish))
        .route("/player_color", get(route::http::detect_player_color))
        .route("/config", get(route::http::get_config_handler))
        .route("/config", patch(route::http::patch_config_handler))
        .route("/board", get(route::http::get_current_board))
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

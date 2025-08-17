use axum::{
    Router,
    routing::{any, get, patch, post},
};
use cheatess_core::utils::parser::parse_args_from;

use http::{HeaderValue, Method, header::CONTENT_TYPE};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod route;
mod wrappers;

use route::{AppState, ENGINE_PATH, IntConfig};
use wrappers::args;

#[tokio::main]
async fn main() {
    let args = parse_args_from(vec!["target/debug/backend", "stockfish", "-p", ENGINE_PATH]);
    let state = AppState {
        stockfish: Arc::new(Mutex::new(None)),
        ext_config: Arc::new(Mutex::new(args::CheatessArgsDto::from(&args))),
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

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/game", any(route::ws::game_handler))
        .route("/init", post(route::http::init))
        .route("/int_config", get(route::http::get_int_config))
        .route("/ext_config", get(route::http::get_ext_config))
        .route("/ext_config", patch(route::http::update_ext_config))
        .route("/board", get(route::http::get_prev_board))
        .with_state(state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let port = match std::env::var("PORT") {
        Ok(val) => val,
        Err(_) => {
            panic!("Not found `PORT` env variable.");
        }
    };
    let listener = tokio::net::TcpListener::bind(&format!("http://127.0.0.1:{port}"))
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

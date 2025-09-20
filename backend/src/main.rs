use axum::Router;
use cheatess_core::utils::parser::parse_args_from;

use http::{Method, header::CONTENT_TYPE};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

mod route;
mod wrappers;

use route::{AppState, IntConfig};
use wrappers::args;

use crate::wrappers::func::ProdFunc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let be_port =
        std::env::var("BACKEND_PORT").expect("Missing 'BACKEND_PORT' env variable in .env");

    let fe_port =
        std::env::var("FRONTEND_PORT").expect("Missing 'FRONTEND_PORT' env variable in .env");

    let args = parse_args_from(vec![
        "target/debug/backend",
        "stockfish",
        "-p",
        &std::env::var("ENGINE_PATH").unwrap(),
        "--pv",
        "3",
        "-v",
    ]);

    cheatess_core::logger::init_with_buffer(args.verbose.into());

    let origins = [
        format!("http://localhost:{fe_port}").parse().unwrap(),
        format!("http://127.0.0.1:{fe_port}").parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .merge(route::ws::router())
        .merge(route::http::router())
        .with_state(AppState {
            stockfish: Arc::new(Mutex::new(None)),
            ext_config: Arc::new(Mutex::new(args::CheatessArgsDto::from(&args))),
            int_config: Arc::new(Mutex::new(IntConfig::new())),
            funcs: Arc::new(ProdFunc),
        })
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(&format!("127.0.0.1:{be_port}"))
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

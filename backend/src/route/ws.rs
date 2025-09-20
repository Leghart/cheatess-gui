use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, routing::any};
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
};
use futures_util::{
    SinkExt,
    stream::{SplitSink, StreamExt},
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value, json};

use cheatess_core::engine::Color;
use cheatess_core::monitor::Monitor;

use super::StockfishSummary;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/game", any(game_handler))
        .route("/logs", any(collect_logs_handler))
}

async fn game_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| start_game(socket, axum::extract::State(state)))
}

async fn collect_logs_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(get_logs)
}

async fn send(sender: &mut SplitSink<WebSocket, Message>, msg: WsResponse) {
    let json = serde_json::to_string(&msg).unwrap();
    match sender.send(json.into()).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("WebSocketResponse error: {e}");
        }
    }
}

#[derive(Serialize, Deserialize)]
enum WsResponse {
    GameOver,
    NextMove(Box<CorrectMove>),
}

#[derive(Serialize, Deserialize)]
struct CorrectMove {
    last_move: Option<String>,
    summary: Option<Vec<StockfishSummary>>,
    raw_board: [[char; 8]; 8],
}

async fn start_game(mut socket: WebSocket, State(state): State<AppState>) {
    if socket
        .send(Message::Ping(Bytes::from_static(&[1])))
        .await
        .is_ok()
    {
        log::info!("Starting...");
    } else {
        log::error!("Socket disconnected");
        return;
    }

    if let Some(Err(e)) = socket.recv().await {
        log::error!("client abruptly disconnected {e}");
        return;
    }

    let (mut sender, _receiver) = socket.split();

    tokio::spawn(async move {
        let monitor_name: Option<String>;
        let piece_threshold: f64;
        let board_threshold: f64;
        let pieces: std::collections::HashMap<char, std::sync::Arc<cheatess_core::procimg::Mat>>;
        let player_color: Color;
        let monitor: Monitor;
        let coords: (u32, u32, u32, u32);
        let diff_level: i32;
        let pv: usize;
        {
            let ext_config = state.ext_config.lock().await;
            let int_config = state.int_config.lock().await;

            coords = int_config.coords.unwrap();
            pieces = int_config.pieces.clone().unwrap();
            player_color = int_config.color.unwrap();

            monitor_name = ext_config.monitor.as_ref().unwrap().name.clone();
            monitor = match state.funcs.get_monitor(monitor_name).await {
                Ok(m) => m,
                Err(e) => {
                    log::error!("Failed to get monitor: {e}");
                    return;
                }
            };
            pv = ext_config.stockfish.as_ref().unwrap().pv.unwrap();

            piece_threshold = ext_config
                .proc_image
                .as_ref()
                .unwrap()
                .piece_threshold
                .unwrap();

            board_threshold = ext_config
                .proc_image
                .as_ref()
                .unwrap()
                .board_threshold
                .unwrap();

            diff_level = ext_config
                .proc_image
                .as_ref()
                .unwrap()
                .difference_level
                .unwrap();
        }

        let mut last_move: Option<String>;
        loop {
            let prev_mat: cheatess_core::procimg::Mat;
            let prev_board: [[char; 8]; 8];
            {
                let int_config = state.int_config.lock().await;
                prev_mat = int_config.prev_board_mat.clone().unwrap();
                prev_board = int_config.prev_board.unwrap();
            }

            let cropped = match cheatess_core::utils::monitor::get_cropped_screen(
                &monitor, coords.0, coords.1, coords.2, coords.3,
            ) {
                Ok(img) => img,
                Err(e) => {
                    log::error!("Failed to capture screen: {e}");
                    continue;
                }
            };
            let gray_board = match cheatess_core::core::procimg::image_buffer_to_gray_mat(cropped) {
                Ok(mat) => mat,
                Err(e) => {
                    log::error!("Failed to convert image to gray mat: {e}");
                    continue;
                }
            };

            match cheatess_core::core::procimg::are_images_different(
                &prev_mat,
                &gray_board,
                diff_level,
            ) {
                Ok(result) => {
                    if !result {
                        continue;
                    }
                }
                Err(e) => {
                    log::error!("Failed to compare images: {e}");
                    continue;
                }
            }

            let new_raw_board = match cheatess_core::core::procimg::find_all_pieces(
                &gray_board,
                &pieces,
                piece_threshold,
                board_threshold,
            ) {
                Ok(board) => board,
                Err(e) => {
                    log::error!("Failed to detect pieces: {e}");
                    continue;
                }
            };
            log::debug!("detected all pieces: {new_raw_board:?}");

            let (mv, mv_type) = {
                match cheatess_core::core::engine::detect_move(
                    &prev_board,
                    &new_raw_board,
                    &player_color,
                ) {
                    Ok(result) => result,
                    Err(_) => continue,
                }
            };
            log::debug!("detected move: {mv:?}");
            log::debug!("detected move type: {mv_type:?}");
            last_move = Some(mv.clone());

            {
                let mut stockfish = state.stockfish.lock().await;
                match stockfish.as_mut().unwrap().make_move(vec![mv]) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Failed to make move in stockfish: {e}");
                        continue;
                    }
                };
            }

            let current_board = cheatess_core::core::engine::create_board_from_data::<
                cheatess_core::printer::DefaultPrinter,
            >(new_raw_board, &player_color);

            {
                let mut stockfish = state.stockfish.lock().await;
                let summary = match stockfish.as_mut().unwrap().summary(pv) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("Failed to get stockfish summary: {e}");
                        continue;
                    }
                };

                let mut stockfish_summary: Vec<StockfishSummary> = Vec::new();
                for sum in summary {
                    if sum.main_line.is_empty() {
                        log::info!("Not found stockfish best lines: Game over");
                        send(&mut sender, WsResponse::GameOver).await;
                        break;
                    }
                    stockfish_summary.push(StockfishSummary {
                        main_line: sum.main_line,
                        evaluation: sum.eval,
                    });
                }

                send(
                    &mut sender,
                    WsResponse::NextMove(Box::new(CorrectMove {
                        last_move,
                        summary: Some(stockfish_summary),
                        raw_board: *current_board.raw(),
                    })),
                )
                .await;
            }

            let mut int_config = state.int_config.lock().await;
            int_config.prev_board = Some(*current_board.raw());
            int_config.prev_board_mat = Some(gray_board);
            log::debug!("Save previous board: {:?}", int_config.prev_board);
        }
    });
}

async fn get_logs(mut socket: WebSocket) {
    loop {
        let logs = cheatess_core::logger::collect_logs();
        if !logs.is_empty() {
            let payload: Vec<Value> = logs
                .into_iter()
                .map(|log| json!({ "level": format!("{:?}", log.level), "message": log.message }))
                .collect();

            if socket
                .send(Message::Text(
                    serde_json::to_string(&payload).unwrap().into(),
                ))
                .await
                .is_err()
            {
                break;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

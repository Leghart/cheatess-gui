use crate::AppState;
use crate::route::ENGINE_PATH;
use axum::extract::State;
use axum::extract::ws::CloseFrame;
use axum::extract::ws::Utf8Bytes;
use axum::response::IntoResponse;
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
};
use cheatess_core::core::stockfish::Stockfish;
use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use std::path::PathBuf;

pub async fn game_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| start_game(socket, axum::extract::State(state)))
}

pub async fn start_game(mut socket: WebSocket, State(state): State<AppState>) {
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

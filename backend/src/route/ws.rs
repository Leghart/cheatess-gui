use crate::AppState;
use crate::wrappers;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
};
use cheatess_core::engine::Color;
use cheatess_core::monitor::Monitor;
use futures_util::SinkExt;
use futures_util::stream::StreamExt;

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

    let (mut sender, _receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        let monitor_number: u8;
        let piece_threshold: f64;
        let board_threshold: f64;
        let pieces: std::collections::HashMap<char, std::sync::Arc<cheatess_core::procimg::Mat>>;
        let player_color: Color;
        let monitor: Monitor;
        let coords: (u32, u32, u32, u32);
        let diff_level: i32;
        {
            let ext_config = state.ext_config.lock().await;
            let int_config = state.int_config.lock().await;

            monitor_number = ext_config.monitor.as_ref().unwrap().number.unwrap();
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
            pieces = int_config.pieces.clone().unwrap();
            let tmp: Option<Color> = int_config.color.map(Into::into);
            player_color = tmp.unwrap();

            monitor = wrappers::methods::get_monitor(monitor_number).await;
            coords = int_config.coords.unwrap();
            diff_level = ext_config
                .proc_image
                .as_ref()
                .unwrap()
                .difference_level
                .unwrap();
        }
        sender
            .send(Message::Text("game started".into()))
            .await
            .unwrap();

        loop {
            let prev_mat: cheatess_core::procimg::Mat;
            let prev_board: [[char; 8]; 8];
            {
                let int_config = state.int_config.lock().await;
                prev_mat = int_config.prev_board_mat.clone().unwrap();
                prev_board = int_config.prev_board.clone().unwrap();
            }

            let cropped = cheatess_core::utils::monitor::get_cropped_screen(
                &monitor, coords.0, coords.1, coords.2, coords.3,
            );
            let gray_board =
                cheatess_core::core::procimg::image_buffer_to_gray_mat(cropped).unwrap();

            if !cheatess_core::procimg::are_images_different(&prev_mat, &gray_board, diff_level) {
                continue;
            }

            let new_raw_board = cheatess_core::core::procimg::find_all_pieces(
                &gray_board,
                &pieces,
                piece_threshold,
                board_threshold,
            );

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
            {
                let mut stockfish = state.stockfish.lock().await;
                stockfish.as_mut().unwrap().make_move(vec![mv]);
            }

            let current_board = cheatess_core::core::engine::create_board_from_data::<
                cheatess_core::printer::DefaultPrinter,
            >(new_raw_board, &player_color);

            {
                let mut stockfish = state.stockfish.lock().await;
                let sf_best_move = stockfish.as_mut().unwrap().get_best_move();
                match sf_best_move {
                    Some(best) => {
                        sender
                            .send(Message::Text(format!("Stockfish best move: {best}").into()))
                            .await
                            .unwrap();
                    }
                    None => {
                        sender
                            .send(Message::Text("Game over".into()))
                            .await
                            .unwrap();
                        break;
                    }
                };
            }

            let mut int_config = state.int_config.lock().await;
            int_config.prev_board = Some(*current_board.raw());
            int_config.prev_board_mat = Some(gray_board);
        }

        "Finished".to_string()
    });
}

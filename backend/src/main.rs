use axum::{
    Router,
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::any,
};
use cheatess_core::core::engine::{Color, DefaultPrinter, create_board_default};

use cheatess_core::core::stockfish::Stockfish;
use serde::{Deserialize, Serialize};

use axum::extract::State;
use serde_json;
use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::sync::Arc;
//allows to split the websocket stream into separate TX and RX branches
use futures_util::stream::StreamExt;

use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    stockfish: Arc<Mutex<Option<Stockfish>>>,
}

#[derive(Debug)]
enum Command {
    Test,
    GetBoard,
    InitStockfish,
    Unknown(String),
    None,
}

#[derive(Serialize, Deserialize)]
struct BoardGrid {
    data: [[String; 8]; 8],
}

#[tokio::main]
async fn main() {
    let stockfish = Arc::new(Mutex::new(None));
    let state = AppState { stockfish };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/ws", any(ws_handler))
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

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, axum::extract::State(state)))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, State(state): State<AppState>) {
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

    let cmd: Command;
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            let msg_result = process_message_to_cmd(msg);
            if msg_result.is_break() {
                return;
            }
            cmd = msg_result.continue_value().expect("Ureachable");
        } else {
            println!("client abruptly disconnected");
            return;
        }
    } else {
        unreachable!();
    }

    match cmd {
        Command::GetBoard => {
            let board = create_board_default::<DefaultPrinter>(&Color::Black);

            let data = board.raw().map(|row| row.map(|c| c.to_string()));
            let grid = BoardGrid { data };
            let json = serde_json::to_string(&grid).unwrap();
            if socket.send(Message::Text(json.into())).await.is_err() {
                println!("Could not send board to client!");
                return;
            };
        }
        Command::Test => {
            if socket
                .send(Message::Text("Congrats...".into()))
                .await
                .is_err()
            {
                println!("Could not send test message to client!");
                return;
            };
        }
        Command::InitStockfish => {
            init_stockfish_instance(axum::extract::State(state))
                .await
                .unwrap();
            if socket.send(Message::Text("ok".into())).await.is_err() {
                println!("Could not send board to client!");
                return;
            };
        }
        _ => panic!("Can not process command"),
    }

    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    // let mut send_task = tokio::spawn(async move {
    //     let n_msg = 20;
    //     for i in 0..n_msg {
    //         if sender
    //             .send(Message::Text(format!("Server message {i} ...").into()))
    //             .await
    //             .is_err()
    //         {
    //             return i;
    //         }

    //         tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    //     }

    //     println!("Sending close to client ...");
    //     if let Err(e) = sender
    //         .send(Message::Close(Some(CloseFrame {
    //             code: axum::extract::ws::close_code::NORMAL,
    //             reason: Utf8Bytes::from_static("Goodbye"),
    //         })))
    //         .await
    //     {
    //         println!("Could not send Close due to {e}, probably it is ok?");
    //     }
    //     n_msg
    // });

    // This second task will receive messages from client and print them on server console
    // let mut recv_task = tokio::spawn(async move {
    //     let mut cnt = 0;
    //     while let Some(Ok(msg)) = receiver.next().await {
    //         cnt += 1;
    //         // print message and break if instructed to do so
    //         if process_message_to_cmd(msg).is_break() {
    //             break;
    //         }
    //     }
    //     cnt
    // });

    // If any one of the tasks exit, abort the other.
    // tokio::select! {
    //     rv_a = (&mut send_task) => {
    //         match rv_a {
    //             Ok(a) => println!("{a} messages sent to client"),
    //             Err(a) => println!("Error sending messages {a:?}")
    //         }
    //         recv_task.abort();
    //     },
    //     rv_b = (&mut recv_task) => {
    //         match rv_b {
    //             Ok(b) => println!("Received {b} messages"),
    //             Err(b) => println!("Error receiving messages {b:?}")
    //         }
    //         send_task.abort();
    //     }
    // }

    println!("Websocket context destroyed");
}

fn process_message_to_cmd(msg: Message) -> ControlFlow<(), Command> {
    match msg {
        Message::Text(t) => {
            let cmd = match t.as_ref() {
                "init_stockfish\n" => Command::InitStockfish,
                "get_board\n" => Command::GetBoard,
                "test\n" => Command::Test,
                other => Command::Unknown(other.to_string()),
            };
            ControlFlow::Continue(cmd)
        }

        Message::Close(_) => ControlFlow::Break(()),
        _ => ControlFlow::Continue(Command::None),
    }
}

async fn init_stockfish_instance(State(state): State<AppState>) -> Result<(), String> {
    let mut sf_guard = state.stockfish.lock().await;

    if sf_guard.is_some() {
        println!("Stockfish already created. Skipping...");
        return Ok(());
    }

    let path = PathBuf::from("/home/leghart/projects/cheatess/stockfish-ubuntu-x86-64-avx2");
    let depth = 5;

    let sf = Stockfish::new(&path, depth);
    println!("version {}", sf.version);
    *sf_guard = Some(sf);

    Ok(())
}

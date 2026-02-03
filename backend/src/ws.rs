use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};

use crate::pty::spawn_pty;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Create PTY session
    let pty_handle = match spawn_pty() {
        Ok(handle) => handle,
        Err(e) => {
            eprintln!("Failed to create PTY session: {}", e);
            return;
        }
    };

    let mut reader_rx = pty_handle.reader_rx;
    let writer_tx = pty_handle.writer_tx;

    // Task: PTY -> WebSocket (send terminal output to browser)
    let send_task = tokio::spawn(async move {
        while let Some(data) = reader_rx.recv().await {
            if ws_sender.send(Message::Binary(data)).await.is_err() {
                break;
            }
        }
    });

    // Task: WebSocket -> PTY (send user input to terminal)
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if writer_tx.send(text.into_bytes()).await.is_err() {
                        break;
                    }
                }
                Message::Binary(data) => {
                    if writer_tx.send(data).await.is_err() {
                        break;
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

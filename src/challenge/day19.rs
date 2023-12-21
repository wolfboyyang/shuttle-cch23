use std::{
    collections::HashMap,
    ops::ControlFlow,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::{watch, RwLock};

#[derive(Clone)]
struct GameState {
    start: Arc<AtomicBool>,
    room_channel: Arc<RwLock<HashMap<u32, watch::Sender<Message>>>>,
    views: Arc<AtomicUsize>,
}

pub fn task() -> Router {
    // create a receiver/sender for a channel for us to send and receive messages over

    let state = GameState {
        start: Arc::new(AtomicBool::new(false)),
        room_channel: Arc::new(RwLock::new(HashMap::new())),
        views: Arc::new(AtomicUsize::new(0)),
    };

    Router::new()
        .route("/ws/ping", get(game_handler))
        .route("/reset", post(reset))
        .route("/views", get(get_views))
        .route("/ws/room/:room/user/:user", get(chat_handler))
        .with_state(state)
}

async fn game_handler(ws: WebSocketUpgrade, State(state): State<GameState>) -> impl IntoResponse {
    tracing::info!("new client connected");
    ws.on_upgrade(move |socket| handle_game_socket(socket, state))
}

async fn reset(State(state): State<GameState>) {
    state.views.store(0, Ordering::Relaxed);
}

async fn get_views(State(state): State<GameState>) -> impl IntoResponse {
    state.views.load(Ordering::Relaxed).to_string()
}

async fn chat_handler(
    Path((room, user)): Path<(u32, String)>,
    ws: WebSocketUpgrade,
    State(state): State<GameState>,
) -> impl IntoResponse {
    tracing::info!("{user} connected to room {room}");

    if !state.room_channel.read().await.contains_key(&room) {
        tracing::info!("create channel for room {room}");
        let (tx, _rx) = watch::channel(Message::Text("{}".to_string()));
        state.room_channel.write().await.insert(room, tx);
    }

    ws.on_upgrade(move |socket| handle_chat_socket(socket, room, user, state))
}

async fn handle_game_socket(socket: WebSocket, state: GameState) {
    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (sender, mut receiver) = socket.split();
    //state.gamer.as_mut() = Some(sender);
    let sender = Arc::new(RwLock::new(sender));
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if process_game_message(msg, sender.clone(), state.clone())
                .await
                .is_break()
            {
                break;
            }
        } else {
            tracing::info!("client abruptly disconnected");
            break;
        }
    }
    state.start.store(false, Ordering::Relaxed);
    //state.gamer = None;
    // returning from the handler closes the websocket connection
    println!("Websocket context destroyed");
}

async fn handle_chat_socket(socket: WebSocket, room: u32, user: String, state: GameState) {
    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    let mut rx = state
        .room_channel
        .read()
        .await
        .get(&room)
        .unwrap()
        .subscribe();
    tracing::info!("{} joined room {}", user, room);

    // This task will receive watch messages and forward it to this connected client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(()) = rx.changed().await {
            //tracing::info!("new message received sync to {}", send_user);

            let msg = rx.borrow().clone();

            if sender.send(msg).await.is_ok() {
                //send_state.views.fetch_add(1, Ordering::Relaxed);
                //tracing::info!("{}", send_state.views.load(Ordering::Relaxed));
                // tracing::info!(
                //     "views updated: {}",
                //     send_state.views.load(Ordering::Relaxed)
                // );
                //tracing::info!("viewed by {} room {}", send_user, room);
            } else {
                break;
            }
        }
    });

    let recv_user = user.clone();
    // This task will receive messages from this client.
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if process_chat_message(msg, room, recv_user.clone(), state.clone())
                    .await
                    .is_continue()
                {
                    continue;
                }
            }

            tracing::info!("{} left room {}", recv_user, room);
            break;
        }
    });

    //If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // returning from the handler closes the websocket connection
    println!("Websocket context destroyed");
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
async fn process_game_message(
    msg: Message,
    sender: Arc<RwLock<SplitSink<WebSocket, Message>>>,
    state: GameState,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => match t.as_str() {
            "ping" => {
                tracing::info!("client sent ping");
                if state.start.load(Ordering::Relaxed) {
                    tracing::info!("pong");
                    if sender
                        .write()
                        .await
                        .send(Message::Text(String::from("pong")))
                        .await
                        .is_err()
                    {
                        tracing::info!("client abruptly disconnected");
                        return ControlFlow::Break(());
                    }
                } else {
                    tracing::info!(">>> game has not started ");
                }
            }
            "serve" => {
                state.start.store(true, Ordering::Relaxed);
                tracing::info!("game start");
            }
            _ => {
                tracing::info!(">>> client sent str: {t:?}");
                if !state.start.load(Ordering::Relaxed) {
                    state.start.store(false, Ordering::Relaxed);
                    tracing::info!("game over");
                } else {
                    tracing::info!(">>> game has not started ");
                }
            }
        },
        Message::Binary(d) => {
            tracing::info!(">>> client sent {} bytes: {:?}", d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                tracing::info!(
                    ">>> client sent close with code {} and reason `{}`",
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> client somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            tracing::info!(">>> client sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            tracing::info!(">>> client sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
async fn process_chat_message(
    msg: Message,
    room: u32,
    user: String,
    state: GameState,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(text) => {
            let msg = serde_json::from_str::<Value>(&text).unwrap();
            let message = msg.get("message").unwrap().as_str().unwrap();
            if msg.get("user").is_none() {
                // this is a tweet
                //tracing::info!("received new tweet: {} from {}", message, user);
                if message.len() > 128 {
                    tracing::info!("message too long");
                    return ControlFlow::Continue(());
                }

                let broadcast_msg = serde_json::json!({
                    "user": user,
                    "message": message,
                });

                if state
                    .room_channel
                    .write()
                    .await
                    .get(&room)
                    .unwrap()
                    .send(Message::Text(broadcast_msg.to_string()))
                    .is_ok()
                {
                    let count = state
                        .room_channel
                        .write()
                        .await
                        .get(&room)
                        .unwrap()
                        .receiver_count();
                    state.views.fetch_add(count, Ordering::Relaxed);
                    // tracing::info!(
                    //     "message broadcasted to {} users in room {} make the views: {}",
                    //     count,
                    //     room,
                    //     state.views.load(Ordering::Relaxed)
                    // );
                } else {
                    tracing::info!("client disconnected");
                    return ControlFlow::Break(());
                }
            }
            // else {
            //     //this is a broadcast message
            //     tracing::info!(
            //         "received new message from user: {:?}, message{}",
            //         user,
            //         message
            //     );
            // }

            //} else {
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                tracing::info!(
                    ">>> client sent close with code {} and reason `{}`",
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> {} somehow sent close message without CloseFrame", user);
            }
            return ControlFlow::Break(());
        }
        _ => {
            tracing::info!(">>> client sent something else: {:?}", msg);
        }
    }
    ControlFlow::Continue(())
}

use std::time::Duration;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::http::Response;
use axum::response::{Html, IntoResponse};
use axum::Json;
use axum::{extract::State, routing::get, Router};
use sysinfo::{System, MINIMUM_CPU_UPDATE_INTERVAL};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

type Snapshot = Vec<f32>;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<Snapshot>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<Snapshot>(1);
    let app_state = AppState{
        tx: tx.clone()
    };

    let router = Router::new()
        .route("/", get(root_route))
        .route("/index.js", get(indexmjs_route))
        .route("/index.css", get(indexcss_route))
        .route("/api/cpu", get(cpu_usage))
        .route("/realtime/cpu", get(realtime_cpu_usage))
        .with_state(app_state.clone());

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new_all();
        loop {
            sys.refresh_cpu_usage();
            let usage: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let _ = tx.send(usage);

            std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL)
        }
    });

    let listener = TcpListener::bind("0.0.0.0:3123").await.unwrap();
    let addr = listener.local_addr().unwrap();
    println!("Listen on {addr}");

    axum::serve(listener, router).await.unwrap();
}

async fn root_route() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(markup)
}

async fn indexmjs_route() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.mjs").await.unwrap();
    Response::builder()
        .header("Content-Type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

async fn indexcss_route() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.css").await.unwrap();
    Response::builder()
        .header("Content-Type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

async fn cpu_usage(State(state): State<AppState>) -> impl IntoResponse {
    let mut rx = state.tx.subscribe();
    let value = rx.recv().await.unwrap_or(vec![]);
    return Json(value)
}

async fn realtime_cpu_usage(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |ws: WebSocket| async {
        realtime_cpu_stream(state, ws).await
    })
}

async fn realtime_cpu_stream(app_state: AppState, mut ws: WebSocket) {
    let mut rx = app_state.tx.subscribe();
    while let Ok(msg) = rx.recv().await {
        let payload = serde_json::to_string(&msg).unwrap();
        ws.send(Message::Text(payload.into())).await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await
    }
}


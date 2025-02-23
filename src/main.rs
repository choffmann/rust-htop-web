use core::f64;
use std::sync::{Arc, Mutex};
use std::fmt::Write; 

use axum::http::Response;
use axum::response::{Html, IntoResponse};
use axum::Json;
use axum::{extract::State, routing::get, Router};
use sysinfo::System;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_route))
        .route("/index.js", get(indexmjs_route))
        .route("/index.css", get(indexcss_route))
        .route("/api/cpu", get(cpu_usage))
        .route("/api/mem", get(mem_usage))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new_all()))
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
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu_usage();

    let usage: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    Json(usage)
}

async fn mem_usage(State(state): State<AppState>) -> impl IntoResponse {
    let mut s = String::new();
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_memory();

    fn to_gb(v: u64) -> f64 {
        v as f64 / (1024u64.pow(3) as f64)
    }

    let total = to_gb(sys.total_memory());
    let free_mem = to_gb(sys.free_memory());
    let used_mem = to_gb(sys.used_memory());

    writeln!(s, "Total: {} GB", total).unwrap();
    writeln!(s, "Free: {} GB", free_mem).unwrap();
    writeln!(s, "Used: {} GB", used_mem).unwrap();

    s
}

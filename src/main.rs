use std::sync::{Arc, Mutex};

use axum::http::Response;
use axum::response::{Html, IntoResponse};
use axum::Json;
use axum::{extract::State, routing::get, Router};
use sysinfo::{System, MINIMUM_CPU_UPDATE_INTERVAL};
use tokio::net::TcpListener;


type Cpus = Vec<f32>;

#[derive(Default, Clone)]
struct AppState {
    cpus: Arc<Mutex<Cpus>>,
}

#[tokio::main]
async fn main() {
    let app_state = AppState::default();

    let router = Router::new()
        .route("/", get(root_route))
        .route("/index.js", get(indexmjs_route))
        .route("/index.css", get(indexcss_route))
        .route("/api/cpu", get(cpu_usage))
        .with_state(app_state.clone());

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new_all();
        loop {
            sys.refresh_cpu_usage();
            let usage: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let mut cpus = app_state.cpus.lock().unwrap();
            *cpus = usage;
            drop(cpus);

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
    let usage = state.cpus.lock().unwrap().clone();
    Json(usage)
}

//async fn mem_usage(State(state): State<AppState>) -> impl IntoResponse {
//    let mut s = String::new();
//    let mut sys = state.sys.lock().unwrap();
//    sys.refresh_memory();
//
//    fn to_gb(v: u64) -> f64 {
//        v as f64 / (1024u64.pow(3) as f64)
//    }
//
//    let total = to_gb(sys.total_memory());
//    let free_mem = to_gb(sys.free_memory());
//    let used_mem = to_gb(sys.used_memory());
//
//    writeln!(s, "Total: {} GB", total).unwrap();
//    writeln!(s, "Free: {} GB", free_mem).unwrap();
//    writeln!(s, "Used: {} GB", used_mem).unwrap();
//
//    s
//}

use core::f64;
use std::sync::{Arc, Mutex};
use std::fmt::Write; 

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
        .route("/cpu", get(cpu_usage))
        .route("/mem", get(mem_usage))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new_all()))
        });
    let listener = TcpListener::bind("0.0.0.0:3123").await.unwrap();

    let addr = listener.local_addr().unwrap();
    println!("Listen on {addr}");

    axum::serve(listener, router).await.unwrap();
}

async fn root_route() -> &'static str {
    "Hello World!"
}

async fn cpu_usage(State(state): State<AppState>) -> String {
    let mut s = String::new();
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu_usage();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        let i = i + 1;
        writeln!(s, "CPU {i}: {}%", cpu.cpu_usage()).unwrap();
    }

    s
}

async fn mem_usage(State(state): State<AppState>) -> String {
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

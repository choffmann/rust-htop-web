use axum::{routing::get, Router};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root_route));
    let listener = TcpListener::bind("0.0.0.0:3123").await.unwrap();

    let addr = listener.local_addr().unwrap();
    println!("Listen on {addr}");

    axum::serve(listener, router).await.unwrap();
}

async fn root_route() -> &'static str {
    "Hello world!"
}

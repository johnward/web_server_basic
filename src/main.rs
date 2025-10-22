use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new().route("/", get(hello_world));

    // Run it with hyper on localhost:8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

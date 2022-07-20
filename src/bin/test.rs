use axum::{extract::ConnectInfo, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
        format!("Hello {}", addr)
    }

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("server failed");
}

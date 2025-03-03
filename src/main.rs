mod routes;

use reqwest::Client;
use routes::*;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let app = Router::new()
        .route("/proxy/{*wildcard}", get(proxy::handle_proxy))
        .with_state(client);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

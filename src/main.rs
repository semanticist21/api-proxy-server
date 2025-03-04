mod routes;

use reqwest::Client;
use routes::*;

use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let app = Router::new()
        .route("/proxy/{*wildcard}", get(proxy::handle_proxy))
        .with_state(client)
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        );

    let listener = tokio::net::TcpListener::bind("localhost:3333")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

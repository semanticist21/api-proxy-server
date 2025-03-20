mod routes;

use reqwest::Client;
use routes::*;

use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let app = Router::new()
        .route("/proxy/{*wildcard}", get(proxy::handle_proxy))
        .with_state(client)
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5055").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

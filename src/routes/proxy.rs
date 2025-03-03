use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::{HeaderMap, Response},
};
use reqwest::{Client, Method};

async fn handle_proxy(
    method: Method,
    Path(url): Path<String>,
    headers: HeaderMap,
    State(client): State<Client>,
    body: Request<Body>,
) -> Result<Response<Body>, String> {
}

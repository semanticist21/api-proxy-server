use std::collections::HashMap;

use axum::{
    body::{Body, Bytes},
    extract::{Path, Query, State},
    http::{HeaderMap, Response},
    response::IntoResponse,
    Json,
};
use reqwest::{Client, Method};

const INCLUDE_ON_REQUEST_HEADERS: &[&str] = &["Authorization"];

pub async fn handle_proxy(
    method: Method,
    Path(url): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
    State(client): State<Client>,
    body: Bytes,
) -> Result<Response<Body>, String> {
    println!("{:?}", url);

    let mut request_builder = client.request(method, url);

    for (key, value) in headers.iter() {
        let lowered_header_keys = INCLUDE_ON_REQUEST_HEADERS
            .iter()
            .map(|h| h.to_string().to_lowercase())
            .collect::<Vec<String>>();

        let is_bypass_header = lowered_header_keys.contains(&key.as_str().to_lowercase());

        if is_bypass_header {
            request_builder = request_builder.header(key, value);
        }
    }

    let response = request_builder
        .body(body)
        .query(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();

    let json = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    Ok((status, Json(json)).into_response())
}

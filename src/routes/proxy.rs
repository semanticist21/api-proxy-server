use axum::{
    body::Bytes,
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use reqwest::{Client, Method};

const INCLUDE_ON_REQUEST_HEADERS: &[&str] = &["Authorization"];

pub async fn handle_proxy(
    method: Method,
    Path(url): Path<String>,
    headers: HeaderMap,
    State(client): State<Client>,
    body: Bytes,
) -> Result<Json<serde_json::Value>, String> {
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
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(json))
}

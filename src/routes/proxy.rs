use axum::{
    body::{Body, Bytes},
    extract::{Path, State},
    http::{HeaderMap, Response},
};
use flate2::read::GzDecoder;
use reqwest::{Client, Method};
use std::io::Read;

pub async fn handle_proxy(
    method: Method,
    Path(url): Path<String>,
    headers: HeaderMap,
    State(client): State<Client>,
    body: Bytes,
) -> Result<Response<Body>, String> {
    let mut request_builder = client.request(method, url);

    for (key, value) in headers.iter() {
        if key.as_str().to_lowercase() != "host" {
            request_builder = request_builder.header(key, value);
        }
    }

    let response = request_builder
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let remote_headers = response.headers().clone();
    let mut remote_body = response.bytes().await.map_err(|e| e.to_string())?;

    if let Some(content_encoding) = remote_headers.get("content-encoding") {
        if content_encoding.to_str().unwrap() == "gzip" {
            let mut decoder = GzDecoder::new(&remote_body[..]);
            let mut uncompressed = Vec::new();
            decoder
                .read_to_end(&mut uncompressed)
                .map_err(|e| e.to_string())?;
            remote_body = Bytes::from(uncompressed);
        }
    }

    let mut response = Response::new(Body::from(remote_body));
    for (key, value) in remote_headers.iter() {
        response.headers_mut().append(key, value.clone());
    }

    println!("Response: {:?}", response);

    Ok(response)
}

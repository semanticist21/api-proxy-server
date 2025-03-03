use axum::{
    body::Bytes,
    extract::{Path, State},
    http::HeaderMap,
};
use reqwest::{Client, Method};

pub async fn handle_proxy(
    method: Method,
    Path(url): Path<String>,
    headers: HeaderMap,
    State(client): State<Client>,
    body: Bytes,
) -> Result<String, String> {
    let mut request_builder = client.request(method, url);

    for (key, value) in headers.iter() {
        if key.as_str().to_lowercase() != "host" && key.as_str().to_lowercase() != "accept-encoding"
        {
            request_builder = request_builder.header(key, value);
        }
    }

    let response = request_builder
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    println!("{}", text);

    // Ok(response)
    Ok(text)
}

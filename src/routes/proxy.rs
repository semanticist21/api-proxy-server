use std::collections::HashMap;

use axum::{
    body::{Body, Bytes},
    extract::{Path, Query, State},
    http::{HeaderMap, Response},
    response::IntoResponse,
    Json,
};
use reqwest::{Client, Method};

// Headers to forward from client requests to target API
const INCLUDE_ON_REQUEST_HEADERS: &[&str] = &[
    "Authorization",
    "Content-Type", 
    "Content-Disposition", 
    "Content-Length",
    "Accept",
    "X-Requested-With"
];

pub async fn handle_proxy(
    method: Method,
    Path(url): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
    State(client): State<Client>,
    body: Bytes,
) -> Result<Response<Body>, String> {
    println!("{:?}", url);

    // Add a user agent to make sites like Google respond properly
    let mut request_builder = client.request(method.clone(), url.clone())
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36");

    // Forward specified headers from the original request
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

    // Send the request
    let response = request_builder
        .body(body)
        .query(&params)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    // Get status and headers
    let status = response.status();
    let headers = response.headers().clone();
    
    // Check if the response is JSON
    if let Some(content_type) = headers.get("content-type") {
        let content_type = content_type.to_str().unwrap_or("");
        
        if content_type.contains("application/json") {
            // Handle JSON response
            let json = response
                .json::<serde_json::Value>()
                .await
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;
                
            return Ok((status, Json(json)).into_response());
        }
    }
    
    // Handle non-JSON responses (HTML, text, etc.)
    
    // Create response builder and set status
    let mut res = Response::builder().status(status);
    
    // Copy important headers but skip problematic ones that might cause conflicts
    for (key, value) in headers.iter() {
        // Skip headers that might cause issues with chunked transfer
        let header_name = key.as_str().to_lowercase();
        if !header_name.contains("transfer-encoding") && 
           !header_name.contains("content-length") {
            res = res.header(key, value);
        }
    }
    
    // Set content type if it exists
    if let Some(content_type) = headers.get("content-type") {
        res = res.header("content-type", content_type);
    }

    // Get the response bytes
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    
    // Set content-length header based on actual bytes
    res = res.header("content-length", bytes.len().to_string());
        
    res.body(Body::from(bytes))
        .map_err(|e| format!("Failed to create response: {}", e))
}

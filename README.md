# API Proxy Server

A lightweight API proxy server that forwards requests to target services.

## Purpose

Forwards API requests to avoid CORS issues and add authentication layers to third-party APIs.

### Headers

The proxy will automatically forward the `Authorization` header from your requests to the target API. This allows you to maintain authentication while going through the proxy.

To forward additional headers, modify the `INCLUDE_ON_REQUEST_HEADERS` constant in `src/routes/proxy.rs`:

```rust
const INCLUDE_ON_REQUEST_HEADERS: &[&str] = &["Authorization", "Content-Type", "Your-Custom-Header"];
```

## Docker Usage

### Build

```bash
docker build -t api-proxy-server:latest .
```

### Run (foreground)

```bash
docker run -p 5055:5055 api-proxy-server:latest
```

### Run (background/daemon)

```bash
docker run -d -p 5055:5055 api-proxy-server:latest
```

## Example

To proxy a request to JSONPlaceholder:

```
GET http://localhost:5055/proxy/https://jsonplaceholder.typicode.com/todos/1
```

## Development

```bash
cargo run
```

# API Proxy Server

A lightweight API proxy server that forwards requests to target services.

## Purpose

Forwards API requests to avoid CORS issues and add authentication layers to third-party APIs.

### Headers

The proxy forwards essential headers to support various request types:

```rust
const INCLUDE_ON_REQUEST_HEADERS: &[&str] = &[
    "Authorization",
    "Content-Type", 
    "Content-Disposition", 
    "Content-Length",
    "Accept",
    "X-Requested-With"
];
```

### Supported Request Types

- JSON API calls
- HTML/plain text responses
- Multipart form data (file uploads)

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
# Remove existing container if needed
docker rm -f api-proxy-server || true

# Run in background with a container name for easy management
docker run -d --name api-proxy-server -p 5055:5055 api-proxy-server:latest
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

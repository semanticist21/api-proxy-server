FROM arm64v8/rust:1-bullseye AS builder

WORKDIR /app
COPY . ./

RUN cargo build --release

FROM debian:bullseye-slim AS runner

COPY --from=builder /dist/target/release/api-proxy-server /usr/local/bin/api-proxy-server

CMD ["/usr/local/bin/api-proxy-server"]


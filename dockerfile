FROM arm64v8/rust:1-bullseye AS builder

WORKDIR /app
COPY . ./

RUN cargo build --release

FROM debian:bullseye-slim AS runner

COPY --from=builder /app/dist/target/release/sidecar /usr/local/bin/sidecar

CMD ["/usr/local/bin/sidecar"]


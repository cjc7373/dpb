FROM rust:1.84 AS builder
WORKDIR /app
COPY . .
# the /app/target is not available in the second stage, so copy the binary
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp target/release/dpb-rs ./dpb-rs

FROM debian:12-slim
WORKDIR /app
COPY --from=builder /app/dpb-rs ./
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/Rocket.toml ./
RUN mkdir ./data

## ensure the container listens globally on port 8080
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

CMD ["/app/dpb-rs"]

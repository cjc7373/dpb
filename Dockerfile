FROM rust:1.84 AS builder
WORKDIR /app
COPY . .
RUN cargo fetch
RUN cargo build --release

FROM debian:12-slim
COPY --from=builder /app/target/release/dpb-rs /dpb-rs
CMD ["/dpb-rs"]

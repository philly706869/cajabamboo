FROM rust:trixie AS builder
COPY . /app
WORKDIR /app
RUN cargo build --release

FROM debian:trixie-slim
COPY --from=builder /app/target/release/cajabamboo-backend /app
WORKDIR /app
CMD ["./cajabamboo-backend"]

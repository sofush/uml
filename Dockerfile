# Build stage
FROM rust:bullseye AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y upx-ucl && \
    rm -rf /var/lib/apt/lists/*
RUN cargo install wasm-pack

# Build the binary
WORKDIR /app
COPY . .
RUN wasm-pack build --target web --no-typescript --no-pack -d static/wasm uml-wasm
RUN cargo build --release --package uml-server

# Reduce file size
RUN strip target/release/uml-server
RUN upx target/release/uml-server

# Image stage
FROM debian:bullseye-slim
COPY --from=builder /app/target/release/uml-server /uml-server
EXPOSE 8080
ENTRYPOINT ["/uml-server"]

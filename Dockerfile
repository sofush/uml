FROM rust:latest AS rust-builder

# Copy project source code
WORKDIR /app
COPY . .

# Build WASM
RUN cargo install wasm-pack
RUN wasm-pack build --target web --no-typescript --no-pack -d wasm uml-wasm

# Run HTTP server
EXPOSE 8080
CMD ["cargo", "run", "--release"]

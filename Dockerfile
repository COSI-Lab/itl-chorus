
### Build the frontend
FROM rust:latest as frontend
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Requires the wasm target
RUN rustup target add wasm32-unknown-unknown

# Install trunk
RUN cargo install --git https://github.com/thedodd/trunk.git trunk

# Build the frontend
COPY common common
COPY frontend frontend
WORKDIR /frontend

RUN trunk build --release

### Build the backend
FROM rust:latest as backend
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

COPY common common
COPY server server
WORKDIR /server

# TODO: Use cargo-chef https://stackoverflow.com/a/64528456
RUN cargo build --release

### Build the final image
FROM debian:buster-slim

WORKDIR /app

# Copy the frontend and backend
COPY --from=frontend /frontend/dist dist
COPY --from=backend /server/target/release/server server

# Run the backend
CMD ["./server"]

### Build the frontend
FROM rust:latest as frontend
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ARG TRUNK_VERSION=v0.16.0

# Requires the wasm target
RUN rustup target add wasm32-unknown-unknown

# Install trunk
RUN wget -qO- https://github.com/thedodd/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
RUN mv trunk /usr/local/bin

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
FROM rust:1.82.0-slim-bookworm AS builder

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu

RUN cargo install sea-orm-cli

COPY ./ ./migration

RUN cargo build --manifest-path ./migration/Cargo.toml

CMD ["sea-orm-cli", "migrate"]
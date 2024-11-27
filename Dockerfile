# Build stage
FROM rust:1.82.0-slim-bookworm AS builder

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY ./entity/Cargo.toml ./entity/Cargo.lock ./entity/
COPY ./settings/Cargo.toml ./settings/Cargo.lock ./settings/
RUN mkdir -p ./src && echo 'fn main() {}' > ./src/main.rs
RUN mkdir -p ./entity/src && echo '' > ./entity/src/main.rs
RUN mkdir -p ./settings/src && echo '' > ./settings/src/main.rs
RUN cargo build --release

RUN rm -rf ./src
RUN rm -rf ./entity/src
RUN rm -rf ./settings/src
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS run
EXPOSE 3000
COPY --from=builder /usr/src/app/target/release/api /usr/local/bin
CMD ["/usr/local/bin/api"]
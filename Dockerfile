# Build stage
FROM rust:1.82.0-slim-bookworm AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY ./entity/Cargo.toml ./entity/Cargo.lock ./entity/
RUN mkdir ./src && echo 'fn main() {}' > ./src/main.rs
RUN mkdir ./entity/src && echo '' > ./entity/src/main.rs
RUN cargo build --release

RUN rm -rf ./src
RUN rm -rf ./entity/src
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS run
EXPOSE 3000
COPY --from=builder /usr/src/app/target/release/api /usr/local/bin
CMD ["/usr/local/bin/api"]
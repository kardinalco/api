# Build stage
FROM rust:bookworm AS builder
WORKDIR /usr/src/app
# Build dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo 'fn main() {}' > ./src/main.rs
RUN cargo build --release
# Replace with real src
RUN rm -rf ./src
COPY ./src ./src
# break the Cargo cache
RUN touch ./src/main.rs
# Build the project
RUN cargo build --release
# Run stage
FROM debian:bookworm-slim AS run
RUN apt-get update && apt install -y openssl && rm -rf /var/lib/apt/lists/* && apt-get clean
COPY --from=builder /usr/src/app/target/release/api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/api"]
#!/usr/bin/env bash
mkdir "prisma-cli";
mkdir "prisma-cli/src";
touch "prisma-cli/Cargo.toml";
echo '[package]
name = "prisma-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
prisma-client-rust-cli = { git = "https://github.com/Brendonovich/prisma-client-rust", tag = "0.6.11" }' > "prisma-cli/Cargo.toml";


echo 'fn main() {
          prisma_client_rust_cli::run();
      }' > "prisma-cli/src/main.rs";
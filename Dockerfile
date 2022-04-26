# Use the official Rust image.
# https://hub.docker.com/_/rust
# FROM rustlang/rust:nightly AS chef
# USER root
# RUN cargo install cargo-chef
# RUN rustup target add x86_64-unknown-linux-musl
# WORKDIR /usr/src/app
#
# FROM chef as planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json
#
# FROM chef AS builder
# COPY --from=planner /usr/src/app/recipe.json recipe.json
# RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# RUN cargo install --path . --release --target x86_64-unknown-linux-musl --bin server

FROM alpine
WORKDIR /usr/src/app
COPY bins/server .
# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080

# Run the web service on container startup.
CMD ["./server"]

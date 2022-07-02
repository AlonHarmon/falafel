# This dockerfile purpes is to test and publish the libary,
# that happens in dockerfile inorder to not be coupled to a cirtin ci platform

FROM rust:1.62.0

ARG crates_token

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo test

RUN cargo login $crates_token

RUN cargo publish

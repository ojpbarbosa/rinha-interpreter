FROM rust:1.70.0-slim-buster

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release

ENTRYPOINT ["./target/release/rinha"]

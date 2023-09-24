FROM rust:1.70.0-slim-buster AS build

RUN cargo new --bin rinha-interpreter
WORKDIR /rinha-interpreter

COPY Cargo.toml /rinha-interpreter
COPY Cargo.lock /rinha-interpreter
COPY src /rinha-interpreter/src

RUN cargo build --release

FROM debian:bookworm

COPY --from=build /rinha-interpreter/target/release/rinha-interpreter /rinha-interpreter

ENTRYPOINT ["/rinha-interpreter", "/var/rinha/source.rinha.json"]

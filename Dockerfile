FROM rust:1.70.0-slim-buster AS build

RUN cargo new --bin rinha
WORKDIR /rinha

COPY Cargo.toml /rinha/
COPY Cargo.lock /rinha/
COPY src /rinha/src

RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /rinha/target/release/rinha-interpreter /rinha

CMD "/rinha"

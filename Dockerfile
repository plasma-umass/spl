FROM rust:latest
WORKDIR /root

RUN mkdir -p /root/deps/src
COPY spl-lib/Cargo.toml /root/deps/Cargo.toml
COPY spl-lib/Cargo.lock /root/deps/Cargo.lock
RUN touch /root/deps/src/lib.rs
WORKDIR /root/deps
RUN cargo build

WORKDIR /root/spl-lib
COPY spl-lib .
RUN mv /root/deps/target target

RUN cargo test

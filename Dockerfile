FROM ubuntu:18.04
MAINTAINER Arjun Guha <arjun@cs.umass.edu>
WORKDIR /root

RUN apt-get update -y
RUN apt-get install -y curl build-essential libssl-dev pkg-config zlib1g-dev
RUN curl https://sh.rustup.rs > /root/rustup.sh
RUN chmod a+x /root/rustup.sh
RUN /root/rustup.sh -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p /root/deps/src
COPY spl-lib/Cargo.toml /root/deps/Cargo.toml
COPY spl-lib/Cargo.lock /root/deps/Cargo.lock
RUN touch /root/deps/src/lib.rs
RUN cd deps && cargo build

COPY spl-lib /root/spl-lib
WORKDIR /root/spl-lib
RUN mv /root/deps/target target
RUN cargo build
RUN cargo test

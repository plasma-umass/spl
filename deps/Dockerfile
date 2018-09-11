FROM ubuntu:18.04
MAINTAINER Arjun Guha <arjun@cs.umass.edu>
WORKDIR /root
RUN mkdir -p /root/spl-deps/src
COPY Cargo.lock /root/spl-deps/Cargo.lock
COPY Cargo.toml /root/spl-deps/Cargo.toml
RUN apt-get update -y
RUN apt-get install -y curl build-essential libssl-dev pkg-config zlib1g-dev
RUN curl https://sh.rustup.rs > /root/rustup.sh
RUN chmod a+x /root/rustup.sh
RUN /rroot/ustup.sh -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /root
RUN touch /root/spl-deps/src/lib.rs
WORKDIR /root/spl-deps
RUN cargo build


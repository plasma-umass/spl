FROM ubuntu:18.04
MAINTAINER Arjun Guha <arjun@cs.umass.edu>
WORKDIR /root
RUN apt-get update -y
RUN apt-get install -y curl build-essential libssl-dev pkg-config zlib1g-dev
RUN curl https://sh.rustup.rs > rustup.sh
RUN chmod a+x /rustup.sh
RUN /rustup.sh -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY cloud-build-spl-cargo.tgz /root/cloud-build-spl-cargo.tgz
RUN tar xzf cloud-build-spl-cargo.tgz
RUN mkdir -p /root/spl-deps/src
COPY spl-lib/Cargo.lock /root/spl-deps/Cargo.lock
COPY spl-lib/Cargo.toml /root/spl-deps/Cargo.toml
RUN touch /root/spl-deps/src/lib.rs
WORKDIR /root/spl-deps
RUN cargo build


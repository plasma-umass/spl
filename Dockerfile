FROM gcr.io/arjunguha-research-group/spl-deps:latest
MAINTAINER Arjun Guha <arjun@cs.umass.edu>
WORKDIR /root

RUN apt-get update -y
RUN apt-get install -y curl build-essential libssl-dev pkg-config zlib1g-dev
RUN curl https://sh.rustup.rs > /root/rustup.sh
RUN chmod a+x /root/rustup.sh
RUN /root/rustup.sh -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY spl-lib /root/spl-lib
RUN mkdir -p /root/deps/src
COPY spl-lib/Cargo.toml /root/deps/Cargo.toml
COPY spl-lib/Cargo.lock /root/deps/Cargo.lock
RUN touch /root/deps/src/lib.rs
RUN cd deps && cargo build
WORKDIR /root/spl-lib
RUN cargo build
RUN cargo test
RUN tar cz /root/.cargo > /workspace/cloud-build-spl-cargo.tgz
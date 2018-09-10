FROM arjunguha/rust
MAINTAINER Arjun Guha <arjun@cs.umass.edu>
COPY cloud-build-spl-cargo.tgz /root/cloud-build-spl-cargo.tgz
WORKDIR /root
RUN tar xzf cloud-build-spl-cargo.tgz
COPY spl-lib /root/spl-lib
WORKDIR /root/spl-lib
RUN cargo build
RUN cargo test
RUN tar cz /root/.cargo > /root/cloud-build-spl-cargo.tgz
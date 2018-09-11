FROM gcr.io/arjunguha-research-group/spl-deps:latest
MAINTAINER Arjun Guha <arjun@cs.umass.edu>
COPY spl-lib /root/spl-lib
WORKDIR /root/spl-lib
RUN cargo build
RUN cargo test
RUN tar cz /root/.cargo > /root/cloud-build-spl-cargo.tgz
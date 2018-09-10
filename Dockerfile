FROM ubuntu:18.04
MAINTAINER Arjun Guha <arjun@cs.umass.edu>
RUN apt-get update -y
RUN apt-get install -y curl
RUN curl https://sh.rustup.rs > rustup.sh
RUN chmod a+x /rustup.sh
RUN /rustup.sh -y
ENV PATH="/root/.cargo/bin:${PATH}"
COPY spl-lib /root/spl-lib
WORKDIR /root/spl-lib
RUN cargo build
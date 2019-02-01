FROM debian:stretch-slim
WORKDIR /root
LABEL org.label-schema.name="hx" \
      org.label-schema.description="Futuristic take on hexdump, made in rust" \
      org.label-schema.url="https://hub.docker.com/r/sitkevij/hex/" \
      org.label-schema.usage="https://github.com/sitkevij/hex/blob/master/README.md" \
      org.label-schema.vcs-url="https://github.com/sitkevij/hex" \
      org.label-schema.vendor="sitkevij" \
      org.label-schema.version="0.2.1" \
      maintainer="https://github.com/sitkevij"
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    ca-certificates curl build-essential
    # file autoconf automake autotools-dev libtool xutils-dev && \
    # rm -rf /var/lib/apt/lists/*
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
ENV PATH=/root/.cargo/bin:$PATH
COPY . .
RUN cargo build
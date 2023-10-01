FROM rust:latest as builder
WORKDIR /root
ENTRYPOINT  ["hx"]
LABEL org.label-schema.name="hx" \
    org.label-schema.description="Futuristic take on hexdump, made in Rust." \
    org.label-schema.url="https://hub.docker.com/r/sitkevij/hx" \
    org.label-schema.usage="https://github.com/sitkevij/hex/blob/master/README.md" \
    org.label-schema.vcs-url="https://github.com/sitkevij/hex" \
    org.label-schema.vendor="sitkevij" \
    org.label-schema.version="latest" \
    maintainer="https://github.com/sitkevij"
ENV PATH=/root/.cargo/bin:$PATH
COPY . .
RUN ls -lt
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/hx /usr/local/bin/hx
ENTRYPOINT  ["hx"]

FROM rust:alpine
WORKDIR /root
ENTRYPOINT  ["target/release/hx"]
LABEL org.label-schema.name="hx" \
    org.label-schema.description="Futuristic take on hexdump, made in rust" \
    org.label-schema.url="https://hub.docker.com/r/sitkevij/hx" \
    org.label-schema.usage="https://github.com/sitkevij/hex/blob/master/README.md" \
    org.label-schema.vcs-url="https://github.com/sitkevij/hex" \
    org.label-schema.vendor="sitkevij" \
    org.label-schema.version="latest" \
    maintainer="https://github.com/sitkevij"
ENV PATH=/root/.cargo/bin:$PATH
COPY . .
RUN cargo build --release
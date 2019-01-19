FROM sitkevij/alpine-rust:rustup.rs
ENTRYPOINT  ["hx"]
WORKDIR     /tmp/hex
LABEL org.label-schema.name="hx" \
      org.label-schema.description="Futuristic take on hexdump, made in rust" \
      org.label-schema.url="https://hub.docker.com/r/sitkevij/hex/" \
      org.label-schema.usage="https://github.com/sitkevij/hex/blob/master/README.md" \
      org.label-schema.vcs-url="https://github.com/sitkevij/hex" \
      org.label-schema.vendor="sitkevij" \
      org.label-schema.version="0.2.0" \
      maintainer="https://github.com/sitkevij"
# RUN echo "export PATH=$PATH:~/.cargo/bin"  >> ~/.profile && source ~/.profile
COPY . .
# RUN cat ~/.profile
# RUN echo $PATH
# RUN ls ~/.cargo/bin
# https://blog.rust-lang.org/2016/05/13/rustup.html
# rustup target add arm-linux-androideabi
# rustup target add x86_64-unknown-linux-musl
# rustup target add x86_64-unknown-linux-glibc
RUN echo "export PATH=$PATH:~/.cargo/bin"  >> ~/.profile && source ~/.profile && rustup self update && cargo build --target=x86_64-unknown-linux-musl


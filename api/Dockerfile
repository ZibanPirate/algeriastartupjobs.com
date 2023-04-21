FROM rust:1.67
WORKDIR /usr/asj/api
COPY . .
RUN cargo clean
RUN cargo build --release
# FROM ubuntu:18.04
# COPY . /app
# RUN apt-get -y update
# RUN apt-get -y install curl
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"
# RUN cargo --version
# CMD ["pwd && pwd"]

# CMD cargo clean
# CMD cargo build --release
FROM rust:1.67
WORKDIR /usr/asj/api
COPY . .
RUN cargo clean
RUN cargo build --release

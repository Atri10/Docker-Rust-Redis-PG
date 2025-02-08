# Stage 1: Build the Rust application
FROM rust:1.79.0 AS builder

WORKDIR  /usr/src/redis-curd

COPY . .

ENV CARGO_INCREMENTAL=0
ENV CARGO_NET_RETRY=10
ENV RUSTUP_MAX_RETRIES=10

RUN apt-get update && apt-get install -y iputils-ping

RUN apt install -y libpq-dev

RUN cargo install diesel_cli@2.0.1 --no-default-features --features postgres

RUN cargo clean

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/redis-curd"]

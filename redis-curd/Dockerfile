# Stage 1: Build the Rust application
FROM rust:1.76.0-alpine3.19 as builder

# Set the working directory within the container
WORKDIR  /usr/src/redis-curd

# Copy the entire project directory into the container
COPY . .

# Set environment variables
ENV CARGO_INCREMENTAL=0
ENV CARGO_NET_RETRY=10
ENV RUSTUP_MAX_RETRIES=10
ENV RUST_BACKTRACE="short"

# Build the application in release mode
RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/redis-curd"]

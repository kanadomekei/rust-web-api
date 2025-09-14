# syntax=docker/dockerfile:1.6

FROM rust:1.82-slim AS builder
WORKDIR /app

# 1) Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN rustup toolchain install nightly \
 && rustup default nightly \
 && mkdir -p src && echo "fn main(){}" > src/main.rs \
 && cargo build --release \
 && rm -rf src

# 2) Build application
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# 3) Runtime image
FROM debian:bookworm-slim AS runtime
ENV APP_HOME=/app \
    DATABASE_URL=sqlite:///data/app.db \
    RUST_LOG=info
WORKDIR ${APP_HOME}

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-web-api /usr/local/bin/rust-web-api

RUN mkdir -p /data

EXPOSE 8080
CMD ["rust-web-api"]

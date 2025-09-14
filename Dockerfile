# syntax=docker/dockerfile:1.6

FROM rust:1.82-slim AS builder
WORKDIR /app

# 1) Install nightly (edition 2024 requires nightly cargo)
RUN rustup toolchain install nightly && rustup default nightly

# 2) Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs \
 && cargo +nightly build --release \
 && rm -rf src

# 3) Build application
COPY src ./src
COPY migrations ./migrations
RUN cargo +nightly build --release

# 4) Runtime image
FROM debian:bookworm-slim AS runtime
ENV APP_HOME=/app \
    DATABASE_URL=mysql://root:@tidb:4000/test \
    RUST_LOG=info
WORKDIR ${APP_HOME}

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-web-api /usr/local/bin/rust-web-api

RUN mkdir -p /data

EXPOSE 8080
CMD ["rust-web-api"]

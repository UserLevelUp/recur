FROM rust:1.85.0-bullseye AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --profile release-safe --locked

FROM debian:bullseye-slim

COPY --from=builder /app/target/release-safe/recur /usr/local/bin/recur

ENTRYPOINT ["recur"]

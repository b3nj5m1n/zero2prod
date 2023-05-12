FROM rust:1.67 AS chef
# FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 as chef
RUN apt update
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bullseye-slim AS runtime

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV ZERO2PROD_ENV production
ENV EMAIL_CLIENT__API_KEY "abcdef"

ENTRYPOINT ["./zero2prod"]
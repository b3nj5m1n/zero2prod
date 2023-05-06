FROM rust:1.67 AS builder

WORKDIR /app
RUN apt update
RUN cargo install empty-library || :
ENV SQLX_OFFLINE true
COPY . .
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

ENTRYPOINT ["./target/release/zero2prod"]
FROM rust:1.98-alpine AS builder

WORKDIR /app

RUN apk add --no-cache \
    musl-dev \
    build-base

COPY Cargo.toml ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM alpine:3.22

RUN apk add --no-cache \
    ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/lightning-nodes-service /usr/local/bin/lightning-nodes-service
COPY --from=builder /app/migrations ./migrations

ENV APP_HOST=0.0.0.0
ENV APP_PORT=3000

EXPOSE 3000

CMD ["lightning-nodes-service"]
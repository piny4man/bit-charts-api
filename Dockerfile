FROM rust:1.78 AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev \
    libssl-dev \
    ; \
    rm -rf /var/lib/apt/lists/*;

WORKDIR /app

COPY --from=builder /app/target/release/bit-charts-api .
EXPOSE 4500

CMD ["./bit-charts-api"]


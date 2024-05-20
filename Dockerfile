FROM rust:1.78 AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev \
    ; \
    rm -rf /var/lib/apt/lists/*;

COPY --from=builder /app /
EXPOSE 4500
CMD ["./app"]


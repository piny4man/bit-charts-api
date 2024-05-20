FROM rust:1.78 AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/bit-charts-api .
EXPOSE 4500
ENV PORT=4500

CMD ["./bit-charts-api"]


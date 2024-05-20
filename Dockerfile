FROM rust:1.78 AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/app .
EXPOSE 4500
ENV PORT=4500

CMD ["./app"]


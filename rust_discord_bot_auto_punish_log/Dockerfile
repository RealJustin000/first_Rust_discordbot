FROM rust:1.74 as builder

WORKDIR /usr/src/app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/discord_bot .
COPY .env .env
CMD ["./discord_bot"]
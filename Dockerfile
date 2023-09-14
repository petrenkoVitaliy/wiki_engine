FROM rust:bookworm as builder
WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY Rocket.toml Rocket.toml
COPY src ./src

RUN cargo install --path .

FROM debian:bookworm-slim as runner
RUN apt-get update && apt-get install postgresql -y
RUN apt-get update && apt install -y openssl

COPY --from=builder /usr/local/cargo/bin/wiki-engine /usr/local/bin/wiki-engine

EXPOSE 8000

CMD ["wiki-engine"]
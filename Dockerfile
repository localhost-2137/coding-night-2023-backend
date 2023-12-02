FROM rust:1.74-slim-bookworm AS builder
WORKDIR /app
RUN apt update && apt install -y libssl-dev pkg-config
RUN cargo install sqlx-cli --no-default-features --features sqlite

# Build cache
COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN mkdir -p ./src
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > ./src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/coding_night_2023_backend*

COPY . .
ENV DATABASE_URL="sqlite:db.db"
RUN sqlx database create && sqlx migrate run

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/coding-night-2023-backend /app/coding-night-2023-backend
COPY --from=builder /app/db.db /app/default.db
COPY ./docker-entrypoint.sh /app/docker-entrypoint.sh

RUN mkdir -p /app/data
ENV DATABASE_URL="sqlite:/app/data/db.db"

EXPOSE 3000
ENTRYPOINT ["/app/docker-entrypoint.sh"]

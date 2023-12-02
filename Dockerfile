FROM rust:1.74-slim-bookworm
WORKDIR /app
RUN apt update && apt install -y libssl-dev pkg-config
RUN cargo install sqlx-cli --no-default-features --features sqlite

# Build cache
COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN mkdir -p ./src
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > ./src/main.rs
RUN cargo build
#RUN rm -f target/release/deps/coding_night_2023_backend
RUN rm -f target/debug/deps/coding_night_2023_backend

COPY . .
ENV DATABASE_URL="sqlite:db.db"
RUN sqlx database create && sqlx migrate run

EXPOSE 3000
CMD ["cargo", "run"]

FROM rust

WORKDIR /app

RUN cargo install sqlx-cli

COPY . .
RUN cargo build

RUN cargo sqlx database create
RUN cargo sqlx migrate run

EXPOSE 3000
CMD ["cargo", "run"]

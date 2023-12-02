FROM rust

WORKDIR /app

RUN cargo install sqlx-cli

COPY . .

RUN cargo sqlx database create
RUN cargo sqlx migrate run

RUN cargo build

EXPOSE 3000
CMD ["cargo", "run"]

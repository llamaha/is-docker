FROM rust:latest

WORKDIR /
COPY . .

RUN cargo build --release
RUN cargo test

CMD ["cargo", "run", "--example", "example"]

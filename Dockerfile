FROM rust:latest

WORKDIR .

COPY . .

EXPOSE 8000

ENTRYPOINT cargo build --release && cargo run --release

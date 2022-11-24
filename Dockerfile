FROM rust:latest as build

WORKDIR .

COPY . .

FROM debian:11.5-slim

WORKDIR .

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo --help

COPY --from=build . .

EXPOSE 8000

ENTRYPOINT cargo build --release && cargo run --release

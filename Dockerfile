FROM rust:1.65
RUN rustup default nightly-2022-11-06
WORKDIR /app

RUN cargo install --version 1.8.0 just
RUN cargo install --version 2.0.1 diesel_cli --no-default-features --features postgres
COPY . .
RUN cargo build

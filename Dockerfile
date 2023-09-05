FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

RUN apt-get update \
    && apt-get install -y libpq-dev libssl-dev pkg-config

WORKDIR /app

ENV CARGO_INCREMENTAL=0
ENV CARGO_NET_RETRY=2
ENV RUSTUP_MAX_RETRIES=2
ENV RUST_BACKTRACE="short"
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL="sparse"

ARG DB_MODE=diesel
ARG SERVER_MODE=actix
ARG DATABASE_URL

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

COPY . .

ENV DATABASE_URL=${DATABASE_URL}

RUN cargo build --release --features "${DB_MODE},${SERVER_MODE}"

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y ca-certificates libpq-dev curl procps

ARG BIN_DIR=/local/bin
ARG BINARY=dbbench

EXPOSE 8080

RUN mkdir -p ${BIN_DIR}

COPY --from=builder /app/target/release/dbbench ${BIN_DIR}/dbbench

WORKDIR ${BIN_DIR}

CMD ./dbbench

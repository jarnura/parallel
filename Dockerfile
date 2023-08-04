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

ARG MODE=diesel

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build --features ${MODE}

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y ca-certificates libpq-dev curl procps

ARG BIN_DIR=/local/bin
ARG BINARY=dbbench

EXPOSE 8080

RUN mkdir -p ${BIN_DIR}

COPY --from=builder /app/target/debug/dbbench ${BIN_DIR}/dbbench

WORKDIR ${BIN_DIR}

CMD ./dbbench
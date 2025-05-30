FROM rust:slim AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY src ./src

RUN RUSTFLAGS="--cfg tokio_unstable" cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/tokio_signal_handler /usr/src/app/

CMD ["./tokio_signal_handler"]
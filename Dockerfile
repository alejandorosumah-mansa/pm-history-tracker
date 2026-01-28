# Multi-stage Dockerfile for PM History Tracker

# Stage 1: Build
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml .
COPY crates ./crates

# Build dependencies (cached layer)
RUN cargo build --release

# Stage 2: API Server
FROM debian:bookworm-slim as api

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/pm-api /usr/local/bin/pm-api

EXPOSE 3000

CMD ["pm-api"]

# Stage 3: Worker
FROM debian:bookworm-slim as worker

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/pm-worker /usr/local/bin/pm-worker

CMD ["pm-worker"]

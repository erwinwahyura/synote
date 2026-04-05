# Simple Dockerfile - no caching tricks, just build
FROM rust:1.88 AS builder

WORKDIR /app

# Copy and build directly (no dummy stage)
COPY backend/ ./
RUN cargo build --release -j 1

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates wget curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/synote /app/synote

# Create data directory
RUN mkdir -p /app/data/notes

# Copy production config
COPY docker/config.prod.toml /app/config.toml

EXPOSE 8080

CMD ["./synote"]

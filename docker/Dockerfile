FROM rust:1.88 AS builder

WORKDIR /app

# Copy manifests first for caching
COPY backend/Cargo.toml backend/Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Copy source and force full rebuild
COPY backend/src ./src
RUN cargo clean && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates wget curl procps && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/synote /app/synote

# Create data directory
RUN mkdir -p /app/data/notes

# Copy production configuration (binds to 0.0.0.0 for Docker)
COPY docker/config.prod.toml /app/config.toml

EXPOSE 8080

# Run with logging
CMD ["./synote"]

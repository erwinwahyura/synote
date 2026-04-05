FROM rust:1.88 AS builder

WORKDIR /app

# Copy manifests first for caching
COPY backend/Cargo.toml backend/Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Copy source and build
COPY backend/src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/synote /app/synote

# Create data directory
RUN mkdir -p /app/data/notes

# Copy configuration
COPY config.toml /app/config.toml

EXPOSE 8080

CMD ["./synote"]

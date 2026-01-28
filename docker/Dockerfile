FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY backend/Cargo.toml backend/Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {println!(\"dummy\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY backend/src ./src

# Build the actual application
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

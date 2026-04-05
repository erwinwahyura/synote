FROM rust:1.88 as builder

WORKDIR /app

# Copy entire backend
COPY backend/ ./

# Build the application
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

# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/gambas-matcher-mock /usr/local/bin/

# Copy example configuration
COPY examples /app/examples

# Expose the default port
EXPOSE 8080

# Set the default command
CMD ["gambas-matcher-mock", "--config", "/app/config.yaml"]

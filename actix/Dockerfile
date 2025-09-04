# Build stage
FROM rust:1.75-slim as builder

# Install required packages for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd --create-home --shell /bin/bash app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/actix-api /usr/local/bin/actix-api

# Change ownership of the binary
RUN chown app:app /usr/local/bin/actix-api

# Switch to non-root user
USER app

# Set the working directory
WORKDIR /home/app

# Expose the port
EXPOSE 8080

# Set environment variables
ENV PORT=8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:$PORT/ || exit 1

# Run the application
CMD ["actix-api"]
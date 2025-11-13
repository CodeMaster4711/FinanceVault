# Multi-stage build for FinanceVault Backend (with embedded frontend proxy)

###################
# Backend Build Stage
###################
FROM rust:latest AS backend-builder

WORKDIR /usr/src/app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Copy dependencies manifests
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/Cargo.toml
COPY entity/Cargo.toml ./entity/Cargo.toml

# Create a dummy lib.rs to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN mkdir -p migration/src && touch migration/src/lib.rs
RUN mkdir -p entity/src && touch entity/src/lib.rs
RUN cargo build --release

# Copy the rest of the source code
COPY . .

# Rebuild with real source
RUN touch src/main.rs && cargo build --release

###################
# Final Runtime Stage
###################
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Create data directory for SQLite
RUN mkdir -p /data

# Copy backend binary from builder
COPY --from=backend-builder /usr/src/app/target/release/backend /app/backend

# Expose backend port (frontend proxy is handled by backend)
EXPOSE 8000

# Set environment variables
ENV DATABASE_URL=sqlite:/data/finance.db
ENV RUST_LOG=info
ENV FRONTEND_URL=http://localhost:3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD ["/app/backend", "--health-check"] || exit 1

# Start backend (it will proxy to frontend)
CMD ["./backend"]
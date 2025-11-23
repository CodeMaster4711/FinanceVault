# Multi-stage build for FinanceVault Backend (with embedded frontend proxy)

###################
# Frontend Build Stage
###################
FROM node:22-alpine AS frontend-builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ .
RUN npm run build

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

# Copy dependencies manifests from backend directory
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/migration ./migration
COPY backend/entity ./entity

# Create a dummy lib.rs to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the rest of the backend source code
COPY backend/src ./src

# Rebuild with real source
RUN touch src/main.rs && cargo build --release

###################
# Final Runtime Stage
###################
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (Node.js + Rust deps)
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    libssl3 \
    sqlite3 \
    && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Create data directory for SQLite
RUN mkdir -p /data

# Copy frontend build
COPY --from=frontend-builder /app/build /app/frontend/build
COPY --from=frontend-builder /app/package.json /app/frontend/
COPY --from=frontend-builder /app/node_modules /app/frontend/node_modules

# Copy backend binary from builder
COPY --from=backend-builder /usr/src/app/target/release/backend /app/backend

# Copy start script
COPY scripts/start_single_container.sh /app/start.sh
RUN chmod +x /app/start.sh

# Expose backend port (frontend proxy is handled by backend)
EXPOSE 8000

# Set environment variables
ENV DATABASE_URL=sqlite:/data/finance.db
ENV RUST_LOG=info
ENV FRONTEND_URL=http://localhost:3000
ENV FRONTEND_PROXY_URL=http://localhost:3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD ["/app/backend", "--health-check"] || exit 1

# Start both services
CMD ["/app/start.sh"]
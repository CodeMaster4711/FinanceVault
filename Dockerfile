# Multi-stage build for FinanceVault - Backend + Frontend in one image

###################
# Frontend Build Stage
###################
FROM node:20-alpine AS frontend-builder

WORKDIR /frontend

# Copy frontend package files
COPY frontend/package*.json ./
RUN npm ci

# Copy frontend source
COPY frontend/ ./

# Build frontend for production
RUN npm run build

###################
# Backend Build Stage
###################
FROM rust:1.70-slim AS backend-builder

WORKDIR /backend

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY backend/Cargo.toml ./
# Cargo.lock wird automatisch generiert falls nicht vorhanden

# Copy entity and migration crates
COPY backend/entity/ ./entity/
COPY backend/migration/ ./migration/

# Create dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source code
COPY backend/src/ ./src/

# Build the backend
RUN cargo build --release

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
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create directories
RUN mkdir -p /data /app/frontend

# Copy backend binary from builder
COPY --from=backend-builder /backend/target/release/backend /app/backend

# Copy frontend build from builder
COPY --from=frontend-builder /frontend/.svelte-kit/output /app/frontend

# Copy startup script
COPY docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

# Expose ports
EXPOSE 8000 3000

# Set environment variables
ENV DATABASE_URL=sqlite:/data/finance.db
ENV RUST_LOG=info
ENV NODE_ENV=production
ENV PORT=3000
ENV ORIGIN=http://localhost:3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8000/ && curl -f http://localhost:3000/ || exit 1

# Use entrypoint script to start both services
ENTRYPOINT ["/app/docker-entrypoint.sh"]

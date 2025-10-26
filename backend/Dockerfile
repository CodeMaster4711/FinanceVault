FROM rust:latest

WORKDIR /usr/src/app

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

# Compile the application
RUN cargo build --release

# Set the entrypoint to the compiled binary
CMD ["./target/release/backend"]
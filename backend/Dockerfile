# ---------------------------------------------------------
# 1. Builder stage
# ---------------------------------------------------------
FROM rust:1.91 AS builder

# Install SQLx CLI (optional but recommended for migrations)
#RUN cargo install sqlx-cli --no-default-features --features rustls,mysql

WORKDIR /app

# Cache deps
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

# Build release binary
RUN cargo build --release

# ---------------------------------------------------------
# 2. Runtime stage (tiny image)
# ---------------------------------------------------------
FROM debian:bookworm-slim AS runtime

# Install minimal runtime deps (ca-certificates for HTTPS)
RUN apt-get update && apt-get install -y ca-certificates && apt-get clean

WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

# If you want the migrations available at runtime (recommended)
COPY --from=builder /app/migrations ./migrations

# Expose backend port
EXPOSE 8080

# Command to start backend
CMD ["backend"]

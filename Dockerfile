# Multi-stage Dockerfile for Mutsea
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release --bin mutsea-server

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create mutsea user
RUN useradd -m -u 1001 mutsea

# Copy binary
COPY --from=builder /app/target/release/mutsea-server /usr/local/bin/
COPY --from=builder /app/config/mutsea.example.toml /etc/mutsea/mutsea.toml

# Create data directories
RUN mkdir -p /var/lib/mutsea/assets /var/log/mutsea \
    && chown -R mutsea:mutsea /var/lib/mutsea /var/log/mutsea

USER mutsea
WORKDIR /var/lib/mutsea

EXPOSE 8080 9000

CMD ["mutsea-server", "--config", "/etc/mutsea/mutsea.toml"]
EOF

# Docker Compose for development
cat > docker-compose.yml << 'EOF'
version: '3.8'

services:
  mutsea:
    build: .
    ports:
      - "8080:8080"  # HTTP
      - "9000:9000"  # LLUDP
    environment:
      - MUTSEA_DATABASE_URL=postgresql://mutsea:mutsea@postgres/mutsea
      - MUTSEA_REDIS_URL=redis://redis:6379
    depends_on:
      - postgres
      - redis
    volumes:
      - mutsea_data:/var/lib/mutsea
      - mutsea_logs:/var/log/mutsea

  postgres:
    image: postgres:15
    environment:
      - POSTGRES_DB=mutsea
      - POSTGRES_USER=mutsea
      - POSTGRES_PASSWORD=mutsea
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  redis:
    image: redis:7
    volumes:
      - redis_data:/data
    ports:
      - "6379:6379"

volumes:
  mutsea_data:
  mutsea_logs:
  postgres_data:
  redis_data:
# Etapa de build
FROM rust:1.87.0 as builder

WORKDIR /app
COPY . .

RUN cargo build --release && ls -l target/release/

# Etapa final
FROM debian:stable-20250520-slim

WORKDIR /app
COPY --from=builder /app/target/release/failtracker-backend /usr/local/bin/

EXPOSE 3000
CMD ["failtracker-backend"]
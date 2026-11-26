FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY build.rs ./
COPY proto ./proto

# Кэш зависимостей: собираем с заглушкой, пока реальный src не скопирован.
RUN mkdir src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/deps/user_service* target/release/user-service*

COPY src ./src
COPY proto ./proto

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/user-service /usr/local/bin/user-service

CMD ["user-service"]

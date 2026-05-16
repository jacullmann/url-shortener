FROM rust:1.95-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "" > src/lib.rs

RUN cargo build --release

RUN rm -rf src

COPY . .

RUN touch src/main.rs src/lib.rs

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server .

COPY --from=builder /app/db ./db

EXPOSE 3000

CMD ["./server"]

FROM rust:1.75-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ ./src/

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ai-social-media-agent /usr/local/bin/

ENV HOST=0.0.0.0
ENV PORT=3000
ENV RUST_LOG=info

EXPOSE 3000

CMD ["ai-social-media-agent"]
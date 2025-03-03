FROM rust:1.85 AS builder
WORKDIR /usr/src/async-openai
COPY . .
RUN cargo install --path .

FROM debian:12.9-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ollama-chat /usr/local/bin/ollama-chat
CMD ["ollama-chat"]

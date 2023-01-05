FROM rust:1.66 as builder
WORKDIR /usr/src/nucleon
COPY . .
RUN cargo install --path .
 
FROM debian:11-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/nucleon /usr/local/bin/nucleon
CMD ["nucleon", "-bind 0.0.0.0:8000", "-r redis://nucleonredis"]
FROM rust:1.66 as builder
WORKDIR /usr/src/nucleon
COPY . .
RUN cargo install --path .

FROM debian:11-slim
COPY --from=builder /usr/local/cargo/bin/nucleon /usr/local/bin/nucleon
ENV DEFAULT_ARGS "-bind 0.0.0.0:8000 0.0.0.0:8001 --no-redis true"
CMD ["nucleon"]
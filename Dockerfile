FROM rust:latest AS builder
WORKDIR /usr/src/connect2
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/connect2 /usr/local/bin/connect2
CMD ["connect2"]

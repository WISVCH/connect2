FROM rust:latest AS builder

# Install ca-certificates package
RUN apt-get update && apt-get install -y ca-certificates

WORKDIR /usr/src/connect2
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim


# Copy ca-certificates from builder image
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

COPY --from=builder /usr/local/cargo/bin/connect2 /usr/local/bin/connect2
CMD ["connect2"]
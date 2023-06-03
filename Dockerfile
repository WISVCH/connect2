FROM rust:latest

WORKDIR /usr/src/connect2
COPY . .
RUN cargo install --path .

CMD ["connect2"]

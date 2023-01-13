FROM rust:latest as builder
WORKDIR /usr/src/cell
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/cell /usr/local/bin/cell
COPY --from=builder /usr/src/cell/transactions.csv /usr/local/transactions.csv
CMD ["cell", "/usr/local/transactions.csv"]

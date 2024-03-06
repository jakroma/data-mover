FROM rust:1.75 as builder
RUN USER=root cargo new --bin data-mover
WORKDIR /data-mover
COPY ./ ./
RUN cargo build --release

FROM ubuntu:latest
COPY --from=builder /data-mover/target/release/data-mover /usr/local/bin/data-mover

CMD ["data-mover"]
FROM rust:1.75 as builder

RUN USER=root cargo new --bin cli
WORKDIR /data-mover

COPY ./ ./

RUN cargo build --bin data-mover --release

FROM debian:buster-slim
COPY --from=builder /data-mover/target/release/data-mover .

CMD ["/data-mover"]

FROM rust:latest as builder
WORKDIR /root
COPY ./ ./
RUN cargo build -r


FROM ubuntu
COPY --from=builder /root/target/release/id /root/id
CMD "/root/id"

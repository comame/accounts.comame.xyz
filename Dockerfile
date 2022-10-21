FROM ubuntu
RUN apt update -y && apt install -y ca-certificates
COPY ./target/release/id /root/id
COPY ./static ./static
CMD "/root/id"

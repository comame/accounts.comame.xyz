FROM ubuntu
COPY ./target/release/id /root/id
COPY ./static ./static
CMD "/root/id"

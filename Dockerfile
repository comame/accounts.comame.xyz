FROM ubuntu
RUN apt update -y && apt install -y ca-certificates

COPY ./entrypoint /root/entrypoint

COPY ./target/release/id /root/id
COPY ./static /static

COPY ./dashboard/out /root/dashboard

EXPOSE 8080 8081

CMD "/root/entrypoint"

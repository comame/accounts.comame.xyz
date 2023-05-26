FROM ubuntu
RUN apt update -y && apt install -y ca-certificates

COPY ./entrypoint /root/entrypoint

COPY ./target/release/id /root/id
COPY ./static /static

COPY ./dashboard/out /root/dashboard

CMD "/root/entrypoint"

EXPOSE 8080 # Main server
EXPOSE 8081 # Admin dashboard

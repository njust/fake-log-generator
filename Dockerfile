FROM ubuntu:latest
WORKDIR app
COPY target/release/fake-log-generator .
CMD ./fake-log-generator
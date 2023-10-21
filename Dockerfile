FROM --platform=linux/amd64 debian:latest
WORKDIR /
ENTRYPOINT ["./bootstrap.sh"]
FROM ubuntu:latest
WORKDIR /
LABEL authors="kew"

ENTRYPOINT ["./bootstrap.sh"]
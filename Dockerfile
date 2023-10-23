# Rosetta2Go
# https://github.com/Cookie-Cats/meow
# MIT License
# Copyright (c) 2022 Metaphorme <https://github.com/Metaphorme>


FROM rust:latest as builder

WORKDIR /usr/src

RUN git clone https://github.com/Cookie-Cats/meow.git

WORKDIR /usr/src/meow

RUN cargo install --path .


FROM redhat/ubi9-minimal:latest

LABEL org.opencontainers.image.authors="Metaphorme" \
      org.opencontainers.image.documentation="https://github.com/Cookie-Cats/meow" \
      org.opencontainers.image.source="https://github.com/Cookie-Cats/meow"

COPY --from=builder /usr/local/cargo/bin/meow /usr/local/bin/meow

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/meow"]

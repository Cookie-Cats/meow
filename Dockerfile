# Rosetta2Go
# https://github.com/Cookie-Cats/meow
# MIT License
# Copyright (c) 2022 Metaphorme <https://github.com/Metaphorme>


FROM rust:alpine as builder

WORKDIR /usr/src

ARG CARGO_PKG_VERSION

RUN apk --update add git musl-dev \
    && git clone https://github.com/Cookie-Cats/meow.git

WORKDIR /usr/src/meow

RUN cargo install --path .


FROM alpine:latest

LABEL org.opencontainers.image.authors="Metaphorme" \
      org.opencontainers.image.documentation="https://github.com/Cookie-Cats/meow" \
      org.opencontainers.image.source="https://github.com/Cookie-Cats/meow"

COPY --from=builder /usr/local/cargo/bin/meow /usr/local/bin/meow

ENTRYPOINT ["/usr/local/bin/meow"]

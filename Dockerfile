FROM rust:1.82 AS builder

WORKDIR /meter

RUN cargo install cornucopia && cornucopia --version

COPY . .

RUN cargo fetch

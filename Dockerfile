FROM rust:1.82 AS builder

WORKDIR /meter

RUN export APP_PROFILE=test
RUN export RUST_BACKTRACE=1
RUN export RUST_LOG=debug

RUN cargo install cornucopia && cornucopia --version

COPY . .

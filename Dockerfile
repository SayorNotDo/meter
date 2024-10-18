FROM rust:latest as builder

WORKDIR /meter

RUN apt-get update && apt-get install lld clang -y

COPY . .

RUN chmod +x start.sh

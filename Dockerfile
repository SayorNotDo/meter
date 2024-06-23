FROM rust:latest as builder

WORKDIR /meter

COPY . .

RUN chmod +x start.sh
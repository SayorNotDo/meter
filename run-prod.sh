#!/usr/bin/env bash

export $(cat .env | xargs)
cargo build --release && ./target/release/server

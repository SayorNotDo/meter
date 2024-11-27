#!/usr/bin/env bash

export $(cat .env | xargs)
echo "Install Migrate related Tool"
cargo install cornucopia

echo "Start building..."
cargo build --release && ./target/release/server

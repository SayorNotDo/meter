#!/usr/bin/env bash

export $(cat .env | xargs)

cargo install systemfd cornucopia cargo-watch

systemfd --no-pid -s http::3000 -- cargo watch -x run

#!/bin/sh

cd /opt/code

cargo check
cargo build

cargo watch -w src -x run
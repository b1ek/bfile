#!/bin/sh

cd /opt/code

cargo check
cargo build

cargo watch -w src -w templates/source -w static -x run

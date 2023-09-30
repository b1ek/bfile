#!/bin/sh

cargo check
cargo build

cargo watch -w src -w templates -x run
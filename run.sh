#!/bin/sh

set -e
set -x

cargo build --release --bin render_test
RUST_LOG=info ./target/release/render_test
feh out/test.png

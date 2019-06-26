#!/bin/sh

set -e
set -x

cargo build --release --bin rusty_light
RUST_LOG=info ./target/release/rusty_light
feh out/test.png

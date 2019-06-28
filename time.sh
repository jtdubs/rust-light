#!/bin/bash

set -e
set -x

cargo build --release --bin rusty_light
time ./target/release/rusty_light --camera perspective --filter gaussian --fov 60 --output out/bench.png --res 1080p --samples 16

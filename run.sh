#!/bin/sh

set -e
set -x

cargo run --release --bin render_test
feh out/test.png

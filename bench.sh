#!/bin/sh

set -e
set -x

cargo build --release --bin render_test
perf record -ga ./target/release/rusty_light
perf report -i perf.data

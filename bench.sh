#!/bin/sh

set -e
set -x

perf record -ga cargo run --release --bin render_test
perf report -i perf.data

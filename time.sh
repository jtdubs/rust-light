#!/bin/bash

set -e
set -x

time cargo run --release --bin render_test

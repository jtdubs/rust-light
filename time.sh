#!/bin/bash

set -e
set -x

cargo build --release --bin rusty_light
time ./target/release/rusty_light

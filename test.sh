#!/bin/sh

cargo run --bin sampler_test > out/sampler.gp
cargo run --bin filter_test > out/filter.gp
cargo run --bin camera_test > out/camera.gp

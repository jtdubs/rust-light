#!/bin/sh

set -e
set -x

time ./target/release/rusty_light --camera perspective-lens --filter box --focal-distance   2 --fov 60 --lens-radius 0.04 --output out/focus_002_04.png --res 1080p --samples 128
time ./target/release/rusty_light --camera perspective-lens --filter box --focal-distance   5 --fov 60 --lens-radius 0.04 --output out/focus_005_04.png --res 1080p --samples 128
time ./target/release/rusty_light --camera perspective-lens --filter box --focal-distance  10 --fov 60 --lens-radius 0.04 --output out/focus_010_04.png --res 1080p --samples 128
time ./target/release/rusty_light --camera perspective-lens --filter box --focal-distance  20 --fov 60 --lens-radius 0.04 --output out/focus_020_04.png --res 1080p --samples 128
time ./target/release/rusty_light --camera perspective-lens --filter box --focal-distance  40 --fov 60 --lens-radius 0.04 --output out/focus_040_04.png --res 1080p --samples 128
time ./target/release/rusty_light --camera perspective-lens --filter box --focal-distance 100 --fov 60 --lens-radius 0.04 --output out/focus_100_04.png --res 1080p --samples 128

time ./target/release/rusty_light --camera perspective-lens --filter box --focal-distance  10 --fov 60 --lens-radius 0.0  --output out/focus_010_00.png --res 1080p --samples 128
time ./target/release/rusty_light --camera perspective-lens --filter box --focal-distance  10 --fov 60 --lens-radius 0.02 --output out/focus_010_02.png --res 1080p --samples 128
time ./target/release/rusty_light --camera perspective-lens --filter box --focal-distance  10 --fov 60 --lens-radius 0.08 --output out/focus_010_08.png --res 1080p --samples 128
time ./target/release/rusty_light --camera perspective-lens --filter box --focal-distance  10 --fov 60 --lens-radius 0.16 --output out/focus_010_16.png --res 1080p --samples 128

time ./target/release/rusty_light --camera perspective --filter box --fov 60 --output out/perspective.png --res 1080p --samples 128
time ./target/release/rusty_light --camera ortho --filter box --scale 4 --output out/ortho.png --res 1080p --samples 128

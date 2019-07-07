#!/usr/bin/env bash

cargo run > ./test.ppm
convert test.ppm output.jpg &
rm ./test.ppm &
feh ./output.jpg &


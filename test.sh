#!/usr/bin/env bash

cargo run > ./test.ppm
feh ./test.ppm &

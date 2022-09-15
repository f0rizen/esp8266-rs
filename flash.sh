#!/bin/bash

device=/dev/ttyUSB0
if [[ $# == 1 ]]; then
    device=$1
fi
docker run -v "$(pwd):/espflash" --device=$device -it --rm esprs/espflash \
    --release $device

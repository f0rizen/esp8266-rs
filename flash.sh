#!/bin/bash

docker run -v "$(pwd):/espflash" --device=/dev/ttyUSB0 -it --rm esprs/espflash \
    --release /dev/ttyUSB0

#!/bin/bash

arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7m-none-eabi/debug/rs-stm32

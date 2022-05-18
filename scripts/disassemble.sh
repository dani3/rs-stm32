#!/bin/bash

APP_NAME=rs-stm32

cargo objdump --bin $APP_NAME --release -- --disassemble --no-show-raw-insn --print-imm-hex

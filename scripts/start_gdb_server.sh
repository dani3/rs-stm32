#!/bin/bash

pushd $OPENOCD_HOME
openocd -s scripts/ -f interface/stlink.cfg -f target/stm32f1x.cfg

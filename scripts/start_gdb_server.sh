#!/bin/bash

openocd -s $OPENOCD_HOME/scripts/ -f interface/stlink.cfg -f target/stm32f1x.cfg

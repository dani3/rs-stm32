#!/bin/bash

APP_NAME=rs-stm32

cargo size --bin $APP_NAME --release -- -A

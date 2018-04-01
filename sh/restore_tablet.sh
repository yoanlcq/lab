#!/bin/bash

for i in `xsetwacom --list | cut -c 38-39`; do
    #xsetwacom --set $i Area 312 267 59148 33272
    #xsetwacom --set $i Area 400 400 59152 33448
    xsetwacom --set $i Area 441 372 59111 33277
    #xsetwacom --set $i MapToOutput 1920x1080+1366+0
    xsetwacom --set $i MapToOutput 1920x1080+0+0
done

# xrandr --output HDMI-0 --gamma 1.4:1.4:1.45 --brightness 0.90
xrandr --output HDMI-0 --gamma 1:1:1 --brightness 1

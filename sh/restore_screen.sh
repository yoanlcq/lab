#!/bin/bash

xrandr --newmode "1366x768" 60.0  1366 1072 1176 1328  768 771 775 768 -hsync +vsync
xrandr --addmode LVDS1 "1366x768"
xrandr --output LVDS1 --mode "1366x768"


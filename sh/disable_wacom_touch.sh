#!/bin/bash

for (( i=0 ; i<256 ; ++i )); do
    xsetwacom --set $i Touch off > /dev/null
done


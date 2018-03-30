#!/bin/bash

if [[ $# -lt 1 ]]; then
    echo "I need a path to an image."
    exit
fi

function require {
    type $1 >/dev/null 2>&1 || { echo >&2 "I require the $1 utility but it's not installed. Exiting."; exit 1; }
}
function fn {
    gsettings set org.gnome.desktop.background $@
}

require gsettings
require realpath

fn draw-background false
fn picture-uri file://$(realpath $1)
# Run this command to see the possible values :
# gsettings range org.gnome.desktop.background picture-options
fn picture-options centered
fn picture-opacity 100
fn primary-color 0
fn secondary-color 0
fn draw-background true

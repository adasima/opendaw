#!/bin/bash
Xvfb :99 -screen 0 1280x720x24 &
XVFB_PID=$!
sleep 3
export DISPLAY=:99
./target/debug/aura_daw &
DAW_PID=$!
sleep 5
scrot -d 1 screenshot.png
kill $DAW_PID
kill $XVFB_PID

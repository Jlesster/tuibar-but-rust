#!/bin/bash

MONITOR_WIDTH=$(hyprctl monitors -j | jq '.[0].width')
BAR_HEIGHT=30

echo "Monitor width: $MONITOR_WIDTH"
echo "Killing existing jlessbar..."

echo "Launching kitty with jlessbar..."
kitty \
  --class jlessbar \
  -e $(which jlessbar) &

KITTY_PID=$!
echo "Kitty PID: $KITTY_PID"

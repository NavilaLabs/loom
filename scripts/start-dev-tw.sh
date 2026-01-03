#!/bin/bash
set -e

echo "Starting watching tailwind..."

cd /workspaces/loom/interfaces/gui/packages/ui
deno run -A npm:@tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch

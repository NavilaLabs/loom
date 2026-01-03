#!/bin/bash
set -e

echo "Starting development server..."

cd /workspaces/loom/interfaces/gui/packages/web
dx serve --fullstack

#!/bin/bash
set -e

echo "Adding component..."

cd /workspaces/loom/interfaces/gui/packages/ui/src/components/atoms
dx components add "$1"

echo "Component $1 added."
#!/bin/bash
set -e

read -p "Enter the scope of the new module: " scope
read -p "Enter the name of the new migration: " name

sea-orm-cli migrate generate --migration-dir /workspaces/loom/loom-migrations/loom-$scope-migrations --universal-time $name

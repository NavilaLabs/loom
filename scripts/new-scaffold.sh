#!/bin/bash

# the base directory of the project
BASE_DIR=/workspaces/loom/loom-core/src

# get the user defined name
read -p "Enter the name of the database scope: " db_scope
read -p "Enter the name of the new module: " module_name

# create the module directory
mkdir -p "$BASE_DIR/$db_scope/$module_name"

# copy the scaffold files
cp -r /workspaces/loom/scripts/resources/new_scaffold/* "$BASE_DIR/$db_scope/$module_name"

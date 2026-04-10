#!/bin/sh

# Set the secret if not already provided
export APP_AUTHENTICATION_SECRET="${APP_AUTHENTICATION_SECRET:-$(openssl rand -hex 32)}"

# Start the daemons in the background
# echo "Starting Admin Projection Daemon..."
# ./admin-projection-daemon &

# echo "Starting Tenant Projection Daemon..."
# ./tenant-projection-daemon &

# Start the main server in the foreground
# 'exec' makes this process PID 1, which is important for Docker
echo "Starting Main Server..."
exec ./server

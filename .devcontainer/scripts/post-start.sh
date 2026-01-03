#!/bin/bash

chmod +x /workspaces/loom/scripts/*.sh
echo 'export PATH="/workspaces/loom/scripts:$PATH"' >> ~/.bashrc
source ~/.bashrc

curl -fsSL https://deno.land/install.sh | sh && ~/.deno/bin/deno install -g npm:tailwind npm:@tailwindcss/cli

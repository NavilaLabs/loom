serve:
    cd /workspaces/loom/loom-presentation/gui/packages/web && \
    dx serve --fullstack --port 8080 --addr 0.0.0.0

project-admin:
    cargo run -p loom --bin admin-projection-daemon

watch-tw:
    cd /workspaces/loom/loom-presentation/gui/packages/ui && \
    deno run -A npm:@tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch

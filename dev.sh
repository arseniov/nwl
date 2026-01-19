#!/bin/bash
# NWL Development Workflow

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXAMPLE_DIR="$SCRIPT_DIR/examples/demo"

# Cleanup function to stop dev server on exit
cleanup() {
    if [ -n "$NPM_PID" ] && kill -0 "$NPM_PID" 2>/dev/null; then
        echo ""
        echo "Stopping dev server..."
        kill "$NPM_PID" 2>/dev/null || true
        wait "$NPM_PID" 2>/dev/null || true
    fi
    # Also kill any leftover vite processes for this project
    pkill -f "vite.*examples/demo" 2>/dev/null || true
}

# Set up signal handlers
trap cleanup EXIT
trap 'cleanup; exit 130' INT TERM

build_compiler() {
    echo "Building NWL compiler..."
    cargo build --release --manifest-path "$SCRIPT_DIR/Cargo.toml"
    echo "Compiler built successfully!"
}

case "$1" in
  build)
    build_compiler
    ;;
  compile)
    INPUT="${2:-$EXAMPLE_DIR/pages/home.yaml}"
    OUTPUT="${3:-$EXAMPLE_DIR/src/home.tsx}"

    if [ ! -f "$INPUT" ]; then
        echo "Error: Input file not found: $INPUT"
        exit 1
    fi

    echo "Compiling $INPUT -> $OUTPUT"
    "$SCRIPT_DIR/target/release/nwl" compile "$INPUT" -o "$OUTPUT"
    echo "Done!"
    ;;
  build-project)
    build_compiler
    echo ""
    echo "Building project in examples/demo..."
    "$SCRIPT_DIR/target/release/nwl" build "$EXAMPLE_DIR"
    ;;
  dev)
    echo "=== NWL Development Workflow ==="
    echo ""

    if [ ! -f "$SCRIPT_DIR/target/release/nwl" ]; then
        echo "Compiler not found. Building..."
        build_compiler
        echo ""
    fi

    echo "Building project (generates routes from nwl.yaml)..."
    "$SCRIPT_DIR/target/release/nwl" build "$EXAMPLE_DIR"

    echo ""
    echo "Starting Vite dev server..."
    echo "Press Ctrl+C to stop"
    echo ""

    cd "$EXAMPLE_DIR"
    npm run dev -- --host 0.0.0.0 &
    NPM_PID=$!
    wait $NPM_PID
    ;;
  install)
    echo "Installing dependencies for demo project..."
    cd "$EXAMPLE_DIR"
    npm install
    ;;
  watch)
    echo "=== Watching for YAML changes ==="
    echo "Press Ctrl+C to stop"
    echo ""

    if [ ! -f "$SCRIPT_DIR/target/release/nwl" ]; then
        echo "Building compiler first..."
        build_compiler
    fi

    inotifywait -m -e close_write "$EXAMPLE_DIR/pages"/*.yaml "$EXAMPLE_DIR/nwl.yaml" -q |
    while read -r filename; do
        echo "[$(date '+%H:%M:%S')] Changed: $filename"
        if "$SCRIPT_DIR/target/release/nwl" build "$EXAMPLE_DIR" 2>/dev/null; then
            echo "  -> Built successfully"
        else
            echo "  -> Build failed"
        fi
    done
    ;;
  *)
    echo "NWL Development Workflow"
    echo ""
    echo "Usage: ./dev.sh <command>"
    echo ""
    echo "Commands:"
    echo "  build-project - Build compiler and build demo project"
    echo "  build         - Build only the NWL compiler"
    echo "  compile       - Compile single YAML file (args: [input] [output])"
    echo "  dev           - Build project and start dev server"
    echo "  install       - Install npm dependencies for demo"
    echo "  watch         - Watch YAML files and auto-rebuild"
    echo ""
    echo "Demo project: examples/demo/"
    echo "Routes configured in: examples/demo/nwl.yaml"
    echo ""
    echo "Routes:"
    grep "path:" "$EXAMPLE_DIR/nwl.yaml" | sed 's/.*path: */  /'
    echo ""
    ;;
esac

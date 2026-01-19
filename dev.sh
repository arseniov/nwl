#!/bin/bash
# Dev workflow helper for NWL development

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

build_compiler() {
    echo "Building NWL compiler..."
    cargo build --release
    echo "Compiler built successfully!"
}

compile_page() {
    INPUT="${1:-examples/home.yaml}"
    OUTPUT="${2:-src/App.tsx}"

    if [ ! -f "$INPUT" ]; then
        echo "Error: Input file not found: $INPUT"
        exit 1
    fi

    echo "Compiling $INPUT -> $OUTPUT"
    ./target/release/nwl compile "$INPUT" -o "$OUTPUT"
    echo "Done!"
}

case "$1" in
  build)
    build_compiler
    ;;
  compile)
    compile_page "$2" "$3"
    ;;
  dev)
    echo "=== NWL Development Workflow ==="
    echo ""

    if [ ! -f "./target/release/nwl" ]; then
        echo "Compiler not found. Building..."
        build_compiler
        echo ""
    fi

    echo "Compiling page..."
    compile_page "examples/home.yaml" "src/App.tsx"

    echo ""
    echo "Starting Vite dev server..."
    echo "Press Ctrl+C to stop"
    echo ""

    npm run dev -- --host 0.0.0.0
    ;;
  watch)
    echo "=== Watching for YAML changes ==="
    echo "Press Ctrl+C to stop"
    echo ""

    if [ ! -f "./target/release/nwl" ]; then
        echo "Building compiler first..."
        cargo build --release
    fi

    inotifywait -m -e close_write examples/*.yaml pages/*.yaml -q |
    while read -r filename; do
        echo "[$(date '+%H:%M:%S')] Changed: $filename"
        INPUT="$filename"
        OUTPUT="src/App.tsx"

        if ./target/release/nwl compile "$INPUT" -o "$OUTPUT" 2>/dev/null; then
            echo "  -> Compiled successfully"
        else
            echo "  -> Compilation failed"
        fi
    done
    ;;
  build-page)
    build_compiler
    compile_page "$2" "$3"
    ;;
  *)
    echo "NWL Development Workflow"
    echo ""
    echo "Usage: ./dev.sh <command>"
    echo ""
    echo "Commands:"
    echo "  build-page - Build compiler and compile page"
    echo "  build      - Build only the NWL compiler"
    echo "  compile    - Compile YAML to React (args: [input] [output])"
    echo "              Default: examples/home.yaml -> src/App.tsx"
    echo "  dev        - Build, compile, and start dev server"
    echo "  watch      - Watch YAML files and auto-compile"
    echo ""
    ;;
esac

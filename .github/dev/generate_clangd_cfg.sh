#!/bin/bash

set -e
set -u
set -o pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
C_DIR="$REPO_ROOT/src/c"

echo "Generating compile_commands.json using bear..."

# Ensure bear is installed
if ! command -v bear &> /dev/null; then
    echo "Error: 'bear' is not installed. Install it with 'sudo apt install bear' or 'brew install bear'." >&2
    exit 1
fi

(
    cd "$C_DIR"
    bear -- make clean
    bear -- make
)

mv "$C_DIR/compile_commands.json" "$REPO_ROOT/"

echo "compile_commands.json has been placed in the repository root."

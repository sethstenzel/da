#!/usr/bin/env bash
set -euo pipefail

ORIGIN="$PWD"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

trap "cd '$ORIGIN'" EXIT

echo "Building release binary..."
cd "$SCRIPT_DIR/da"
cargo build --release

echo "Binary: $SCRIPT_DIR/da/target/release/da"
echo ""
echo "To install manually (no package manager):"
echo "  cp $SCRIPT_DIR/da/target/release/da ~/.local/bin/"
echo "  da shell-init   # then follow the instructions"

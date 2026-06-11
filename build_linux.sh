#!/usr/bin/env bash
set -euo pipefail

ORIGIN="$PWD"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

trap "cd '$ORIGIN'" EXIT

echo "Building release binary..."
cd "$SCRIPT_DIR/da"
cargo build --release

echo "Binary: $SCRIPT_DIR/da/target/release/da"

if command -v cargo-deb &>/dev/null; then
    echo "Building .deb package..."
    cargo deb
    DEB=$(ls target/debian/*.deb 2>/dev/null | head -1)
    echo "Package: $DEB"
else
    echo ""
    echo "Tip: install cargo-deb to also build a .deb package:"
    echo "  cargo install cargo-deb"
fi

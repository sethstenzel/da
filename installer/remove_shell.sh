#!/usr/bin/env bash
set -euo pipefail

PROFILES=(
    "$HOME/.bashrc"
    "$HOME/.zshrc"
    "$HOME/.bash_profile"
)

for p in "${PROFILES[@]}"; do
    [ -f "$p" ] || continue
    grep -v '^dacd()' "$p" > "$p.da_tmp" && mv "$p.da_tmp" "$p"
    echo "Removed 'dacd' from $p"
done

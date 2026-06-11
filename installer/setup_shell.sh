#!/usr/bin/env bash
set -euo pipefail

FN='dacd() { if [ -z "$1" ]; then echo "Usage: dacd <alias>"; return; fi; local path; path=$(da "$1"); [ $? -eq 0 ] && cd "$path"; }'

PROFILES=()
[ -f "$HOME/.bashrc" ]       && PROFILES+=("$HOME/.bashrc")
[ -f "$HOME/.zshrc" ]        && PROFILES+=("$HOME/.zshrc")
[ -f "$HOME/.bash_profile" ] && PROFILES+=("$HOME/.bash_profile")

if [ ${#PROFILES[@]} -eq 0 ]; then
    echo "No shell profiles found. Add this line manually to ~/.bashrc or ~/.zshrc:"
    echo "  $FN"
    exit 0
fi

for p in "${PROFILES[@]}"; do
    # Remove any existing dacd line before re-adding
    grep -v '^dacd()' "$p" > "$p.da_tmp" && mv "$p.da_tmp" "$p"
    printf '\n%s\n' "$FN" >> "$p"
    echo "Installed 'dacd' in $p"
done

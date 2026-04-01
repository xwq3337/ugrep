#!/bin/bash
set -e

BIN_NAME="ugrep"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

# --- Remove binary ---
if [ -f "$INSTALL_DIR/$BIN_NAME" ]; then
    rm "$INSTALL_DIR/$BIN_NAME"
    info "Removed $INSTALL_DIR/$BIN_NAME"
else
    warn "$INSTALL_DIR/$BIN_NAME not found, skipping."
fi

# --- Clean PATH from shell rc files ---
for RC in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.bash_profile" "$HOME/.profile"; do
    if [ -f "$RC" ] && grep -q "$INSTALL_DIR" "$RC"; then
        sed -i.bak "/export PATH=.*:$INSTALL_DIR/d" "$RC" && rm -f "$RC.bak"
        info "Cleaned PATH entry from $RC"
    fi
done

info "ugrep has been uninstalled."

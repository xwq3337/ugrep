#!/bin/bash
set -e

REPO_URL="https://github.com/xwq3337/ugrep.git"
BIN_NAME="ugrep"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

cleanup() {
    if [ -n "$TMPDIR" ] && [ -d "$TMPDIR" ]; then
        rm -rf "$TMPDIR"
    fi
}
trap cleanup EXIT

# --- Check dependencies ---
command -v git >/dev/null 2>&1   || { error "git is required but not installed."; exit 1; }
command -v cargo >/dev/null 2>&1 || {
    warn "cargo not found, attempting to install Rust via rustup..."
    command -v curl >/dev/null 2>&1 || command -v wget >/dev/null 2>&1 || {
        error "curl or wget is required to install rustup."
        exit 1
    }
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    command -v cargo >/dev/null 2>&1 || { error "cargo still not found after rustup install."; exit 1; }
    info "Rust installed successfully."
}

# --- Clone & build ---
TMPDIR=$(mktemp -d)
info "Cloning $REPO_URL ..."
git clone --depth 1 "$REPO_URL" "$TMPDIR/ugrep"

cd "$TMPDIR/ugrep"
info "Building ugrep (release mode) ..."
cargo build --release

# --- Install ---
mkdir -p "$INSTALL_DIR"
cp "target/release/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

# --- Update PATH if needed ---
if ! echo ":$PATH:" | grep -q ":$INSTALL_DIR:"; then
    SHELL_RC="$HOME/.bashrc"
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_RC="$HOME/.zshrc"
    fi
    echo "" >> "$SHELL_RC"
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_RC"
    warn "$INSTALL_DIR is not in your PATH."
    warn "Added it to $SHELL_RC. Run 'source $SHELL_RC' or open a new terminal."
fi

info "ugrep installed to $INSTALL_DIR/$BIN_NAME"
info "Run 'ugrep --help' to get started."

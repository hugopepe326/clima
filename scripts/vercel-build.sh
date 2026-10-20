#!/usr/bin/env bash
# Build para Vercel: instala toolchain si falta y compila el WASM.
set -euo pipefail
cd "$(dirname "$0")/.."

export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

if ! command -v cargo >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi

rustup target add wasm32-unknown-unknown

if ! command -v trunk >/dev/null 2>&1; then
  TRUNK_VERSION="0.22.0"
  ARCH="$(uname -m)"
  case "$ARCH" in
    x86_64) TARGET="x86_64-unknown-linux-musl" ;;
    aarch64|arm64) TARGET="aarch64-unknown-linux-musl" ;;
    *) echo "Arquitectura no soportada: $ARCH" >&2; exit 1 ;;
  esac
  mkdir -p "$HOME/.local/bin"
  curl -sSfL "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-${TARGET}.tar.gz" \
    | tar -xz -C "$HOME/.local/bin" trunkfi

trunk build --release
rm -f dist/.trunk-lock

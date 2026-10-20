#!/usr/bin/env bash
# Build reproducible de FedeWeather.
# Uso:
#   ./build.sh                                  -> dist/ con public_url "/"
#   TRUNK_BUILD_PUBLIC_URL=/repo/ ./build.sh    -> dist/ para subdirectorio
set -euo pipefail

cd "$(dirname "$0")"

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: Rust no está instalado. Instalá rustup desde https://rustup.rs" >&2
  exit 1
fi

if ! rustup target list --installed | grep -qx wasm32-unknown-unknown; then
  rustup target add wasm32-unknown-unknown
fi

if ! command -v trunk >/dev/null 2>&1; then
  echo "error: Trunk no está instalado. Ejecutá: cargo install trunk --locked" >&2
  exit 1
fi

echo "→ Compilando FedeWeather (release)…"
trunk build --release
rm -f dist/.trunk-lock

echo "✓ Listo: dist/"

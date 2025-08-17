#!/usr/bin/env bash
set -euo pipefail

# Install Rust if missing
if ! command -v cargo >/dev/null 2>&1; then
  echo "Installing Rust via rustup..."
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source "$HOME/.cargo/env"
fi

# Install protoc if missing
if ! command -v protoc >/dev/null 2>&1; then
  echo "Installing protobuf compiler (protoc)..."
  sudo apt-get update && sudo apt-get install -y protobuf-compiler
fi

echo "Setup complete."

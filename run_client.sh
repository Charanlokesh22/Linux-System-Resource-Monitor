#!/usr/bin/env bash
set -euo pipefail
cargo run --release -- client --addr http://127.0.0.1:50051 --stream

#!/usr/bin/env bash
set -euo pipefail
RUST_LOG=info CPU_THRESHOLD_PCT=${CPU_THRESHOLD_PCT:-90} MEM_THRESHOLD_PCT=${MEM_THRESHOLD_PCT:-90} DISK_THRESHOLD_PCT=${DISK_THRESHOLD_PCT:-90} \
cargo run --release -- server --addr 0.0.0.0:50051 --interval_ms 1000 --top_n 5

#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
cd "$d/source"
export CARGO_TARGET_DIR="$root/lab/target/watch-pdf-byte-retention-2026-11-22"
export CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
TZ=America/Sao_Paulo date -Iseconds > ../build-start.txt
cargo build --locked --offline -j 2 -p typst-wiring --bin typst --release > ../build.log 2>&1
cp "$CARGO_TARGET_DIR/release/typst" ../candidate.elf
TZ=America/Sao_Paulo date -Iseconds > ../build-end.txt

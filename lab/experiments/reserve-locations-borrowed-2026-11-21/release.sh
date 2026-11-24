#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
out="$(git -C "$d" rev-parse --show-toplevel)/lab/results/construction/diagnostics/reserve-locations-borrowed-2026-11-21"
ram=/dev/shm/tekt-context-preserve-20261121
cd "$ram/source"
ulimit -v 12582912
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
date -Iseconds > "$out/build-start.txt"
rustc -Vv > "$out/rustc.txt"
(find .cargo 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring -type f -print0 | sort -z | xargs -0 sha256sum) > "$out/candidate-source.sha256"
cargo build --locked --offline -j 1 -p typst-wiring --bin typst --release
cp "$ram/target/release/typst" "$d/candidate.elf"
sha256sum "$d/candidate.elf" > "$out/candidate-elf.sha256"
date -Iseconds > "$out/build-end.txt"

#!/usr/bin/env bash
set -euo pipefail
dir=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$dir" rev-parse --show-toplevel)
out="$root/lab/results/construction/diagnostics/references-500-pre-bidi-emission-2026-11-21"
mkdir -p "$out"
ram=$(mktemp -d /dev/shm/tekt-pre-bidi-emission.XXXXXXXX)
printf '%s\n' "$ram" > "$out/ram-path.txt"
git -C "$root" rev-parse HEAD > "$out/head.txt"
git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-before.sha256"
git -C "$root" diff HEAD --stat > "$out/diffstat-before.txt"
date -Iseconds > "$out/build-start.txt"
mkdir -p "$ram/src/lab" "$ram/tmp"
cd "$root"
rsync -a .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring "$ram/src/"
rsync -a lab/benches "$ram/src/lab/"
python3 -B "$dir/census.py" "$ram/src"
cd "$ram/src"
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0
export TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
cargo build --locked --offline -j 8 -p typst-wiring --bin typst --release > "$out/build.log" 2>&1
cp "$ram/target/release/typst" "$ram/census"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
sha256sum "$ram/census" > "$out/census-elf.sha256"
date -Iseconds > "$out/build-end.txt"
cd "$root"
date -Iseconds > "$out/census-start.txt"
python3 -B "$dir/run-census.py" > "$out/census.log" 2>&1
date -Iseconds > "$out/census-end.txt"
git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"

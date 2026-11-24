#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
out="$root/lab/results/construction/diagnostics/$(basename "$d")"
ram=$(mktemp -d /dev/shm/tekt-context-unit.XXXXXXXX)
trap 'rm -rf -- "$ram"' EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/unit-mount.txt"
date -Iseconds > "$out/unit-start.txt"
mkdir -p "$ram/src/lab" "$ram/tmp"
cd "$root"
rsync -a .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring "$ram/src/"
rsync -a lab/benches "$ram/src/lab/"
python3 -B "$root/lab/experiments/references-500-visible-textual-style-collapse-2026-11-21/prototype.py" "$ram/src"
cd "$ram/src"
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
cargo build --locked --offline -j 12 -p typst-core > "$out/unit-compile.log" 2>&1
for arm in atomized retained; do
    mkdir "$ram/$arm"
    tar -xzf "$out/$arm-source.tar.gz" -C "$ram/$arm"
    dir="$ram/$arm/03_infra/src"
    {
        printf '%s\n' 'use typst_core::entities::{content::Content, style_chain::StyleChain, value::Value};'
        sed -n '/^fn figure_context_supplement/,/^}/p' "$dir/pipeline.rs"
        printf '%s\n' '#[path = "pipeline/substitution/mod.rs"] mod substitution;'
    } > "$dir/harness.rs"
    rustc --test --edition 2024 -L "dependency=$ram/target/debug/deps" \
        --extern "typst_core=$ram/target/debug/libtypst_core.rlib" \
        "$dir/harness.rs" -o "$ram/test-$arm"
    printf '%s\n' "$arm"
    "$ram/test-$arm" substitution_preserves_only_unchanged_owners
done | tee "$out/unit.log"
date -Iseconds > "$out/unit-end.txt"

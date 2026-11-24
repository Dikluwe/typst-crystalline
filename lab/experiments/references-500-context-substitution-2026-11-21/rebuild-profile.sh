#!/usr/bin/env bash
# Recreate the symbol table of the retained ELF and decode the archived samples.
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
out="$root/lab/results/construction/diagnostics/$(basename "$d")"
prior="$root/lab/experiments/references-500-visible-textual-style-collapse-2026-11-21"
# The prior build used this disposable absolute path.
ram=/dev/shm/tekt-context-substitution.SUvxx9kr
mkdir -- "$ram"
trap 'rm -rf -- "$ram"' EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/breakdown-mount.txt"
date -Iseconds > "$out/breakdown-retry-start.txt"
mkdir -p "$ram/src/lab" "$ram/tmp"
cd "$root"
rsync -a .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring "$ram/src/"
rsync -a lab/benches "$ram/src/lab/"
python3 -B "$prior/prototype.py" "$ram/src"
cd "$ram/src"
find .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring -type f -print0 |
    sort -z | xargs -0 sha256sum | cmp "$out/source-base.sha256" -
tar -xzf "$out/retained-source.tar.gz"
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
cargo build --locked --offline -j 8 -p typst-wiring --bin typst --release
elf="$ram/target/release/typst"
actual=$(sha256sum "$elf" | cut -d ' ' -f 1)
expected=$(awk '$2=="retained" {print $1}' "$out/binaries.sha256")
printf 'retained measured %s\nretained rebuilt %s\n' "$expected" "$actual" >> "$out/breakdown-binary.txt"
nm -n -S -C --defined-only "$elf" > "$ram/retained.symbols"
if test "$expected" = "$actual"; then
    python3 -B "$d/breakdown.py" "$out" "$ram/retained.symbols"
else
    cp "$elf" "$ram/retained"
    cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
    cp "$root/lab/experiments/unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
    export TEKT_LAB_WORK="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/retained" TEKT_CONTEXT_OUT="$out"
    export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
    python3 -B "$d/breakdown.py" "$out" "$ram/retained.symbols" fresh
fi
date -Iseconds > "$out/breakdown-end.txt"

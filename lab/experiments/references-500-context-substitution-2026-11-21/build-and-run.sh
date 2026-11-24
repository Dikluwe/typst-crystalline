#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
name=$(basename "$d")
out="$root/lab/results/construction/diagnostics/$name"
prior="$root/lab/experiments/references-500-visible-textual-style-collapse-2026-11-21"
ram=$(mktemp -d /dev/shm/tekt-context-substitution.XXXXXXXX)
finish() {
    local status=$?
    for file in warm.log compile.log measurements.jsonl directed.json phases.json unit.log; do
        if test -f "$ram/$file"; then cp "$ram/$file" "$out/"; fi
    done
    if test -d "$ram/cases"; then
        (cd "$ram" && find cases -type f \( -name watch.log -o -name maps -o -name samples.bin \) -print0 |
            tar --null -T - -czf "$out/cases.tar.gz")
    fi
    date -Iseconds > "$out/end.txt"
    git -C "$root" diff HEAD --stat > "$out/diffstat-after.txt"
    printf '%s\n' "$status" > "$out/exit.txt"
    rm -rf -- "$ram"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/mount.txt"
git -C "$root" rev-parse HEAD > "$out/head.txt"
git -C "$root" diff HEAD --stat > "$out/diffstat-before.txt"
cd "$root"
date -Iseconds > "$out/start.txt"
: > "$out/build-times.txt"
mkdir -p "$ram/src/lab" "$ram/tmp"
rsync -a .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring "$ram/src/"
rsync -a lab/benches "$ram/src/lab/"
python3 -B "$prior/prototype.py" "$ram/src"
cp "$ram/src/03_infra/src/pipeline.rs" "$ram/pipeline.orig"
product=(.cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring)
(cd "$ram/src" && find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum) > "$out/source-base.sha256"
cmp "$out/source-base.sha256" "$prior/candidate-input-manifest.sha256"
cp "$prior/candidate.elf" "$ram/control"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
cp "$root/lab/experiments/unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram/tmp" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/control"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
cd "$ram/src"
export CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
rustc -Vv > "$out/rustc.txt"
# Start compiling the common dependencies while the laboratory recipes are prepared.
cargo build --locked --offline -j 8 -p typst-core --release 2>&1 | tee "$ram/warm.log"
for arm in atomized retained; do
    cp "$ram/pipeline.orig" "$ram/src/03_infra/src/pipeline.rs"
    python3 "$d/prototype.py" "$ram/src" "$arm"
    find 03_infra/src/pipeline/substitution -type f -print0 | sort -z | xargs -0 sha256sum > "$out/$arm-sources.sha256"
    tar -czf "$out/$arm-source.tar.gz" 03_infra/src/pipeline.rs 03_infra/src/pipeline/substitution
    printf '%s %s\n' "$arm" "$(date -Iseconds)" >> "$out/build-times.txt"
    cargo build --locked --offline -j 8 -p typst-wiring --bin typst --release 2>&1 | tee -a "$ram/compile.log"
    cp "$ram/target/release/typst" "$ram/$arm"
    printf '%s %s\n' "$arm" "$(date -Iseconds)" >> "$out/build-times.txt"
done
(cd "$ram" && sha256sum control atomized retained vanilla profiler.so) > "$out/binaries.sha256"
for arm in control atomized retained; do nm -n -S -C --defined-only "$ram/$arm" > "$ram/$arm.symbols"; done
export TEKT_CONTEXT_OUT="$out"
date -Iseconds > "$out/bench-start.txt"
python3 "$d/bench.py"
date -Iseconds > "$out/bench-end.txt"
cargo test --locked --offline -j 8 -p typst-infra --lib --release substitution_preserves_only_unchanged_owners 2>&1 | tee "$ram/unit.log"

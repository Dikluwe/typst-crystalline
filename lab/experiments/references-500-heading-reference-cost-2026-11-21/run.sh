#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
results="$root/lab/results/construction/diagnostics"
out="$results/$(basename "$d")"
parent="$results/references-500-pre-bidi-page-reuse-2026-11-21"
old="$results/references-500-context-substitution-2026-11-21"
links="$results/references-500-link-redundant-vectors-2026-11-21"
proof_dir="$root/lab/experiments/references-500-heading-reference-proof-2026-11-21"
expected_proof="$results/$(basename "$proof_dir")"
# Same absolute path as the recorded ELF, in this sandbox's tmpfs.
ram=/dev/shm/tekt-context-substitution.SUvxx9kr
mkdir -- "$ram"
finish() {
    local status=$?
    date -Iseconds > "$out/end.txt"
    printf '%s\n' "$status" > "$out/exit.txt"
    git -C "$root" diff HEAD --stat > "$out/diffstat-after.txt"
    git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
    for file in measurements.jsonl attribution.json native.json; do
        if test -f "$ram/$file"; then cp "$ram/$file" "$out/"; fi
    done
    if test "$status" != 0 && test -f "$ram/candidate"; then
        recovery=$(mktemp -d /tmp/tekt-heading-reference-cost-recovery.XXXXXXXX)
        cp "$ram/candidate" "$recovery/"
        for extra in proof bridge; do
            if test -f "$ram/$extra"; then cp "$ram/$extra" "$recovery/"; fi
        done
        printf '%s\n' "$recovery" > "$out/recovery.txt"
    fi
    rm -rf -- "$ram"
}
trap finish EXIT
git -C "$root" rev-parse HEAD > "$out/head.txt"
git -C "$root" diff HEAD --stat > "$out/diffstat-before.txt"
git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-before.sha256"
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/mount.txt"
date -Iseconds > "$out/build-start.txt"
mkdir -p "$ram/src/lab" "$ram/tmp"
cd "$root"
rsync -a .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring "$ram/src/"
rsync -a lab/benches "$ram/src/lab/"
python3 -B "$root/lab/experiments/references-500-visible-textual-style-collapse-2026-11-21/prototype.py" "$ram/src"
cd "$ram/src"
product=(.cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring)
find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum | cmp "$old/source-base.sha256" -
tar -xzf "$old/retained-source.tar.gz"
cp "$links/link_capture.candidate.rs" 01_core/src/compiler/layout/link_capture.rs
tar -xzf "$parent/candidate-source.tar.gz"
find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum | cmp "$parent/candidate-input.sha256" -
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
rustc -Vv > "$out/rustc.txt"
if test -n "${1:-}"; then
    cp "$1/candidate" "$ram/candidate"
else
    cargo build --locked --offline -j 8 -p typst-wiring --bin typst --release
    cp "$ram/target/release/typst" "$ram/candidate"
fi
date -Iseconds > "$out/build-end.txt"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
for arm in candidate vanilla; do
    expected=$(awk -v arm="$arm" '$2==arm {print $1}' "$parent/binaries.sha256")
    printf '%s  %s\n' "$expected" "$ram/$arm" | sha256sum --check
done
(cd "$ram" && sha256sum candidate vanilla) > "$out/binaries.sha256"
date -Iseconds > "$out/variant-build-start.txt"
python3 -B "$proof_dir/prototype.py" "$ram/src"
cargo build --locked --offline -j 8 -p typst-wiring --bin typst --release
cp "$ram/target/release/typst" "$ram/proof"
date -Iseconds > "$out/variant-build-end.txt"
(cd "$ram" && sha256sum -c "$expected_proof/binaries.sha256")
find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum | cmp "$expected_proof/proof-input.sha256" -
date -Iseconds > "$out/bridge-build-start.txt"
python3 -B "$d/bridge.py" "$ram/src"
cargo build --locked --offline -j 8 -p typst-wiring --bin typst --release
cp "$ram/target/release/typst" "$ram/bridge"
date -Iseconds > "$out/bridge-build-end.txt"
find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/bridge-input.sha256"
(cd "$ram" && sha256sum candidate bridge proof vanilla) > "$out/binaries.sha256"
for arm in candidate bridge proof; do nm -n -S -C --defined-only "$ram/$arm" > "$ram/$arm.symbols"; done
cp "$root/lab/experiments/unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
cc -O2 -Wall -Wextra -shared -fPIC -o "$ram/trace-clock.so" "$root/lab/experiments/references-500-pdf-opening-2026-11-20/trace-clock.c" -ldl
export TEKT_LAB_WORK="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_CONTEXT_OUT="$out" TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$results/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/bench-start.txt"
python3 -B "$d/bench.py"
date -Iseconds > "$out/bench-end.txt"
cmp "$out/diffstat-before.txt" <(git -C "$root" diff HEAD --stat)

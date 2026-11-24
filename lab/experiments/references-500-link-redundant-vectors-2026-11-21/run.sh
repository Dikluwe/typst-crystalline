#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
out="$root/lab/results/construction/diagnostics/$(basename "$d")"
old="$root/lab/results/construction/diagnostics/references-500-context-substitution-2026-11-21"
prior="$root/lab/experiments/references-500-visible-textual-style-collapse-2026-11-21"
ram=/dev/shm/tekt-context-substitution.SUvxx9kr
mkdir -- "$ram"
finish() {
    local status=$?
    for file in measurements.jsonl directed.json timing.json census.json focused.log; do
        if test -f "$ram/$file"; then cp "$ram/$file" "$out/"; fi
    done
    date -Iseconds > "$out/end.txt"
    git -C "$root" diff HEAD --stat > "$out/diffstat-after.txt"
    git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
    printf '%s\n' "$status" > "$out/exit.txt"
    if test "$status" != 0; then
        recovery=$(mktemp -d /tmp/tekt-link-vectors-recovery.XXXXXXXX)
        for arm in control candidate census; do
            if test -f "$ram/$arm"; then cp "$ram/$arm" "$recovery/"; fi
        done
        printf '%s\n' "$recovery" > "$out/recovery.txt"
        if test -d "$ram/directed"; then
            (cd "$ram" && find directed -name '*.log' -type f -print0 |
                tar --null -T - -czf "$out/directed-failure-logs.tar.gz")
        fi
    fi
    rm -rf -- "$ram"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/mount.txt"
git -C "$root" rev-parse HEAD > "$out/head.txt"
git -C "$root" diff HEAD --stat > "$out/diffstat-before.txt"
git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-before.sha256"
date -Iseconds > "$out/start.txt"
mkdir -p "$ram/src/lab" "$ram/tmp"
cd "$root"
rsync -a .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring "$ram/src/"
rsync -a lab/benches "$ram/src/lab/"
python3 -B "$prior/prototype.py" "$ram/src"
cd "$ram/src"
find .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring -type f -print0 |
    sort -z | xargs -0 sha256sum | cmp "$old/source-base.sha256" -
tar -xzf "$old/retained-source.tar.gz"
component=01_core/src/compiler/layout/link_capture.rs
cp "$component" "$ram/link_capture.control.rs"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
export TEKT_LAB_WORK="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/control" TEKT_CONTEXT_OUT="$out"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
rustc -Vv > "$out/rustc.txt"
build() {
    printf '%s begin %s\n' "$1" "$(date -Iseconds)" | tee -a "$out/build-times.txt"
    cargo build --locked --offline -j 8 -p typst-wiring --bin typst --release
    cp "$ram/target/release/typst" "$ram/$1"
    printf '%s end %s\n' "$1" "$(date -Iseconds)" | tee -a "$out/build-times.txt"
}
: > "$out/build-times.txt"
build control
for arm in control vanilla; do
    expected=$(awk -v arm="$arm" '$2==(arm=="control"?"retained":arm) {print $1}' "$old/binaries.sha256")
    test "$(sha256sum "$ram/$arm" | cut -d ' ' -f 1)" = "$expected"
done
python3 -B "$d/prototype.py" "$component" candidate
cp "$component" "$out/link_capture.candidate.rs"
diff -u --label control --label candidate "$ram/link_capture.control.rs" "$component" > "$out/candidate.diff" || test "$?" = 1
build candidate
(cd "$ram" && sha256sum control candidate vanilla) > "$out/binaries.sha256"
date -Iseconds > "$out/bench-start.txt"
python3 -B "$d/bench.py"
date -Iseconds > "$out/bench-end.txt"
cp "$ram/link_capture.control.rs" "$component"
python3 -B "$d/prototype.py" "$component" census
cp "$component" "$out/link_capture.census.rs"
build census
(cd "$ram" && sha256sum census) > "$out/census-binary.sha256"
python3 -B "$d/census.py"
cmp "$out/diffstat-before.txt" <(git -C "$root" diff HEAD --stat)

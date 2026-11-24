#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
out="$root/lab/results/construction/diagnostics/$(basename "$d")"
binaries=${1:?directory of previously built atomized and retained ELFs}
ram=$(mktemp -d /dev/shm/tekt-context-substitution-replay.XXXXXXXX)
finish() {
    local status=$?
    for file in directed.json measurements.jsonl phases.json; do
        if test -f "$ram/$file"; then cp "$ram/$file" "$out/"; fi
    done
    if test -d "$ram/cases"; then
        (cd "$ram" && find cases -type f \( -name watch.log -o -name maps -o -name samples.bin \) -print0 |
            tar --null -T - -czf "$out/cases.tar.gz")
    fi
    git -C "$root" diff HEAD --stat > "$out/diffstat-after.txt"
    printf '%s\n' "$status" > "$out/replay-exit.txt"
    date -Iseconds > "$out/replay-end.txt"
    rm -rf -- "$ram"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/replay-mount.txt"
cp "$binaries/atomized" "$binaries/retained" "$ram/"
cp "$root/lab/experiments/references-500-visible-textual-style-collapse-2026-11-21/candidate.elf" "$ram/control"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
cp "$root/lab/experiments/unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
(cd "$ram" && sha256sum --check "$out/binaries.sha256")
for arm in control atomized retained; do nm -n -S -C --defined-only "$ram/$arm" > "$ram/$arm.symbols"; done
mkdir -p "$ram/tmp" "$ram/src/lab"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram/tmp" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/control"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
export TEKT_CONTEXT_OUT="$out"
date -Iseconds > "$out/replay-start.txt"
python3 "$d/bench.py"

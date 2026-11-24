#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
results="$root/lab/results/construction/diagnostics"
out="$results/$(basename "$d")"
ram=$(mktemp -d /dev/shm/tekt-visible-scopes-verify.XXXXXXXX)
stage=verification
if test "${2:-}" = capture; then stage=capture; fi
cleanup() {
    local status=$?
    date -Iseconds > "$out/$stage-end.txt"
    printf '%s\n' "$status" > "$out/$stage-exit.txt"
    rm -rf -- "$ram"
}
trap cleanup EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/$stage-mount.txt"
date -Iseconds > "$out/$stage-start.txt"
cp "$1/candidate" "$ram/candidate"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum --check "$out/binaries.sha256")
cc -O2 -Wall -Wextra -shared -fPIC -o "$ram/trace-clock.so" "$root/lab/experiments/references-500-pdf-opening-2026-11-20/trace-clock.c" -ldl
export TEKT_LAB_WORK="$ram" TEKT_CONTEXT_OUT="$out" TEKT_EXE="$ram/candidate" PYTHONDONTWRITEBYTECODE=1
export TEKT_VISIBLE_GOLD="$results/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
if test "$stage" = capture; then
    python3 -B "$d/capture.py"
else
    python3 -B "$d/bench.py" --verify-only
fi
cmp "$out/diffstat-before.txt" <(git -C "$root" diff HEAD --stat)

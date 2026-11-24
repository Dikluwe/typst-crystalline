#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-page-operator-cost-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-vanilla-scopes.XXXXXXXX)"
finish() {
    (cd "$ram" && find . -type f \( -name gdb.log -o -name symbols.json -o -name scopes.json \) -print0 |
        tar --null -T - -czf "$out/vanilla-scopes-raw.tar.gz")
    rm -rf -- "$ram"
}
trap finish EXIT
test -e "$out/vanilla-symbols-verified"
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/vanilla"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/vanilla-scopes-start.txt"
python3 "$d/vanilla-scopes.py" > "$out/vanilla-scopes.jsonl"
date -Iseconds > "$out/vanilla-scopes-end.txt"

#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-page-operator-cost-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-operator-scopes.XXXXXXXX)"
finish() {
    local code=$?
    if test -e "$ram/scopes.jsonl"; then cp "$ram/scopes.jsonl" "$out/"; fi
    if test -d "$ram/scopes"; then
        (cd "$ram" && find scopes -type f \( -name gdb.log -o -name symbols.json -o -name scopes.json \) -print0 |
            tar --null -T - -czf "$out/scopes-raw.tar.gz")
    fi
    rm -rf -- "$ram"
    exit "$code"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/../references-500-visible-page-operators-2026-11-21/candidate.elf" "$ram/candidate"
cp "$d/../references-500-visible-edit-one-root-2026-11-21/candidate.elf" "$ram/hybrid"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum -c "$root/lab/results/construction/diagnostics/references-500-visible-page-operators-2026-11-21/binaries.sha256")
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/hybrid"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/scopes-start.txt"
python3 "$d/scopes.py"
date -Iseconds > "$out/scopes-end.txt"

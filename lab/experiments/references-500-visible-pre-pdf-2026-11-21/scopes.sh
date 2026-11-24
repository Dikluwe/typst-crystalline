#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-pre-pdf-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-visible-pre-pdf-scopes.XXXXXXXX)"
finish() {
    if test -f "$ram/scopes.jsonl"; then cp "$ram/scopes.jsonl" "$out/"; fi
    if test -d "$ram/scopes"; then
        (cd "$ram" && find scopes -name '*.json' -o -name '*.log' | tar -T - -czf "$out/scopes-raw.tar.gz")
    fi
    rm -rf -- "$ram"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/../references-500-visible-stable-cids-2026-11-21/candidate.elf" "$ram/candidate"
cp "$d/../references-500-visible-page-fonts-2026-11-21/candidate.elf" "$ram/hybrid"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum -c "$root/lab/results/construction/diagnostics/references-500-visible-stable-cids-2026-11-21/binaries.sha256")
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/hybrid"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/scopes-start.txt"
python3 "$d/scopes.py"
date -Iseconds > "$out/scopes-end.txt"

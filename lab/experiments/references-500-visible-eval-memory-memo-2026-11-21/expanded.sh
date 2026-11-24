#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-eval-memory-memo-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-memory-memo-expanded.XXXXXXXX)"
finish() {
    if test -e "$ram/expanded.json"; then cp "$ram/expanded.json" "$out/"; fi
    if test -d "$ram/census"; then
        (cd "$ram" && find census -type f \( -name 'census.json' -o -name '*.log' \) -print0 |
            tar --null -T - -czf "$out/expanded-census.tar.gz")
    fi
    rm -rf -- "$ram"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/../references-500-visible-stable-cids-2026-11-21/candidate.elf" "$ram/cap16"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/cap16"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/expanded-start.txt"
python3 "$d/expanded.py"
date -Iseconds > "$out/expanded-end.txt"

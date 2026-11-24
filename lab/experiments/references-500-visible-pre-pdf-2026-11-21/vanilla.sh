#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-pre-pdf-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-visible-pre-pdf-vanilla.XXXXXXXX)"
finish() {
    local status=$?
    if test -f "$ram/vanilla-phases.json"; then cp "$ram/vanilla-phases.json" "$out/"; fi
    if test -d "$ram/vanilla-traces"; then
        (cd "$ram" && find vanilla-traces -type f \( -name 'timings-3.json' -o -name clock.bin -o -name watch.log \) -print0 |
            tar --null -T - -czf "$out/vanilla-traces.tar.gz")
    fi
    rm -rf -- "$ram"
    exit "$status"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
cp "$d/../references-500-watch-physical-2026-11-20/control.elf" "$ram/tekt"
test "$(sha256sum "$ram/vanilla" | cut -d ' ' -f 1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
cc -O2 -Wall -Wextra -shared -fPIC -o "$ram/trace-clock.so" "$d/../references-500-pdf-opening-2026-11-20/trace-clock.c" -ldl
sha256sum "$ram/trace-clock.so" > "$out/trace-clock.sha256"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/tekt"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/vanilla-start.txt"
python3 "$d/vanilla.py"
date -Iseconds > "$out/vanilla-end.txt"

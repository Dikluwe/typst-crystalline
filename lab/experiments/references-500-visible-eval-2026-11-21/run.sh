#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-eval-2026-11-21"
suffix="${TEKT_EVAL_CAPACITY:+-cap32}"
ram="$(mktemp -d /dev/shm/tekt-visible-eval.XXXXXXXX)"
finish() {
    for file in gdb.log census.json; do
        if test -e "$ram/$file"; then cp "$ram/$file" "$out/${file%.*}$suffix.${file##*.}"; fi
    done
    rm -rf -- "$ram"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/mount.txt"
cp "$d/../references-500-visible-stable-cids-2026-11-21/candidate.elf" "$ram/candidate"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
git -C "$root" rev-parse HEAD > "$out/head.txt"
git -C "$root" diff HEAD --stat > "$out/diffstat.txt"
date -Iseconds > "$out/start$suffix.txt"
python3 "$d/census.py"
date -Iseconds > "$out/end$suffix.txt"
objdump -dC --start-address=0x1fca2b0 --stop-address=0x1fcc1a1 "$ram/candidate" > "$out/loop.asm"

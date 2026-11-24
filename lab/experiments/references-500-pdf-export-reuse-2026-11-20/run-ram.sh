#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
while ! { test -s "$d/candidate-end.txt" && test -x "$d/candidate.elf"; }; do
    sleep 1
done
ram="$(mktemp -d /dev/shm/tekt-export-reuse-run.XXXXXXXX)"
finish() {
    local status=$?
    for tree in runs fresh directed; do
        if test -d "$ram/$tree"; then cp -a "$ram/$tree" "$d/"; fi
    done
    rm -rf -- "$ram"
    exit "$status"
}
trap finish EXIT
findmnt -T "$ram" > "$d/benchmark-ram-mount.txt"
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/tekt.elf" "$ram/tekt"
cp "$d/candidate.elf" "$ram/candidate"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram"
python3 "$d/bench.py"

#!/usr/bin/env bash
set -euo pipefail
d="$(cd -- "$(dirname -- "$0")" && pwd)"
mode="${1:-bench}"
case "$mode" in
    bench) output=runs ;;
    census) output=census ;;
    *) exit 2 ;;
esac
while ! { test -s "$d/control-end.txt" && test -s "$d/candidate-end.txt" &&
          test -x "$d/control.elf" && test -x "$d/candidate.elf"; }; do
    sleep 1
done
ram="$(mktemp -d /dev/shm/tekt-comment-work.XXXXXXXX)"
finish() {
    local status=$?
    if test -d "$ram/$output"; then
        cp -a "$ram/$output" "$d/"
    fi
    rm -rf -- "$ram"
    exit "$status"
}
trap finish EXIT
findmnt -T "$ram" > "$d/$mode-ram-mount.txt"
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/control.elf" "$d/candidate.elf" "$ram/"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram"
python3 "$d/$mode.py"

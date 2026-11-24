#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-eval-residual-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-eval-residual.XXXXXXXX)"
finish() {
    local status=$?
    for file in census-summary.json; do
        if test -f "$ram/$file"; then cp "$ram/$file" "$out/"; fi
    done
    for dir in census; do
        if test -d "$ram/$dir"; then
            (cd "$ram" && find "$dir" -type f \( -name '*.log' -o -name 'counts.json' -o -name 'addresses.json' -o -name 'maps' -o -name '*.bin' -o -name 'timings-3.json' \) -print0 |
                tar --null -T - -czf "$out/$dir.tar.gz")
        fi
    done
    rm -rf -- "$ram"
    printf '%s\n' "$status" > "$out/census-exit.txt"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/mount.txt"
cp "$d/../references-500-visible-textual-style-collapse-2026-11-21/candidate.elf" "$ram/candidate"
cp "$d/../references-500-visible-eval-protected-memo-2026-11-21/candidate.elf" "$ram/control"
test "$(sha256sum "$ram/control" | cut -d ' ' -f 1)" = c059a3dfa68b84b7801074fb7cc8c06aca75dc5100dd1c8fd7781615b3d75f0d
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum candidate control vanilla) > "$out/binaries.sha256"
test "$(sha256sum "$ram/candidate" | cut -d ' ' -f 1)" = c2262d461c8ff488f1bc10feed4baffcfa0107b4cdeb54c39cb6d2568b7ff564
test "$(sha256sum "$ram/vanilla" | cut -d ' ' -f 1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/census-start.txt"
python3 "$d/census.py"
date -Iseconds > "$out/census-end.txt"

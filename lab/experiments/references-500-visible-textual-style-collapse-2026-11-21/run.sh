#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-textual-style-collapse-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-textual-collapse.XXXXXXXX)"
finish() {
    local status=$?
    for file in directed.json census-summary.json measurements.jsonl vanilla-phases.json; do
        if test -f "$ram/$file"; then cp "$ram/$file" "$out/"; fi
    done
    for dir in directed census cases vanilla-traces; do
        if test -d "$ram/$dir"; then
            (cd "$ram" && find "$dir" -type f \( -name '*.log' -o -name 'census.json' -o -name 'addresses.json' -o -name 'maps' -o -name '*.bin' -o -name 'timings-3.json' \) -print0 |
                tar --null -T - -czf "$out/$dir.tar.gz")
        fi
    done
    rm -rf -- "$ram"
    printf '%s\n' "$status" > "$out/run-exit.txt"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/mount.txt"
cp "$d/candidate.elf" "$ram/candidate"
cp "$d/../references-500-visible-eval-protected-memo-2026-11-21/candidate.elf" "$ram/control"
test "$(sha256sum "$ram/control" | cut -d ' ' -f 1)" = c059a3dfa68b84b7801074fb7cc8c06aca75dc5100dd1c8fd7781615b3d75f0d
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum candidate control vanilla) > "$out/binaries.sha256"
test "$(sha256sum "$ram/candidate" | cut -d ' ' -f 1)" = "$(cat "$d/candidate.sha256")"
test "$(sha256sum "$ram/vanilla" | cut -d ' ' -f 1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
for arm in candidate control; do
    nm -n -S -C --defined-only "$ram/$arm" > "$out/$arm.symbols"
done
cp "$d/../unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
test "$(sha256sum "$ram/profiler.so" | cut -d ' ' -f 1)" = 69a8f4f7a9103d0cc302ffee138990ea744531aa89fa0b89ae6faa85251d1d30
cc -O2 -Wall -Wextra -shared -fPIC -o "$ram/trace-clock.so" "$d/../references-500-pdf-opening-2026-11-20/trace-clock.c" -ldl
(cd "$ram" && sha256sum profiler.so trace-clock.so) > "$out/instrumentation.sha256"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
git -C "$root" rev-parse HEAD > "$out/head.txt"
git -C "$root" diff HEAD --stat > "$out/diffstat-before.txt"
git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-before.sha256"
date -Iseconds > "$out/start.txt"
python3 "$d/bench.py"
date -Iseconds > "$out/census-start.txt"
python3 "$d/census.py"
date -Iseconds > "$out/vanilla-start.txt"
python3 "$d/vanilla.py"
date -Iseconds > "$out/end.txt"
git -C "$root" diff HEAD --stat > "$out/diffstat-after.txt"
git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
cmp "$out/diffstat-before.txt" "$out/diffstat-after.txt"
cmp "$out/product-before.sha256" "$out/product-after.sha256"

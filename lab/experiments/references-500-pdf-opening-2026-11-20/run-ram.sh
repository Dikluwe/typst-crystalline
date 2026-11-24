#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
output="$root/lab/results/construction/diagnostics/references-500-pdf-opening-2026-11-20"
ram="$(mktemp -d /dev/shm/tekt-pdf-opening.XXXXXXXX)"
finish() {
    local status=$?
    find "$ram/cases" -name out.pdf -delete 2>/dev/null || true
    for f in "$ram"/cases "$ram"/*.json "$ram"/*.jsonl "$ram"/*.symbols; do
        if test -e "$f"; then cp -a "$f" "$output/"; fi
    done
    rm -rf -- "$ram"
    exit "$status"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$output/ram-mount.txt"
cd "$root"
if [[ "${1:-}" == --trace-cpu-only || "${1:-}" == --font-control ]]; then
    date --iso-8601=seconds > "$output/${1#--}-start.txt"
    cc -O2 -Wall -Wextra -shared -fPIC -o "$ram/trace-clock.so" "$d/trace-clock.c" -ldl
    sha256sum "$ram/trace-clock.so" > "$output/trace-clock.sha256"
elif test "${1:-}" = --directed-only; then
    date --iso-8601=seconds > "$output/directed-start.txt"
else
    date --iso-8601=seconds > "$output/start.txt"
    git rev-parse HEAD HEAD^{tree} > "$output/head.txt"
    git diff HEAD --stat > "$output/diffstat-before.txt"
    git diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.lock Cargo.toml | sha256sum > "$output/product-before.sha256"
fi
cp "$d/../references-500-pdf-export-reuse-2026-11-20/candidate.elf" "$ram/candidate"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
cp "$d/../unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
(cd "$ram" && sha256sum candidate vanilla profiler.so) > "$output/binaries.sha256"
(cd "$ram" && sha256sum -c "$d/expected.sha256")
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1
for exe in candidate vanilla; do
    nm -n -S -C --defined-only "$ram/$exe" > "$ram/$exe.symbols"
done
python3 "$d/opening.py" "$@"
git diff HEAD --stat > "$output/diffstat-after.txt"
cmp "$output/diffstat-before.txt" "$output/diffstat-after.txt"
git diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.lock Cargo.toml | sha256sum > "$output/product-after.sha256"
cmp "$output/product-before.sha256" "$output/product-after.sha256"
date --iso-8601=seconds > "$output/end.txt"

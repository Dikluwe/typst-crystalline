#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-published-7c912bd-2026-11-21"
cache="$root/lab/target/published-7c912bd"
ram="${TEKT_RUN_ROOT:-$cache/runs}"
mkdir -p "$ram"
prior="$root/lab/results/construction/diagnostics/references-500-visible-page-operator-cost-2026-11-21"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
if [[ "${1:-}" == '--census' ]]; then
    date -Iseconds > "$out/census-start.txt"
    python3 "$d/census.py" > "$out/census.log" 2>&1
    date -Iseconds > "$out/census-end.txt"
    exit 0
fi
if [[ "${1:-}" != '--resume-profile' ]]; then
    cp --reflink=auto "$cache/candidate" "$ram/candidate"
    cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
    cp "$d/../unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
    (cd "$ram" && sha256sum candidate vanilla profiler.so) > "$out/binaries.sha256"
    test "$(sha256sum "$ram/vanilla" | cut -d ' ' -f1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
    test "$(sha256sum "$ram/profiler.so" | cut -d ' ' -f1)" = 69a8f4f7a9103d0cc302ffee138990ea744531aa89fa0b89ae6faa85251d1d30
    "$ram/vanilla" --version > "$out/vanilla-version.txt"
    nm -n -S -C --defined-only "$ram/candidate" > "$out/tekt.symbols"
    cp "$prior/vanilla-symbols-twin.txt" "$out/vanilla.symbols"
    test -f "$prior/vanilla-symbols-verified"
    objcopy --dump-section ".text=$ram/vanilla.text" "$ram/vanilla" "$ram/vanilla-objcopy.tmp"
    test "$(sha256sum "$ram/vanilla.text" | cut -d ' ' -f1)" = 63b6b461ecc49b5558928e6351418af3af7abfb9b8651055aad5ce5e2ff11fce
    sha256sum "$ram/vanilla.text" "$out/vanilla.symbols" > "$out/vanilla-symbols.sha256"
    rm -f "$ram/vanilla.text" "$ram/vanilla-objcopy.tmp"
    python3 "$d/observe.py" validate > "$out/validation.log" 2>&1
    tar -czf "$out/witness.tar.gz" -C "$ram" witness fresh
    date -Iseconds > "$out/nominal-start.txt"
    cat /proc/pressure/cpu > "$out/nominal-pressure-start.txt"
    python3 "$d/observe.py" nominal > "$out/nominal.log" 2>&1
    date -Iseconds > "$out/nominal-end.txt"
    cat /proc/pressure/cpu > "$out/nominal-pressure-end.txt"
fi
if [[ "${1:-}" == "--nominal-only" ]]; then exit 0; fi
date -Iseconds > "$out/profile-start.txt"
python3 "$d/observe.py" profile > "$out/profile.log" 2>&1
date -Iseconds > "$out/profile-end.txt"
(cd "$ram" && find profile-cases -type f \( -name maps -o -name samples.bin -o -name watch.log \) -print0 | tar --null -T - -czf "$out/profiles.tar.gz")
python3 "$d/analyze.py" > "$out/profile-summary.log"

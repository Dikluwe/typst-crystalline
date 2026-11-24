#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
archive="$root/lab/results/construction/diagnostics/vanilla-resync-2026-11-19"
out="$root/lab/results/construction/diagnostics/references-500-visible-page-operator-cost-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-visible-vanilla-symbols.XXXXXXXX)"
trap 'rm -rf -- "$ram"' EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/vanilla-symbols-mount.txt"
date -Iseconds > "$out/vanilla-symbols-start.txt"
mkdir "$ram/tmp"
cp -a "$archive/target" "$ram/target"
cd "$archive/upstream-checkout"
test "$(git rev-parse HEAD)" = a6dcd82927b2a44bb089bc9151e29566e84e7d5a
test "$(git rev-parse HEAD^{tree})" = 20464de0800655e5b1b4f796f4d5aa912c683517
test -z "$(git status --porcelain --untracked-files=no)"
export CARGO_TARGET_DIR="$ram/target" TMPDIR="$ram/tmp" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=a6dcd82927b2a44bb089bc9151e29566e84e7d5a CARGO_BUILD_JOBS=1
taskset -c 9,11,13,15 cargo +1.92.0 rustc --locked --offline --release -p typst-cli --bin typst -- -C strip=none
pinned="$root/lab/typst-original/target/release/typst"
unstripped="$ram/target/release/typst"
nm -n -S -C --defined-only "$unstripped" > "$out/vanilla-symbols-twin.txt"
objcopy --dump-section .text="$ram/pin.text" "$pinned" "$ram/pin-copy"
objcopy --dump-section .text="$ram/twin.text" "$unstripped" "$ram/twin-copy"
(cd "$ram" && sha256sum pin.text twin.text) > "$out/vanilla-text.sha256"
readelf -W -S "$pinned" | grep -E ' \.text ' > "$out/vanilla-pin-text-section.txt"
readelf -W -S "$unstripped" | grep -E ' \.text ' > "$out/vanilla-twin-text-section.txt"
sha256sum "$pinned" "$unstripped" > "$out/vanilla-symbols-binaries.sha256"
date -Iseconds > "$out/vanilla-symbols-end.txt"
cmp "$ram/pin.text" "$ram/twin.text"
test "$(awk '{print $4}' "$out/vanilla-pin-text-section.txt")" = "$(awk '{print $4}' "$out/vanilla-twin-text-section.txt")"
touch "$out/vanilla-symbols-verified"

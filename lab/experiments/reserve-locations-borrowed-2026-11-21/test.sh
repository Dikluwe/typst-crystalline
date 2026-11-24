#!/usr/bin/env bash
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$here" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/$(basename "$here")"
ram=/dev/shm/tekt-context-preserve-20261121
cd "$ram/source"
ulimit -v 12582912
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
python3 "$here/stage.py" "$1" "$ram/source"
date -Iseconds > "$out/$1-start.txt"
status=0
if [[ "$1" = red ]]; then filter=borrowed_mapper_reservation_tests; else filter=compiler::introspect; fi
cargo test --locked --offline -j 1 -p typst-core --lib "$filter" > "$out/$1.log" 2>&1 || status=$?
date -Iseconds > "$out/$1-end.txt"
exit "$status"

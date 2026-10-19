#!/usr/bin/env bash
# Compile every legacy section with the crystalline binary in normal and
# oracle-PDF modes. Both outputs are produced by the same crystalline binary;
# the vanilla oracle is compiled separately.
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INPUT_DIR="$REPO_ROOT/lab/corpus/legacy-sections"
OUTPUT_DIR="$REPO_ROOT/lab/results/legacy-sections-compilation"
CRYSTALLINE_BIN="$REPO_ROOT/lab/target/product/release/typst"

if [[ ! -x "$CRYSTALLINE_BIN" ]]; then
    echo "error: binary not found or not executable: $CRYSTALLINE_BIN" >&2
    exit 1
fi

if [[ ! -d "$INPUT_DIR" ]]; then
    echo "error: input directory not found: $INPUT_DIR" >&2
    exit 1
fi

mkdir -p "$OUTPUT_DIR"

ok=0
fail=0
fail_list=()

while IFS= read -r -d '' src; do
    base="$(basename "$src" .typ)"

    # The crystalline CLI accepts `typst <INPUT> [OUTPUT]` without `compile`.
    if "$CRYSTALLINE_BIN" "$src" "$OUTPUT_DIR/${base}_crystalline.pdf" 2>"$OUTPUT_DIR/${base}_crystalline.err"; then
        rm -f "$OUTPUT_DIR/${base}_crystalline.err"
        ok=$((ok + 1))
    else
        fail=$((fail + 1))
        fail_list+=("${base} [crystalline]")
        echo "FAILED crystalline: $base (see $OUTPUT_DIR/${base}_crystalline.err)" >&2
    fi

    # P980 operator-parity oracle mode uses the same crystalline binary.
    if "$CRYSTALLINE_BIN" --oracle-pdf "$src" "$OUTPUT_DIR/${base}_oracle.pdf" 2>"$OUTPUT_DIR/${base}_oracle.err"; then
        rm -f "$OUTPUT_DIR/${base}_oracle.err"
        ok=$((ok + 1))
    else
        fail=$((fail + 1))
        fail_list+=("${base} [oracle]")
        echo "FAILED oracle: $base (see $OUTPUT_DIR/${base}_oracle.err)" >&2
    fi
done < <(find "$INPUT_DIR" -maxdepth 1 -type f -name '*.typ' -print0 | sort -z)

echo
echo "Completed: $ok successful compilations, $fail failures."
if [[ $fail -gt 0 ]]; then
    printf '  - %s\n' "${fail_list[@]}"
    exit 1
fi

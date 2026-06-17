#!/usr/bin/env bash
# Rig de perf P330 restaurado (P353 Estágio 0).
# Mede o pipeline completo eval→layout→export PDF sobre o corpus 10× (70030
# linhas = medicao-pre-f-passo-318-corpus.typ ×10). Ferramenta: /usr/bin/time
# -f "%e" (hyperfine indisponível neste ambiente); 12 runs, 1ª descartada.
# Caveat de ambiente: comparar SEMPRE antes/depois na MESMA sessão (deriva
# entre sessões, ver medicao-pre-f-passo-330.md). RUST_MIN_STACK=33554432.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${1:-$ROOT/target/release/typst}"
CORPUS1="$ROOT/00_nucleo/diagnosticos/medicao-pre-f-passo-318-corpus.typ"
CORPUS10="$(mktemp /tmp/perf-corpus-10x.XXXX.typ)"
trap 'rm -f "$CORPUS10" /tmp/perf-out.pdf' EXIT
for _ in $(seq 1 10); do cat "$CORPUS1"; done > "$CORPUS10"
RUNS="${2:-12}"
times=()
for _ in $(seq 1 "$RUNS"); do
  t=$(/usr/bin/time -f "%e" "$BIN" "$CORPUS10" -o /tmp/perf-out.pdf 2>&1 >/dev/null)
  times+=("$t")
done
python3 - "${times[@]}" <<'PY'
import sys, statistics
xs=[float(x) for x in sys.argv[1:]][1:]  # descarta a 1ª (warm-up)
print(f"n={len(xs)}  media={statistics.mean(xs):.4f}s  sigma={statistics.pstdev(xs):.4f}s  min={min(xs):.4f}  max={max(xs):.4f}")
PY

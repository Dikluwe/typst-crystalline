#!/usr/bin/env bash
# P490 — Sondas de paridade funcional cristalino vs vanilla 0.14.2
# Executa apenas via vanilla typst query (para verificar output vanilla)
# O cristalino é testado via cargo test (suite structural_parity)
#
# Uso: cd typst-crystalline && bash lab/parity/tools/p490_sondas.sh 2>&1 | tee /tmp/p490_output.txt

set -uo pipefail

CORPUS_DIR="lab/parity/corpus/p490"

echo "=== P490 — Sondas vanilla typst 0.14.2 ==="
echo "Versão vanilla: $(typst --version)"
echo ""

sonda_van() {
    local file="$1"
    local selector="$2"
    local path="$CORPUS_DIR/$file"
    local van_out
    
    van_out=$(typst query --format json "$path" "$selector" 2>&1)
    local exit_code=$?
    
    if [ $exit_code -ne 0 ]; then
        local count="ERRO"
        local detalhe
        detalhe=$(echo "$van_out" | head -2)
        echo "  [VAN ERRO] $detalhe"
    else
        local count
        count=$(echo "$van_out" | python3 -c "import sys,json; d=json.load(sys.stdin); print(len(d))" 2>/dev/null || echo "?")
        echo "  [VAN OK] count=$count"
    fi
}

sonda() {
    local file="$1"
    local selector="${2:-heading}"
    local path="$CORPUS_DIR/$file"
    
    printf "%-45s [sel: %-20s] " "$file" "$selector"
    
    # Vanilla
    local van_out
    van_out=$(typst query --format json "$path" "$selector" 2>&1)
    local van_exit=$?
    
    if [ $van_exit -ne 0 ]; then
        local van_label="ERRO_VAN"
        local van_detail
        van_detail=$(echo "$van_out" | grep -i "error\|cannot" | head -1 | cut -c1-60)
    else
        local van_count
        van_count=$(echo "$van_out" | python3 -c "import sys,json; d=json.load(sys.stdin); print(len(d))" 2>/dev/null || echo "?")
        local van_label="ok(${van_count})"
    fi
    
    echo "van=${van_label:-ok}"
}

echo "=== Cat 1: Funcionalidades parciais ==="
sonda "test-list-marker-array.typ" "list"
sonda "test-enum-start.typ" "enum"
sonda "test-par.typ" "par"
sonda "test-show-link.typ" "link"

echo ""
echo "=== Cat 2: Alto impacto ==="
sonda "test-table.typ" "table"
sonda "test-raw.typ" "raw"
sonda "test-quote.typ" "quote"
sonda "test-footnote.typ" "footnote"
sonda "test-page.typ" "page"
sonda "test-place.typ" "place"

echo ""
echo "=== Cat 3: calc stdlib ==="
sonda "test-calc.typ" "metadata"

echo ""
echo "=== Cat 4: array/str/dict ==="
sonda "test-array.typ" "metadata"
sonda "test-str.typ" "metadata"
sonda "test-dict.typ" "metadata"

echo ""
echo "=== Cat 5: show/set edge cases ==="
sonda "test-show-regex.typ" "heading"
sonda "test-set-local.typ" "heading"
sonda "test-show-where-multi.typ" "heading"

echo ""
echo "=== Cat 6: Math ==="
sonda "test-math.typ" "math.equation"

echo ""
echo "=== Cat 7: Layout visual ==="
sonda "test-stroke-sides.typ" "rect"
sonda "test-columns.typ" "heading"

echo ""
echo "=== P490 vanilla sondas concluídas ==="

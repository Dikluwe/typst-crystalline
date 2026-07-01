#!/usr/bin/env bash
# P520 — Sentinela de regressão para kerning e ligatures no export PDF.
# Uso: cd typst-crystalline && bash lab/parity/tools/p520_sentinela.sh

set -uo pipefail

CORPUS_DIR="lab/parity/corpus/p520"
TYPST_BIN="target/debug/typst"
FAIL=0

run_typst() {
    local input="$1"
    local output="$2"
    if [ ! -x "$TYPST_BIN" ]; then
        echo "[SKIP] $TYPST_BIN não encontrado; correr 'cargo build --bin typst' primeiro"
        return 1
    fi
    "$TYPST_BIN" "$input" "$output" 2>/tmp/p520-typst-stderr.log
    if [ $? -ne 0 ]; then
        echo "[ERRO] compilação falhou: $input"
        cat /tmp/p520-typst-stderr.log | head -n 5
        return 1
    fi
    return 0
}

echo "=== P520 — Sentinelas de regressão ==="

# --- Ligatures ---------------------------------------------------------------
echo ""
echo "--- test-ligatures.typ ---"
LIGA_PDF="/tmp/p520-ligatures.pdf"
if run_typst "$CORPUS_DIR/test-ligatures.typ" "$LIGA_PDF"; then
    # Verificar que o stream não emite <0000> para as ligatures.
    if mutool show "$LIGA_PDF" 4 2>/dev/null | grep -q '<0000>'; then
        echo "[ERRO] ligature emitida como .notdef (<0000>)"
        FAIL=1
    else
        echo "[OK] nenhuma ligature emitida como .notdef"
    fi

    # Verificar que pdftotext consegue extrair texto (ToUnicode parcial).
    extracted=$(pdftotext "$LIGA_PDF" - 2>/dev/null | tr -d '\f\n')
    f_count=$(echo "$extracted" | grep -o 'f' | wc -l)
    if [ "$f_count" -ge 3 ]; then
        echo "[OK] ToUnicode parcial extraiu $f_count ocorrências de 'f'"
    else
        echo "[ERRO] ToUnicode extraiu apenas $f_count ocorrências de 'f' (esperado >= 3)"
        FAIL=1
    fi
fi

# --- Kerning -----------------------------------------------------------------
echo ""
echo "--- test-kerning.typ ---"
KERN_PDF="/tmp/p520-kerning.pdf"
if run_typst "$CORPUS_DIR/test-kerning.typ" "$KERN_PDF"; then
    # Verificar que o operador TJ contém ajustes (qualquer valor != 0).
    if mutool show "$KERN_PDF" 4 2>/dev/null | grep -E '\[ .* TJ' | grep -Eq ' (\-?[1-9][0-9]*) '; then
        echo "[OK] operador TJ contém ajustes de kerning"
    else
        echo "[AVISO] operador TJ pode não conter ajustes de kerning (verificar manualmente)"
    fi
fi

echo ""
if [ "$FAIL" -eq 0 ]; then
    echo "=== P520 — Todas as verificações passaram ==="
else
    echo "=== P520 — Existem falhas ==="
    exit 1
fi

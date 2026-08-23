#!/usr/bin/env bash
# Compila todos os .typ de .typ/ para PDF com o binário crystalline, em dois
# modos: normal (`<nome>_crystalline.pdf`) e oráculo de paridade de operador
# (`--oracle-pdf`, P980 — `00_nucleo/prompts/infra/export/oracle.md`), gerando
# `<nome>_oracle.pdf`. Ambos saem do MESMO binário crystalline; "oracle" aqui
# não é o binário vanilla (esse é `_vanilla.pdf`, gerado à parte).
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TYP_DIR="$REPO_ROOT/.typ"
CRYSTALLINE_BIN="$REPO_ROOT/target/release/typst"

if [[ ! -x "$CRYSTALLINE_BIN" ]]; then
    echo "erro: binário não encontrado ou não executável: $CRYSTALLINE_BIN" >&2
    exit 1
fi

if [[ ! -d "$TYP_DIR" ]]; then
    echo "erro: diretório não encontrado: $TYP_DIR" >&2
    exit 1
fi

ok=0
fail=0
fail_list=()

while IFS= read -r -d '' src; do
    base="$(basename "$src" .typ)"

    # crystalline: `typst <INPUT> [OUTPUT]`, sem subcomando `compile`.
    if "$CRYSTALLINE_BIN" "$src" "$TYP_DIR/${base}_crystalline.pdf" 2>"$TYP_DIR/${base}_crystalline.err"; then
        rm -f "$TYP_DIR/${base}_crystalline.err"
        ok=$((ok + 1))
    else
        fail=$((fail + 1))
        fail_list+=("${base} [crystalline]")
        echo "FALHA crystalline: $base (ver ${base}_crystalline.err)" >&2
    fi

    # oráculo de paridade de operador (P980): mesmo binário crystalline, flag `--oracle-pdf`.
    if "$CRYSTALLINE_BIN" --oracle-pdf "$src" "$TYP_DIR/${base}_oracle.pdf" 2>"$TYP_DIR/${base}_oracle.err"; then
        rm -f "$TYP_DIR/${base}_oracle.err"
        ok=$((ok + 1))
    else
        fail=$((fail + 1))
        fail_list+=("${base} [oracle]")
        echo "FALHA oracle: $base (ver ${base}_oracle.err)" >&2
    fi
done < <(find "$TYP_DIR" -maxdepth 1 -type f -name '*.typ' -print0 | sort -z)

echo
echo "Concluído: $ok compilações OK, $fail falhas."
if [[ $fail -gt 0 ]]; then
    printf '  - %s\n' "${fail_list[@]}"
    exit 1
fi

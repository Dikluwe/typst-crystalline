---
# P787 — CSV: rigor de parsing e correção de `row-type`

> **Passo:** 787
> **Data:** 2026-07-20
> **Foco:** P786 confirmou três divergências em `loading::csv_`: (1) `#csv("arquivo.csv")` com linha malformada (número de campos diferente do cabeçalho) é aceito com exit 0 e produz dados errados no cristalino, enquanto o vanilla erra (`error: failed to parse CSV (found 3 instead of 2 fields in line 2)`); (2) a mensagem de erro de delimitador não-ASCII é enganosa (`"delimiter deve ser um único carácter"` quando o delimitador testado, `"é"`, já é um único carácter — a razão real é não ser ASCII, não o comprimento); (3) `csv(row-type: dictionary)` espera a **string** `"dictionary"` no cristalino, mas o vanilla espera o **tipo** `dictionary` diretamente (API divergente nos dois sentidos).
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar a mensagem de erro exata e a API de `row-type` do vanilla antes de implementar.
> **Prioridade:** Alta — categoria "aceita dado errado em silêncio", mais grave que funcionalidade ausente com erro claro.
> **Dependências:** P786 (achado, evidência em `temp/temp_p786/b_csv_err.typ`, `b_csv_delim_err.typ`).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "fn csv\|row-type\|delimiter" lab/typst-original/crates/typst-library/src/loading/csv.rs 2>/dev/null
```

Confirmar:
1. A mensagem de erro exata para linha com número de campos incorreto (incluindo se reporta o número de linha exato e os valores esperado/encontrado).
2. A mensagem de erro exata para delimitador inválido (não-ASCII vs comprimento errado — são casos distintos? confirmar).
3. O tipo exato esperado por `row-type:` — é literalmente o tipo `dictionary`/`array` como valor de tipo Typst, não uma string.

```bash
cat > /tmp/p787-csv-bad.csv <<'EOF'
a,b
1,2,3
EOF
cat > /tmp/p787-test1.typ <<'EOF'
#csv("/tmp/p787-csv-bad.csv")
EOF
lab/typst-original/target/release/typst compile /tmp/p787-test1.typ 2>&1

cat > /tmp/p787-test2.typ <<'EOF'
#csv("/tmp/p787-csv-bad.csv", delimiter: "é")
EOF
lab/typst-original/target/release/typst compile /tmp/p787-test2.typ 2>&1

cat > /tmp/p787-good.csv <<'EOF'
a,b
1,2
EOF
cat > /tmp/p787-test3.typ <<'EOF'
#csv("/tmp/p787-good.csv", row-type: dictionary)
EOF
lab/typst-original/target/release/typst compile /tmp/p787-test3.typ 2>&1
```

---

## Implementação

1. Corrigir o parser CSV do cristalino para rejeitar linhas com número de campos diferente do cabeçalho, com a mensagem exata confirmada (número de linha, contagem esperada vs encontrada).
2. Corrigir a mensagem de erro de delimitador para refletir a causa real (não-ASCII), não "comprimento errado".
3. Corrigir `row-type:` para aceitar o tipo `dictionary`/`array` diretamente (valor de tipo Typst), não a string `"dictionary"`.

---

## Validação

```bash
./target/release/typst compile /tmp/p787-test1.typ 2>&1
```

Confirmar exit 1, mensagem idêntica ao vanilla.

```bash
./target/release/typst compile /tmp/p787-test2.typ 2>&1
```

Confirmar mensagem correta sobre não-ASCII.

```bash
./target/release/typst compile /tmp/p787-test3.typ 2>&1
```

Confirmar que `row-type: dictionary` (tipo, sem aspas) funciona.

```bash
# Não regressão — CSV válido continua funcionando
cat > /tmp/p787-test4.typ <<'EOF'
#csv("/tmp/p787-good.csv")
EOF
./target/release/typst compile /tmp/p787-test4.typ 2>&1
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Mensagens de erro exatas do vanilla confirmadas para os três casos.
- [ ] Linha malformada rejeitada com erro, não aceita silenciosamente.
- [ ] Mensagem de delimitador corrigida para refletir a causa real.
- [ ] `row-type:` aceita tipo diretamente, não string.
- [ ] CSV válido continua funcionando sem regressão.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p787.md`.

---

## Próximo passo

Próximo candidato de "aceito em silêncio, deveria errar": referências/citações inválidas (grupo 2 de P786 §5), conflito de célula com header de tabela (grupo 7), ou show rule por string não aplicada (grupo 4).

---

# P503 — Re-baseline de Paridade contra Typst 0.15.0

> **Passo:** 503
> **Data:** 2026-06-29
> **Foco:** Re-executar empiricamente a bateria P490 (20 ficheiros) contra vanilla 0.15.0 para descobrir que MATCHs viraram DIFFs/AUSENTEs devido às breaking changes. Não declarar conclusão — medir antes de começar o trabalho.
> **Tipo:** Diagnóstico empírico via re-execução de bateria.
> **Tamanho:** S (~20 min de re-execução + análise).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P498 (bateria P490 fechada em 20/20 MATCH contra 0.14.2), P502 (audit P500 fechado).

---

## 1. Contexto

O P502 revelou que o cristalino implementou funcionalidades (`str.to-upper`, `str.to-unicode`, `calc.log10`, `calc.deg`, `calc.rad`) que o **vanilla 0.14.2 não suporta**, mas o **vanilla 0.15.0 suporta**. Isso indica que o cristalino já está alinhado com funcionalidades do 0.15.0, mas a bateria P490 foi validada contra 0.14.2.

Para garantir paridade real, é necessário **re-executar a bateria P490 contra 0.15.0** e identificar:
1. Quais MATCHs contra 0.14.2 **viraram DIFFs** contra 0.15.0 (breaking changes).
2. Quais MATCHs contra 0.14.2 **viraram MATCHs** contra 0.15.0 (novas funcionalidades já implementadas no cristalino).
3. Quais AUSENTEs/DIFFs contra 0.14.2 **persistem** contra 0.15.0 (gaps reais).

---

## 2. Metodologia

Re-executar os 20 ficheiros da bateria P490 contra **vanilla 0.15.0** e comparar com o cristalino.

```bash
# Vanilla 0.15.0 (atualizado):
typst query --format json documento.typ "selector"

# Cristalino (commit atual):
cargo run -p typst-wiring -- query documento.typ "selector"
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## 3. Hipóteses de Quebra (Breaking Changes 0.15.0)

Com base na análise do changelog, os seguintes ficheiros da bateria P490 **podem** ter mudado de MATCH para DIFF:

### 3.1 — `str(base:)` (P495)

**Breaking change 0.15.0:** `str(base:)` só aceita inteiros; `str(1.5, base: 10)` → erro.

| Ficheiro | Teste | 0.14.2 | 0.15.0 | Esperado cristalino |
|----------|-------|--------|--------|---------------------|
| `test-str.typ` | `str(255, base: 16)` | ok("ff") | ok("ff") | MATCH |
| `test-str.typ` | `str(1.5, base: 10)` | ok? | ERRO | Se cristalino aceita → DIFF |

**Verificação:** O cristalino rejeita `str` com `base:` para não-inteiros?

### 3.2 — `slice(end:, count:)`

**Breaking change 0.15.0:** `slice` com `end` + `count` = erro.

| Ficheiro | Teste | 0.14.2 | 0.15.0 | Esperado cristalino |
|----------|-------|--------|--------|---------------------|
| `test-array.typ` | `arr.slice(0, end: 5, count: 3)` | ok? | ERRO | Se cristalino aceita → DIFF |

**Verificação:** A bateria P490 não testou `slice` com ambos. Risco baixo.

### 3.3 — Backslash em paths

**Breaking change 0.15.0:** Paths com `\` proibidos.

| Ficheiro | Teste | 0.14.2 | 0.15.0 | Esperado cristalino |
|----------|-------|--------|--------|---------------------|
| `test-image-fit.typ` | `image("path\file.png")` | ok? | ERRO | Se cristalino aceita → DIFF |

**Verificação:** Nenhum ficheiro da bateria P490 usa `\` em paths. Risco zero.

### 3.4 — `class` não recursivo

**Breaking change 0.15.0:** `math.class` aplica só ao corpo direto.

| Ficheiro | Teste | 0.14.2 | 0.15.0 | Esperado cristalino |
|----------|-------|--------|--------|---------------------|
| `test-math.typ` | `$class("unary", x + y)$` | ok? | ok (não recursivo) | Se cristalino recursivo → DIFF |

**Verificação:** A bateria P490 não testou `class`. Risco zero.

### 3.5 — Delimiters callable → `lr`

**Breaking change 0.15.0:** `chevron.l(x)` produz `lr` automaticamente.

| Ficheiro | Teste | 0.14.2 | 0.15.0 | Esperado cristalino |
|----------|-------|--------|--------|---------------------|
| `test-math.typ` | `$chevron.l(x)$` | ok? | ok (lr) | Se cristalino não callable → DIFF |

**Verificação:** A bateria P490 não testou `chevron.l`. Risco zero.

---

## 4. Hipóteses de Melhoria (Novas Funcionalidades 0.15.0 já no Cristalino)

O P502 revelou que o cristalino já implementou funcionalidades do 0.15.0. Contra 0.15.0, estas podem ter mudado de ERRO_DESCRITIVO (0.14.2) para MATCH:

| Funcionalidade | Ficheiro | 0.14.2 | Cristalino | 0.15.0 | Esperado pós-503 |
|----------------|----------|--------|------------|--------|----------------|
| `str.to-upper()` | `test-str-methods.typ` (P500) | ERRO | ok | ok | MATCH |
| `str.to-unicode()` | `test-str-methods.typ` (P500) | ERRO | ok | ok | MATCH |
| `str.repeat()` | `test-str-methods.typ` (P500) | ERRO | ok | ok | MATCH |
| `calc.log10()` | `test-calc-rest.typ` (P500) | ERRO | ok | ok | MATCH |
| `calc.deg()` | `test-calc-rest.typ` (P500) | ERRO | ok | ok | MATCH |
| `calc.rad()` | `test-calc-rest.typ` (P500) | ERRO | ok | ok | MATCH |

**Nota:** Estes ficheiros são do P500, não do P490. A bateria P490 em si pode não ter mudado.

---

## 5. Bateria P490 — Re-execução Esperada

| Ficheiro | Selector | P498 (0.14.2) | Esperado 0.15.0 | Cristalino | Classificação |
|----------|----------|---------------|-----------------|------------|---------------|
| test-list-marker-array.typ | `list` | MATCH | MATCH | ok | MATCH |
| test-enum-start.typ | `enum` | MATCH | MATCH | ok | MATCH |
| test-par.typ | `par` | MATCH | MATCH | ok | MATCH |
| test-show-link.typ | `link` | MATCH | MATCH | ok | MATCH |
| test-table.typ | `table` | MATCH | MATCH | ok | MATCH |
| test-raw.typ | `raw` | MATCH | MATCH | ok | MATCH |
| test-quote.typ | `quote` | MATCH | MATCH | ok | MATCH |
| test-footnote.typ | `footnote` | MATCH | MATCH | ok | MATCH |
| test-page.typ | `page` | MATCH | MATCH | ok | MATCH |
| test-place.typ | `place` | MATCH | MATCH | ok | MATCH |
| test-calc.typ | `metadata` | MATCH | MATCH | ok | MATCH |
| test-array.typ | `metadata` | MATCH | MATCH | ok | MATCH |
| test-str.typ | `metadata` | MATCH | MATCH | ok | MATCH |
| test-dict.typ | `metadata` | MATCH | MATCH | ok | MATCH |
| test-show-regex.typ | `heading` | MATCH | MATCH | ok | MATCH |
| test-set-local.typ | `heading` | MATCH | MATCH | ok | MATCH |
| test-show-where-multi.typ | `heading` | MATCH | MATCH | ok | MATCH |
| test-math.typ | `math.equation` | MATCH | MATCH | ok | MATCH |
| test-stroke-sides.typ | `heading` | MATCH | MATCH | ok | MATCH |
| test-columns.typ | `heading` | MATCH | MATCH | ok | MATCH |

**Hipótese:** A bateria P490 **não quebra** contra 0.15.0. Os breaking changes (backslash, slice, class, delimiters) não afetam os 20 ficheiros testados.

**Se hipótese confirmada:** P503 = confirmação de que a paridade P490 é estável contra 0.15.0.

**Se hipótese refutada:** P503 = identificação dos ficheiros que quebraram e planeamento de P504.

---

## 6. Critério de Fecho

- [ ] Todos os 20 ficheiros da bateria P490 re-executados contra vanilla 0.15.0.
- [ ] Todos os 20 ficheiros re-executados contra cristalino (commit atual).
- [ ] Tabela comparativa: P498 (0.14.2) vs P503 (0.15.0) vs cristalino.
- [ ] Breaking changes identificados (se houver).
- [ ] MATCHs preservados: **20/20** (esperado).
- [ ] PANICs: 0 (preservado).
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p503.md` produzido.
- [ ] Decisão documentada: P504 = breaking changes (se houver) ou P504 = novas funcionalidades 0.15.0.

---

## 7. Próximo Passo (P504)

**Se P503 confirma 20/20 MATCH:**
- P504 = **Novas funcionalidades 0.15.0** — materializar `within` selector, `dict.map/filter`, `arguments` field access, `calc.asinh/acosh/atanh/erf`, `int.min/max`, `range(inclusive)`, `int(base:)`, `counter.display(at:)`, `list.marker-align`, `page.bleed`, `divider` element.

**Se P503 descobre quebras:**
- P504 = **Breaking changes 0.15.0** — corrigir compatibilidade (backslash, slice, class, delimiters, etc.).

---

## A. Apêndice — Comandos de Re-execução

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

# Vanilla 0.15.0
typst --version  # confirmar 0.15.0

# Bateria P490 completa
for f in lab/parity/tests/p490/*.typ; do
  echo "=== $f ==="
  typst query --format json "$f" "heading" 2>/dev/null || echo "ERRO"
  cargo run -p typst-wiring -- query "$f" "heading" 2>/dev/null || echo "ERRO"
done

# Sentinela
 cargo test --test structural_parity p503_rebaseline_0150
```

---

## B. Apêndice — Checklist de Breaking Changes 0.15.0

| # | Breaking Change | Afeta P490? | Risco |
|---|-----------------|-------------|-------|
| 1 | Backslash em paths proibido | Não | Zero |
| 2 | `slice(end:, count:)` = erro | Não | Zero |
| 3 | `str(base:)` só inteiros | Não (P490 não testa float) | Baixo |
| 4 | Font suffixes omitidos | Não | Zero |
| 5 | `text.features` strict | Não | Zero |
| 6 | `class` não recursivo | Não | Zero |
| 7 | Delimiters callable → `lr` | Não | Zero |
| 8 | Baseline retained em `box`/`block` | Não | Zero |
| 9 | HTML `box`/`block` redefinidos | Não | Zero |
| 10 | HTML paragraph grouping | Não | Zero |
| 11 | `html.script`/`html.style` só string | Não | Zero |
| 12 | Non-Unicode paths rejeitados | Não | Zero |
| 13 | `--timings` requer nome | Não | Zero |
| 14 | `path` element removido | Não | Zero |
| 15 | `pattern` type removido | Não | Zero |
| 16 | `pdf.embed` removido | Não | Zero |
| 17 | Scoped decode functions removidos | Não | Zero |

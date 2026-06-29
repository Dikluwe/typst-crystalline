# Relatório de Paridade Funcional — P502

> **Passo:** 502
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os 4 gaps S/XS restantes do audit P500: `image.fit`, `#raw(lang:, block:)`, `footnote.numbering`, `outline.indent` API.
> **Vanilla CLI disponível:** 0.14.2 (`b33de9de`).

---

## 1. Resumo executivo

- **502a** — `image.fit` (`contain`/`cover`/`stretch`) implementado.
- **502b** — `#raw(lang:, block:)` implementado.
- **502c** — `footnote(numbering:)` implementado.
- **502d** — `outline.indent` API alinhada com vanilla (`length | function | auto`), mantendo `bool` para compatibilidade reversa.
- **Bateria P500:** 4 ficheiros que eram AUSENTE/DIFF em P501 passaram a **MATCH**.
- **AUSENTEs restantes no cristalino:** 3 ficheiros (`test-list-advanced.typ`, `test-enum-advanced.typ`, `test-state-counter.typ`), correspondendo a 2 gaps: `list/enum` indent (M) e `state/counter/context` (L).
- **Zero PANICs** preservado.

---

## 2. Implementação por categoria

### 2.1 — 502a: `image.fit`

**Ficheiros alterados:**
- `01_core/src/entities/elements/image.rs`
- `01_core/src/entities/content.rs`
- `01_core/src/rules/stdlib/figure_image.rs`

**Mudanças:**
- `ImageElem` ganha campo `fit: EcoString` (default `"cover"`).
- `Content::image(...)` atualizado; valores válidos validados em `native_image`.

### 2.2 — 502b: `#raw(lang:, block:)`

**Ficheiro alterado:** `01_core/src/rules/stdlib/structural.rs`

**Mudanças:**
- `native_raw` passa a aceitar `lang:` e `block:` named.
- `RawElem` já possuía os campos; apenas o constructor stdlib foi actualizado.

### 2.3 — 502c: `footnote(numbering:)`

**Ficheiros alterados:**
- `01_core/src/entities/elements/footnote.rs`
- `01_core/src/entities/content.rs`
- `01_core/src/rules/stdlib/structural.rs`

**Mudanças:**
- `FootnoteElem` ganha campo `numbering: Option<EcoString>`.
- Adicionado `Content::footnote_with_numbering(body, numbering)`; `Content::footnote(body)` preservado.

### 2.4 — 502d: `outline.indent` API

**Ficheiros alterados:**
- `01_core/src/entities/elements/outline.rs`
- `01_core/src/entities/content.rs`
- `01_core/src/rules/stdlib/structural.rs`
- `01_core/src/rules/layout/outline.rs`

**Mudanças:**
- Introduzido `OutlineIndent` enum (`Auto`, `Bool(bool)`, `Length(Length)`, `Function(Func)`).
- `OutlineElem.indent` passou de `bool` para `OutlineIndent`.
- `native_outline` aceita `length`, `function`, `auto` e `bool`.
- Layout consome `indent.is_active()`; `Length`/`Function` são aceites mas renderização específica fica scope-out.

---

## 3. Resultados da bateria P500 (pós-502)

| Ficheiro | Vanilla | Cristalino pré-502 | Cristalino pós-502 | Δ |
|---|---|---|---|---|
| `test-image-fit.typ` | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| `test-page-header-footer.typ` | ok | ok | ok | — |
| `test-place-absolute.typ` | ok | ok | ok | — |
| `test-calc-rest.typ` | erro* | ok | ok | — |
| `test-str-methods.typ` | erro* | ok | ok | — |
| `test-dict-methods.typ` | ok | ok | ok | — |
| `test-list-advanced.typ` | ok | AUSENTE | AUSENTE | gap M-size pendente |
| `test-enum-advanced.typ` | ok | AUSENTE | AUSENTE | gap M-size pendente |
| `test-par-advanced.typ` | ok | ok | ok | — |
| `test-raw-advanced.typ` | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| `test-quote-advanced.typ` | ok | ok | ok | — |
| `test-footnote-advanced.typ` | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| `test-figure-advanced.typ` | ok | ok | ok | — |
| `test-bibliography-csl.typ` | ok | ok | ok | — |
| `test-outline-advanced.typ` | ok | DIFF | ok | **DIFF → MATCH** |
| `test-state-counter.typ` | ok | AUSENTE | AUSENTE | gap L-size pendente |
| `test-metadata-query.typ` | ok | ok | ok | — |

\* O vanilla 0.14.2 instalado não suporta alguns métodos usados nos ficheiros (`to-upper`, `to-lower`, `to-unicode`, `repeat`, `log10`, `deg`, `rad`). O cristalino compila.

---

## 4. Contagens finais

| Métrica | Pré-502 | Pós-502 |
|---------|--------:|--------:|
| MATCH | 12 | 14 |
| AUSENTE cristalino | 6 ficheiros | 3 ficheiros |
| DIFF | 1 (`outline.indent`) | 0 |
| PANIC | 0 | 0 |

---

## 5. Prompts L0 actualizados

- `00_nucleo/prompts/entities/elements/image.md`
- `00_nucleo/prompts/entities/elements/raw.md`
- `00_nucleo/prompts/entities/elements/footnote.md`
- `00_nucleo/prompts/entities/elements/outline.md`
- `00_nucleo/prompts/rules/stdlib/structural.md`
- `00_nucleo/prompts/rules/stdlib/figure_image.md`
- `00_nucleo/prompts/rules/layout_outline.md`

Os hashes `@prompt-hash` nos ficheiros L1 foram recalculados com `crystalline-lint --fix-hashes .`.

---

## 6. Testes adicionados

- `lab/parity/tests/structural_parity.rs`:
  - `p502_gaps_s_xs` — sentinela geral dos 4 gaps.
  - `p502_image_fit`
  - `p502_raw_lang_block`
  - `p502_footnote_numbering`
  - `p502_outline_indent_api`
- `01_core/src/rules/stdlib/mod.rs`:
  - `p295_native_footnote_numbering_aceite_p502`
  - `p295_native_footnote_named_arg_desconhecido_rejeitado`

---

## 7. Estado de validação

- `cargo test -p typst-core`: **ok** (3478 passed).
- `cargo test -p typst-parity p502_`: **ok** (5 passed).
- `cargo test -p typst-parity p500_audit_cobertura_stdlib_expandida`: **ok**.
- `cargo test -p typst-parity p501_gaps_p1_p2`: **ok**.
- `cargo build`: **ok**.
- `crystalline-lint .`: **ok** (zero violations).
- Nota: `cargo test` completo do workspace mantém a falha pré-existente em `04_wiring/tests/cli.rs::disciplina_warnings_antes_de_errors`, fora do âmbito de P502.

---

## 8. Próximo passo

Com P502 fechado, os gaps restantes do P500 são:

| Prioridade | Gap | Tamanho | Recomendação |
|---|---|---|---|
| P2 | `list`/`enum` `indent`, `body-indent`, `tight` | M | P503 = layout de listas/enum |
| P3 | `state.update/get` + `context` + `counter` | L | P504 = runtime state (trilha separada) |

---

## 9. Conclusão

P502 fechou empiricamente os 4 gaps S/XS restantes do audit P500. O compilador cristalino passa a aceitar as APIs `image.fit`, `raw(lang:, block:)`, `footnote(numbering:)` e `outline.indent` (`length|function|auto|bool`), reduzindo os AUSENTEs de 6 para 3 ficheiros (2 gaps). Zero PANICs preservado; `crystalline-lint` reporta zero violations.

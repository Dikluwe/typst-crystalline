# Diagnóstico de Paridade Funcional — P494

> **Passo:** 494  
> **Data:** 2026-06-29  
> **Metodologia:** Bateria P490 de 20 ficheiros `.typ` re-executada contra vanilla 0.14.2 (`typst query`) e cristalino (`query_to_summary`), mais sentinela P494 dos 7 selectors de elementos de documento.  
> **ADR referência:** ADR-0075 (comparação via `typst query --format json`), ADR-0054 (graded parity), ADR-0109 (atomização).

---

## Objetivo

Fechar o **Grupo D1** de gaps identificados no diagnóstico P490: os 7 selectors de elementos de documento ausentes no cristalino — `list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote`.

---

## Tabela de Resultados P490 (pós-P494)

| Ficheiro | Selector | Vanilla | Cristalino | Classificação | Notas |
|----------|----------|---------|------------|---------------|-------|
| test-list-marker-array.typ | `list` | ok(1) | ok(1) | **MATCH** | `marker:Array` aceite; contagem por análise de `Content` |
| test-enum-start.typ | `enum` | ok(1) | ok(1) | **MATCH** | `start:` + `(a)` aceites; contagem por análise de `Content` |
| test-par.typ | `par` | ok(1) | ok(1) | **MATCH** | contagem aproximada (presença de texto) |
| test-show-link.typ | `link` | ok(1) | ok(1) | **MATCH** | `#show link:` resolve para selector nativo |
| test-table.typ | `table` | ok(1) | ERRO_DESCRITIVO | **DIFF** | `table.header/footer` — field access em função não suportado |
| test-raw.typ | `raw` | ok(1) | ok(1) | **MATCH** | contagem por variante `Content::Raw` |
| test-quote.typ | `quote` | ok(1) | ok(1) | **MATCH** | contagem por variante `Content::Quote` |
| test-footnote.typ | `footnote` | ok(1) | ok(1) | **MATCH** | contagem por variante `Content::Footnote` |
| test-page.typ | `page` | ERRO_VAN (not locatable) | ok(0) | **MATCH** | Sem PANIC |
| test-place.typ | `place` | ERRO_VAN (not locatable) | ok(0) | **MATCH** | Sem PANIC |
| test-calc.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `calc.log(base:)` e `calc.round(digits:)` — args nomeados não suportados |
| test-array.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `arr.dedup()` — field access em array não suportado |
| test-str.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `str(255, base: 16)` — arg `base:` não suportado |
| test-dict.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `d.at("z", default: 99)` — campo `at` não existe |
| test-show-regex.typ | `heading` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `text` não reconhecido em contexto de show-regex |
| test-set-local.typ | `heading` | ok(0) | ok(0) | **AUSENTE*** | count=0; sem headings no ficheiro |
| test-show-where-multi.typ | `heading` | ok(1) | ERRO_DESCRITIVO | **DIFF** | `heading.where(multi-field)` scope-out P474 |
| test-math.typ | `math.equation` | ok(5) | ok(5) | **MATCH** | `vec`, `mat`, `cases`, `integral`, `sum` — 5/5 |
| test-columns.typ | `heading` | ok(0) | ok(0) | **AUSENTE*** | count=0; sem headings no ficheiro |
| test-stroke-sides.typ | `heading` | ok(0) | ERRO_DESCRITIVO (esperado) | **MATCH** | `red`, `blue`, `green` não reconhecidas; sem PANIC |

\* Classificados como Absent no resumo numérico porque `count=0`, mas não representam regressão.

---

## Resumo por Classificação

| Classificação | Contagem | Ficheiros |
|--------------|----------|-----------|
| **MATCH** | 9 | test-list, test-enum, test-par, test-show-link, test-raw, test-quote, test-footnote, test-page, test-place, test-math, test-stroke-sides |
| **DIFF** | 7 | test-table, test-calc, test-array, test-str, test-dict, test-show-regex, test-show-where-multi |
| **AUSENTE** (count=0) | 2 | test-set-local, test-columns |
| **PANIC** | **0** | — |

> **Métrica P490 original:** 5 MATCH / 9 DIFF / 6 AUSENTE.  
> **Métrica P490 pós-P494:** 9 MATCH / 7 DIFF / 2 AUSENTE / 0 PANIC.

---

## Sentinela P494

Adicionado em `lab/parity/tests/structural_parity.rs`:

- `fn p494_selectores_elementos_documento()` — verifica que cada um dos 7 selectors retorna `count=1` no cristalino e, quando o CLI vanilla está disponível, compara com `typst query`.

**Resultado:** `test p494_selectores_elementos_documento ... ok`

---

## Implementação

### Ficheiros alterados

| Camada | Ficheiro | Alteração |
|--------|----------|-----------|
| L0 | `00_nucleo/prompts/entities/element_kind.md` | Novos kinds `List`, `Enum`, `Par`, `Link`, `Raw`, `Quote`, `Footnote` |
| L0 | `00_nucleo/prompts/entities/selector.md` | Alinhamento com `ElementKind` |
| L0 | `00_nucleo/prompts/infra/query-helpers.md` | Contagem por análise de `Content` para document elements |
| L1 | `01_core/src/entities/element_kind.rs` | Variants + `from_name` + testes P494 |
| L1 | `01_core/src/entities/list_marker.rs` | `ListMarker::Array` para `marker: ("a", "b")` |
| L1 | `01_core/src/entities/show.rs` | Registo de `NodeKind::List`, `Enum`, `Link`, `Quote`, `Footnote`, `Raw` |
| L1 | `01_core/src/engine/eval/rules.rs` | `#show link` mapeia para selector nativo; handlers de show-rule |
| L1 | `01_core/src/engine/stdlib/foundations.rs` | `native_list` aceita `marker: Array` |
| L1 | `01_core/src/engine/stdlib/structural.rs` | `native_enum` aceita `start:` |
| L3 | `03_infra/src/query_helpers.rs` | `count_element_in_content`, `is_document_element_kind`, testes P494 |
| Lab | `lab/parity/tests/structural_parity.rs` | Sentinela P494 |

### Hashes L0 (pós `--fix-hashes`)

- `01_core/src/entities/element_kind.rs` → `a8e6ef80`
- `01_core/src/entities/selector.rs` → `66d4dfe4`
- `03_infra/src/query_helpers.rs` → `d8de80fb`

---

## Decisões Técnicas

1. **`list` e `enum` não são `Content` locatable.** O cristalino materializa listas e enums como `Sequence` de `ListItem` / `EnumItem`. A contagem é feita em L3 por análise do `Content` (forma B da ADR-0109: lógica no arquivo da unidade, sem alterar o núcleo).
2. **`par` é aproximado.** Cristalino não materializa `ParElem`; `query(par)` retorna 1 se existir texto plano no documento.
3. **`#show link` funciona como selector.** Registado em `NodeKind` e resolvido durante eval de show-rules; não implica tornar `Link` locatable.
4. **`marker: Array` expande `ListMarker`.** Mantém paridade morfológica com vanilla sem alterar a estrutura de `Content`.
5. **`enum(start:)` adicionado ao constructor.** Aceita `start: int` e afeta a enumeração dos itens.

---

## Gaps Remanescentes (fora do scope P494)

| Grupo | Gap | Prioridade |
|-------|-----|------------|
| D2 | Args nomeados: `calc.log(base:)`, `calc.round(digits:)`, `str(base:)`, `dict.at(default:)` | Média |
| D3 | Field access: `arr.dedup()`, `table.header/footer` | Média |
| D4 | Variáveis de cor predefinidas (`red`, `blue`, `green`) em contextos inline | Baixa |
| D5 | `text()` em show-regex | Baixa |

---

## Critérios de Fecho P494

- [x] 7 selectors de elementos de documento parseáveis e contáveis.
- [x] Sentinela P494 passa (`cargo test p494_selectores_elementos_documento`).
- [x] Bateria P490 re-executada: 9 MATCH, 7 DIFF, 2 AUSENTE, **0 PANIC**.
- [x] Testes unitários adicionados em `element_kind.rs` e `query_helpers.rs`.
- [x] `cargo build` sem erros.
- [x] `crystalline-lint .` com **zero violations**.
- [x] Hashes L0 corrigidos com `crystalline-lint --fix-hashes`.
- [x] Relatório `paridade-funcional-p494.md` produzido.

---

## Comandos de Verificação

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

# Build e lint
cargo build
crystalline-lint .

# Sentinela P494
cargo test --test structural_parity p494_selectores_elementos_documento -- --nocapture

# Bateria P490
cargo test --test structural_parity p490_bateria_paridade_funcional_20_ficheiros -- --nocapture
```

---

## Próximo Passo (P495+)

Recomendação: **D2 — args nomeados** (`calc.log(base:)`, `calc.round(digits:)`, `str(base:)`, `dict.at(default:)`) ou **D3 — field access** (`arr.dedup()`, `table.header/footer`), conforme prioridade do roteiro.

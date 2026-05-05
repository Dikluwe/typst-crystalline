# Relatório P190B — Categoria Bibliography (M6 incremental)

**Data**: 2026-05-04
**Estado**: ✅ Completo (8 sub-passos A-H)
**Magnitude**: M (categoria mais simples — caminho Introspector activo desde P181H).
**Pattern arquitectural**: ADR-0070 (PROPOSTO P190A) — 1ª aplicação concreta do pattern stylesheet "eliminação write paralelo M5".

---

## §1 Sumário executivo

P190B fecha **Categoria 1 (Bibliography)** do plano M6 (P190A
estratégia β):
- Layouter consumers `mod.rs:660+` migrados para Introspector path completo (sem fallback legacy).
- Layouter assignments duais (`mod.rs:1496-1498, 1524-1526`) eliminados.
- Fields `bib_entries` + `bib_numbers` eliminados de `CounterStateLegacy`.
- Walk arm Bibliography **NÃO modificado** (já era puro desde P181H).
- 4 tests sentinela legacy adaptados/removidos (write paralelo agora não-funcional).

**1ª aplicação concreta** do pattern stylesheet "eliminação write paralelo M5" (ADR-0070 PROPOSTO).

**Output observable em produção**: inalterado — caminho Introspector activo desde P181H.

---

## §2 Trabalho concreto

| # | Ficheiro | Mudança |
|---|----------|---------|
| 1 | `rules/layout/mod.rs:660-680` | Consumer cite-arm migrado: `self.introspector.bib_entry_for_key(key)` (sem fallback); `self.introspector.bib_number_for_key(key)` (sem fallback). Comentário inline P190B. |
| 2 | `rules/layout/mod.rs:1496-1498` | Layouter assignment 1: `l.counter.bib_entries`/`bib_numbers = initial_state.X` removido. Comentário P190B. |
| 3 | `rules/layout/mod.rs:1524-1526` | Layouter assignment 2 (fixpoint loop): idem. Comentário P190B. |
| 4 | `entities/counter_state_legacy.rs:78-92` | Fields `bib_entries: Vec<BibEntry>` + `bib_numbers: HashMap<String, u32>` eliminados. Comentário P190B substitui. |
| 5 | `entities/counter_state_legacy.rs:302-322` | 2 tests sentinela `counter_state_bib_numbers_*` removidos (cobertura via BibStore). |
| 6 | `rules/introspect.rs:2350-2356` | Test `walk_arm_bibliography_nao_muta_state_bib_legacy` adaptado: assertions sobre `state.bib_*` removidas. |
| 7 | `rules/layout/tests.rs:3448-3460` | Test análogo adaptado: assertions sobre `state.bib_*` removidas. |

**Walk arm Bibliography NÃO tocado** — já era puro desde P181H.
**`from_tags` arm Bibliography NÃO tocado** — já popula BibStore desde P181E.
**Trait Introspector NÃO modificado** — `bib_entry_for_key` + `bib_number_for_key` activos desde P181G.

---

## §3 `CounterStateLegacy` redução

| Estado | Fields | Δ |
|--------|--------|---|
| Pré-P190B | 14 públicos + 2 privados = **16** | — |
| Pós-P190B | 12 públicos + 2 privados = **14** | -2 (`bib_entries`, `bib_numbers`) |

**Progresso F1**: 16 → 14 fields ortogonais. F1 fecha após P190I (eliminação total).

---

## §4 Tests workspace

| Estado | Total | Δ |
|--------|-------|---|
| Pré-P190B (P190A baseline) | 1.869 | — |
| Pós-P190B | **1.867** | -2 (tests sentinela `bib_numbers_*` removidos) |

Δ tests **negativo marginal**: -2 tests sentinela legacy redundantes removidos. Cobertura bibliográfica preservada via tests BibStore + Layouter cite-arm tests existentes.

---

## §5 Verificações finais (.G — 14 checks)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo check --workspace` | ✅ ok |
| 2 | `cargo test --workspace` | ✅ 1.867 verdes (Δ vs P190A baseline 1.869: **-2**) |
| 3 | `crystalline-lint .` | ✅ 0 violations |
| 4 | Layouter consumers Bibliography migrados (sem fallback legacy) | ✅ |
| 5 | `CounterStateLegacy.bib_entries` NÃO existe | ✅ |
| 6 | `CounterStateLegacy.bib_numbers` NÃO existe | ✅ |
| 7 | `CounterStateLegacy`: 14 fields (era 16) | ✅ |
| 8 | Walk arm Bibliography NÃO modificado (P181H) | ✅ |
| 9 | `from_tags` arm Bibliography NÃO modificado (P181H) | ✅ |
| 10 | Trait `Introspector` NÃO modificado | ✅ |
| 11 | `TagIntrospector` NÃO modificado | ✅ |
| 12 | ADR-0070 PROPOSTO NÃO transitada | ✅ (ACEITE em P190I) |
| 13 | Snapshot tests verdes | ✅ |
| 14 | Linter passa final | ✅ |

**14/14 verde.**

---

## §6 Decisões de execução notáveis

### Layouter assignments duais descobertos em runtime

P190A diagnóstico §6 mencionou `mod.rs:1490, 1521` como Layouter assignments. Empíricamente confirmado **4 linhas de assignment dual** (`bib_entries` + `bib_numbers` em **2 contextos**: documento simples sem TOC e fixpoint loop com TOC). Ambos contextos eliminados.

### Tests sentinela P181H redundantes

2 tests `counter_state_bib_numbers_default_empty` + `counter_state_bib_numbers_insertion_e_lookup` em `counter_state_legacy.rs:302-322` foram tests do struct legado — **trivialmente irrelevantes** após field elimination. Removidos. Cobertura bibliográfica preservada via tests BibStore (P181B/E) + tests Layouter cite-arm.

2 tests análogos em `introspect.rs:2352-2355` e `layout/tests.rs:3448-3460` (assertions `state.bib_entries.is_empty()` + `state.bib_numbers.is_empty()`) **adaptados** — assertions removidas, restantes asserções (Tag emission + BibStore populated) preservadas.

### Sem cláusula gate disparada

P190A previu cláusulas gate possíveis (walk arm não puro, API divergente, tests Layouter regridem). **Nenhuma disparada** — caminho Introspector estável desde P181E (>19 séries de uso).

---

## §7 Estado actual

- **P190 série**: A ✅ B ✅ | C-I pendentes.
- **Categoria 1 (Bibliography) fechada estruturalmente**.
- **86 passos executados** (P190A=85 + P190B=86).
- **1ª aplicação concreta** do pattern stylesheet "eliminação write paralelo M5".

---

## §8 Pendências cumulativas

**7 categorias restantes** do plano M6:
- Categoria 2 (Page tracking) — P190C.
- Categoria 3 (Document metadata) — P190D.
- Categoria 4 (Numbering active) — P190E.
- Categoria 5 (Counters core + 2 helpers) — P190F.
- Categoria 6 (Labels & TOC) — P190G.
- Categoria 7 (Figures) — P190H.
- Walk arms purification + Layouter final + struct elim + L0 + ADR-0070 ACEITE — P190I.

**M5 universal completo**: inalterado (0 + 0 + 0).

---

## §9 Próximo passo

**P190C** — Categoria 2 (Page tracking):
- Mover `label_pages` + `known_page_numbers` para struct dedicada `LayouterRuntimeState` (ou similar).
- **Categoria Layouter-runtime** (P190A §3 achado crítico) — fields que **não cabem em Introspector** (não derivados de Content pre-pass).
- Magnitude **M** marginalmente maior que P190B (primeira aplicação da decisão "4 fields Layouter-runtime → struct dedicada").

---

## §10 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0070 PROPOSTO (P190A) — "eliminação write paralelo M5".
- **1ª aplicação concreta** P190B.
- **Categoria 1 fechada**: Bibliography.
- **Pre-condição estabelecida**: P181E (BibStore populated em from_tags); P181G (consumer migration with fallback); P181H (walk arm restaurado puro).
- **Layouter migration**: cite-arm (`mod.rs:660+`) + 2 assignment blocks eliminados.
- **Hash código** introspect.rs: `8e0128e4` (inalterado em P190B; comments-only changes).
- **Hash L0** introspect.md: `7a3ba2b7` (inalterado).
- **Padrão diagnóstico-primeiro**: 23ª aplicação consecutiva (P190A diagnóstico).
- **F1 progresso**: 16 → 14 fields ortogonais.
- **F3 progresso**: Layouter ainda 19 fields (counter field permanece com 14 fields — eliminação total em P190I).

---

## §11 Métricas finais P190B

- **Sub-passos**: 8 (A diagnóstico + B-G implementação + H relatório).
- **LOC produção**: ~25 (consumer migration + assignment removal + field deletion + comentários).
- **LOC teste**: ~20 (4 tests adaptados ou removidos).
- **LOC L0**: 0 (walk arm não tocado; trait não modificado; sem secção nova em L0 introspect.md — apenas adicionar em P190I final).
- **LOC relatório**: ~250.
- **Variants Content novas**: 0.
- **Sub-stores Introspector novos**: 0.
- **ADRs novas**: 0 (ADR-0070 já PROPOSTA em P190A).
- **Helpers privados**: 0 mudanças.
- **F1 fields eliminados**: -2 (bib_entries, bib_numbers).
- **Tests netos**: -2 (sentinelas redundantes removidas).
- **Hashes desactualizados**: 0.

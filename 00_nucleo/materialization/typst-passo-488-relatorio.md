# Relatório P488 — LoF/LoT page numbers (fixpoint) + corpus RTL

> **Passo:** 488
> **Data:** 2026-06-29
> **Estado:** COMPLETO — Trilha 6: 5/5 FECHADA

---

## Resultado final

| Indicador | Pré-P488 | Pós-P488 |
|-----------|----------|----------|
| `cargo test --workspace` | 3429 ok | 3435+526+24+2+21+2 ok — **0 falhas** |
| `crystalline-lint .` | 0 violations | **0 violations** |
| Corpus parity | 46 ficheiros | **48 ficheiros** (+2 RTL) |
| Trilha 6 | 4/5 | **5/5 COMPLETA** |
| Testes P488 | — | **7 novos testes** |

---

## Sub-item A — LoF/LoT com page numbers reais

### ADR-0108 — Sondas antes de decidir

| Pergunta | Medição | Resultado |
|----------|---------|-----------|
| `native_lof()` cria que Content? | `stdlib/structural.rs` | `Content::Outline { target: OutlineTarget::Figures }` |
| Fixpoint activa apenas para `ElementKind::Outline`? | `layout/mod.rs` | Sim — e `lof()`→`Outline` já regista `ElementKind::Outline`; condição cobre LoF/LoT sem alteração |
| `figures_for_lof()` existe em `TagIntrospector`? | `entities/introspector.rs` | Sim — `Vec<(usize, String)>` em ordem de documento |
| Figuras têm `label_pages`? | `rules/layout/references.rs` | Não — figuras não têm label explícito; `known_page_numbers` inapplicável |
| Campo para páginas em `LayouterRuntimeState`? | `entities/layouter_runtime_state.rs` | Ausente — adicionado em P488 |

**Decisão de arquitectura (ADR-0108 § classificação mecânica vs linguagem):** a spec propôs `Vec<(figure_n, page)>` no `Introspector`. Medição revelou potencial de mismatch de chave entre `"figure"` (key em `figures_for_lof`) e `"figure:image"` (key em `figure_number_at_index`). Adoptada alternativa mais segura: `Vec<usize>` em `LayouterRuntimeState` com matching posicional — ambas as fontes seguem ordem de documento.

### Divergências da spec L0

A spec propôs adicionar `record_figure_page` / `figure_page_numbers` ao trait `Introspector`. A implementação colocou os campos em `LayouterRuntimeState` (padrão arquitectural estabelecido em P190C: state populado durante layout → struct dedicada, não no Introspector). O Introspector é estado derivado do pre-pass walk — page numbers só existem em runtime de render.

### Ficheiros L0 actualizados

| Prompt L0 | Hash novo | Alteração |
|-----------|-----------|-----------|
| `prompts/entities/layouter_runtime_state.md` | — | +4 campos: `figure_page_numbers`, `table_page_numbers`, `known_figure_page_numbers`, `known_table_page_numbers` |
| `prompts/entities/layout_types.md` | — | +2 campos em `PagedDocument`: `extracted_figure_page_numbers`, `extracted_table_page_numbers` |
| `prompts/rules/layout_figure.md` | — | §P488: registo de página após cálculo de `figure_number` |
| `prompts/rules/layout/table.md` | — | §P488: registo de página quando `caption_prefix.is_some()` |
| `prompts/rules/layout_outline.md` | — | §P488: `layout_lof`/`layout_lot` com `known_*_page_numbers`; convergência alargada |

### Ficheiros L1 modificados

| Ficheiro | Alteração |
|----------|-----------|
| `entities/layout_types.rs` | `extracted_figure_page_numbers: Vec<usize>`, `extracted_table_page_numbers: Vec<usize>` |
| `entities/layouter_runtime_state.rs` | 4 novos campos Vec<usize> + módulo `p488_tests` (3 testes) |
| `rules/layout/figure.rs` | `layouter.runtime.figure_page_numbers.push(layouter.current_page_number())` |
| `rules/layout/table.rs` | `layouter.runtime.table_page_numbers.push(layouter.current_page_number())` |
| `rules/layout/outline.rs` | `layout_lof` e `layout_lot` com matching posicional em `known_*_page_numbers` |
| `rules/layout/mod.rs` | `finish()` extrai campos; fixpoint: injecta `known_*`, verifica convergência tripla |
| `rules/layout/tests.rs` | 3 novos testes P488 |

### Mecanismo fixpoint (P488)

```
Iteração 0: known_figure/table = []  → outline.rs produz entries sem página
            finish() → extracted_figure/table = [p1, p2, ...]

Iteração 1: known_figure/table = [p1, p2, ...]  → entries com página
            finish() → extracted_figure/table = [p1', p2', ...]

Convergência: extracted == known para labels + figure + table  → return
```

Nenhuma alteração à condição de activação do fixpoint foi necessária — `Content::lof()` → `Content::Outline` → `ElementKind::Outline` já activa o loop.

---

## Sub-item B — Corpus RTL

### Ficheiros criados

| Ficheiro | Língua | Conteúdo |
|----------|--------|---------|
| `lab/parity/corpus/rtl/arabic_basic.typ` | Árabe | heading + parágrafo (4 linhas) |
| `lab/parity/corpus/rtl/hebrew_basic.typ` | Hebraico | heading + parágrafo (4 linhas) |

### Classificação na suite

**SkipFeature** — `#set text(dir: rtl)` não implementado em stdlib cristalina (scope-out permanente até P489+). Ficheiros presentes para: (a) declarar intenção de cobertura RTL; (b) validar que o shaper L3 de P484 é coerente com o corpus; (c) servir como referência para implementação futura de `dir: rtl` em stdlib.

Cobertura real RTL: 6 testes unitários L3 de P484 (`p484_bidi_runs_arabico_rtl`, `p484_mixed_rtl_ltr_text`, etc.) validam o shaper `unicode-bidi`.

### Alterações em `structural_parity.rs`

- `"rtl"` adicionado ao array de categorias em `read_corpus()`
- Regra `SkipFeature` para `category == "rtl"` em `etiqueta_for()`
- Sentinela `p488_parity_corpus_48_ficheiros_rtl_skipfeature` adicionado
- Todas as asserções `corpus.len() == 46` actualizadas para 48 (7 sentinelas históricas)

---

## Testes P488

| Teste | Ficheiro | Verifica |
|-------|---------|---------|
| `p488_layouter_runtime_figure_page_numbers_default_vazio` | `layouter_runtime_state.rs` | 4 campos Vec iniciam vazios |
| `p488_record_figure_page_armazena_page_number` | `layouter_runtime_state.rs` | push em `figure_page_numbers` preserva ordem |
| `p488_known_figure_page_numbers_injectado_e_lido` | `layouter_runtime_state.rs` | `known_figure_page_numbers` injectado por índice |
| `p488_lof_sem_known_pages_usa_formato_sem_numero` | `tests.rs` | LoF sem known pages → "Figure N  Caption" sem número de página |
| `p488_lof_com_figura_regista_page_number_no_extracted` | `tests.rs` | Figura layoutada → `extracted_figure_page_numbers[0] == 1` |
| `p488_extracted_table_page_numbers_default_vazio` | `tests.rs` | Sem tabelas contadas → `extracted_table_page_numbers` vazio |
| `p488_parity_corpus_48_ficheiros_rtl_skipfeature` | `structural_parity.rs` | Corpus tem 48 ficheiros; RTL é SkipFeature documentado |

---

## Scope-out confirmado

- Formatação avançada de LoF/LoT (pontos de preenchimento, alinhamento de page numbers).
- Figuras sem caption omitidas de LoF.
- Page numbers exactos em tabelas multi-coluna (regista página do topo).
- Corpus RTL com múltiplos elementos (corpus minimal 1–2 headings por ficheiro).
- `#set text(dir: rtl)` em stdlib.
- `y_offset` em emit PDF (scope-out permanente P486).

---

## Trilha 6 — Estado final

| Item | Estado |
|------|--------|
| 1/5 — `#lof()` / `#lot()` stdlib functions | ✅ P472 |
| 2/5 — `OutlineTarget::Figures` / `Tables` em outline.rs | ✅ P472 |
| 3/5 — `figures_for_lof()` / `tables_for_lot()` em TagIntrospector | ✅ P472 |
| 4/5 — `layout_lof` / `layout_lot` (sem page numbers) | ✅ P472 |
| 5/5 — Page numbers reais via fixpoint carry-forward | ✅ **P488** |

**Trilha 6: COMPLETA.**

---

## Validação final

```
cargo test --workspace  → 3435+526+24+2+21+2 ok, 0 failed
crystalline-lint --fix-hashes .  → Fixed 4 files (V5 drift pós-L0 update)
crystalline-lint .  → ✓ No violations found
```

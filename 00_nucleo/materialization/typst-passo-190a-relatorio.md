# Relatório P190A — Diagnóstico eliminação `CounterStateLegacy`

**Data**: 2026-05-04
**Magnitude**: S-M (mais que passos anteriores — inventário extenso).
**Estado**: Completo.
**Marco arquitectural relevante**: M5 universal completo (P200B); início M6.
**Resultado ADR**: ADR-0070 PROPOSTO (será criado em P190A.N).

---

## §1 Sumário executivo

P190A audita o último marco da arquitectura cristalina M5
após 22 séries de execução: **eliminação total de
`CounterStateLegacy`** + cleanup do write paralelo M5 +
migração final de helpers e Layouter consumers.

Auditoria empírica revela:
- **16 fields** no struct (14 públicos + 2 privados) — F1 estimou 18+2; correcção -2.
- **8 métodos públicos** — F1 estimou 25; correcção -17.
- **API interna** ao crate — sem re-export em `lib.rs`; eliminação livre.
- **10 ocorrências** `self.counter.X` em Layouter (`equation.rs:3` + `mod.rs:7+`).
- **>14 mutações legacy** em walk arms migrados em M5.
- **4 helpers `compute_*`** — 2 elimináveis (walk-internal), 2 migráveis para Introspector path location-aware.

P190 implementação granular em **8 sub-passos** (estratégia β
incremental por categoria):

| Sub | Categoria | Magnitude |
|-----|-----------|-----------|
| P190B | Bibliography | M |
| P190C | Page tracking | M |
| P190D | Document metadata | M |
| P190E | Numbering active | M |
| P190F | Counters core + 2 helpers | M+ |
| P190G | Labels & TOC | M |
| P190H | Figures | M |
| P190I | Walk arms purification + Layouter final + struct elim + L0 + ADR-0070 ACEITE | M+ |

**Total agregado**: **L cross-modular**.

Estado projectado pós-P190: **F1 fechado**; **F3 parcialmente fechado** (Layouter perde 1 field); walk puro; struct eliminado; 4 helpers eliminados (2) ou migrados (2); Layouter migrado para Introspector path completo.

---

## §2 Contexto

P190 inicia **M6** após M5 universal completo (P200B). Marco pre-condição:
- 9 séries materializadas P189B-P200.
- 5 variantes operacionais ADR-0069 consolidadas.
- 7 aplicações ADR-0069 stylesheet.
- 4 helpers privados família ADR-0069.
- Trait Introspector: 20 métodos.
- TagIntrospector: 9 sub-stores.
- ElementPayload: 13 variants.
- ElementKind: 10.
- Content: + 1 variant em P199B.

Trabalho M6: cleanup do write paralelo M5 (mutações legacy preservadas em walk arms) + eliminação de struct + migração final de Layouter.

**Diferença face a séries P181-P200**: P190 é **categoria nova de trabalho** sem precedente directo. Pattern stylesheet é **eliminação de write paralelo M5** — recíproco à pattern ADR-0069 stylesheet (que adicionou caminho Introspector em paralelo a legacy).

---

## §3 Inventário 16 campos

### Privados (2)

| # | Campo | Type | Cobertura |
|---|-------|------|-----------|
| 1 | `hierarchical` | `HashMap<String, Vec<usize>>` | ✅ via CounterRegistry (`apply_hierarchical_at`) |
| 2 | `flat` | `HashMap<String, usize>` | ✅ via CounterRegistry (`apply_at`, `flat_counter_at`) |

### Públicos (14)

| # | Campo | Cobertura | Categoria |
|---|-------|-----------|-----------|
| 1 | `numbering_active` | ✅ StateRegistry | Numbering active |
| 2 | `resolved_labels` | ✅ ResolvedLabelStore | Labels & TOC |
| 3 | `headings_for_toc` | ✅ headings_for_toc store (P200B) | Labels & TOC |
| 4 | `auto_label_counter` | ⚠️ walk-internal (local) | Labels & TOC |
| 5 | `label_pages` | ❌ Layouter-only | Page tracking |
| 6 | `known_page_numbers` | ❌ Layouter-only | Page tracking |
| 7 | `has_outline` | ✅ kind_index[Outline] | Document metadata |
| 8 | `is_readonly` | ❌ Layouter runtime (DEBT-13) | Document metadata |
| 9 | `figure_numbers` | ✅ CounterRegistry figure:{kind} | Figures |
| 10 | `figure_label_numbers` | ✅ figure_label_numbers store | Figures |
| 11 | `local_figure_counters` | ⚠️ walk-internal | Figures |
| 12 | `lang` | ❌ config field externo | Document metadata |
| 13 | `bib_entries` | ✅ BibStore | Bibliography |
| 14 | `bib_numbers` | ✅ BibStore | Bibliography |

**Achado crítico**: 4 campos genuinamente Layouter-runtime (`label_pages`, `known_page_numbers`, `is_readonly`, `lang`) — não cabem em Introspector (não derivados de Content pre-pass). Devem mover para struct dedicada `LayouterRuntimeState` ou similar.

---

## §4 Inventário 8 métodos

100% substituíveis por trait methods Introspector existentes:
- `is_numbering_active` → `is_numbering_active_at` (P185B).
- `step_hierarchical` → `apply_hierarchical_at` (interno from_tags).
- `format_hierarchical` → `formatted_counter` (P170) ou `formatted_counter_at` (P185B).
- `step_flat` → `apply_at(Step)` (interno from_tags).
- `update_flat` → `apply_at(Update)` (interno from_tags).
- `get_flat` → `flat_counter_at` (P185B).
- `display_value` → wrapper sobre `formatted_counter`.
- `new()` → eliminado com struct.

---

## §5 Inventário 4 helpers `compute_*`

| Helper | Decisão M6 |
|--------|------------|
| `compute_labelled` | **Migrar** para Introspector path location-aware |
| `compute_heading_auto_toc` | **Migrar** para Introspector path location-aware |
| `compute_figure` | **Eliminar** (walk-internal) |
| `compute_heading_for_toc` | **Eliminar** (walk-internal) |

---

## §6 Inventário Layouter consumers (10 ocorrências)

`grep self.counter.X` em `01_core/src/rules/layout/`:
- `equation.rs:33, 35, 109` (3).
- `mod.rs:328, 343, 356, 499, 665, 673, 1136` (7).

**Inclui mutações Layouter próprias** (`equation.rs:35`, `mod.rs:328`) — eliminar; counter populado pelo walk pre-pass.

---

## §7 Decisões cláusula 1–9

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Estratégia | β — incremental por categoria (8 sub-passos) |
| 2 | Ordem | Bibliography → Page tracking → Document metadata → Numbering active → Counters core+helpers → Labels & TOC → Figures → Walk purification + Layouter final + struct elim |
| 3 | Forma final | α — eliminação total (com excepção pragmática: 4 fields Layouter-runtime movem para `LayouterRuntimeState`) |
| 4 | API pública | Interna ao crate — eliminação livre, não breaking change |
| 5 | Helpers | 2 eliminados (walk-internal) + 2 migrados (Introspector path) |
| 6 | Layouter | Field `counter` substituído por `runtime: LayouterRuntimeState`; mutações Layouter eliminadas; reads via Introspector |
| 7 | Walk arms | 14 mutações legacy eliminadas; walk torna-se puro |
| 8 | Tests | Padrão pragmático auditor #1; tests sentinela legacy preservados como histórico ou removidos quando redundantes |
| 9 | Critério fecho | `grep CounterStateLegacy` retorna zero; F1 fechado; walk puro; Layouter migrado |

---

## §8 Magnitude consolidada

- P190A diagnóstico: S-M.
- P190B-P190I implementação: 8 × M = L cross-modular.
- ADR-0070 PROPOSTO em P190A.N; ACEITE após P190I.

Total agregado: **L cross-modular** com componentes:
- ~120-200 LOC eliminadas (struct + 8 methods + 2 helpers + 14 walk mutações).
- ~80-150 LOC adicionadas (consumer migrations + LayouterRuntimeState struct + 2 helpers migrados).
- ~50-100 LOC tests adaptadas/eliminadas.

---

## §9 ADR-0070 PROPOSTO

**Título**: "Eliminação `CounterStateLegacy` — fim de M5 universal completo".

**Estado**: PROPOSTO em P190A. ACEITE após P190 série fechar (P190I).

**Contexto**: M5 universal completo P200B; write paralelo M5 ainda activo; struct legado de 16 fields + 8 métodos com cobertura Introspector ~75% (12/16 fields cobertos; 4 fields Layouter-runtime).

**Decisão**: eliminação total via 8 sub-passos incrementais por categoria (estratégia β); 4 fields Layouter-runtime movem para struct dedicada; 2 helpers walk-internal eliminados; 2 helpers migram para Introspector path location-aware; 14 mutações walk arm eliminadas; Layouter migrado para Introspector path completo.

**Consequências**:
- F1 fecha (`CounterStateLegacy` eliminado).
- F3 parcialmente fecha (Layouter -1 field).
- Walk torna-se puro (sem mutações).
- Trait Introspector estável (20 métodos).
- TagIntrospector estável (9 sub-stores).

**Alternativas avaliadas**:
- Incremental por field (descartada — 16+ sub-passos demasiado pequenos).
- Big-bang (descartada — risco substancial L+ num único passo).
- Façade temporário (descartada — adia problema).
- Rename (descartada — esconde problema).

**Pattern stylesheet**: "eliminação write paralelo M5" — recíproco à pattern ADR-0069 (que adicionou caminho Introspector em paralelo a legacy). Sem nova variante operacional ADR-0069.

---

## §10 DEBT M5-residual + DEBT M6 documentação

**DEBT M5-residual**: M5 universal completo desde P200B. **0 excepções activas + 0 residuos + 0 pré-requisitos**.

**DEBT M6 documentação** (P200C §8) **fecha por execução em P190 série**.

**F1 fecha após P190I**.
**F3 parcialmente fecha após P190I** (Layouter perde 1 field; restantes 18 fields são responsabilidade de M7+).

---

## §11 Regra dos 2 eixos aplicada por campo

Tabela completa em `diagnostico-eliminacao-counter-state-legacy-passo-190a.md` §11. Resumo:
- **Eixo 1 (consumer durante walk)**: 7/16 campos têm consumer durante walk (helpers ou walk arms internos).
- **Eixo 2 (sub-store activo)**: 12/16 campos têm cobertura Introspector activa; 4 campos Layouter-runtime sem cobertura (mover para `LayouterRuntimeState`).

---

## §12 Próximo sub-passo concreto

**P190B — Categoria Bibliography**:

1. Confirmar empíricamente walk arm Bibliography puro desde P181H.
2. Migrar Layouter consumer (`mod.rs:665, 673`):
   - `self.counter.bib_entries.iter().find(...)` → `self.introspector.bib_store.lookup(...)`.
   - `self.counter.bib_numbers.get(...)` → `self.introspector.bib_store.assigned_number(...)`.
3. Eliminar fields `bib_entries` + `bib_numbers` de `CounterStateLegacy`.
4. Adaptar tests Layouter dependentes.
5. L0 actualizado.
6. Tests workspace verdes.
7. Hash actualizado via `crystalline-lint --fix-hashes`.

**Critério de fecho P190B**: tests workspace 1869 verdes; lint zero violations; struct reduzido a 14 fields.

Magnitude **M** (categoria mais simples — caminho Introspector activo desde P181H; consumer migration directa).

---

## §13 Restrições mantidas

- ✅ Zero código tocado em P190A.
- ✅ Zero testes modificados.
- ✅ Sem reservas de identificadores criadas.
- ✅ Walk não modificado.
- ✅ `from_tags` não tocado.
- ✅ Trait `Introspector` não modificado.
- ✅ `TagIntrospector` não modificado.
- ✅ Layouter não modificado.
- ✅ `CounterStateLegacy` não eliminado.
- ✅ 4 helpers `compute_*` não modificados.
- ✅ Lacunas residuais (#1, #1b, #2) não materializadas.
- ✅ Linguagem operacional sem inflação retórica.
- ✅ Regra dos 2 eixos aplicada empiricamente por campo.
- ✅ Pattern ADR-0069 reaproveitado (helpers; sub-stores).
- ✅ Plano P190B-P190I sem cláusulas condicionais.
- ✅ ADR-0070 PROPOSTO criado em P190A.N (next).
- ✅ DEBT M6 documentação registada (fecha por execução).

---

## §14 Linhagem

- **Marco pre-condição**: M5 universal completo (P200B).
- **Pattern stylesheet**: "eliminação write paralelo M5" (recíproco à pattern ADR-0069).
- **Categorias arquitecturais**:
  - 6 categorias campos (Counters core, Numbering active, Labels & TOC, Page tracking, Document metadata, Figures, Bibliography).
  - 8 sub-passos B-I.
- **ADR**: ADR-0070 PROPOSTO em P190A.N; ACEITE após P190I.
- **F1**: fecha após P190.
- **F3**: parcialmente fecha após P190.
- **Trait Introspector**: estável (20 métodos).
- **TagIntrospector**: estável (9 sub-stores).
- **ElementPayload**: estável (13 variants).
- **ElementKind**: estável (10).
- **Content**: estável (13 variants após P199B).
- **Padrão diagnóstico-primeiro**: 23ª aplicação consecutiva.
- **Pattern stylesheet ADR-0069**: 7 aplicações consolidadas; sem nova aplicação em P190 (eliminação, não adição).
- **L0 alvo**: ~5-7 ficheiros (a confirmar empíricamente em P190B+).
- **Próximo sub-passo**: **P190B** — categoria Bibliography. Magnitude M.

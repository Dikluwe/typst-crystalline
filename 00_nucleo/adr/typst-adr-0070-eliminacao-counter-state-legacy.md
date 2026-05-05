# ADR-0070 — Eliminação `CounterStateLegacy` (fim de M5 universal)

**Estado**: **ACEITE** (P190I — M6 fechado).
**Data**: 2026-05-04 (PROPOSTA P190A); 2026-05-05 (ACEITE P190I).
**Marco arquitectural pre-condição**: M5 universal completo (P200B).
**Marco arquitectural pós-condição**: **M6 fechado**; F1 fechado; F3 parcialmente fechado.

---

## Validação empírica P190I

| Aspecto | Pré-P190 | Pós-P190I | Estado |
|---------|----------|-----------|--------|
| `CounterStateLegacy` fields | 16 | **0 (struct eliminada)** | ✅ |
| Walk fn `state` parameter | sim | **eliminado** | ✅ |
| Walk fn signature | 5 params | 7 params (+intr +auto_label_counter +lang -state) | ✅ |
| Layouter `counter` field | 19/20 fields | **eliminado** | ✅ |
| Layouter consumers self.counter | 10+ | **0** | ✅ |
| Helpers eliminados | — | `compute_figure` (P190H); `layout_set_*`, `layout_counter_update`, `format_counter_display` (P190G/I) | ✅ |
| API pública | `(state, intr)` | `intr` | breaking change ✅ |
| Pattern "eliminação write paralelo M5" | — | **8 aplicações** (B-I) | ✅ |
| Pattern stylesheet "Layouter-runtime → struct dedicada" | — | **2 aplicações** (P190C, P190D) | ✅ |
| Pattern "1ª aplicação directa ADR-0071 em P190" | — | **3 aplicações** (G, H, I) | ✅ |
| Tests workspace | 1812 | 1802 | Δ -10 marginal ✅ |
| Linter | 0 violations | 0 violations | ✅ |

---

## Contexto

Após **22 séries de execução** (P181-P200), o projecto cristalino atingiu **M5 universal completo** em P200B: todos walk arms fechados estruturalmente, 0 excepções activas, 0 residuos, 0 pré-requisitos. Pattern ADR-0069 stylesheet aplicado em 7 séries consecutivas com 5 variantes operacionais consolidadas e 4 helpers privados.

**`CounterStateLegacy`** (`01_core/src/entities/counter_state_legacy.rs`) — struct unificada de 16 fields (14 públicos + 2 privados) e 8 métodos, originalmente desenhada como state mutable durante o walk pre-pass. Após M5 universal completo, esta struct contém **write paralelo M5 ainda activo**:
- Mutações legacy preservadas em todos walk arms migrados em M5.
- 4 helpers privados `compute_*` leem state legacy durante walk.
- Layouter assignments duais (`mod.rs:1490, 1521`) fazem `l.counter = initial_state.X.clone()`.
- Layouter consumers fazem `self.counter.X` em 10 ocorrências.

**Inventário pós-P200B**:
- 12/16 campos têm cobertura Introspector activa (CounterRegistry, StateRegistry, ResolvedLabelStore, headings_for_toc store, BibStore, kind_index).
- 4/16 campos são genuinamente Layouter-runtime (`label_pages`, `known_page_numbers`, `is_readonly`, `lang`) — não derivados de Content pre-pass; não cabem em Introspector.
- 8/8 métodos têm equivalente Introspector via 20 trait methods.

**API**: `pub mod counter_state_legacy;` em `entities/mod.rs:32` — interno ao crate `typst-core`. Sem re-export em `lib.rs`. **Eliminação livre** — não breaking change para consumers externos.

**F1** (auditoria-fresh-projecto.md) e **F3** (Layouter 19 fields herdados) ficam parcialmente fechados após eliminação.

---

## Decisão

**Eliminar `CounterStateLegacy` totalmente** via 8 sub-passos incrementais por categoria (estratégia β):

1. **P190B — Bibliography**: eliminar `bib_entries`, `bib_numbers`; migrar Layouter consumer (`mod.rs:665, 673`) para `intr.bib_store` queries.
2. **P190C — Page tracking**: mover `label_pages`, `known_page_numbers` para struct dedicada `LayouterRuntimeState`.
3. **P190D — Document metadata**: eliminar `has_outline` (substituído por `intr.kind_index[Outline]`); mover `is_readonly` + `lang` para `LayouterRuntimeState`.
4. **P190E — Numbering active**: eliminar `numbering_active`; migrar consumers Layouter + walk arm Equation gate; eliminar mutações walk arms SetHeading/SetEquation.
5. **P190F — Counters core + 2 helpers**: migrar `compute_labelled` + `compute_heading_auto_toc` para Introspector path location-aware (`intr.flat_counter_at`, `intr.formatted_counter_at`); eliminar `flat`, `hierarchical`; eliminar 6 dos 8 métodos.
6. **P190G — Labels & TOC**: eliminar `resolved_labels`, `headings_for_toc`, `auto_label_counter`; eliminar `compute_heading_for_toc` (walk-internal); migrar consumer outline.
7. **P190H — Figures**: eliminar `figure_numbers`, `figure_label_numbers`, `local_figure_counters`; eliminar `compute_figure` (walk-internal); migrar Layouter consumer (`mod.rs:499`).
8. **P190I — Walk arms purification + Layouter final + struct elim**: eliminar 14 mutações legacy em walk arms; substituir Layouter field `counter` por `runtime: LayouterRuntimeState`; eliminar struct `CounterStateLegacy` + 8 métodos restantes; cleanup L0; relatório consolidado P190; ADR-0070 ACEITE.

**Pattern stylesheet**: "eliminação write paralelo M5" — recíproco à pattern ADR-0069 stylesheet (que adicionou caminho Introspector em paralelo a legacy). **Sem nova variante operacional ADR-0069** — eliminação é cleanup, não adição.

---

## Consequências

### Positivas

- **F1 fecha** — `CounterStateLegacy` eliminado.
- **F3 parcialmente fecha** — Layouter perde 1 field embebido (`counter`).
- **Walk torna-se puro** — 14 mutações legacy eliminadas em walk arms.
- **DEBT M6 documentação** (P200C §8) **fecha por execução**.
- **Eliminação ~120-200 LOC** (struct + 8 methods + 2 helpers + 14 walk mutações).
- **Trait Introspector estável** (20 métodos).
- **TagIntrospector estável** (9 sub-stores).

### Neutras

- **`LayouterRuntimeState` struct nova** — 4 fields Layouter-runtime preservados num agrupamento dedicado. Não é regressão; é arrumação semântica.
- **2 helpers migrados** (`compute_labelled`, `compute_heading_auto_toc`) para Introspector path location-aware — complexidade marginalmente maior na implementação dos helpers (location parameter), mas elimina dependência de state legacy.

### Riscos

- **Risco moderado-alto**: P190 é trabalho cross-modular sem precedente directo no projecto. Cláusulas gate substanciais prováveis durante implementação.
  - **Mitigação**: estratégia β (incremental por categoria) reduz risco face a γ (big-bang). Cada sub-passo categoria fecha tests workspace verdes antes de prosseguir.

- **Risco de regressão Layouter**: 10 ocorrências `self.counter.X` migrar para `self.introspector.X` ou `self.runtime.X`. Mutações próprias do Layouter (`equation.rs:35`, `mod.rs:328`) eliminadas — counter populado pelo walk pre-pass.
  - **Mitigação**: tests Layouter exhaustivos (1.869 baseline); auditoria empírica em cada sub-passo; snapshot tests para regressões visuais.

- **Risco em consumer migration**: ~50-80 tests dependentes directa ou indirectamente.
  - **Mitigação**: padrão pragmático auditor #1; tests adaptados conforme `self.counter.X` for substituído.

---

## Alternativas avaliadas

### α — Incremental por field (descartada)

Cada um dos 16 fields migra independentemente. **16+ sub-passos** pequenos.

**Razões para descartar**:
- Sub-passos demasiado pequenos perdem coerência arquitectural.
- Categorias têm dependências cruzadas (helpers leem múltiplos fields).
- Magnitude agregada idêntica a β mas granularidade inadequada.

### γ — Big-bang (descartada)

Toda a eliminação numa série única L+.

**Razões para descartar**:
- Risco substancial — eliminar 16 fields + 8 methods + 4 helpers + Layouter migration + 14 walk mutações simultaneamente é alta probabilidade de cláusula gate substancial.
- Sem precedente directo no projecto para trabalho cross-modular desta magnitude num único passo.
- Dificulta debug de regressões — mudanças simultâneas em múltiplas dimensões mascaram causas.

### Façade temporário (descartada)

`CounterStateLegacy` torna-se struct vazia implementando `Deref<Target = TagIntrospector>` ou similar.

**Razões para descartar**:
- Adia o problema sem o resolver.
- Engana — código continua a usar `CounterStateLegacy::X` que delega; aparenta state legacy ainda activo.
- Acumula DEBT em vez de eliminar.

### Rename (descartada)

Renomear `CounterStateLegacy` para `IntrospectionState` ou similar.

**Razões para descartar**:
- Esconde o problema sem o resolver.
- Mantém 16 fields heterogéneos sem categorização.
- Não fecha F1 — só dá nome diferente.

---

## Estado de execução

**P190A (este passo)** — diagnóstico completo:
- Inventário 16 fields + 8 métodos + 4 helpers + 14 walk mutações + 10 Layouter consumers + ~50-80 tests dependentes.
- 9 cláusulas fechadas com decisão literal.
- Plano 8 sub-passos B-I.
- ADR-0070 PROPOSTO (este ficheiro).

**P190B-P190I (futuro)** — implementação:
- 8 sub-passos × M = L cross-modular agregado.
- Cada sub-passo fecha tests workspace verdes antes de prosseguir.

**P190 série fechada** — ADR-0070 ACEITE; F1 fechado; F3 parcialmente fechado.

---

## Cross-references

- **P189B** — declaração das 6 excepções M5 + Reserva 1 + lacuna #3.
- **P200B** — marco M5 universal completo.
- **P200C §8** — DEBT M6 documentação registada.
- **F1** (auditoria-fresh-projecto.md) — `CounterStateLegacy` 16 fields heterogéneos identificado (correcção -2 vs F1 original).
- **F3** (auditoria-fresh-projecto.md) — Layouter 19 fields herdados.
- **ADR-0068** — Layouter location-aware (PROPOSTO em P185A; ACEITE após série P185-P188).
- **ADR-0069** — Post-recursion Tag emission (PROPOSTO em P195B; ACEITE em P195E). Pattern stylesheet aplicado em 7 séries consecutivas (P195D, P196B, P197B, P198B, P198C, P199B, P200B).
- **DEBT-12** (page tracking via known_page_numbers) — preservada em `LayouterRuntimeState`.
- **DEBT-13** (outline freeze via is_readonly) — preservada em `LayouterRuntimeState`.

---

## Padrão estabelecido

ADR-0070 estabelece **pattern stylesheet "eliminação write paralelo M5"** para futuras eliminações de structs legacy análogas. Pattern complementar a ADR-0069 (que adicionou caminho Introspector em paralelo a legacy):

| Pattern | ADR | Direção |
|---------|-----|---------|
| Adição write paralelo | ADR-0069 | Legacy + Introspector path em paralelo |
| **Eliminação write paralelo** | **ADR-0070** | **Eliminar legacy quando Introspector path estável** |

Aplicação futura: structs legacy análogas (se existirem em outros domínios) podem ser eliminadas seguindo padrão β-incremental por categoria documentado nesta ADR.

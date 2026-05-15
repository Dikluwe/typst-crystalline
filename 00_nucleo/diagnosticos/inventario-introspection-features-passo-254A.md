# Inventário factual Introspection — anexo Passo 254A

**Data**: 2026-05-15
**Tipo**: anexo factual ao diagnóstico principal
(`diagnostico-introspection-actualizado-passo-254A.md`)
**Função**: tabela exaustiva feature × estado × passo de
materialização × ficheiro.

---

## Tabela A — Features observable user-facing

| # | Feature | Estado | Passo materialização | Ficheiro principal | Notas |
|---|---------|--------|----------------------|--------------------|-------|
| 1 | `counter()` | implementado | P60-62 | `rules/introspect.rs` | Subset minimal single-pass; runtime queries adicionadas M9 |
| 2 | `state(key, init)` | implementado | P171 | `rules/introspect/from_tags.rs` + `entities/content.rs` (variant State + StateUpdate) | Runtime mutable; fixpoint convergence |
| 3 | `state.update(key, value)` | implementado | P171 | idem | Variant `StateUpdate` |
| 4 | `metadata(value)` | implementado | P169 | `entities/metadata_store.rs` + `entities/content.rs` (variant Metadata) | Append-only; query via `query_metadata()` |
| 5 | `here()` | implementado | série M9 | `rules/introspect/fixpoint.rs` | Position-aware |
| 6 | `locate(loc)` | implementado | série M9 | idem | Location lookup |
| 7 | `query(target)` | implementado | série M9 | `entities/introspector.rs` (TagIntrospector::query_*) | Por kind / por label / unique |
| 8 | `position(target)` | parcial | série M9 | idem | `position_of` retorna `Some` para subset; mapa positions ainda incompleto |
| 9 | `measure()` | implementado | P204G | `typst_infra::measurements` | Exposto stdlib L4 via P204G |
| 10 | `counter.at(loc)` | implementado | série M9 | `rules/introspect/fixpoint.rs` | Location-aware counter query |
| 11 | `counter.final()` | implementado | série M9 | idem | Valor final do counter |
| 12 | `counter.display(format)` | implementado | série M9 | idem | Formatting via numbering |
| 13 | cross-document cite refs | **ausente** | — | — | Requer pipeline multi-document; fora de Introspection puro |

**Total implementado/parcial**: 12/13 = ~92%.
**Total ausente**: 1/13 = ~8%.

---

## Tabela B — Features arquitecturais (engine, types, infrastructure)

| # | Componente | Estado | Passo | Ficheiro |
|---|------------|--------|-------|----------|
| 1 | `Introspector` trait | implementado | P165 | `entities/introspector.rs` |
| 2 | `TagIntrospector` struct | implementado | P165 | idem |
| 3 | `LabelRegistry` sub-store | implementado | série M3-M5 | `entities/label_registry.rs` (assumido) |
| 4 | `CounterRegistry` sub-store | implementado | série M3-M5 | `entities/counter_registry.rs` (assumido) |
| 5 | `MetadataStore` sub-store | implementado | P169 | `entities/metadata_store.rs` |
| 6 | `ResolvedLabelStore` sub-store | implementado | P193B | `entities/resolved_label_store.rs` |
| 7 | `Location` type | implementado | série M5 | (location.rs) |
| 8 | `is_locatable` pura | implementado | P164 | `rules/introspect/locatable.rs` |
| 9 | TOC fixpoint loop | implementado | pré-P192B (estruturalmente) | `rules/layout/mod.rs:1515` |
| 10 | `run_fixpoint` runtime | implementado | M7 (P175-P179) | `rules/introspect/fixpoint.rs` |
| 11 | `compute_tags_hash` convergence | implementado | M7 | `rules/introspect/fixpoint.rs` |
| 12 | `extract_payload` exaustivo | implementado | P162+ | `rules/introspect.rs` |
| 13 | `from_tags` construtor | implementado | série M3+ | `rules/introspect/from_tags.rs` |
| 14 | `CounterStateLegacy` | **removido** | P190I (M6) | (eliminado) |
| 15 | `comemo::Track` integração | **ausente** | — | M8 dedicado (futuro) |
| 16 | Re-walks parciais granulares | **ausente** | — | M8 dedicado |
| 17 | Multi-document state | **ausente** | — | Bloco C cross-módulo |

**Total implementado**: 13/17.
**Total ausente**: 3/17 (comemo, re-walks parciais, multi-doc).
**Total removido (legacy)**: 1/17.

---

## Tabela C — Variants Content locatable

| # | Variant | Locatable? | Passo origem | Passo locatable |
|---|---------|------------|--------------|-----------------|
| 1 | `Heading` | sim | pré-P156 | P164 baseline |
| 2 | `Figure` | sim | P75 | P164 baseline |
| 3 | `Cite` | sim | P159A | P164 baseline |
| 4 | `Metadata` | sim | P169 | P169 |
| 5 | `State` | sim | P171 | P171 |
| 6 | `StateUpdate` | sim | P171 | P171 |
| 7 | `Outline` | sim | pré | P178 |
| 8 | `Bibliography` | sim | P159A | P181D |
| 9 | `SetHeadingNumbering` | sim | P182C | P182C |
| 10 | `Equation` | sim | pré-P186 | P186D |
| 11-56 | 46 outros variants | não | vário | n/a |

**Total locatable**: 10/56 = ~18%.
**Match exaustivo garantido** em `is_locatable` — compile error
se variant novo for adicionado sem decisão explícita.

---

## Tabela D — Marcos arquitecturais Introspection

| Marco | Significado | Passo de fecho | Status |
|-------|-------------|----------------|--------|
| M1 | Walk DFS baseline + `extract_payload` | P162 | fechado pré-P164 |
| M2 | `is_locatable` extraído pura | P164 | fechado |
| M3 | `TagIntrospector` struct | P165 | fechado |
| M4 | `introspect_with_introspector` | (não claro do contexto disponível) | provavelmente fechado série M5 |
| M5 universal | Todos walk arms migrados | P200B | fechado (primeira vez desde P189B) |
| M6 | `CounterStateLegacy` eliminado | P190I | fechado |
| M7 | Fixpoint runtime estrutural | P192B | ACEITE com qualificação intermédia |
| M8 | `comemo::Track` adopção | (futuro) | **não iniciado** |
| M9 | Features stdlib runtime | 11/11 | fechado |

**Único marco não fechado**: M8 (comemo adopção).

---

## Tabela E — Comparação numérica P160 vs P254A

| Métrica | P160 (2026-04-25) | P254A (2026-05-15) | Δ |
|---------|-------------------|---------------------|---|
| Features observable implementadas | 1/13 | 12/13 | +11 |
| Features observable parciais | 1/13 | 1/13 | 0 |
| Features observable ausentes | 11/13 | 1/13 | -10 |
| Cobertura observable A.9 | ~17% | ~85-92% | +~70pp |
| Variants Content locatable | 3 (heading/figure/cite) | 10 | +7 |
| Marcos arquitecturais fechados | 0 | M2/M3/(M4)/M5/M6/M7/M9 = 6-7 | +6-7 |
| ADR-0066 status | (não criada) | ACEITE | promoção dupla |
| Linhas `rules/introspect/*` | 1108 (introspect.rs) | 1108+ + 626 (fixpoint.rs) + from_tags.rs + locatable.rs | ampliado |
| `CounterStateLegacy` | 333 linhas, 14 fields públicos | **eliminado** (M6) | -333 |

---

## Notas metodológicas

1. **Origem do número "~17%"** no resumo cumulativo pós-P254: o
   número provém literalmente de P160 §4, citado sem
   actualização. Não reflecte estado factual de 2026-05-15.

2. **Limitação deste inventário**: alguns passos da série M9
   (P175-P179) não estão totalmente expandidos no contexto
   disponível — estimativas para `here()`, `locate()`, `query()`
   detalhadas são derivadas de menções cruzadas em ADRs/L0
   prompts, não de leitura directa do código actual.

3. **Validação recomendada**: antes de actuar sobre as
   recomendações §6 do diagnóstico principal, verificar
   empiricamente o estado de `position_of()` (parcial) e a
   contagem real de tests `fixpoint.rs` E2E.

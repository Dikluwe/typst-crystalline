# P190 série — Pause/Resume Tracker

**Estado**: ✅ **PRONTO PARA RETOMAR** após P191C fechar (ADR-0071 ACEITE).
**Razão da pausa anterior**: barreira arquitectural identificada em P190F §3 — walk fn não tinha acesso a Introspector.
**Resolução**: P191 série completa — ADR-0071 ACEITE em P191C; mecanismo Opção A implementado e validado empíricamente.
**Próximo sub-passo**: **P190G** (Categoria 6 — Labels & TOC).

---

## P190 sub-passos executados

| Sub | Estado | Categoria | Δ fields | Notas |
|-----|--------|-----------|----------|-------|
| P190A | ✅ Completo | Diagnóstico | 0 | Plano β incremental + 8 sub-passos |
| P190B | ✅ Completo | Bibliography | -2 (`bib_entries`, `bib_numbers`) | 1ª aplicação ADR-0070 stylesheet |
| P190C | ✅ Completo | Page tracking | -2 (`label_pages`, `known_page_numbers`) | LayouterRuntimeState criada (2 fields); padrão "Layouter-runtime → struct dedicada" 1ª aplicação |
| P190D | ✅ Completo | Document metadata | -2 (`has_outline`, `is_readonly`) | LayouterRuntimeState ganha `is_readonly` (3 fields); `lang` defer |
| P190E | ✅ Completo | Numbering active (parcial) | 0 (defer) | Caso 1 — walk readers preservam write paralelo M5; field defer P190F |
| P190F | ⚠️ **Escopo reduzido** | Counters core (Layouter side apenas) | 0 (defer) | Barreira arquitectural identificada — P191 redesign needed |

**Total fields eliminados pré-pausa**: 6 (16 → 10).

---

## P191 série (ramo paralelo) — executada com sucesso

| Sub | Estado | Magnitude | Notas |
|-----|--------|-----------|-------|
| P191A | ✅ Completo | S-M | Diagnóstico — ADR-0071 PROPOSTO; 4 opções avaliadas; Opção A escolhida |
| P191B | ✅ Completo | M+ | Walk fn signature change + 25 call sites + 12 arms via populate_intr_from_tag_start + helper compute_heading_auto_toc migrado + walk arm Equation gate migrado + from_tags eliminado |
| P191C | ✅ Completo | S-M | Helper compute_labelled migrado (4 arms) + caller adapt + ADR-0071 ACEITE + tracker actualizado |

**ADR-0071**: PROPOSTA P191A → **ACEITE P191C** (validação empírica completa).

---

## P190 sub-passos pendentes

| Sub | Estado | Categoria | Pre-condição |
|-----|--------|-----------|--------------|
| **P190G** | 🟢 **PRONTO** | Labels & TOC (`auto_label_counter`, `resolved_labels`, `headings_for_toc`) | ✅ ADR-0071 ACEITE |
| **P190H** | ⏸️ Pendente | Figures (`figure_numbers`, `figure_label_numbers`, `local_figure_counters`) | P190G |
| **P190I** | ⏸️ Pendente | Walk arms purification + Layouter final + struct elim + ADR-0070 ACEITE | P190G + P190H |

---

## 4 defers acumulados durante P190

| Defer | Origem | Status pós-P191 | Resolução |
|-------|--------|-----------------|-----------|
| `lang` | P190D | resolvível via parameter passing (Opção β P191C) ou eliminação | P190G ou P190I |
| `numbering_active` | P190E | **resolvível** — `intr.is_numbering_active_at` activo | P190E continuação ou P190G |
| `flat` | P190F | **resolvível** — `intr.flat_counter_at` activo | P190G/H após |
| `hierarchical` | P190F | **resolvível** — `intr.formatted_counter_at` activo | P190G após |

**Mudança post-P191**: defers `numbering_active`, `flat`, `hierarchical`
agora têm path Introspector disponível para consumers walk-side
(via API location-aware P185B). Eliminação dos fields legacy
torna-se factível em P190G/H/I.

---

## Snapshot pós-P191

- Tests workspace: **1812** (lib).
- Linter: zero violations.
- `CounterStateLegacy`: 10 fields.
- `LayouterRuntimeState`: 3 fields.
- 94 passos executados (P191B=93 + P191C=94).
- Defers acumulados: 4 (todos com path Introspector resolvível).
- ADR-0071: ACEITE.

---

## Sequência projectada

1. ✅ **P190A-F** — pause após barreira identificada.
2. ✅ **P191A** — diagnóstico ADR-0071 PROPOSTO.
3. ✅ **P191B** — implementar Opção A; migrar 1 helper validation.
4. ✅ **P191C** — migrar 2º helper; **ADR-0071 ACEITE**.
5. 🟢 **P190G** — Labels & TOC; resolve defer `lang` (parcial). **PRONTO PARA RETOMAR**.
6. ⏸️ **P190H** — Figures.
7. ⏸️ **P190I** — Walk purification + struct elim + ADR-0070 ACEITE.

---

## Lembrete formal

**Ao iniciar P190G** (próximo passo concreto):
- ✅ ADR-0071 ACEITE.
- ✅ Mecanismo Opção A funcional (`intr: &mut TagIntrospector` no walk).
- ✅ 2 helpers walk-readers migrados (`compute_heading_auto_toc`,
  `compute_labelled`).
- ✅ Walk arm Equation gate via `is_numbering_active_at`.
- ✅ `from_tags::from_tags` eliminado.
- 🟢 Continuar P190 com escopo aplicável (sem barreira).
- 🟢 4 defers resolvíveis via Introspector path location-aware.

---

## Marco arquitectural

Após P191C (este passo):
- **F1 desbloqueado** — pendente apenas P190G/H/I para fechar.
- **F3 parcialmente desbloqueado** — Layouter pode eliminar campo
  `counter` em P190I.
- **5 ADRs ciclo M5/M6**: ADR-0068 (location-aware ACEITE),
  ADR-0069 (write paralelo ACEITE), ADR-0070 (eliminação write
  paralelo PROPOSTO; ACEITE em P190I), **ADR-0071 (walk pipeline
  redesign ACEITE em P191C)**.
- **M5 universal completo** mantido.
- **M6 desbloqueado** — barreira P190F resolvida.
- Após P190G/H/I executados → **F1 fecha**, **F3 parcialmente
  fecha**. Desbloqueia M7 (loop fixpoint), M8 (memoização comemo).

---

## Próximo passo concreto

**P190G** — Categoria 6: Labels & TOC.
Magnitude esperada: M.
Trabalho:
- Migrar consumers walk-side dos 3 fields restantes da categoria
  (`auto_label_counter`, `resolved_labels`, `headings_for_toc`)
  para Introspector path location-aware ou eliminar fields se
  consumers walk migrados em P191.
- 1ª aplicação directa do mecanismo P191/ADR-0071.
- Resolve parcialmente defer `lang`.

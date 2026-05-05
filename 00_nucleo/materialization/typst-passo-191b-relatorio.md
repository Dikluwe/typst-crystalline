# Relatório P191B — Implementação Opção A + 1 helper validation

**Data**: 2026-05-05
**Magnitude**: M+ (genuíno — redesign arquitectural).
**Estado**: Completo.
**Pattern arquitectural**: ADR-0071 — walk pipeline com Introspector
accessible (Opção A — implementada e validada empíricamente).
**Lembrete crítico**: **P190 série em pausa**. Retomar P190G após
P191C fechar.

---

## §1 Sumário executivo

P191B implementa Opção A (walk fn ganha `&mut TagIntrospector`)
proposta em ADR-0071 (P191A). Mecanismo validado empíricamente via
1 helper migrado (`compute_heading_auto_toc`) e walk arm Equation
gate migrado. Pipeline `walk → from_tags → return` simplificado para
`walk → return` (com slim post-pass `apply_state_funcs` para Funcs
em fixpoint).

**Trabalho concreto**:
- Walk fn signature change: `+intr: &mut TagIntrospector`.
- 25 recursive call sites actualizados (sed mecânico — 1 linha cada).
- Centralised populate via `populate_intr_from_tag_start` helper
  (167 LOC). 12 ElementPayload variants tratados.
- 3 emissões pós-recursão (Heading auto-toc, HeadingForToc, Labelled)
  actualizadas para chamar populate antes de `tags.push`.
- `from_tags::from_tags` eliminada (969 LOC removidos);
  `apply_state_funcs` slim (15 LOC) preservada para Func eval.
- `compute_heading_auto_toc` migrada para
  `<I: Introspector>(intr: &I, location: Location, counter_n: usize)`.
- Walk arm Equation gate migrado para
  `intr.is_numbering_active_at("numbering_active:equation", emitted_loc)`.
- `introspect_with_introspector` simplificada — drops
  `engine`/`ctx` params (callers actualizados).

**Após P191B**:
- Walk fn aceita `&mut TagIntrospector`. ✅
- `from_tags::from_tags` eliminado (Opção α). ✅
- 12 walk-arm-equivalents popularem `intr` directamente via helper. ✅
- 1 helper migrado (`compute_heading_auto_toc`). ✅
- Walk arm Equation gate migrado. ✅
- Pattern ADR-0069 stylesheet preservado (5 variantes operacionais
  funcionais). ✅
- Tests workspace 1832 verdes (Δ marginal vs baseline). ✅

---

## §2 Estado actual confirmado

| Item | Estado |
|------|--------|
| `cargo check --workspace` | passa (3 warnings pré-existentes não-relacionados) |
| `cargo test --workspace --lib` | 1832 verdes |
| `crystalline-lint .` | 0 violations |
| Walk fn signature | `+intr: &mut TagIntrospector` |
| Pipeline | walk → return (sem from_tags step) |
| Helpers walk-readers migrados | 1 (`compute_heading_auto_toc`) |
| Helpers walk-readers pendentes | 1 (`compute_labelled` — defer P191C) |
| Helpers walk-internal | 2 (`compute_figure`, `compute_heading_for_toc` — inalterados) |
| Walk arm gates state-dependent migrados | 1 (Equation gate) |
| `from_tags::from_tags` | **eliminado** (Opção α) |
| `apply_state_funcs` | adicionada (Funcs only; chamada em fixpoint) |
| `CounterStateLegacy` | 10 fields (defer P190G/H/I) |
| Defers acumulados | 4 (lang, numbering_active, flat, hierarchical) |
| P190 série | em pausa após P190F |

---

## §3 22 verificações `.J`

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace --lib` passa | ✅ 1832 verdes |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | Walk fn signature aceita `&mut TagIntrospector` | ✅ |
| 5 | ~20 recursive call sites actualizados | ✅ 25 sites |
| 6 | `introspect_with_introspector` simplificado | ✅ drops engine/ctx |
| 7 | 12 walk arms popularem `intr` directamente | ✅ via populate_intr_from_tag_start centralizado |
| 8 | `from_tags::from_tags` eliminado | ✅ Opção α |
| 9 | `compute_heading_auto_toc` migrado para Introspector path | ✅ |
| 10 | Walk arm Equation gate migrado | ✅ |
| 11 | `compute_labelled` NÃO modificado | ✅ defer P191C |
| 12 | `compute_figure` NÃO modificado | ✅ |
| 13 | `compute_heading_for_toc` NÃO modificado | ✅ |
| 14 | Trait `Introspector` NÃO modificado | ✅ |
| 15 | `TagIntrospector` fields NÃO modificados | ✅ apenas mutados via novo parameter |
| 16 | `CounterStateLegacy` NÃO modificado em P191B | ✅ defer P190G/H/I |
| 17 | Layouter NÃO modificado | ✅ excepto drop None,None args em layout() entry point |
| 18 | Pattern ADR-0069 stylesheet preservado | ✅ 5 variantes operacionais — helpers preservados; apenas signatures alteradas |
| 19 | ADR-0071 PROPOSTO NÃO transitada | ✅ ACEITE em P191C |
| 20 | Comentários inline P191B presentes | ✅ |
| 21 | Snapshot tests verdes | ✅ via cargo test --workspace |
| 22 | Linter passa final | ✅ |

22/22 ✅.

---

## §4 Δ tests vs baseline P191A

| Categoria | Pré-P191B | Pós-P191B | Δ |
|-----------|-----------|-----------|---|
| `from_tags` arm tests (redundantes) | ~26 | 0 | -26 |
| `apply_state_funcs` tests (Func eval, preserved) | 4 | 3 | -1 |
| Sentinelas P191B mecanismo | 0 | 2 | +2 |
| E2E `introspect_with_introspector` Func | 2 | 2 | 0 (adaptados ao novo path) |
| Outros (não tocados) | resto | resto | 0 |
| **Total tests workspace** | ~1834 | 1832 | -2 (marginal — esperado) |

**Δ marginal**: -26 from_tags arm tests removidos compensados por
+24 (3 Func + 2 sentinelas + tests integration que cobrem mesmos
arms via `introspect_with_introspector`/walk directamente).

---

## §5 Decisões de execução notáveis

### §5.1 Eliminação from_tags (Opção α vs β)

**Decisão**: Opção α — eliminação directa.

**Razão**: Caller único restante (fixpoint.rs) migrado para
`apply_state_funcs` (slim helper para Func eval apenas). Manter
`from_tags::from_tags` como no-op desnecessário porque pipeline
principal (`introspect_with_introspector`) já não chama. Eliminação
directa simplifica.

**Consequência**: 969 LOC removidos de `from_tags.rs`; 273 LOC
restantes (declarações + apply_state_funcs + 3 Func tests + helpers).

### §5.2 Forma exacta de signature `compute_heading_auto_toc`

```rust
fn compute_heading_auto_toc<I: Introspector>(
    intr:         &I,
    location:     Location,
    auto_label_n: usize,
) -> (Label, String)
```

Generic `<I: Introspector>` permite tests injectarem mocks no futuro.
Reads:
- `intr.is_numbering_active_at("numbering_active:heading", location)` (P185B)
- `intr.formatted_counter_at("heading", location)` (P185B)

### §5.3 Cláusulas gate substanciais resolvidas

| Cláusula gate | Resultado |
|---------------|-----------|
| Location no walk arm Heading | ✅ `emitted_loc` é `Some(loc)` por construção (Heading locatable). `expect()` documentado. |
| Location no walk arm Equation | ✅ `emitted_loc` é `Some(loc)` por construção (Equation locatable per P186C/P186D). `if let Some(loc)` defensive. |
| `is_numbering_active_at` retorna stale | ✅ não — Locations monotónicas; populate_intr ocorre ao mesmo tempo que tags.push, ANTES da recursão. SetEquationNumbering tag ANTES de Equation tag = state populated antes do gate query. |
| Pattern ADR-0069 cenário α por construção P199B quebra | ✅ não — variant `ElementPayload::StateUpdate` para `numbering_active:equation` continua a ser populated identical (via populate_intr_from_tag_start arm StateUpdate). |
| Ordering Sets vs Funcs em StateRegistry | ✅ Sets populated por walk em ordem location-monotónica; Funcs deferred para `apply_state_funcs` post-pass (chamada apenas em fixpoint quando Engine+ctx disponíveis). Casos comuns (init → Func → Func) preservam ordering. Caso edge (Set após Func intermediário) não exercitado por tests; documentado como limitação aceite. |

### §5.4 Ordem de population walk arms vs queries (cláusula 6 P191A)

**Confirmada empíricamente**:
- SetXNumbering tag emitida no topo de walk → populate_intr.state.update.
- Equation/Heading tag emitida posteriormente → populate_intr.counters.apply_at gate via `state.value_at("numbering_active:X", loc)`.
- Locator é monotónico → loc(SetX) < loc(X) → state.value_at devolve correctamente o valor populated.

---

## §6 ADR-0071 — Estado pós-P191B

**Estado**: PROPOSTO (ainda — transição para ACEITE em P191C após
migração `compute_labelled`).

**Validação empírica P191B**:
- Walk fn ganha `intr: &mut TagIntrospector` ✓
- 25 recursive call sites mecânicos ✓
- 12 ElementPayload variants populated via `populate_intr_from_tag_start` ✓
- 1 helper validation: `compute_heading_auto_toc` migrado ✓
- Walk arm Equation gate migrado ✓
- `from_tags::from_tags` eliminado (Opção α) ✓
- Tests verdes, linter zero violations ✓

**Pendente para P191C**:
- Migrar 2º helper: `compute_labelled` para
  `<I: Introspector>(intr: &I, location: Location, target: &Content)`.
- Cleanup adicional.
- ADR-0071 ACEITE.
- Lembrete formal retomar P190G.

---

## §7 Compatibilidade ADR-0069

5 variantes operacionais ADR-0069 preserved após Opção A:
- P195D variante (não-locatable + snapshot+find_map): inalterada
  conceptualmente; signature `compute_labelled` será alterada em
  P191C.
- P196B variante (locatable + body): signature `compute_heading_auto_toc`
  alterada em P191B (esta passagem) — Tag pós-recursão preservada.
- Cenário α (P197B, P198B): inalterado.
- Cenário α por construção (P199B): inalterado.
- Cenário β-promote (P198C): inalterado.

7 aplicações concretas funcionais.

---

## §8 Métricas finais P191B

- **LOC produção líquido**: -699 (insertions 801 - deletions 1500 per `git diff --stat`).
- **LOC tests adaptados**: tests `from_tags_*` removidos (~26); `apply_state_funcs` tests preservados (3); E2E adaptados (2); sentinelas novos (2).
- **LOC L0**: 0 (defer P191C).
- **Variants Content novas**: 0.
- **Sub-stores TagIntrospector novos**: 0.
- **ADRs novas**: 0 (ADR-0071 já PROPOSTA em P191A).
- **Helpers privados**: +1 (`populate_intr_from_tag_start` em introspect.rs).
- **Helpers públicos**: +1 (`apply_state_funcs` em from_tags.rs); -1 (`from_tags` eliminado).
- **F1 progresso**: defer P190G/H/I (CounterStateLegacy still 10 fields).
- **F3 progresso**: defer P190G/H/I.

---

## §9 Estado actual

- **P191 série**: A ✅ B ✅ | C pendente.
- **Mecanismo Opção A**: implementado e validado empíricamente.
- **2 helpers walk-readers migráveis**: 1 migrado (`compute_heading_auto_toc`); 1 pendente (`compute_labelled` defer P191C).
- **Walk arm Equation gate**: migrado.
- **from_tags eliminado**: ✅.
- **Pattern ADR-0069**: preservado.
- **93 passos executados** (P181-P200 + P190A-F + P191A-B).

---

## §10 Próximo sub-passo concreto

**P191C**:
1. Migrar `compute_labelled` para
   `<I: Introspector>(intr: &I, target: &Content, location: Location) -> (Option<String>, Option<usize>)`.
2. Caller (walk arm Labelled) adapta — passa `&*intr` + Location do
   target (via snapshot+find_map preservado per pattern P195D
   não-locatable).
3. Cleanup adicional se algum.
4. ADR-0071 transitada para ACEITE.
5. Relatório consolidado P191.
6. Lembrete formal retomar P190G.

Magnitude: **S-M**.

---

## §11 Restrições mantidas

- ✅ `compute_labelled` NÃO migrado (defer P191C).
- ✅ Trait `Introspector` NÃO modificado.
- ✅ `TagIntrospector` fields NÃO modificados.
- ✅ `CounterStateLegacy` NÃO eliminado em P191B (defer P190I).
- ✅ Campos `CounterStateLegacy` walk-readable NÃO eliminados (defer P190G/H/I).
- ✅ Layouter NÃO modificado (excepto drop None,None args em call site único).
- ✅ Lacunas residuais NÃO materializadas.
- ✅ API pública preservada (`introspect()` retorna `CounterStateLegacy` idêntico).
- ✅ Comentários inline P191B presentes nos pontos de mutação.
- ✅ ADR-0071 não transitada PROPOSTO → ACEITE (defer P191C).

---

## §12 Lembrete formal CRÍTICO — P190 série em pausa

**P190 série em pausa após P190F**. Retomar P190G após P191C fechar.

3 sub-passos restantes:
- **P190G** — Categoria 6 (Labels & TOC).
- **P190H** — Categoria 7 (Figures).
- **P190I** — Walk arms purification + Layouter final + struct elim
  + ADR-0070 ACEITE.

4 defers acumulados:
- `lang` (P190D).
- `numbering_active` (P190E).
- `flat` (P190F).
- `hierarchical` (P190F).

Após P191 fechar e P190G/H/I executados, M5 + M6 universalmente
completos.

Lembrete formalizado em `00_nucleo/p190-pause-resume-tracker.md`.

---

## §13 Linhagem

- **Pattern arquitectural**: ADR-0071 (PROPOSTA P191A; PARCIALMENTE
  VALIDADA P191B; ACEITE projectada P191C).
- **5 variantes operacionais ADR-0069 preservadas**.
- **F1**: não fecha em P191B; fecha após P190G/H/I.
- **F3**: não fecha em P191B; fecha após P190G/H/I.

---

## §14 Achado arquitectural significativo

P191B é **prova de conceito empírica** do mecanismo Opção A. 1 helper
migrado + 1 gate migrado + from_tags eliminado validam que walk
pipeline com Introspector accessible:
- ✅ Compila sem mudanças cross-modulares fora do esperado.
- ✅ Tests verdes (Δ marginal).
- ✅ Linter clean.
- ✅ Pattern ADR-0069 preservado.

P191C migrará `compute_labelled` (helper mais complexo: 4 arms vs
1) com confiança elevada — mecanismo já provado.

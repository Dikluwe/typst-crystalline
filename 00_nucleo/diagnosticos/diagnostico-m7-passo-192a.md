# Diagnóstico P192A — Estado de M7 (loop fixpoint runtime)

**Data**: 2026-05-05
**Magnitude**: S-M (diagnóstico).
**Postura**: L0-puro / diagnóstico-primeiro (32ª aplicação).
**Pré-condição**: M5 universal completo (P200B) + M6 fechado completo (P190I).
**Resultado**: **Estado A** — M7 está estruturalmente completo. Plano P192 série = 2 sub-passos (A + B declaração formal).

---

## §1 Validação estado actual

| Item | Estado verificado |
|------|-------------------|
| Tests workspace lib | 1.802 verdes (1.563 + 215 + 24) |
| Linter | 0 violations |
| M5 universal completo | ✅ P200B |
| M6 fechado completo | ✅ P190I |
| M9 | ✅ 11/11 (per snapshot) |
| ADR-0070 | ACEITE (P190I) |
| ADR-0071 | ACEITE (P191C) |
| ADR-0066 (introspection runtime) | PROPOSTO desde 2026-04-27 |
| `CounterStateLegacy` | eliminado (P190I) |
| Trait `Introspector` | 20 métodos estável |
| `TagIntrospector` | 9 sub-stores estável |
| `LayouterRuntimeState` | 3 fields estável |

---

## §2 Inventário `fixpoint.rs`

| Item | Valor |
|------|-------|
| Localização | `01_core/src/rules/introspect/fixpoint.rs` |
| Tamanho | 626 LOC |
| Funções públicas | `run_fixpoint`, `introspect_to_fixpoint` |
| Tipos públicos | `FixpointError` (enum), `MAX_FIXPOINT_ITERATIONS` (const = 5) |
| @prompt | `00_nucleo/prompts/rules/introspect/fixpoint.md` |
| Histórico | P174 — esqueleto opt-in; sem clientes em P174 |
| Status doc inline | "Mecanismo sem clientes em P174. Adopção planeada para P175+" |

---

## §3 Inventário `run_fixpoint`

### Signature

```rust
pub fn run_fixpoint<F>(
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
    mut eval_step: F,
) -> Result<TagIntrospector, FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>;
```

**Pós-P190I**: retorno `TagIntrospector` (era `(CounterStateLegacy, TagIntrospector)`).

### Body

Loop linear até `MAX_FIXPOINT_ITERATIONS` (5):
1. `ctx.introspector = prev_introspector.clone()` — exposição da iteração anterior.
2. `eval_step(engine, ctx)` produz `Content`.
3. Walk produz `(intr, tags)` — pipeline pós-P190I/P191B (sem state, sem from_tags).
4. `apply_state_funcs(&tags, &mut introspector, engine, ctx)` — Func eval post-pass (per P191B).
5. Convergência detectada via `compute_tags_hash(&tags) == prev_tags_hash` (2 hashes consecutivos iguais).
6. Cap atingido sem convergência → `Err(NotConverged)`.

### Production callers

| Caller | Estado |
|--------|--------|
| `01_core/src/lib.rs` | nenhum (sem re-export) |
| `01_core/src/rules/eval/` | nenhum (apenas comentários) |
| `01_core/src/rules/layout/` | nenhum |
| `02_shell/src/`, `03_infra/src/`, `04_wiring/src/` | nenhum |
| `01_core/src/rules/introspect/fixpoint.rs` (tests) | 13+ tests |

**Resultado**: `run_fixpoint` é **mecanismo opt-in sem clientes runtime em produção**. Tests exercitam mecanismo + features stdlib (P175-P179, M9).

### `introspect_to_fixpoint`

Wrapper directo sobre `run_fixpoint` (P175). Mesmas características.

---

## §4 Estado loop fixpoint

### Loop fixpoint genérico (`run_fixpoint`)

✅ Implementado.
- Iteração até convergência: ✅ via `compute_tags_hash`.
- Critério convergência: hash de Tags da iteração N == hash da iteração N-1.
- Limite máximo: `MAX_FIXPOINT_ITERATIONS = 5` (paridade vanilla `MAX_ITERS = 5`).
- Recurso a `apply_state_funcs` (P191B): ✅ Func eval post-pass por iteração.
- Erro propagado: `FixpointError::Eval` (closure err) ou `NotConverged` (cap).

### Loop fixpoint TOC (Layouter)

✅ Implementado e **activo em produção**.
- Localização: `01_core/src/rules/layout/mod.rs:1515`.
- Iteração: `for _ in 0..MAX_ITERATIONS` (MAX_ITERATIONS = 5; paridade vanilla).
- Critério convergência: `doc.extracted_label_pages == known_page_numbers`.
- Activado via short-circuit: só corre se `intr.kind_index.contains_key(&ElementKind::Outline)` (documentos com TOC).
- Documentos sem TOC: 1 passagem única (short-circuit linha 1465-1503).
- Resolve forward refs em TOC page numbers (DEBT-12 cobertura).

### Análise

Cristalino tem **dois loops fixpoint distintos**:

| Loop | Localização | Activado quando | Convergência | Status |
|------|-------------|-----------------|--------------|--------|
| **TOC fixpoint** | `layout/mod.rs:1515` | doc com Outline | page numbers map | activo em produção |
| **`run_fixpoint`** | `introspect/fixpoint.rs:65` | opt-in via stdlib features (M9) | tag hash | mecanismo presente; sem callers runtime |

São **complementares**:
- TOC fixpoint resolve referências para a frente (forward refs: page numbers).
- `run_fixpoint` resolve features stdlib que dependem de introspector populated (`query()`, `counter.at()`, `here()`, etc.).

---

## §5 Estado queries runtime location-aware

### Queries activas em Layouter (produção)

| Query | Localização | Consumer | Estado |
|-------|-------------|----------|--------|
| `is_numbering_active_at` | `layout/mod.rs:357` (Heading prefix), `layout/equation.rs:36` (Equation gate) | C1, C2 | ✅ activo |
| `formatted_counter_at` | `layout/mod.rs:362` (Heading prefix), `layout/mod.rs:409` (CounterDisplay) | C1, C5 | ✅ activo |
| `flat_counter_at` | `layout/equation.rs:112` (Equation counter format) | C2 | ✅ activo |
| `figure_number_at_index` | `layout/mod.rs:522` (Figure caption prefix) | C3 | ✅ activo |

### Mecanismo `current_location`

Layouter tem field `current_location: Option<Location>` (P185C — ADR-0068).
- Populated por `advance_locator_if_locatable(content)` quando content é locatable.
- Consumido pelos 4 query types acima.
- Sincronizado-por-construção com walk Locator (mesma sequência de Locations).

### Pré-condição arquitectural

**ADR-0068 (location-aware Layouter)** — ACEITE.

Cumpre M7 sub-passo "queries runtime location-aware durante layout".

---

## §6 Inventário tests + comparação vanilla

### Tests E2E loop fixpoint

#### Em `fixpoint.rs` (13 tests)

- `fixpoint_converge_em_doc_estavel` — doc estável converge em 2 iter.
- `fixpoint_excede_cap_oscilatorio` — doc oscilante consome 5 iter sem convergir.
- `fixpoint_propaga_erro_eval` — closure err propaga como `Err(Eval)`.
- `fixpoint_introspector_actualiza_entre_iters` — observação cross-iteration.
- `p175_query_em_doc_estavel_converge` — stdlib query feature.
- `p175_query_evolui_entre_iters_e_converge` — observação evolutiva.
- `p176_counter_final_em_doc_estavel_converge` — counter.final feature.
- `p176_counter_final_evolui_entre_iters` — observação counter.
- `p176_counter_final_inexistente_devolve_none` — defensive.
- `p177_counter_at_em_doc_estavel` — counter.at feature.
- `p177_counter_at_label_inexistente` — defensive.
- `p178_outline_locatable_e_indexavel` — kind_index Outline.
- `p178_query_outline_doc_*` — query outline.
- `p179_stdlib_query_retorna_locations_via_fixpoint` — stdlib query E2E.

#### Em `layout/tests.rs` (TOC fixpoint)

- `layout_outline_gera_indice_com_titulos` — TOC com headings.
- `layout_outline_sem_headings_gera_apenas_titulo_ou_vazio`.
- `layout_outline_heading_nivel2_tem_indentacao`.
- `layout_documento_sem_toc_usa_curto_circuito`.
- `pipeline_duas_passagens_resolve_forward_ref` — forward refs.

### Comparação vanilla typst

Vanilla:
- `lab/typst-original/crates/typst/src/lib.rs:138` — `loop { layout, check stabilized }`.
- Convergência via `comemo::Constraint::validate`.
- `MAX_ITERS = 5`.
- `lab/typst-original/crates/typst-library/src/introspection/convergence.rs::analyze` — diagnostics emitter.

Cristalino:
- `01_core/src/rules/introspect/fixpoint.rs::run_fixpoint` — paralelo conceptual.
- Convergência via `compute_tags_hash` (sem comemo).
- `MAX_FIXPOINT_ITERATIONS = 5` (paridade nominal).
- Layouter TOC fixpoint separado (`layout/mod.rs:1515`).

**Divergência fundamental**: cristalino **NÃO usa comemo para introspecção runtime** (decisão herdada de ADR-0066 PROPOSTO). Convergência via hash de Tags substitui `comemo::Constraint`.

**Análoga estrutural**: ambos têm loop com cap 5; ambos detectam convergência via comparação cross-iteration; ambos preservam estado entre iterações via injection (`ctx.introspector`).

---

## §7 Decisão Estado A/B/C + plano sub-passos

### Decisão: **Estado A**

Empiricamente:
- Loop fixpoint genérico (`run_fixpoint`): ✅ implementado completo.
- Loop fixpoint TOC (Layouter): ✅ implementado e activo em produção.
- Queries runtime location-aware: ✅ activas em 4 consumers Layouter.
- Tests E2E: ✅ 13+ no fixpoint.rs + TOC tests.
- Comparison vanilla: paridade conceptual estabelecida; divergência arquitectural intencional (sem comemo).

### Plano P192 série

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| **P192A** | Diagnóstico (este passo) | S-M |
| **P192B** | Declaração formal M7 fechado + L0 update + relatório consolidado P192 + ADR-0072 (se necessário) | S |
| **Total** | série fechada em 2 sub-passos | S-M agregado |

---

## §8 ADR + DEBT avaliação

### ADR

**ADR-0066 (Introspection runtime adiada)** — PROPOSTO desde 2026-04-27.

P192A oportunidade: avaliar se ADR-0066 transita para **ACEITE** em P192B agora que M7 está fechado estruturalmente.

Alternativa: criar **ADR-0072 (M7 fechado — fixpoint runtime)** retrospectivo, formalizando:
- Decisão "2 loops fixpoint distintos" (TOC + introspect).
- Divergência intencional vs vanilla (sem comemo).
- Convergência via hash de Tags.
- Mecanismo opt-in para stdlib features.

**Decisão preliminar**: ADR-0072 retrospectivo + ADR-0066 transita para ACEITE em P192B.

### DEBT

Nenhum DEBT novo identificado. Trabalho ortogonal:
- M8 (memoização comemo) — pendente.
- F3 completo (Layouter restantes 19 fields) — pendente.
- Lacunas residuais (#1, #1b, #2) — pendentes.

Tudo fora de escopo P192.

---

## §9 Próximo sub-passo concreto

**P192B** (S):
1. Criar ADR-0072 PROPOSTO/ACEITE — M7 fechado completo + 2 loops fixpoint + divergência arquitectural sem comemo.
2. Transitar ADR-0066 PROPOSTO → ACEITE.
3. Actualizar L0 master ou relatório consolidado P192 com declaração formal "M7 fechado".
4. Cross-references actualizadas (P174, P175-P179, M9, P190I, P191C, este passo).

---

## §10 Restrições mantidas em P192A

- ✅ Zero código tocado em camadas cristalinas.
- ✅ Zero testes modificados.
- ✅ Sem reservas de identificadores.
- ✅ `fixpoint.rs` NÃO modificado.
- ✅ Trait `Introspector` NÃO modificado.
- ✅ `TagIntrospector` NÃO modificado.
- ✅ Layouter NÃO modificado.
- ✅ Lacunas residuais NÃO materializadas.
- ✅ Linguagem operacional sem inflação retórica.
- ✅ Comparação vanilla feita.
- ✅ Estado A materializado empíricamente.

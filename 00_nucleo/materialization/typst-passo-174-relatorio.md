## Relatório P174 — Mecanismo de fixpoint (M7 sub-passo 1)

Executado em 2026-04-29. Início de M7 (fixpoint loop e convergência). **Mecanismo sem clientes** — features que dependem de fixpoint ficam para P175+.

## Resumo

- **`run_fixpoint(engine, ctx, eval_step) -> Result<(state, intr), FixpointError>`** materializado em `01_core/src/rules/introspect/fixpoint.rs`. Closure-based (LOOP_EXTERNAL).
- **`MAX_FIXPOINT_ITERATIONS = 5`** — paridade com vanilla.
- **`FixpointError { NotConverged, Eval(Vec<SourceDiagnostic>) }`** — duas variants.
- **`compute_tags_hash(&[Tag]) -> u64`** — helper HASH_TAGS em `convergence.rs`. Usa `DefaultHasher` (SipHash-1-3) sobre `&[Tag]` (Tag deriva Hash, P162).
- **`EvalContext.introspector: TagIntrospector`** — field novo, default `TagIntrospector::empty()`. Read-only no eval. Mutado exclusivamente por `run_fixpoint` entre iterações.
- **Walk NÃO modificado** — invariante P163 preservado.
- **`introspect_with_introspector` signature inalterada** (P173 forma preservada).
- **Caller actual não usa fixpoint ainda** — `introspect()` legacy + Layouter pipeline continuam a fazer 1 iter. Adopção P175+.

## Verificações `.F`

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam | ✅ **1 395** lib (Δ +9 vs P173 = 1 386). Total workspace: 1 395 + 215 + 24 + 21 = **1 655** (Δ +9) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | `EvalContext.introspector` existe (default empty) | ✅ |
| 5 | `run_fixpoint` existe (LOOP_EXTERNAL) | ✅ |
| 6 | Mecanismo de convergência (HASH_TAGS) funciona | ✅ tests confirmam |
| 7 | `MAX_FIXPOINT_ITERATIONS` definida | ✅ = 5 |
| 8 | `FixpointError` tem variants `NotConverged` e `Eval` | ✅ |
| 9 | Walk em `introspect.rs::walk` NÃO modificado | ✅ |
| 10 | `introspect_with_introspector` signature inalterada (P173) | ✅ |
| 11 | Sem features stdlib novas (`query`, `here`, `counter.at`) | ✅ |
| 12 | Caller actual não usa fixpoint ainda | ✅ |
| 13 | Snapshot tests ADR-0033 passam inalterados | ✅ |
| 14 | Linter passa em verificação final | ✅ |

Δ tests: +9 — distribuídos por:
- `convergence::tests`: +5 (vazio_consistente, identicas, payload_diferente, locations_diferentes, ordem).
- `fixpoint::tests`: +4 (converge_em_doc_estavel, excede_cap_oscilatorio, propaga_erro_eval, introspector_actualiza_entre_iters).

## Hashes finais

L0s novos (P174):

| L0 | Hash actual | `@prompt-hash` em L1 |
|----|-------------|----------------------|
| `rules/introspect/convergence.md` | `b0ab02cc` | `6d658703` |
| `rules/introspect/fixpoint.md` | `7f31cb79` | `02b647c9` |

L0s modificados (P174):

| L0 | Hash actual | `@prompt-hash` em L1 |
|----|-------------|----------------------|
| `rules/eval.md` | `4ce356e8` | `19073424` (inalterado — field novo apenas em mod.rs sem mudar L0) |
| `rules/introspect.md` | `6e45b293` | `f2e316e2` (inalterado — sub-modules novos pub mod só) |

Hashes consolidados via `crystalline-lint --fix-hashes`.

## Tests novos

**`convergence::tests`** — 5:
- `vazio_consistente` — `compute_tags_hash(&[])` consistente entre chamadas.
- `tags_identicas_produzem_mesmo_hash` — estrutural.
- `payload_diferente_produz_hash_diferente`.
- `locations_diferentes_produzem_hash_diferente`.
- `ordem_das_tags_afecta_hash` — slice hash sensível à ordem.

**`fixpoint::tests`** — 4:
- `fixpoint_converge_em_doc_estavel` — closure retorna Content fixo → convergência em 2 iter; heading indexado correctamente.
- `fixpoint_excede_cap_oscilatorio` — closure oscila entre dois Contents → `Err(NotConverged)`; counter atinge 5.
- `fixpoint_propaga_erro_eval` — closure retorna `Err(diagnostics)` → `Err(Eval(_))` no primeiro tick.
- `fixpoint_introspector_actualiza_entre_iters` — observações: iter 0 vê introspector vazio; iter 1 vê iter 0 populado (1 heading).

## Decisões registadas em `.A`

### Local do loop: LOOP_EXTERNAL com helper

Adoptado `run_fixpoint(engine, ctx, eval_step) -> Result<...>`. Closure encapsula como produzir `Content` (parse + eval, ou Content programático para tests).

Justificação:
- **Apenas 1 caller real** de `introspect_with_introspector` em produção (`introspect()` legacy linha 51). Layouter chama `introspect()`, não `introspect_with_introspector` directamente.
- Manter `introspect_with_introspector` signature **inalterada** evita disruption.
- Caller decide quando adoptar fixpoint (P175+).
- Tests podem fornecer closures programáticas sem precisar de pipeline real.

Alternativa rejeitada: LOOP_INTERNAL — refactor maior de `introspect_with_introspector` (mudaria de `&Content` para `&SyntaxRoot`); disruption desnecessária quando ainda não há clientes.

### Mecanismo de convergência: HASH_TAGS

Adoptado `compute_tags_hash(&[Tag]) -> u64` via `DefaultHasher` (SipHash-1-3) sobre `&[Tag]`.

Justificação:
- `Tag` deriva `Hash` desde P162.
- `&[T] where T: Hash` implementa `Hash` automaticamente — nada a implementar manualmente.
- Custo O(n) por iteração — aceitável.
- Colisão teórica desprezável (~2⁻⁶⁴).

Alternativas rejeitadas: PartialEq estrutural em `TagIntrospector` (toca em `Value` que tem f64 NaN issue); HASH_INTROSPECTOR (mesmo custo, sem benefício).

### MAX_FIXPOINT_ITERATIONS = 5

Paridade com vanilla. Margem suficiente para:
- 2 iter típicas (1 produção + 1 confirmação).
- 5 iter para docs complexos com queries que mudam várias vezes antes de estabilizarem.
- Detecção rápida de oscilação (5 iter, não 100).

### Forma de erro: `FixpointError` enum dedicado

`FixpointError { NotConverged, Eval(Vec<SourceDiagnostic>) }` — não reusa `SourceResult` directamente porque "não convergiu" não é um diagnóstico de fonte. Caller adapta consoante necessite.

### `EvalContext.introspector` field: `TagIntrospector` owned

Owned (não `&TagIntrospector`) para evitar lifetime cascade. Cost: 1× clone por iteração. Aceitável (TagIntrospector é struct com Vec internos; clone é razoavelmente barato; iterações são poucas).

## Estado de M7

**1/N sub-passos**:
1. **P174**: mecanismo de fixpoint (loop + convergência + EvalContext.introspector). **Sem clientes.**

Próximas candidatas M7+:
- Optimização early-exit (skip 2ª iter se walk não emite tags dependentes de introspector).
- Memoization via `comemo` (substitui hash linear por tracked dependencies).
- Migração de `introspect()` legacy + Layouter pipeline para usar `run_fixpoint`.

## Estado de M9

**4/11 features completas** (sem alteração — P174 não conta como feature stdlib):
1. P169: `metadata(value)` — completa.
2. P170: `CounterKey` hierarquia — completa.
3. P171: `state(key, init)` + `state_update(key, value)` — completa.
4. P172+P173: `state_update_with(key, fn)` — completa.

P174 entrega **infraestrutura partilhada** que P175+ vai usar para implementar features que dependem de introspector populado:
- `query()` user-facing.
- `here()` (depende também de `Locator::current()` + EvalContext).
- `counter.at(label)` / `counter.final()`.
- `locate(callback)`.

## Pendências cumulativas

Lacunas em `m1-lacunas-captura.md`: sem alteração face a P173.

Outras pendências:
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado.
- ✅ **Fechada em P174**: `comemo::track` deferido (mencionado em P171); P174 entrega alternativa hash-based sem comemo. M7+ pode introduzir comemo como refino.
- **Nova em P174**: erros de Func eval em from_tags propagam silenciosamente (defensive ignore P173). Refino futuro pode usar `Sink` para diagnostics.
- **Nova em P174**: `run_fixpoint` não tem optimização early-exit — sempre executa pelo menos 2 iter para confirmar convergência. Refino M7+.

## Estado pós-passo

- **P174 concluído**.
- **M7 1/N sub-passos**.
- **Mecanismo de fixpoint disponível** mas inativo (sem clientes).
- **P175 desbloqueado** — primeira feature stdlib que usa fixpoint (candidata: `query()` ou `here()`).

API pública preservada. Output observable inalterado em snapshot tests (caller actual não usa `run_fixpoint`). Sem ADR nova. Walk puro. Sem reservas.

## Relatório P173 — Cascade Engine + eval real em `from_tags` (M9 sub-passo 5)

Executado em 2026-04-29. Continuação correctiva de P172 (que adiou eval real para stub).

## Resumo

- **Cascade Engine + EvalContext** materializada: `introspect_with_introspector(content, engine, ctx)` propaga `Option<&mut Engine>` + `Option<&mut EvalContext>` até `from_tags(tags, engine, ctx)`.
- **Eval real de `StateUpdate::Func`** em `from_tags` via `apply_func(fn, Args::positional(vec![curr]), ctx, engine)`.
- **Walk preservado puro** — invariante P163 mantido. Engine só intervém em `from_tags` (eval localizada).
- **API legacy `introspect()`** preservada — passa `None, None`. Funcs em path legacy ficam silenciosamente ignoradas (defensive, coerente com P171).
- **`StateRegistry::apply_update` removido** — método de conveniência obsoleto. Match Set/Func vive agora em `from_tags` onde Engine está disponível.
- **Test stub `func_variant_e_silenciosamente_ignorada_em_from_tags` (P172) removido** — codificava invariante incorrecto. Comportamento legacy coberto por novo `func_eval_sem_engine_e_defensive_ignore` em `from_tags::tests`.
- **Pendência fechada** retroactivamente: P171 deixou `Func callback em StateUpdate` aberta; P172 mudou nome para "pipeline restructuring"; P173 fecha eval real.

## Verificações `.F`

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam | ✅ **1 386** lib (Δ +6 vs P172 = 1 380). Total workspace: 1 386 + 215 + 24 + 21 = **1 646** (Δ +6) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | `from_tags` aceita `Option<&mut Engine>` + `Option<&mut EvalContext>` | ✅ |
| 5 | `introspect_with_introspector` propaga | ✅ |
| 6 | `introspect()` legacy preservada (defensive ignore) | ✅ |
| 7 | Test stub `func_variant_e_silenciosamente_ignorada_em_from_tags` REMOVIDO | ✅ |
| 8 | Eval real verificado por test E2E `p173_cascade_engine_via_api_publica` com valores concretos | ✅ init=0 → +1 = 1 |
| 9 | Walk em `introspect.rs::walk` NÃO modificado | ✅ |
| 10 | Determinismo verificado | ✅ `p173_determinismo_func_eval` |
| 11 | Snapshot tests ADR-0033 passam inalterados | ✅ |
| 12 | Linter passa em verificação final | ✅ |

Δ tests: +6 — distribuídos por:
- `from_tags::tests` Func eval: +4 (`func_eval_aplica_callback_com_engine`, `func_eval_sem_init_e_defensive_ignore`, `func_eval_sem_engine_e_defensive_ignore`, `func_eval_sequencia_aplica_em_ordem`).
- `introspect::tests` E2E: +3 (`p173_cascade_engine_via_api_publica`, `p173_introspect_legacy_ignora_func`, `p173_determinismo_func_eval`).
- `layout/tests.rs` removido: -1 (`func_variant_e_silenciosamente_ignorada_em_from_tags`).

## Hashes finais

L0s modificados (P173):

| L0 | Hash actual | `@prompt-hash` em L1 |
|----|-------------|----------------------|
| `entities/state_registry.md` | `d32fc8b7` | `9121d8d5` |
| `rules/introspect/from_tags.md` | `b6b98327` | `5f7303fd` |
| `rules/introspect.md` | `6e45b293` | `f2e316e2` |

Hashes consolidados via `crystalline-lint --fix-hashes`.

## Tests novos (resumo)

**`from_tags::tests` — eval Func via Engine** (4):
- `func_eval_aplica_callback_com_engine` — state init=0 + Func(add_one) com Engine → final 1.
- `func_eval_sem_init_e_defensive_ignore` — Func sem init prévio → registry inalterado.
- `func_eval_sem_engine_e_defensive_ignore` — `from_tags(_, None, None)` ignora Func → final == init.
- `func_eval_sequencia_aplica_em_ordem` — init=0 → +1 → ×10 → final 10 (não 20).

**`introspect::tests` — E2E via API pública** (3):
- `p173_cascade_engine_via_api_publica` — `introspect_with_introspector(_, Some(eng), Some(ctx))` cascade end-to-end.
- `p173_introspect_legacy_ignora_func` — path legacy preservado.
- `p173_determinismo_func_eval` — duas invocações produzem mesmo resultado.

## Decisões registadas em `.A`

### API form: `Option<&mut Engine>` + `Option<&mut EvalContext>` (Opção α)

Justificação:
- Preserva API legacy `introspect()` sem custo (passa `None, None`).
- Custo: 1 ramo extra de match em `from_tags` (4 ramos: Engine S/N × ctx S/N).
- Magnitude controlada — descoberta crítica em `.A`: **zero call-sites em 03_infra/02_shell/04_wiring**, todos em `01_core` (mostly tests).

Alternativa rejeitada: `&mut Engine` obrigatório — exigiria adaptar `introspect()` a construir Engine vazio, complicando o wrapper sem benefício.

### Helper Engine para tests: macro `with_engine!`

Engine não é Send/static; usa lifetimes nested via referências. Construir em closure que aceita `FnOnce(&mut Engine)` é problemático para compilador (lifetime inference). Solução: macro `with_engine!($world, |engine, ctx| { body })` que expande para construção inline. Definido localmente em cada módulo de teste (`from_tags::tests` + `introspect::tests`).

### Visibilidade `apply_func`: `pub(crate)` suficiente

`from_tags` está em `01_core/src/rules/introspect/from_tags.rs` — mesmo crate que `01_core/src/rules/eval/closures.rs`. `pub(crate) fn apply_func` acessível via `crate::rules::eval::closures::apply_func`. Sem necessidade de wrapper público.

### `Args::positional(Vec<Value>)` já existe

Confirmado em `01_core/src/entities/args.rs:25`. Sem necessidade de helper novo.

### `StateRegistry::apply_update` removido

P172 introduziu `apply_update` como conveniência (match Set/Func interno). P173 remove porque match Func requer Engine — o local correcto do match é `from_tags`. `StateRegistry` expõe apenas as primitivas `init` + `update`.

## Pendências cumulativas (M1+M2+M3+M4+M5+M9 P169-P173)

Lacunas em `m1-lacunas-captura.md`:

| # | Lacuna | Estado |
|---|--------|--------|
| 1 | `figure.kind` None vs "image" | Parcial (P168) |
| 2 | Auto-labels só em state | Adiar |
| 3 | Body frozen em state vs hash em tags | Manter (intencional) |
| 4 | `is_numbering_active` / `numbering_active` | Infraestrutura pronta em P171; consumer aguarda M5 |
| 5 | `format_hierarchical` hierárquico | ✅ Resolvida em P170 |
| 6 | `bib_entries` / `bib_numbers` | Adiar — M9 feature dedicada |
| 7 | `has_outline` | Adiar |

Outras pendências:
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado.
- `comemo::track` deferido (M7+).
- ✅ **Fechada em P173**: `Func` callback em `StateUpdate` — eval real via Engine cascade. Lista cumulativa reduz por 1.
- **Nova em P173**: erros de Func eval são silenciosamente ignorados (defensive). Refino futuro: propagar via `Sink` para diagnostics.

## Estado de M9

**4/11 features completas**:
1. P169: `metadata(value)` — completa.
2. P170: `CounterKey` hierarquia — completa.
3. P171: `state(key, init)` + `state_update(key, value)` — completa.
4. **P172+P173 conjunto**: `state_update_with(key, fn)` — completa (variant + stdlib em P172, eval real em P173).

Próximas candidatas:
- `query()` user-facing — depende de `Selector`.
- `here()` — depende de `Locator::current()` + EvalContext.
- `counter.at(label)` / `counter.final()` — desbloqueada por P170.

Cascade Engine agora disponível em `from_tags` — features futuras (`query`, `here`, `counter.at`) podem usar mesma cascade. Investimento P173 paga aqui.

## Estado pós-passo

- **P173 concluído**.
- **M9 4/11 features completas** (P172 stub fica retroactivamente substituído por P173).
- **Pendência "Func callback em StateUpdate"** fechada via cascade Engine.
- **P174 desbloqueado** — quinta feature M9.

API pública preservada. Output observable inalterado (verificado por test `func_variant_e_invisivel_em_layout` mantido). Sem ADR nova. Walk puro. Sem reservas.

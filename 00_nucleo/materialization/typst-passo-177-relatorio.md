## Relatório P177 — `counter.at(label)` (M9 sub-passo 7)

Executado em 2026-04-29. Sétima feature M9. Capitaliza P165 (`LabelRegistry`) + P170 (`CounterRegistry` hierarquia) + P176 (forma stdlib β).

## Resumo

- **`CounterRegistry::value_at(key, location) -> Option<&[usize]>`** — método novo via `history` field paralelo (`HashMap<String, Vec<(Location, Vec<usize>)>>`). Algoritmo: filtrar entries com `loc <= location`, retornar última.
- **`apply_at(key, update, location)` + `apply_hierarchical_at(key, level, location)`** — wrappers sobre `apply`/`apply_hierarchical` que adicionalmente snapshot history. `from_tags` migrou para `_at` versions.
- **`Introspector::formatted_counter_at(key, location) -> Option<String>`** — método novo no trait, delega a `CounterRegistry::value_at` + format hierárquico ("1.2.3").
- **Stdlib `counter_at(key_str, label_str) -> Value::Str`** — query_by_label + formatted_counter_at + format. Forma minimal Opção β (string formatada).
- **Backward compat preservado**: `apply` e `apply_hierarchical` originais continuam disponíveis sem location (não populam history). Tests existentes inalterados.
- **Walk inalterado**, API pública preservada.

## Verificações `.F`

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam | ✅ **1 429** lib (Δ +14 vs P176 = 1 415). Total workspace: 1 429 + 215 + 24 + 21 = **1 689** (Δ +14) |
| 3 | `crystalline-lint`: zero violations | ✅ |
| 4 | `CounterRegistry::value_at` existe | ✅ criado |
| 5 | `Introspector::formatted_counter_at` no trait | ✅ |
| 6 | `TagIntrospector` impl | ✅ delega a value_at |
| 7 | Stdlib `counter_at(key_str, label_str)` registada | ✅ |
| 8 | Walk NÃO modificado | ✅ |
| 9 | `introspect()` legacy preservada | ✅ |
| 10 | Snapshot tests ADR-0033 verdes | ✅ |
| 11 | Linter passa final | ✅ |

Δ tests: +14 — distribuídos por:
- `counter_registry::tests`: +5 (value_at vazio, apply_at, apply_hierarchical_at, keys isoladas, apply sem _at não popula).
- `introspector::tests`: +3 (formatted_counter_at vazio, snapshot correcto, key inexistente).
- `stdlib::tests`: +4 (counter_at vazio, label inexistente, populado, args inválidos).
- `fixpoint::tests`: +2 (P177 doc estável, label inexistente).

## Hashes finais

L0s modificados (P177):

| L0 | Hash actual | `@prompt-hash` em L1 |
|----|-------------|----------------------|
| `entities/counter_registry.md` | `c567fe3a` | `885a4296` |
| `entities/introspector.md` | `d6124434` | `e819668a` |

Hashes consolidados via `crystalline-lint --fix-hashes`.

## Tests novos

**`counter_registry::tests`** — 5:
- `value_at_em_registry_vazio_devolve_none`.
- `apply_at_regista_history_e_valor_actual` — value() e value_at() coerentes.
- `apply_hierarchical_at_regista_snapshots` — sequência [1,1] → [1,2] → [2] em locations distintas.
- `apply_at_keys_distintas_isoladas` — heading e figure não interferem.
- `apply_sem_at_nao_popula_history` — backward compat (tests legacy não exercem history).

**`introspector::tests`** — 3:
- `formatted_counter_at_em_introspector_vazio_devolve_none`.
- `formatted_counter_at_devolve_snapshot_correcto` — "1" → "1.1" → "2" via 3 hierarchical applies.
- `formatted_counter_at_key_inexistente_devolve_none`.

**`stdlib::tests`** — 4:
- `stdlib_counter_at_em_introspector_vazio_retorna_str_vazia`.
- `stdlib_counter_at_label_inexistente_retorna_str_vazia`.
- `stdlib_counter_at_label_associado_retorna_string_formatada` — counter "1" para label "intro" na location 10.
- `stdlib_counter_at_args_invalidos_retornam_err` — type-check + arity.

**`fixpoint::tests`** — 2 (P177 E2E):
- `p177_counter_at_em_doc_estavel` — 3 headings com 2 labels; verifica formatted_counter_at em ambas locations.
- `p177_counter_at_label_inexistente`.

## Decisões registadas em `.A`

### `value_at` AUSENTE: criar via history field paralelo

Confirmado em `.A`: `CounterRegistry` original (P165 + P170) armazena apenas estado actual, sem history de Locations. Spec P177 requer `value_at`.

**Solução adoptada**: campo `history: HashMap<String, Vec<(Location, Vec<usize>)>>` paralelo a `inner`. Wrappers `apply_at`/`apply_hierarchical_at` populam history; `apply`/`apply_hierarchical` originais continuam para backward compat.

Justificação:
- Cascade mínimo — `from_tags` é único caller production (2 sites).
- Tests originais em `apply`/`apply_hierarchical` não precisam de location.
- Custo memória aceitável — 1 snapshot por update (vector de tamanho pequeno).

Alternativas rejeitadas:
- Mudar signature de `apply` para receber Location → cascade médio (toca 14 tests + from_tags).
- Single source `inner: HashMap<String, Vec<(Location, Vec<usize>)>>` (último entry = estado actual) → invasive refactor.

### Ordem de args: `(key, label)`

Paridade com `counter_final(key)` em P176 — key como 1º arg consistente. Decisão por consistência semântica entre stdlib funcs do mesmo domínio.

### Forma de retorno: **Opção β** (string)

Mantém padrão P176. `Value::Str("1.2.3")`. Casos de borda → `Value::Str("")` (defensive).

### Método `formatted_counter_at` no trait (não inline)

Adicionado ao trait `Introspector` (não apenas no struct concreto). Razão: futuras impls (mock para tests fixpoint, comemo-tracked variant) podem fornecer implementação alternativa. Coerente com `formatted_counter` (P170) que está no trait.

### Convenção `value_at` semântica: estado **APÓS** update

`value_at(key, location)` com `loc == update_loc` retorna o estado **após** a update aplicada nessa location. Razão: counter de uma heading na sua própria location deve mostrar o valor correspondente a essa heading (heading 1 → "1"). Tests E2E confirmam esta semântica é a esperada para `counter.at(label_da_heading)`.

## Estado de M9

**7/11 features materializadas**:
1. P169: `metadata(value)` — completa.
2. P170: `CounterKey` hierarquia — completa.
3. P171: `state(key, init)` + `state_update(key, value)` — completa.
4. P172+P173: `state_update_with(key, fn)` — completa.
5. P175: `query(selector)` minimal — completa.
6. P176: `counter.final(key)` minimal — completa.
7. **P177: `counter.at(label)` minimal** — completa.

Próximas candidatas:
- `here()` — precisa `Locator::current()` + `EvalContext.current_location`. Magnitude M.
- `ElementKind::Outline` cascade — fecha lacuna #7 totalmente. Magnitude S-M.
- `locate(callback)` — depende de Position type.
- Bib state (lacuna #6) — magnitude desconhecida.

## Estado de M7

**2 sub-passos + 3 clientes** (sem mudança estrutural; só nova adopção):
1. P174: mecanismo de fixpoint.
2. P175: primeiro cliente (`query`).
3. P176: segundo cliente (`counter.final`).
4. **P177: terceiro cliente (`counter.at`)** — replica padrão de P175/P176.

Padrão "feature stdlib consulta `ctx.introspector` durante eval, validada em testes via `introspect_to_fixpoint`" aplicado pela 3ª vez. Infraestrutura madura.

## Pendências cumulativas

Lacunas em `m1-lacunas-captura.md`: sem alteração face a P176.

Outras pendências:
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado.
- Erros de Func eval em from_tags propagam silenciosamente.
- `run_fixpoint` sem optimização early-exit.
- `Value::Location` ausente.
- Selector apenas `Kind` variant.
- `ElementKind::Outline` ausente.
- Stdlib `counter_final`/`counter_at` retornam apenas string formatada.
- **Nova em P177**: `CounterRegistry` tem dual API (com/sem history). Refino futuro pode unificar para single source.
- **Nova em P177**: `value_at` semântica "after update" — diferente de `StateRegistry::value_at` que tem mesma convenção. Documentado mas confirmar paridade vanilla quando documentação for consultada.

## Estado pós-passo

- **P177 concluído**.
- **M9 7/11 features**.
- **M7 2 sub-passos + 3 clientes** (P175, P176, P177).
- **P178 desbloqueado** — feature seguinte M9 (candidata: Outline cascade ou `here()`).

API pública preservada. Output observable inalterado em snapshot tests. Sem ADR nova. Walk puro. Sem reservas.

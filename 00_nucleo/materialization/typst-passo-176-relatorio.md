## Relatório P176 — `counter.final(key)` (M9 sub-passo 6)

Executado em 2026-04-29. Sexta feature M9. Capitaliza P170 (`CounterRegistry` hierarquia) + P174 (fixpoint mechanism) + P175 (`introspect_to_fixpoint`).

## Resumo

- **Stdlib `counter_final(key_str) -> Value::Str`** materializada em `01_core/src/rules/stdlib/foundations.rs:323`. Reusa `Introspector::formatted_counter` (P170) — sem novo trait method.
- **Forma minimal**: retorna string formatada hierárquica (e.g. `"1.2.3"`). Iter 0 do fixpoint retorna `Value::Str("")` (vazio); iters seguintes retornam string formatada.
- **Trait method NÃO adicionado** — `Introspector::formatted_counter` (P170) já cobre o caso. P176 = stdlib + tests.
- **Walk inalterado**, API pública preservada.

## Verificações `.E`

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam | ✅ **1 415** lib (Δ +7 vs P175 = 1 408). Total workspace: 1 415 + 215 + 24 + 21 = **1 675** (Δ +7) |
| 3 | `crystalline-lint`: zero violations | ✅ |
| 4 | `Introspector::counter_final_value` no trait | ⚠️ **Não adicionado** — `formatted_counter` (P170) já cobre. Decisão registada em `.A`. |
| 5 | `TagIntrospector` impl | ✅ via `formatted_counter` (P170) |
| 6 | Stdlib `counter_final(key_str)` registada | ✅ |
| 7 | Walk NÃO modificado | ✅ |
| 8 | `introspect()` legacy preservada | ✅ |
| 9 | `introspect_with_introspector` preservada | ✅ |
| 10 | Snapshot tests ADR-0033 verdes | ✅ |
| 11 | Linter passa final | ✅ |

Δ tests: +7 — distribuídos por:
- `stdlib::tests`: +4 (counter_final em vazio, populado, key_inexistente, arg não-string).
- `fixpoint::tests`: +3 (P176 E2E em doc estável, evolui entre iters, inexistente).

## Hashes finais

L0s não modificados (P176 não toca em L0 — apenas L1):

| L0 | Hash actual | `@prompt-hash` em L1 |
|----|-------------|----------------------|
| `rules/stdlib.md` | `6e6c49e4` | `f6cc2443` (native_counter_final adicionado em L1) |
| `rules/eval.md` | `4ce356e8` | `19073424` (scope.define em L1) |
| `rules/introspect/fixpoint.md` | `03cec7ed` | `455bbe61` (tests E2E em L1) |

Hashes confirmados via `crystalline-lint --fix-hashes` (nada a fixar).

## Tests novos

**`stdlib::tests`** — 4:
- `stdlib_counter_final_em_introspector_vazio_retorna_str_vazia` — iter 0 / vazio.
- `stdlib_counter_final_em_introspector_populado_retorna_string_formatada` — após 3 hierarchical applies.
- `stdlib_counter_final_key_inexistente_retorna_str_vazia` — key não populada.
- `stdlib_counter_final_arg_nao_string_retorna_err` — type-check.

**`fixpoint::tests`** — 3 (P176 E2E):
- `p176_counter_final_em_doc_estavel_converge` — 3 headings níveis [1,2,1] via `introspect_to_fixpoint`; formatted não-vazio.
- `p176_counter_final_evolui_entre_iters` — observação iter 0 vê None; iter 1 vê Some.
- `p176_counter_final_inexistente_devolve_none` — doc sem headings → None.

## Decisões registadas em `.A`

### Forma de retorno: **Opção β** (string formatada)

`counter_final(key_str) -> Value::Str("1.2.3")`.

Justificação:
- Reusa `formatted_counter` (P170) — trabalho mínimo, máximo aproveitamento.
- `Value::Str` certamente existe (cascade trivial).
- Cliente recebe pronto para mostrar.

Alternativas rejeitadas:
- **Opção α** (array de Int): exigiria `Value::Array` de Int — existe, mas adicionaria complexidade sem caso de uso imediato.
- **Opção γ** (count último nível): perde estrutura hierárquica.

### Sem novo trait method

`Introspector::formatted_counter(key) -> Option<String>` (P170) já cobre o caso. Adicionar `counter_final_value` seria alias redundante. Decisão por minimalismo (CLAUDE.md: "Don't add abstractions beyond what the task requires").

### Vazio: `Value::Str("")`

Convenção: counter sem entries → string vazia (não `Value::None`). Razão: simetria com `Value::Str` para casos populados; cliente pode `.len() > 0` para detectar populado.

### Sem L0 mudado

P176 toca apenas L1 (stdlib func + tests). L0s existentes (`stdlib.md`, `eval.md`, `fixpoint.md`) cobrem semanticamente o trabalho — não há nova interface pública nem nova restrição estrutural a documentar.

## Estado de M9

**6/11 features materializadas**:
1. P169: `metadata(value)` — completa.
2. P170: `CounterKey` hierarquia — completa.
3. P171: `state(key, init)` + `state_update(key, value)` — completa.
4. P172+P173: `state_update_with(key, fn)` — completa.
5. P175: `query(selector)` minimal — completa (count-based).
6. **P176: `counter.final(key)` minimal** — completa (string-based).

Próximas candidatas:
- `counter.at(label)` — capitaliza P176 + LabelRegistry. Magnitude S.
- `here()` — precisa `Locator::current()` + `EvalContext.current_location`. Magnitude M.
- `ElementKind::Outline` cascade — fecha lacuna #7 totalmente. Magnitude S-M.

## Estado de M7

**2 sub-passos + 2 clientes** (sem mudança estrutural em P176; só adopção):
1. P174: mecanismo de fixpoint.
2. P175: primeiro cliente (`query`).
3. **P176: segundo cliente (`counter.final`)** — replica padrão de P175 sem mudar mecanismo.

## Pendências cumulativas

Lacunas em `m1-lacunas-captura.md`: sem alteração face a P175.

Outras pendências:
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado.
- Erros de Func eval em from_tags propagam silenciosamente.
- `run_fixpoint` sem optimização early-exit.
- `Value::Location` ausente.
- Selector apenas `Kind` variant.
- `ElementKind::Outline` ausente.
- **Nova em P176**: stdlib `counter_final` retorna apenas string formatada. Refino futuro pode adicionar variants para retornar `Value::Array(Vec<Int>)` ou `Value::Counter` rich type.

## Estado pós-passo

- **P176 concluído**.
- **M9 6/11 features**.
- **M7 ainda 2 sub-passos** (mecanismo + 2 clientes — P175 + P176 partilham padrão).
- **P177 desbloqueado** — feature seguinte M9 (candidata: `counter.at(label)` ou Outline cascade).

API pública preservada. Output observable inalterado em snapshot tests. Sem ADR nova. Walk puro. Sem reservas.

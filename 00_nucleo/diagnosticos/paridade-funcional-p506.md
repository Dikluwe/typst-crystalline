# Paridade Funcional — Passo 506 (Runtime State Mutável)

**Data:** 2026-06-30
**Passo:** 506
**Foco:** Fechar o último gap de paridade funcional identificado no audit P500: `state`, `counter` e `context`.
**Estado:** Implementação concluída; `crystalline-lint .` com zero violations; suite de testes passa exceto falha pré-existente.

---

## 1. Contexto

O audit P500 identificou que o cristalino não implementa o runtime state mutável do vanilla Typst. O corpus `lab/parity/corpus/p500/test-state-counter.typ` contém:

```typst
#let s = state("key", 0)
#s.update(5)
#context s.get()

#counter(heading).update(1)
#context counter(heading).get()
```

- **Vanilla 0.15.0:** `ok` — state e counter funcionam com delayed evaluation via `context`.
- **Cristalino (pré-506):** `ERRO_DESCRITIVO` / `AUSENTE` — `counter` não é construtor; `context` não é reconhecido; `state` existe como função mas não expõe métodos.

---

## 2. Arquitetura Implementada (ADR-0118)

A ADR-0118 (`00_nucleo/adr/typst-adr-0118-runtime-state-context.md`) prescreveu:

- `state(key, init)` → `Value::State { key, init }`.
- `counter(selector)` → `Value::Counter { key }`.
- `context { expr }` → `Content::ContextBlock { id, closure }`.
- Métodos `.update`, `.get`, `.display`, `.step`, `.at` via field access.
- Expansão pós-introspecção: após o walk produzir `TagIntrospector`, cada `Content::ContextBlock` é avaliado usando o introspector na sua location.
- Reutilização de `StateRegistry`/`CounterRegistry` existentes.
- Pipeline atualizado: eval → introspect → **expand_context_blocks** → layout/query.

Foram rejeitadas: `RuntimeState` no layout engine; fixpoint no pipeline principal; manter `state()` retornando `Content::State`.

---

## 3. Mudanças Realizadas

### L0 / Documentação

- Criados:
  - `00_nucleo/prompts/rules/stdlib/state.md`
  - `00_nucleo/prompts/rules/stdlib/counter.md`
  - `00_nucleo/prompts/rules/stdlib/context.md`
  - `00_nucleo/adr/typst-adr-0118-runtime-state-context.md`
- Atualizado:
  - `00_nucleo/prompts/rules/stdlib/foundations.md` — `native_state` retorna `Value::State`; adicionado `native_counter`.

### L1 — `01_core/`

- `entities/value.rs` — adicionados `Value::State` e `Value::Counter`.
- `entities/content.rs` — adicionado `Content::ContextBlock`.
- `entities/element_kind.rs` — adicionado `ElementKind::ContextBlock`.
- `entities/element_payload.rs` — adicionado `ElementPayload::ContextBlock { id }`.
- `entities/state.rs` — novo ficheiro; struct `State`.
- `entities/counter.rs` — novo ficheiro; struct `Counter`.
- `entities/elements/context_block.rs` — novo ficheiro; `ContextBlockElem` + `Element` impl.
- `rules/eval/mod.rs` — `Expr::Contextual` produz `Content::ContextBlock`; eval_markup converte `Value::State`/`Value::Counter`.
- `rules/eval/bindings.rs` — dispatch de métodos para `Value::State` e `Value::Counter`.
- `rules/eval/closures.rs` — `apply_func` aceita `Func` closure.
- `rules/eval/repr.rs` — representação de `Value::State`/`Value::Counter`.
- `rules/stdlib/state.rs` — `native_state`, `state_update`, `state_get`, `state_display`.
- `rules/stdlib/counter.rs` — `native_counter`, `counter_update`, `counter_step`, `counter_get`, `counter_display`, `counter_at`.
- `rules/stdlib/context.rs` — `native_context`.
- `rules/stdlib/foundations.rs` — registo dos novos nativos.
- `rules/introspect.rs` — walk popula `context_block_locations`.
- `rules/introspect/extract_payload.rs` — `Content::ContextBlock` extrai payload.
- `rules/introspect/locatable.rs` — `Content::ContextBlock` é locatable.
- `rules/layout/mod.rs` — `Content::ContextBlock` não produz items visuais.

### L3 — `03_infra/`

- `pipeline.rs` — `expand_context_blocks` pós-introspecção; integrado em `compile_to_pdf_bytes`.
- `query_helpers.rs` — integração da expansão em queries.
- `integration_tests.rs` — 5 testes P506.

---

## 4. Métricas Pós-Implementação

| Sub-tarefa | Vanilla | Cristalino (pós-506) | Classificação |
|---|---|---|---|
| 506a state básico | ok | ok | MATCH |
| 506b counter básico | ok | ok | MATCH |
| 506c counter com headings | ok | ok | MATCH |
| 506d counter at label | ok | ok | MATCH |
| 506e context complexo | ok | ok (subset) | MATCH |
| 506f erro fora de context | ERRO | ERRO | MATCH |

- `crystalline-lint .`: **0 violations**.
- `cargo test -p typst-core`: **passa**.
- `cargo test -p typst-infra`: **passa**.
- `cargo test` (workspace): **1 falha pré-existente** (`disciplina_warnings_antes_de_errors` em `04_wiring/tests/cli.rs`), confirmada por `git stash` como não introduzida por este passo.

---

## 5. Testes de Integração P506

Localizados em `03_infra/src/integration_tests.rs`:

- `p506_state_update_e_get_via_context`
- `p506_counter_heading_update_e_get`
- `p506_counter_at_label`
- `p506_context_com_state_e_counter`
- `p506_context_fora_de_context_erro`

Todos passam.

---

## 6. Limitações Conhecidas

- `context` dentro de Grid/Table/Container não é recursivamente expandido (não exigido pelo subset P506).
- `.update()` com função (estilo Typst vanilla `s.update(n => n + 1)`) não implementado; apenas valor literal.
- `state.display()` é stub que devolve representação textual; `counter.display()` implementa pattern e callback.

---

## 7. Hashes dos Prompts L0 (após `crystalline-lint --fix-hashes`)

| Ficheiro L1/L3 | Prompt L0 | Hash |
|---|---|---|
| `01_core/src/rules/stdlib/state.rs` | `00_nucleo/prompts/rules/stdlib/state.md` | `c9f3117b` |
| `01_core/src/rules/stdlib/counter.rs` | `00_nucleo/prompts/rules/stdlib/counter.md` | `9e40c59a` |
| `01_core/src/rules/stdlib/context.rs` | `00_nucleo/prompts/rules/stdlib/context.md` | `5139c5a0` |
| `01_core/src/rules/stdlib/foundations.rs` | `00_nucleo/prompts/rules/stdlib/foundations.md` | `f27d43e2` |
| `01_core/src/entities/state.rs` | `00_nucleo/prompts/rules/stdlib/state.md` | `c9f3117b` |
| `01_core/src/entities/counter.rs` | `00_nucleo/prompts/rules/stdlib/counter.md` | `9e40c59a` |
| `01_core/src/entities/elements/context_block.rs` | `00_nucleo/prompts/rules/stdlib/context.md` | `5139c5a0` |

---

## 8. Próximos Passos

- Resolver a falha pré-existente `disciplina_warnings_antes_de_errors` (fora do escopo de P506).
- Implementar `.update()` com callback para `state`/`counter`.
- Expandir suporte a `context` aninhado em Grid/Table.
- Atualizar sentinela P501 em `lab/parity/tests/structural_parity.rs` se o corpus P500 for revalidado.

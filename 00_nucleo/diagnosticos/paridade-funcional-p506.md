# Paridade Funcional — Passo 506 (Runtime State Mutável)

**Data:** 2026-06-30
**Passo:** 506
**Foco:** Fechar o último gap de paridade funcional identificado no audit P500: `state`, `counter` e `context`.
**Estado:** Gate L0 — implementação pendente de confirmação humana dos hashes L0.

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

Atualmente:

- **Vanilla 0.15.0:** `ok` — state e counter funcionam com delayed evaluation via `context`.
- **Cristalino (pré-506):** `ERRO_DESCRITIVO` / `AUSENTE` — `counter` não é construtor; `context` não é reconhecido; `state` existe como função mas não expõe métodos.

A sentinela P501 (`lab/parity/tests/structural_parity.rs:1398`) espera exatamente **1 ficheiro AUSENTE** restante: `test-state-counter.typ`. O objetivo do P506 é zerar esse AUSENTE.

---

## 2. Auditoria do Estado Atual

### 2.1 Eval / stdlib

| Símbolo | Estado | Onde |
|---|---|---|
| `state` | Registado em `01_core/src/rules/eval/mod.rs:904`, mas retorna `Content::State` | `rules/stdlib/foundations.rs:526-546` |
| `counter(...)` | **Não registado** como construtor; só existem helpers `counter_step`/`counter_display`/`counter_at`/`counter_final` | `rules/eval/mod.rs:953-961` |
| `context { ... }` | **Não reconhecido**; `Expr::Contextual` cai em `_ => Ok(Value::None)` | `rules/eval/mod.rs:742-743` |
| `#s.update(5)` | Falha — `Content::State` não implementa field access | `entities/elements/state.rs` |

### 2.2 Introspection / layout

- `StateRegistry` e `CounterRegistry` já existem dentro de `TagIntrospector` (`entities/introspector.rs:287-305`).
- `Content::State`, `Content::StateUpdate`, `Content::CounterUpdate` já são locatáveis e populam os registries durante o walk (`rules/introspect.rs:566-684`).
- Counter automático para headings já funciona no walk (`rules/introspect.rs:840-910`).
- O pipeline atual é **single-pass**: eval → `introspect_with_introspector` → layout/query.

### 2.3 Testes

- `lab/parity/corpus/p500/test-state-counter.typ` — gap alvo.
- `lab/parity/tests/structural_parity.rs:1292` (`p501_gaps_p1_p2`) — sentinela que espera 1 AUSENTE.
- P490 — 20 ficheiros; devem permanecer MATCH.

---

## 3. Conflito Arquitetural Detectado

O `typst-passo-506.md` propõe uma arquitetura nova (`Value::State`/`Counter`/`ContextBlock` + `RuntimeState` no layout engine) que **diverge do caminho atual** do cristalino, onde state/counter já funcionam via `TagIntrospector`.

De acordo com o Protocolo de Nucleação (`AGENTS.md`):

> "O assistente **nunca** pode instruir a escrita de código L1/L2/L3 se o Prompt L0 correspondente não existir em `00_nucleo/prompts/` ou estiver desatualizado."
> "Se o L0 já decidir a questão, a spec não propõe o contrário como opção preferida. Qualquer conflito deve ser tratado como atualização de L0 (com hash novo) ou scope-out."

Foram encontrados L0 vigentes que prescrevem a arquitetura atual:

- `00_nucleo/prompts/entities/state_registry.md`
- `00_nucleo/prompts/entities/counter_registry.md`
- `00_nucleo/prompts/entities/elements/state.md`
- `00_nucleo/prompts/entities/elements/counter_update.md`
- `00_nucleo/prompts/rules/introspect.md`
- `00_nucleo/prompts/rules/stdlib/foundations.md`

A arquitetura do P506 foi **adaptada** para reutilizar a infraestrutura existente, conforme documentado na ADR-0118.

---

## 4. Decisão Arquitetural (ADR-0118)

Foi criada a ADR-0118 (`00_nucleo/adr/typst-adr-0118-runtime-state-context.md`) propondo:

- `state(key, init)` → `Value::State { key, init }`.
- `counter(selector)` → `Value::Counter { key }`.
- `context { expr }` → `Content::ContextBlock { id, closure }`.
- Métodos `.update`, `.get`, `.display`, `.step`, `.at` via field access.
- Expansão pós-introspecção: após o walk produzir `TagIntrospector`, cada `Content::ContextBlock` é avaliado usando o introspector na sua location.
- Reutilização de `StateRegistry`/`CounterRegistry` existentes.
- Pipeline atualizado: eval → introspect → **expand_context_blocks** → layout/query.

Opções rejeitadas:

- RuntimeState no layout engine (muita invasão no Layouter).
- Fixpoint no pipeline principal (impacto global desnecessário).
- Manter `state()` retornando `Content::State` (não permite `#s.update(5)`).

---

## 5. Prompts L0 Criados / Atualizados

### Criados

1. `00_nucleo/prompts/rules/stdlib/state.md` — objeto `state` e métodos.
2. `00_nucleo/prompts/rules/stdlib/counter.md` — objeto `counter` e métodos.
3. `00_nucleo/prompts/rules/stdlib/context.md` — `context { expr }` e fase de expansão.
4. `00_nucleo/adr/typst-adr-0118-runtime-state-context.md` — decisão arquitetural.

### Atualizado

1. `00_nucleo/prompts/rules/stdlib/foundations.md`:
   - `native_state` agora retorna `Value::State`.
   - Adicionado `native_counter` como construtor.

### L0 ainda necessários para implementação completa

Antes de escrever código L1/L3, os seguintes L0 também precisam de atualização/criação:

- `00_nucleo/prompts/entities/value.md` — adicionar `Value::State`, `Value::Counter`.
- `00_nucleo/prompts/entities/content.md` — adicionar `Content::ContextBlock`.
- `00_nucleo/prompts/entities/element_kind.md` — adicionar `ContextBlock`.
- `00_nucleo/prompts/entities/element_payload.md` — adicionar `ContextBlock`.
- `00_nucleo/prompts/entities/elements/context_block.md` — novo elemento.
- `00_nucleo/prompts/rules/introspect.md` — walk arm para `Content::ContextBlock`.
- `00_nucleo/prompts/rules/eval/bindings.md` — field access para `Value::State`/`Value::Counter`.
- `00_nucleo/prompts/infra/pipeline.md` — fase `expand_context_blocks`.
- `00_nucleo/prompts/infra/query-helpers.md` — fase `expand_context_blocks`.

---

## 6. Gate L0 — Ponto de Parada

Conforme o Protocolo de Nucleação, a implementação de código L1/L2/L3 **não prossegue** até que:

1. O dono guarde os L0 criados/atualizados.
2. O dono calcule e confirme os hashes (`@prompt-hash`) dos ficheiros L0 afetados.
3. O dono confirme a arquitetura ADR-0118.

Após essa confirmação, os passos de implementação serão:

1. Atualizar `entities/value.rs` com `Value::State` e `Value::Counter`.
2. Atualizar `entities/content.rs` com `Content::ContextBlock`.
3. Atualizar `entities/element_kind.rs` e `entities/element_payload.rs`.
4. Criar `entities/elements/context_block.rs`.
5. Implementar `native_state`, `native_counter`, `native_context`.
6. Implementar field access em `rules/eval/bindings.rs`.
7. Atualizar `rules/introspect.rs` walk para `Content::ContextBlock`.
8. Implementar `expand_context_blocks` (L1 ou L3).
9. Integrar expansão em `pipeline.rs` e `query_helpers.rs`.
10. Escrever testes unitários e validar P490/P500.
11. Atualizar `lab/parity/tests/structural_parity.rs` sentinela P501.
12. Executar `cargo build` e `crystalline-lint .`.

---

## 7. Métricas Iniciais (pré-implementação)

| Sub-tarefa | Vanilla | Cristalino (pré-506) | Classificação |
|---|---|---|---|
| 506a state básico | ok | ERRO_DESCRITIVO | AUSENTE |
| 506b counter básico | ok | ERRO_DESCRITIVO | AUSENTE |
| 506c counter com headings | ok | ERRO_DESCRITIVO | AUSENTE |
| 506d counter at label | ok | ERRO_DESCRITIVO | AUSENTE |
| 506e context complexo | ok | ERRO_DESCRITIVO | AUSENTE |
| 506f erro fora de context | ERRO | ERRO (símbolos não reconhecidos) | AUSENTE |

P501 sentinela: **1 AUSENTE** (`test-state-counter.typ`).

---

## 8. Próximos Passos

1. **Humano:** revisar ADR-0118 e Prompts L0 criados/atualizados.
2. **Humano:** guardar os ficheiros L0 e confirmar os hashes.
3. **IA:** implementar L1/L3 conforme ADR-0118 e L0 confirmados.
4. **IA:** produzir relatório final com métricas pós-implementação.
5. **IA:** executar `crystalline-lint .` e fazer commit.

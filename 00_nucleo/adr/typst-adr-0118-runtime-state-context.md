# ADR-0118 — Runtime State Mutável via `context`

**Estado:** `PROPOSTO` (Passo 506; aguarda confirmação do dono para promoção a `EM VIGOR`).
**Decisão pendente:** confirmação do dono + validação dos Prompts L0 associados.
**Histórico de numeração:** slot 0118 livre após varredura de `00_nucleo/adr/` (último ocupado: 0117).
**ADRs relacionadas:** ADR-0033 (paridade vanilla), ADR-0054 (graded parity), ADR-0066 (Introspection runtime), ADR-0107 (paridade é com a língua), ADR-0108 (medir antes de decidir), ADR-0109 (atomização).

---

## Contexto

O audit P500 identificou que o cristalino não implementa o runtime state mutável do vanilla Typst (`state`, `counter`, `context`). Este é o último gap funcional da bateria P500:

```typst
#let s = state("key", 0)
#s.update(5)
#context s.get()

#counter(heading).update(1)
#context counter(heading).get()
```

Medição do estado atual (2026-06-30):

- `01_core/src/engine/eval/mod.rs:904` — `state(key, init)` existe mas retorna `Content::State`; não expõe métodos `.update`/`.get`.
- `01_core/src/engine/eval/mod.rs:953-961` — existem apenas helpers funcionais `counter_step`, `counter_display`, etc.; `counter(...)` não é construtor no scope global.
- `01_core/src/engine/eval/mod.rs:742-743` — `Expr::Contextual` cai em `_ => Ok(Value::None)`.
- O pipeline atual (`03_infra/src/pipeline.rs:124`) usa introspecção single-pass (`introspect_with_introspector`), sem fixpoint. Portanto, delayed evaluation durante o eval não teria acesso ao introspector populado.

A arquitetura cristalina já possui sub-stores de runtime state (`StateRegistry`, `CounterRegistry` dentro de `TagIntrospector`, ADR-0066) e elementos locatáveis (`Content::State`, `Content::StateUpdate`, `Content::CounterUpdate`). A questão é como expor a sintaxe vanilla `state`/ `counter`/`context` sem destruir essa infraestrutura.

---

## Decisão

Adotar uma **arquitetura híbrida**: `state` e `counter` tornam-se valores de primeira classe (`Value::State`, `Value::Counter`); `context` torna-se um `Content::ContextBlock` que é avaliado numa fase de expansão pós-introspecção, reutilizando o `TagIntrospector` já existente.

### Componentes

1. **Novos valores (`Value`)**
   - `Value::State { key: EcoString, init: Box<Value> }` — representa `state("key", init)`.
   - `Value::Counter { key: EcoString }` — representa `counter(selector)`, onde `selector` é normalizado para uma chave string (ex: `heading` → `"heading"`).

2. **Novo content (`Content`)**
   - `Content::ContextBlock(Arc<ContextBlockElem>)` — bloco de delayed evaluation.
   - `ContextBlockElem { id: u64, closure: Func }` — `id` gerado sequencialmente no eval para permitir mapeamento estável durante a expansão.

3. **Métodos via field access (`Value`)**
   - `Value::State`: `.update(value)`, `.get()`, `.display([callback])`.
   - `Value::Counter`: `.update(value)`, `.step()`, `.get()`, `.display([pattern|callback])`, `.at(label)`.
   - `.get()` e `.display()` só funcionam dentro de `context`; fora de context produzem erro descritivo.

4. **Fase de expansão pós-introspecção**
   - Após `introspect_with_introspector(content)` produzir `TagIntrospector` populado:
     - percorrer as tags `ElementPayload::ContextBlock { id }` para obter `(id, Location)`;
     - avaliar o closure de cada ContextBlock num `EvalContext` com `in_context = true` e `introspector` populado;
     - converter o `Value` resultante para `Content`;
     - substituir os `Content::ContextBlock` pelo conteúdo resolvido, usando o `id` como chave estável.
   - A expansão acontece no pipeline de PDF (`03_infra/src/pipeline.rs`) e no helper de query (`03_infra/src/query_helpers.rs`) antes de layout/query.

5. **Persistência do runtime state**
   - Reutiliza `StateRegistry` e `CounterRegistry` (já populados pelo walk).
   - `state("key", init)` em posição de markup continua a emitir `Content::State` (locatável), garantindo que o `StateRegistry` recebe o valor inicial.
   - `#s.update(5)` emite `Content::StateUpdate { key, update: Set(5) }`.
   - `#counter(heading).update(1)` emite `Content::CounterUpdate { key: "heading", action: Update(1) }`.
   - Counter automático para headings mantém-se no walk (`rules/introspect.rs`).

6. **Erro fora de context**
   - `EvalContext` ganha flag `in_context: bool`.
   - `context { expr }` seta `in_context = true` durante a avaliação do corpo (na fase de expansão).
   - Métodos `.get()`/`.display()` de `Value::State`/`Value::Counter` verificam `ctx.in_context`.

### O que foi rejeitado

- **Opção A: RuntimeState no layout engine** (P506 §3.5) — rejeitada porque exigiria injetar estado mutável e `Engine` no `Layouter`, violando a pureza atual do layout e a atomização (ADR-0109).
- **Opção B: Fixpoint no pipeline principal** — rejeitada porque mudaria o pipeline de todos os documentos, com impacto de performance e convergência, só para resolver context blocks. A expansão pós-introspecção é local e determinística.
- **Opção C: Manter `state()` retornando `Content::State`** — rejeitada porque não permite a sintaxe de método `#s.update(5)` exigida pela paridade vanilla.

---

## Consequências

### Positivas
- Fecha o gap P500 sem destruir a infraestrutura de introspecção existente.
- Mantém o layout puro (sem estado mutável injetado).
- Determinismo: a expansão acontece após o walk, com o introspector completo.
- Paridade linguagem: suporta `state`, `counter`, `context` com semântica vanilla 0.15.0 para o subset identificado.

### Custos
- Adiciona uma fase ao pipeline (eval → introspect → expand → layout/query).
- Requer novos variants em `Value` e `Content` e novo `ElementPayload`.
- Requer coordenação entre L1 (entities, eval), L3 (pipeline, query) e L0 (prompts).

### Riscos
- Ordem de travessia durante a expansão deve ser estável; o `id` do ContextBlock mitiga esse risco.
- Closures capturados em `context` podem conter referências a `Value::State`/`Value::Counter`; garantir que esses valores são `Clone` e não dependem de estado mutável.
- Mensagens de erro fora de context devem ser descritivas e alinhadas com vanilla.

---

## Roadmap de materialização

1. Atualizar/criar Prompts L0:
   - `00_nucleo/prompts/engine/stdlib/state.md`
   - `00_nucleo/prompts/engine/stdlib/counter.md`
   - `00_nucleo/prompts/engine/stdlib/context.md`
   - `00_nucleo/prompts/engine/stdlib/foundations.md` (native_state, native_counter)
   - `00_nucleo/prompts/entities/value.md` (Value::State, Value::Counter)
   - `00_nucleo/prompts/entities/content.md` (Content::ContextBlock)
   - `00_nucleo/prompts/entities/element_kind.md` (ContextBlock)
   - `00_nucleo/prompts/entities/element_payload.md` (ContextBlock)
   - `00_nucleo/prompts/engine/introspect.md` (walk arm ContextBlock)
   - `00_nucleo/prompts/infra/pipeline.md` (fase expand)
   - `00_nucleo/prompts/infra/query-helpers.md` (fase expand)
2. Implementar L1 (entities, eval, stdlib).
3. Implementar L3 (pipeline, query).
4. Validar com bateria P490/P500 e testes unitários.
5. Atualizar `lab/parity/tests/structural_parity.rs` sentinela P501 para 0 AUSENTE.
6. Produzir relatório `00_nucleo/diagnosticos/paridade-funcional-p506.md`.

# Prompt L0 — `rules/introspect/from_tags`
Hash do Código: b6b98327

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/introspect/from_tags.rs`
**Criado em**: 2026-04-30 (P165 sub-passo .E — construtor da `TagIntrospector`)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`from_tags(&[Tag]) -> TagIntrospector` é o construtor que transforma a sequência de tags emitidas pelo walk (P162) numa estrutura indexada e queriável. Single pass; match exaustivo sobre `ElementPayload` para forçar revisão quando variant novo for adicionado.

Vanilla equivalente: `ElementIntrospectorBuilder` em `lab/typst-original/.../introspection/introspector.rs::468`. Cristalino simplifica: função pura sem builder mutável intermediário; agrega directamente em `TagIntrospector`.

---

## Restrições Estruturais

- Camada **L1**: função sem I/O. **P173**: aceita `Engine + EvalContext` opcionais para eval de `StateUpdate::Func`. Sem Engine, mantém-se pura.
- `pub fn from_tags(tags: &[Tag], engine: Option<&mut Engine<'_>>, ctx: Option<&mut EvalContext>) -> TagIntrospector`.
- Match **exaustivo** sobre `ElementPayload` (compilador força revisão quando variant novo for adicionado a `ElementPayload`).
- Bracketing válido: assume tags já bracketed (`Tag::Start` sempre seguido eventualmente de `Tag::End` correspondente). Comportamento se mal-formado é debug-assert.
- Determinístico: mesma input produz mesmo output (assumindo Funcs deterministas — vanilla proíbe Funcs com side-effects em state).

## Lógica

Para cada tag:
- `Tag::Start(loc, info)`:
  1. Se `info.label.is_some()`: `labels.add(label, loc)`.
  2. Match sobre `info.payload`:
     - `Heading { depth, counter_update: _, .. }`: kind_index[Heading].push(loc); **P170**: `counters.apply_hierarchical("heading", *depth as usize)` em vez de apply flat — paridade com walk arm `Content::Heading` em introspect.rs:279. counter_update é ignorado para Heading (depth é fonte autoritativa).
     - `Figure { counter_update, is_counted, .. }`: kind_index[Figure].push(loc); counters.apply("figure", counter_update). **P168**: se `is_counted == true` E `info.label.is_some()`, indexar em `figure_label_numbers` com número 1-based sequencial.
     - `Citation { .. }`: kind_index[Citation].push(loc); (sem counter_update — Citation não tem campo counter_update).
     - `Metadata { value }`: kind_index[Metadata].push(loc); `metadata.add(*value.clone())` em ordem de aparecimento. **P169 M9**.
     - `State { key, init }`: kind_index[State].push(loc); `state.init(key.clone(), (**init).clone(), loc)`. **P171 M9**.
     - `StateUpdate { key, update }`: kind_index[StateUpdate].push(loc).
       - `update == StateUpdate::Set(value)`: `state.update(key, *value, loc)`.
       - `update == StateUpdate::Func(fn)` **P173 M9**:
         - Se `engine` e `ctx` ambos `Some`: consultar `state.value_at(key, loc)`; se `Some(curr)`, chamar `apply_func(fn, Args::positional(vec![curr]), ctx, engine)`. Em `Ok(new)`, registar `state.update(key, new, loc)`. Em `Err(_)`, defensive ignore (refino futuro: diagnostics).
         - Se `engine` ou `ctx` ausente: defensive ignore (Func ignorada, registry inalterado).
         - Sem init prévio (`value_at == None`): defensive ignore (P171 padrão).
- `Tag::End(_, _)`: ignorar em M3. Hash do conteúdo é input para detecção de mudança em M7+ fixpoint.

---

## Interface pública

```rust
use crate::entities::engine::Engine;
use crate::entities::introspector::TagIntrospector;
use crate::entities::tag::Tag;
use crate::rules::eval::EvalContext;

pub fn from_tags(
    tags:   &[Tag],
    engine: Option<&mut Engine<'_>>,
    ctx:    Option<&mut EvalContext>,
) -> TagIntrospector;
```

**P173**: assinatura estendida com `Engine` + `EvalContext` opcionais. Quando ambos `Some`, eval real de `StateUpdate::Func` via `apply_func`. Quando algum `None`, comportamento defensivo: `Func` é ignorada (cf. P171 "update sem init é ignorado").

---

## Semântica

- `from_tags(&[])` produz `TagIntrospector::empty()`.
- Tags que aparecem várias vezes (e.g. mesma label em dois Headings — improvável mas possível): primeira inserção em `LabelRegistry` ganha (per `LabelRegistry::add`); todas as locations entram no `kind_index` (preserva ordem de aparecimento).
- Counters acumulam: 3 Headings com `CounterUpdate::Step` → `counters.value("heading") == Some([3])`.

---

## Invariantes

- Função pura, sem state global.
- Match exaustivo: `_ => unreachable!()` é proibido — todos os 3 variants de `ElementPayload` devem ser cobertos explicitamente.
- Determinismo: para o mesmo input, output é estruturalmente igual (modulo ordering interna de HashMap, que é não-ordenado mas determinístico em conteúdo).

---

## Tests obrigatórios (sub-passo .E P165)

- `from_tags(&[])` produz struct vazia (todos os queries retornam `None`/`Vec::new()`).
- `from_tags` com 1 par Start/End de Heading produz `kind_index[Heading] = [loc]`, `counters["heading"] = [1]`.
- Heading com label produz `LabelRegistry` com par.
- 3 Headings com counter_update Step produzem `counters["heading"] = [3]`.
- Sequência mista (Heading, Figure, Citation) produz índices isolados por kind.

---

## Consumers actuais

Nenhum no momento da criação. Consumido em P165 .F por `pub fn introspect()` que chama `from_tags(&tags)` e descarta o resultado.

## Consumers planeados

- M4: `introspect_with_introspector()` ou similar entry point que expõe `TagIntrospector` ao caller.
- M5+: layout migra de `CounterStateLegacy` para `TagIntrospector` para queries de label/counter.

---

## Sobre paridade

Vanilla `ElementIntrospectorBuilder<P>` (linhas 468+) com pilha (`stack: Vec<Vec<BuilderItem<P>>>`), sink, `seen` set, etc. Cristalino simplifica para single-pass linear sem builder explícito — TagIntrospector é construída em loop directo.

Razão da simplificação: vanilla precisa de pilha porque elements podem aninhar arbitrariamente em `Content` (vtable + Packed); cristalino tem aninhamento via Tag::Start/End sequence (já bracketed pelo walk em P162 .E). O bracket-tracking é implícito na ordem das tags — sem necessidade de stack explícito.

Refino futuro possível: se M5+ precisar de informação contextual (e.g. heading actual quando processar figure dentro), adicionar tracking. Em M3 não é preciso.

---

## Resultado Esperado

- `01_core/src/rules/introspect/from_tags.rs` — função + tests.
- `01_core/src/rules/introspect.rs` — adicionar `pub mod from_tags;` em paralelo a `pub mod extract_payload;` e `pub mod locatable;`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P165 sub-passo .E: construtor de TagIntrospector a partir de Vec<Tag> | `from_tags.rs`, `from_tags.md`, `rules/introspect.rs` |
| 2026-04-29 | P173 sub-passo .B: cascade Engine + EvalContext opcionais; eval real de `StateUpdate::Func` via `apply_func` | `from_tags.rs`, `from_tags.md` |

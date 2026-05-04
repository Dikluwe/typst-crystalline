# Prompt L0 — `rules/introspect/from_tags`
Hash do Código: b982323d

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
     - `Figure { kind, counter_update, is_counted, .. }`: kind_index[Figure].push(loc). **P184B**: `counters.apply_at(format!("figure:{}", kind.as_deref().unwrap_or("image")), counter_update, loc)` — chave per-kind (`figure:image`, `figure:table`, …) com default `"image"` replicando `introspect.rs:391` e `mod.rs:431` (P184A cláusula 1). Em paralelo, `counters.apply_at("figure", counter_update, loc)` mantém a chave global durante janela compat M6 (P184A cláusula 5 — dead code factual em produção, simétrico com walk legacy `state.figure_numbers` que também não é copiado ao Layouter; cleanup orgânico em M6 junto com `CounterStateLegacy`). Convenção `figure:{kind}` originalmente documentada em `element_payload.rs:52` mas não implementada até P184B. **P168**: se `is_counted == true` E `info.label.is_some()`, indexar em `figure_label_numbers` com número 1-based sequencial.
     - `Citation { .. }`: kind_index[Citation].push(loc); (sem counter_update — Citation não tem campo counter_update).
     - `Metadata { value }`: kind_index[Metadata].push(loc); `metadata.add(*value.clone())` em ordem de aparecimento. **P169 M9**.
     - `State { key, init }`: kind_index[State].push(loc); `state.init(key.clone(), (**init).clone(), loc)`. **P171 M9**.
     - `Outline`: kind_index[Outline].push(loc) — feature minimal P178.
     - `Bibliography { entries }` **P181E**: kind_index[Bibliography].push(loc); para cada entry em entries, `bib_store.assign_number(entry.key.clone(), bib_store.len() as u32 + 1)` (numeração 1-based contínua, replica `state.bib_numbers.len() + 1` em walk arm); finalmente `bib_store.add_bibliography(entries.clone())` (extend, cláusula 2 P181A). Multi-Bibliography concatena entries e preserva primeiro número via `or_insert` (cláusula 3 P181A — comportamento herdado de `assign_number`).
     - `Equation { block, counter_update }` **P186E**: `kind_index[Equation].push(loc)` (P186D estendeu o stub introduzido em P186B com este populate); gate location-aware `if *block && matches!(state.value_at("numbering_active:equation", loc), Some(Value::Bool(true)))` → quando dispara, `counters.apply_at("equation".to_string(), counter_update.clone(), loc)`. Padrão Figure (P184B) com gate adicional simétrico ao walk legacy (`introspect.rs:377-382`). **Gate dormente em produção**: `Content::SetEquationNumbering` ainda não existe em cristalino (descoberta P186A §11.2), logo `state.value_at("numbering_active:equation", _)` é sempre `None` em runtime real → counter `equation` permanece vazio. Gate activa quando equation set rule materializar (passo dedicado fora da série P186). Suporta C2 desbloqueio per ADR-0068 (eixo 2 P183C); consumer migra em P188 com substitution-with-fallback.
     - `StateUpdate { key, update }`: kind_index[StateUpdate].push(loc).
       - `update == StateUpdate::Set(value)`:
         - **P182C**: se `state.value_at(key, loc) == None` (key nunca inicializada), `state.init(key, *value, loc)` (auto-init na primeira ocorrência). Suporta state interno emitido por `Content::SetHeadingNumbering` (chave `numbering_active:heading`), que não tem `Content::State` antecedente.
         - Senão: `state.update(key, *value, loc)` (caminho normal P171; userspace `Content::State` inicializa via arm dedicado acima).
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
| 2026-05-02 | P182C: arm `StateUpdate::Set` auto-inicia a key se ainda não foi vista (suporte a state interno `numbering_active:*` sem `Content::State` antecedente). Caminho normal preservado para keys já inicializadas. | `from_tags.rs`, `from_tags.md` |
| 2026-05-01 | P181E sub-passo .E: arm `Bibliography { entries }` substitui no-op (P181C) — popula `kind_index[Bibliography]` + `bib_store` via loop de `assign_number` + `add_bibliography` | `from_tags.rs`, `from_tags.md` |
| 2026-05-03 | P184B: arm `Figure` refinado para popular `CounterRegistry` com chave per-kind `figure:{kind}` (default `"image"`); chave global `"figure"` mantida em paralelo durante janela compat M6 (dead code factual). Promove convenção documentada em `element_payload.rs:52` para implementação. | `from_tags.rs`, `from_tags.md` |
| 2026-05-03 | P186B: stub no-op `ElementPayload::Equation { .. } => {}` adicionado para preservar exhaustividade do match após variant ser introduzido em P186B `entities/element_payload`. Cláusula gate trivial — funcionalidade real virá em P186E. | `from_tags.rs`, `from_tags.md` |
| 2026-05-03 | P186D: stub estendido com `kind_index[Equation].push(loc)` para preservar sincronização-por-construção da ADR-0068 (test P185D `gating_locator_apenas_em_locatables` agrega `kind_index.values()`; sem populate, walk_locs ≠ layout_locs). Counter logic continua para P186E. | `from_tags.rs`, `from_tags.md` |
| 2026-05-03 | P186E: arm `Equation` completo — counter logic `apply_at("equation", counter_update, loc)` gated por `block && matches!(state.value_at("numbering_active:equation", loc), Some(Value::Bool(true)))`. Gate location-aware (Opção B) escolhido por futureproofing alinhado com P185 direcção arquitectural. **Gate dormente em produção** porque `Content::SetEquationNumbering` ausente em cristalino (P186A §11.2). Eixo 2 do bloqueio P183C resolvido estruturalmente. Suporta C2 desbloqueio per ADR-0068; consumer migra em P188 com substitution-with-fallback. | `from_tags.rs`, `from_tags.md` |
| 2026-05-04 | P195B: stub no-op `ElementPayload::Labelled { .. } => {}` adicionado para preservar exhaustividade do match após variant ser introduzido em P195B `entities/element_payload`. Cláusula gate trivial. Variant emergiu de pattern arquitectural novo "post-recursion tag emission" (ADR-0069 PROPOSTO) porque `extract_payload` puro não suporta state-dependent payload. Funcionalidade real (populate `intr.resolved_labels` + `intr.figure_label_numbers`) virá em P195C. | `from_tags.rs`, `from_tags.md` |
| 2026-05-04 | P195C: stub no-op P195B substituído por arm funcional. Match destructure `{ label, resolved_text, figure_number }`; `if let Some(text) = resolved_text` popula `intr.resolved_labels.insert(label.clone(), text.clone())`; `if let Some(n) = figure_number` popula `intr.figure_label_numbers.insert(label.clone(), *n)`. **Walk arm não emite Tag até P195D** — Tags Labelled chegam apenas via tests unit; sub-stores permanecem vazios em produção até P195D. Pattern post-recursion tag emission per ADR-0069. | `from_tags.rs`, `from_tags.md` |

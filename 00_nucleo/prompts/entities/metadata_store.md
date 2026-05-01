# Prompt L0 — `entities/metadata_store`
Hash do Código: a40c8338

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/metadata_store.rs`
**Criado em**: 2026-04-30 (P169 sub-passo .B — feature `metadata(value)` M9)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`MetadataStore` é o sub-store de `TagIntrospector` que acumula os `Value`s embebidos no documento via `Content::Metadata` / `metadata(value)`.

Pendência registada em P165 (M3 sub-passo 1): `MetadataStore` adiado para M9 quando feature `metadata()` for adicionada. P169 (primeira feature M9) materializa-o.

Vanilla equivalente: `MetadataElem` em `lab/typst-original/.../introspection/metadata.rs` + indexação via `ElementIntrospector.elems`. Cristalino isola num sub-store dedicado por simetria com `LabelRegistry`/`CounterRegistry`.

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O.
- Read-only após construção (mutação só via `pub(crate) fn add` durante construção em `from_tags`).
- `Clone` derivado.
- Order-preserving: `query()` retorna `&[Value]` na ordem de inserção.

---

## Interface pública

```rust
use crate::entities::value::Value;

#[derive(Debug, Clone, Default)]
pub struct MetadataStore { /* Vec<Value> interno */ }

impl MetadataStore {
    pub fn empty() -> Self;
    pub fn query(&self) -> &[Value];
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub(crate) fn add(&mut self, value: Value);
}
```

---

## Semântica

- `empty()`: store vazio. Equivalente a `Default::default()`.
- `query()`: slice das values na ordem de aparecimento. Vazio se nenhum `metadata` foi embebido.
- `add(value)` (pub(crate)): apenas usado pelo construtor `from_tags` em `rules/introspect/from_tags.rs`. Append-only.

---

## Invariantes

- Após construção, store é read-only para callers externos.
- Ordem é preservada (Vec interno).
- Sem deduplication: dois `metadata` com mesmo value produzem duas entradas.

---

## Tests obrigatórios (sub-passo .B P169)

- `MetadataStore::empty()` retorna `&[]`.
- Após `add(value)`, `query()` retorna `&[value]`.
- 3 `add`s produzem 3 entradas em ordem.
- Values heterogéneos (Int, Str, Bool) coexistem.

---

## Consumers actuais

Nenhum no momento da criação. Consumidor imediato em P169 .B:
`rules/introspect/from_tags.rs` arm `ElementPayload::Metadata`.

## Consumers planeados

- `entities/introspector.rs::TagIntrospector.metadata` field — exposição via `query_metadata()`.
- M9+ features que possam referenciar `metadata` (ex: `metadata.where(predicate)` query rica em passos futuros).

---

## Sobre paridade

Vanilla armazena `MetadataElem` em `ElementIntrospector.elems` (Vec genérico de Content). Acesso via query. Cristalino isola num sub-store dedicado por simetria com `LabelRegistry`/`CounterRegistry` e composição visível.

---

## Resultado Esperado

- `01_core/src/entities/metadata_store.rs` — struct + 5 métodos + 4 tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P169 sub-passo .B: sub-store de values metadata para Introspector M9 | `metadata_store.rs`, `metadata_store.md` |

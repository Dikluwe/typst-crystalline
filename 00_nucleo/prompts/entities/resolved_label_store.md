# Prompt L0 — `entities/resolved_label_store`
Hash do Código: ad382e48

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/resolved_label_store.rs`
**Criado em**: 2026-05-04 (P193B sub-passo .B — primeiro passo da
sequência §9 P189 consolidado para fechar M5 universalmente)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`ResolvedLabelStore` é o sub-store de `TagIntrospector` que mapeia
`Label` → texto resolvido (e.g. `"Secção 1.2"`) para suportar
cross-references em `Content::Ref`.

P193A fixou (cláusulas 1, 2, 4, 5):
- **Cláusula 1**: shape interno `labels: HashMap<Label, String>`.
  Replica literalmente `CounterStateLegacy.resolved_labels`.
- **Cláusula 2**: field directo `pub resolved_labels: ResolvedLabelStore`
  em `TagIntrospector`, paralelo a `bib_store: BibStore`.
- **Cláusula 4**: API trait `resolved_label_for(&self, label: &Label) -> Option<&str>`.
- **Cláusula 5**: **sem variante location-aware** — análise dos 2 eixos
  (P193A §1.8) confirmou snapshot final (consumer C4 lê após walk
  completo).

Vanilla equivalente: `Locator::resolve() -> Resolved` (struct rico
com path, key, content). Cristalino simplifica para
`HashMap<Label, String>` por design pré-existente — `BibStore`
(P181B) é precedente arquitectural de "sub-store dedicado com mapa
simples" que `ResolvedLabelStore` replica.

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O, sem estado global.
- Read-only após construção. Mutação só via `pub(crate) fn` durante
  construção em `from_tags` (arm pendente em P195).
- Derives: `Debug`, `Clone`, `Default`. Sem `PartialEq`/`Eq` (sem
  consumer identificado em P193B).
- Tipos internos: `std::collections::HashMap<Label, String>`
  (consistente com `BibStore` P181B + `LabelRegistry`/`StateRegistry`;
  hot-path é `insert` em construção, não lookup hot-loop).

---

## Interface pública

```rust
use std::collections::HashMap;

use crate::entities::label::Label;

#[derive(Debug, Clone, Default)]
pub struct ResolvedLabelStore {
    labels: HashMap<Label, String>,
}

impl ResolvedLabelStore {
    pub fn empty() -> Self;
    pub fn get(&self, label: &Label) -> Option<&str>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub(crate) fn insert(&mut self, label: Label, resolved: String);
}
```

---

## Semântica dos métodos

- `empty()`: store vazio. Equivalente a `Default::default()`.
- `get(label)`: lookup via `HashMap::get`; retorna `Option<&str>`
  (referência interna, sem clone). Replica
  `state.resolved_labels.get(label).map(|s| s.as_str())` actual em
  `layout/references.rs:53` (P194 migrará caller).
- `len()`: número de labels registadas.
- `is_empty()`: `labels.is_empty()`.
- `insert(label, resolved)` (pub(crate)): regista mapeamento.
  Sobrescreve valor anterior se label já registada (paridade com
  `HashMap::insert` legacy). Replica
  `state.resolved_labels.insert(label, text)` actual em walk arms
  Heading/Labelled (E2/E4 P189B excepções).

---

## Invariantes

- Após construção, store é read-only para callers externos.
- Sem deduplicação implícita em `insert`: `HashMap::insert`
  sobrescreve (paridade legacy).
- Lookup determinístico via `HashMap::get`.
- Sem ordem garantida (HashMap interno).

---

## Estado em P193B (janela compat)

**Sub-store fica vazio em produção** durante janela compat M5.
Walks legacy E2/E4 (`Heading` + `Labelled` arms) continuam a popular
`state.resolved_labels` directamente. Consumer C4
(`layout/references.rs:53`) lê de legacy.

Plano de transição (per P193A §2.6):
1. **P193B** (este passo) — sub-store + API trait. Vazio em produção.
2. **P194** — consumer C4 migra para
   `intr.resolved_label_for(label).or_else(|| state.resolved_labels.get(label))`
   (substitution-with-fallback).
3. **P195** — walk arm `Labelled` migra; emite Tag;
   `from_tags` arm popula sub-store. Walk arm `Heading` continua
   a popular legacy também (write paralelo).
4. **P196** — walk arm `Heading` migra; legacy mutation removida;
   sub-store é único populator.
5. **M6** — `state.resolved_labels` removido; fallback removido.

---

## Tests obrigatórios (sub-passo .G P193B)

- `ResolvedLabelStore::empty()` retorna `is_empty() == true`,
  `len() == 0`, `get(any) == None`.
- `insert + get` retorna `Some(text)` para label registada.
- Multiple labels isoladas (cada uma retorna o seu valor).
- `insert` sobrescreve valor anterior para mesma label.
- (em `introspector.rs::tests`): `resolved_label_for` no trait
  delega correctamente para `ResolvedLabelStore::get`.

---

## Consumers actuais

Nenhum em P193B. Sub-store existe vazio em
`TagIntrospector::empty()`.

## Consumers planeados

- **P194**: Consumer C4 (`layout/references.rs:53::layout_ref`)
  consulta via `Introspector::resolved_label_for(&label)` com
  fallback a `state.resolved_labels.get(&label)`.
- **P195**: `from_tags` arm `Labelled` (a criar quando walk migrar)
  popula via `intr.resolved_labels.insert(label, text)`.
- **P196**: `from_tags` arm `Heading` (auto-toc) popula
  similarmente.

---

## Sobre paridade

Vanilla armazena resolução de labels via `Locator::resolve() ->
Resolved` (struct com path completo, key, content reference).
Cristalino divergiu desde P165 (M3 Introspection) para `HashMap<Label, String>` simples em `CounterStateLegacy.resolved_labels`.
`ResolvedLabelStore` continua essa simplificação — **sub-store é
materialização da decisão pré-existente, não decisão nova**.

Diferenças cristalinas:
- Sem path information (apenas texto resolvido).
- Sem content reference (apenas string).
- Sem memoização cross-iteration via comemo.

Adequado para subset minimal. Quando ADR sobre Locator vanilla
for considerado (futuro), `ResolvedLabelStore` pode ser
reformulado mas o seu papel arquitectural (sub-store de
`TagIntrospector` populado em `from_tags`) permanece.

---

## Resultado Esperado

- `01_core/src/entities/resolved_label_store.rs` — struct + 5
  métodos + 4 tests.
- `pub mod resolved_label_store;` em `01_core/src/entities/mod.rs`
  (sem `pub use`; convenção cristalina).
- Field `pub resolved_labels: ResolvedLabelStore` em
  `TagIntrospector`.
- Método `resolved_label_for` no trait `Introspector` + impl
  delegando para `self.resolved_labels.get(label)`.
- `entities/introspector.md` actualizado com field + método
  novos.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-05-04 | P193B sub-passo .B: sub-store para mapeamento `Label → String` resolvido; replica padrão BibStore P181B; passo 1 da sequência §9 P189 consolidado para fechar M5 universalmente | `resolved_label_store.rs`, `resolved_label_store.md`, `entities/mod.rs`, `introspector.rs`, `introspector.md` |

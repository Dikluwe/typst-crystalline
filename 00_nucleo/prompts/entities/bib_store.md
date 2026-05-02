# Prompt L0 — `entities/bib_store`
Hash do Código: 3ea366ac

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/bib_store.rs`
**Criado em**: 2026-05-01 (P181B sub-passo .B — primeiro passo de materialização do plano P181A para fechar lacuna #6)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`BibStore` é o sub-store de `TagIntrospector` que acumula entries
bibliográficas e respectiva numeração 1-based, populadas a partir de
`Content::Bibliography` durante o walk → `from_tags`.

P181A (decisões cláusula 1, 2, 3) fixou:

- **Cláusula 1**: shape interno `entries: Vec<BibEntry>` +
  `numbers: HashMap<String, u32>`. Replica literalmente
  `CounterStateLegacy.bib_entries`/`bib_numbers`. Sem `IndexMap`
  (ADR-0023 disponível mas não invocado — simetria com `MetadataStore`
  P169 e `StateRegistry` P171).
- **Cláusula 2**: `add_bibliography(entries)` faz `extend` (replica
  `state.bib_entries.extend(...)` actual em `introspect.rs:572`).
  Multi-Bibliography concatena por ordem de aparecimento.
- **Cláusula 3**: numbering preserva primeiro número via `or_insert`
  (replica `state.bib_numbers.entry(key).or_insert(next_num)` actual
  em `introspect.rs:570`).

Vanilla equivalente: `Bibliography` interno
(`Arc<ManuallyHash<IndexMap<Label, hayagriva::Entry, FxBuildHasher>>>`)
com memoização via comemo. Cristalino divergência deliberada
(ADR-0062 PROPOSTO `hayagriva` adopt mas não em vigor): subset
minimal sem hayagriva, com `Vec<BibEntry>` + `HashMap<String, u32>`
paralelo. Documentado em `inventario-bib-state.md` (P180) §3.

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O, sem estado global.
- Read-only após construção. Mutação só via `pub(crate) fn` durante
  construção em `from_tags`.
- Derives: `Debug`, `Clone`, `Default`. Sem `PartialEq` / `Eq` (sem
  consumer identificado em P181B; pode ser adicionado se um sub-passo
  posterior precisar).
- Tipos internos: `std::collections::HashMap` (não `FxHashMap` — o
  hot-path é `add_bibliography` em construção, não lookup
  hot-loop; consistência com `LabelRegistry`/`StateRegistry` que
  também usam `HashMap`).

---

## Interface pública

```rust
use std::collections::HashMap;

use crate::entities::bib_entry::BibEntry;

#[derive(Debug, Clone, Default)]
pub struct BibStore {
    entries: Vec<BibEntry>,
    numbers: HashMap<String, u32>,
}

impl BibStore {
    pub fn empty() -> Self;
    pub fn entries(&self) -> &[BibEntry];
    pub fn entry_for_key(&self, key: &str) -> Option<&BibEntry>;
    pub fn number_for_key(&self, key: &str) -> Option<u32>;
    pub fn len(&self) -> usize;
    pub fn numbers_len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub(crate) fn add_bibliography(&mut self, entries: Vec<BibEntry>);
    pub(crate) fn assign_number(&mut self, key: String, number: u32);
}
```

---

## Semântica dos métodos

- `empty()`: store vazio. Equivalente a `Default::default()`.
- `entries()`: slice das entries em ordem de inserção (= ordem de
  aparecimento no walk; multi-Bib concatena).
- `entry_for_key(key)`: linear scan sobre `entries` por `BibEntry.key`.
  `Some(&entry)` se match exacto; `None` se ausente. Replica
  `state.bib_entries.iter().find(|e| e.key == *key)` actual em
  `layout/mod.rs:584`.
- `number_for_key(key)`: lookup O(1) via HashMap. `Some(n)` se key
  registada; `None` caso contrário. Replica
  `state.bib_numbers.get(key).copied()` actual em
  `layout/mod.rs:590`.
- `len()`: número de entries. Equivalente a `entries().len()`.
- `numbers_len()`: número de keys com número atribuído. Cresce com
  `or_insert`; duplicates em multi-Bibliography contam **uma** vez.
  Usado por `from_tags` (P181E) para calcular `next_num` paralelo a
  `state.bib_numbers.len() as u32 + 1` em walk arm
  `Content::Bibliography` (introspect.rs:569).
- `is_empty()`: `entries.is_empty() && numbers.is_empty()`.
- `add_bibliography(entries)` (pub(crate)): faz
  `self.entries.extend(entries)`. **Não** atribui numbers — caller
  (em `from_tags` arm Bibliography, P181E) chama `assign_number` em
  separado para cada entry. Esta separação espelha o walk arm actual
  que faz iteração de números antes do `extend`.
- `assign_number(key, number)` (pub(crate)): usa
  `self.numbers.entry(key).or_insert(number)`. Primeiro número de
  uma key persiste em multi-Bibliography com keys duplicadas.

---

## Invariantes

- Após construção, store é read-only para callers externos.
- Ordem de `entries` é preservada (Vec interno).
- Sem deduplicação em `add_bibliography`: duas Bib com mesmas keys
  produzem duas entradas.
- `numbers.len() <= entries.len()` (apenas keys vistas em
  `assign_number` aparecem em `numbers`; keys duplicadas contam como
  1 em `numbers`).
- Numeração é responsabilidade do caller (passar `next_num =
  numbers.len() + 1` antes de `assign_number`). `BibStore` não
  inventa números.

---

## Tests obrigatórios (sub-passo .C P181B)

- `BibStore::empty()` retorna `entries().is_empty()` + `len() == 0`
  + `is_empty() == true` + `number_for_key("any") == None` +
  `entry_for_key("any") == None`.
- `add_bibliography` com 2 entries produz `len() == 2`; segundo call
  com mais 2 entries produz `len() == 4` (cláusula 2 — `extend`).
- `assign_number("a", 1)` + `assign_number("a", 2)` →
  `number_for_key("a") == Some(1)` (cláusula 3 — `or_insert`).
- `entry_for_key("inexistente")` → `None`.
- `number_for_key("inexistente")` → `None`.
- `entry_for_key("intro")` populado retorna referência ao entry com
  `key == "intro"`.
- `entries()` preserva ordem de inserção em multi-Bib.

---

## Consumers actuais

Nenhum em P181B. Sub-store existe vazio em `TagIntrospector::empty()`.

## Consumers planeados (P181C–P181J)

- **P181C**: `ElementKind::Bibliography` + `ElementPayload::Bibliography
  { entries: Vec<BibEntry> }`.
- **P181D**: `is_locatable(Content::Bibliography) == true`;
  `extract_payload` arm Bibliography.
- **P181E**: `from_tags` arm Bibliography popula
  `bib_store.add_bibliography(entries)` + `assign_number(key, n)`
  em loop sobre entries.
- **P181F**: `Introspector::bib_entry_for_key` +
  `bib_number_for_key` no trait + impl em `TagIntrospector`
  delegando para `self.bib_store.entry_for_key` /
  `self.bib_store.number_for_key`.
- **P181G**: Layouter cite-arm consulta via Introspector.
- **P181H**: walk arm Bibliography puro (mutação directa removida).
- **P181I**: tests E2E + lacuna #6 fechada.
- **P181J**: relatório consolidado.

---

## Sobre paridade

Vanilla armazena entries num `IndexMap<Label, hayagriva::Entry>` com
hashing manual e memoização via comemo. Cristalino divergiu (ADR-0062
PROPOSTO mas não em vigor) para subset minimal sem hayagriva.
`BibStore` reflecte essa divergência:

- Sem CSL formatting (Layouter cite-arm hardcoded nos 4 forms
  `CitationForm::{Normal, Prose, Author, Year}`).
- Lookup O(n) em `entry_for_key` (linear scan) — escala adequada
  para subset minimal cristalino; vanilla usa O(1) via IndexMap.
- Numeração armazenada em `HashMap<String, u32>` separado em vez de
  derivada da posição em IndexMap.

Quando ADR-0062 transitar para `IMPLEMENTADO`, `BibStore` pode ser
reformulado sobre `IndexMap<EcoString, hayagriva::Entry>` mas o seu
papel arquitectural (sub-store de `TagIntrospector` populado em
`from_tags`) permanece.

---

## Resultado Esperado

- `01_core/src/entities/bib_store.rs` — struct + 9 métodos + 7 tests
  (P181B: 8 métodos; P181E: +`numbers_len`).
- `pub mod bib_store;` em `01_core/src/entities/mod.rs` (sem
  `pub use`; convenção cristalina).
- Field `pub bib_store: BibStore` em `TagIntrospector` (composição
  visível; sem getter dedicado — convenção L0
  `entities/introspector.md` explicita).
- `entities/introspector.md` actualizado com field novo.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-05-01 | P181B sub-passo .B: sub-store para entries bibliográficas + numeração 1-based; replica decisões P181A cláusula 1/2/3 | `bib_store.rs`, `bib_store.md` |
| 2026-05-01 | P181E sub-passo .E: método `numbers_len()` adicionado para suportar `from_tags` arm Bibliography (paralelo a `state.bib_numbers.len()` em walk arm) | `bib_store.rs`, `bib_store.md` |

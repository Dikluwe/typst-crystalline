# Prompt L0 — `entities/selector`
Hash do Código: db886542

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/selector.rs`
**Criado em**: 2026-04-29 (P175 sub-passo .B — primeiro consumer fixpoint)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`Selector` é o predicado para `Introspector::query`. Vanilla `foundations/selector.rs` tem variants `Elem`, `Label`, `Where`, `And`, `Or`, `Before`, `After`, `Regex`, `Can`, `Predicate`. Cristalino P175 implementa **apenas `Kind(ElementKind)`**.

Variants futuros (vanilla):
- `Label(Label)` — elemento por label específica.
- `And(...)` / `Or(...)` — combinadores.
- `Where(...)` — predicado sobre fields.
- etc.

P175 deliberadamente minimal: cobre `query(heading)` / `query(figure)` / `query(outline)` cases. Adopção em features que se segmentem (M9+ refino).

---

## Restrições Estruturais

- Camada **L1**: enum puro.
- `Clone`, `PartialEq`, `Eq`, `Hash` derivados (todos triviais via `ElementKind`).
- Imutável.

---

## Interface pública

```rust
use ecow::EcoVec;

use crate::entities::element_kind::ElementKind;
use crate::entities::label::Label;
use crate::entities::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    /// Selector de kind — matches todos os elementos de um tipo.
    Kind(ElementKind),
    /// **P209B (M9c)** — Selector de label específica. Reusa `Label`
    /// L1 (`pub struct Label(pub String)`); Hash derive.
    Label(Label),
    /// **P209B (M9c)** — Selector de Location específica (singleton
    /// match). Reusa `Location` L1; Hash derive. Equivalente
    /// vanilla literal.
    Location(Location),
    /// **P209C (M9c)** — Composição N-ária: TODOS os sub-selectors
    /// devem matchar. Query semantics = intersecção de Vec<Location>.
    /// Vazio: empty Vec (Opção A fixada em P209C C3).
    /// `EcoVec` per paridade vanilla (clone O(1) com Arc interno).
    And(EcoVec<Selector>),
    /// **P209C (M9c)** — Composição N-ária: AO MENOS UM sub-selector
    /// deve matchar. Query semantics = união dedupliquada de
    /// Vec<Location> (ordem de primeira-aparição preservada).
    /// Vazio: empty Vec.
    Or(EcoVec<Selector>),
}
```

---

## Semântica

- `Selector::Kind(k1) == Selector::Kind(k2)` sse `k1 == k2`.
- `Selector::Label(l1) == Selector::Label(l2)` sse `l1 == l2`
  (igualdade structural sobre `Label.0: String`).
- `Selector::Location(loc1) == Selector::Location(loc2)` sse
  `loc1 == loc2`.
- **P209C** `Selector::And(v1) == Selector::And(v2)` sse `v1 == v2`
  (igualdade ordem-sensível dos sub-selectors via `EcoVec` Eq).
- **P209C** `Selector::Or(v1) == Selector::Or(v2)` idem.
- Hash determinístico (delega para Hash dos field types;
  recursivo para `And`/`Or` via discriminant + EcoVec elementos).
- Variants distintos são sempre `!=` independentemente do
  conteúdo (e.g., `Kind(Heading)` ≠ `Label(Label("Heading"))`;
  `And(v)` ≠ `Or(v)` mesmo com mesmo conteúdo).

### Query semantics (consumidas em `Introspector::query`)

- **`And(EcoVec<Selector>)`**: intersecção de `Vec<Location>`
  retornados pelos sub-selectors.
  - **Vazio** (`And(EcoVec::new())`): retorna `vec![]` (Opção A
    fixada em P209C C3). Justificação: cristalino single-pass
    não tem "universo computável" sem walk completo;
    consistência com `Or` vazio.
  - **N elementos**: `sels[0]` ∩ `sels[1]` ∩ ... ∩ `sels[N-1]`,
    preservando ordem do primeiro `Vec`.
- **`Or(EcoVec<Selector>)`**: união dedupliquada de
  `Vec<Location>`.
  - **Vazio** (`Or(EcoVec::new())`): retorna `vec![]`.
  - **N elementos**: união preservando ordem de primeira-aparição
    (via `HashSet::insert` check).

---

## Tests obrigatórios

- `Selector::Kind(Heading) == Selector::Kind(Heading)`.
- `Selector::Kind(Heading) != Selector::Kind(Figure)`.
- Hash determinístico (mesma key duas vezes).
- **P209B**: `Selector::Label(Label("foo".into())) == Selector::Label(Label("foo".into()))`.
- **P209B**: `Selector::Location(loc1) == Selector::Location(loc1)`;
  `!= Selector::Location(loc2)`.
- **P209B**: variants distintos são desiguais mesmo com
  conteúdo "equivalente".
- **P209C**: `And(EcoVec::new())` query devolve `vec![]` (Opção A).
- **P209C**: `Or(EcoVec::new())` query devolve `vec![]`.
- **P209C**: `And([Kind(k), Label(l)])` query devolve intersecção
  ordenada de `query_by_kind(k)` e `query_by_label(l)`.
- **P209C**: `Or([Kind(k1), Kind(k2)])` query devolve união
  dedupliquada na ordem de primeira-aparição.
- **P209C**: nested — `And([Or([...]), Kind(...)])` recursivo
  funciona (query reuso self-referencial).

---

## Consumers

- `Introspector::query(&self, &Selector) -> Vec<Location>` (P175 sub-passo .C).
- Stdlib `query(kind_str)` (P175 sub-passo .D) — constrói `Selector::Kind` internamente.

---

## Sobre paridade

Vanilla tem 10+ variants em `Selector`. Cristalino P175 implementa só `Kind` — minimal viable. Refino futuro adiciona `Label`, combinadores, `Where`, etc. quando consumers reais necessitarem.

---

## Resultado Esperado

- `01_core/src/entities/selector.rs` — enum + 3+ tests
  (P175 base + P209B variants Label/Location).
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-29 | P175 sub-passo .B: tipo Selector minimal para query | `selector.rs`, `selector.md` |
| 2026-05-12 | P209B (M9c — Bloco VI Selector extensions per Q3=α humano): +variants `Label(Label)` + `Location(Location)`. Hash derive preserved (campos Hash trivialmente). Query arms triviais: `Label` delega a `query_by_label`; `Location` retorna singleton `vec![loc]`. Stdlib `native_query`/`native_locate` ganham type dispatch (`Value::Str("<name>")` → Label; `Value::Location(loc)` → Location). | `selector.rs`, `selector.md`, `introspector.rs`, `foundations.rs` |
| 2026-05-12 | P209C (M9c — Bloco VI Selector extensions per C4 P207A): +variants compósitos `And(EcoVec<Selector>)` + `Or(EcoVec<Selector>)`. Hash derive recursivo via discriminant + EcoVec elementos. Query arms: `And` faz intersecção via filter+contains; `Or` faz união dedupliquada via HashSet check preservando ordem. **Opção A** fixada para `And/Or` vazios: ambos retornam `vec![]` (consistência + cristalino single-pass sem universo computável). Stdlib API: **Opção (c) Rust API only** — sem dispatch via `Value` em `native_query`/`native_locate`. | `selector.rs`, `selector.md`, `introspector.rs` |

# Prompt L0 — `entities/position`
Hash do Código: 5f2dbfa9

**Camada**: L1.
**Fase**: M8 / P204D.
**ADRs vinculantes**: ADR-0033 (paridade observable), ADR-0073
(comemo Introspector — paridade vanilla literal).
**Cross-references**: ADR-0066 (Introspection runtime adiada —
ACEITE com nota "intermediário até M8"; superseded em P204H).

---

## Contexto

ADR-0066 (ACEITE em P192B com nota "intermediário até M8")
adia Position runtime concreta para M8. P203 consolidado §13
confirmou Position como concern ortogonal coberto por M8.
P204A diagnóstico (auditoria A2 + A16) mapeou vanilla
`PagedPosition { page: NonZeroUsize, point: Point }` como
forma canónica.

P204D materializa Position concreta no cristalino.

---

## Decisão

Tipo `Position` em L1 (`01_core/src/entities/position.rs`)
réplica vanilla `PagedPosition`:

```text
pub struct Position {
    pub page: NonZeroUsize,  // 1-based
    pub point: Point,         // coordenada 2D na página
}
```

Bounds derivados:
- `Debug`, `Clone`, `Copy`, `PartialEq`.
- **Hash manual** (não derive — `Pt(f64)` bloqueia derive).
  Hash via `to_bits()` de cada `f64` (determinístico;
  evita NaN issues em layout coordinates).

---

## Pipeline

### Vanilla

`PagedIntrospector::new(pages: &[Page])` constrói post-layout
sobre páginas finalizadas (per P203A A7).

### Cristalino — divergência intencional

Layouter popula `runtime.positions: HashMap<Location, Position>`
durante layout single-pass (per P203A C5; P204D C5):

- `LayouterRuntimeState` ganha 4º campo `positions`.
- `Layouter::advance_locator_if_locatable` (mirror do gating
  que set `current_location`) emite Position no momento da
  detecção de locatable.
- Idempotência: `insert` substitui em re-layout (TOC fixpoint).

Saída observable equivalente — mapping
`Location → Position` idêntico para um documento.

---

## Trait API

`Introspector::position_of` migra de stub `Option<()>` para
`Option<Position>` per ADR-0073 (paridade vanilla literal —
`fn position(&self, loc: Location) -> Option<DocumentPosition>`).

`TagIntrospector` impl retorna sempre `None` — Position vive
em `Layouter.runtime.positions` (single-pass populated).
Consumers que precisem de Position acedem via Layouter
directamente.

Future trait impl que tenha acesso a Layouter runtime
(ex: PagedIntrospector pós-layout, análogo vanilla) pode
override e retornar `Some(Position)`.

Per ADR-0073 §C6a (P204D diagnóstico).

---

## Restrições absolutas

- L1 (sem I/O, sem global mutável).
- Sem dependências externas novas (apenas `std::num::NonZeroUsize`).
- Hash determinístico (via `to_bits()` em f64).
- `Position: Send + Sync` (auto-trait — todos os fields são).

---

## Plano de validação

`Position` é considerado materializado quando:

1. Tipo existe em `01_core/src/entities/position.rs` com forma
   `{ page: NonZeroUsize, point: Point }`.
2. `LayouterRuntimeState.positions` campo existe e é
   `HashMap<Location, Position>`.
3. `Layouter::advance_locator_if_locatable` popula
   `runtime.positions` no mesmo gating que set
   `current_location`.
4. `Introspector::position_of` retorna `Option<Position>`.
5. Tests E2E confirmam:
   - Locatable → entry em `runtime.positions`.
   - Non-locatable → sem entry.
6. Tests sentinela confirmam tipo + field existem.
7. Hash impl (manual via `to_bits()`) testado por igualdade
   estrutural.

---

## Cross-references

- P204A diagnóstico C8 (Position concrete sub-passo M8).
- P204D spec (este materializa).
- P203 consolidado §13 (concern ortogonal pós-M8 lacunas).
- ADR-0066 (superseded em P204H).
- ADR-0073 (paridade vanilla literal).
- Vanilla: `lab/typst-original/crates/typst-library/src/introspection/position.rs`.

# Prompt L0 — `entities/selector`
Hash do Código: 3490d19c

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
use crate::entities::element_kind::ElementKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    /// Selector de kind — matches todos os elementos de um tipo.
    Kind(ElementKind),
}
```

---

## Semântica

- `Selector::Kind(k1) == Selector::Kind(k2)` sse `k1 == k2`.
- Hash determinístico (delega para `ElementKind::Hash`).

---

## Tests obrigatórios

- `Selector::Kind(Heading) == Selector::Kind(Heading)`.
- `Selector::Kind(Heading) != Selector::Kind(Figure)`.
- Hash determinístico (mesma key duas vezes).

---

## Consumers

- `Introspector::query(&self, &Selector) -> Vec<Location>` (P175 sub-passo .C).
- Stdlib `query(kind_str)` (P175 sub-passo .D) — constrói `Selector::Kind` internamente.

---

## Sobre paridade

Vanilla tem 10+ variants em `Selector`. Cristalino P175 implementa só `Kind` — minimal viable. Refino futuro adiciona `Label`, combinadores, `Where`, etc. quando consumers reais necessitarem.

---

## Resultado Esperado

- `01_core/src/entities/selector.rs` — enum + 3 tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-29 | P175 sub-passo .B: tipo Selector minimal para query | `selector.rs`, `selector.md` |

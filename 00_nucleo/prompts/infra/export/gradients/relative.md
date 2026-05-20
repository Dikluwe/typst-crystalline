# Prompt L0 — `infra/export/gradients/relative` — RelativeTo resolution
Hash do Código: 14c42db6

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/gradients/relative.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0094 (gradient relative cross-variant)

---

## Contexto

Suporte a `gradient.relative: RelativeTo` (P273):
- `Self`: bbox da própria shape.
- `Parent`: bbox do contentor imediato (Block, Group).

Helpers:
- `resolve_relative` — escolhe entre `self_bbox` e `parent_bbox` baseado em `RelativeTo`.
- `apply_parent_transform` — aplica matriz transform local→parent quando relative=Parent dentro de Transform Group.

## Restrições estruturais

- L3 puro. Lê `RelativeTo` de `typst_core::entities::gradient`.
- `pub(crate)` — chamado por `super::super::builder::emit_gradient_objects` e `super::super::stream::draw_item_local`.
- Sem I/O.

## Interface

```rust
pub(crate) fn resolve_relative(
    relative: RelativeTo,
    self_bbox: Rect,
    parent_bbox: Option<Rect>,
) -> Rect;
pub(crate) fn apply_parent_transform(
    local: Rect,
    effective_parent_bbox: Option<Rect>,
) -> Rect;
```

## Invariantes

- `relative=Self` → ignora parent_bbox.
- `relative=Parent` + `parent_bbox=None` → fallback para `self_bbox` (defensive).
- `apply_parent_transform` é identity quando `effective_parent_bbox=None`.

## Critérios de verificação

Tests `p273_*` (56 testes) em `super::super::tests`.

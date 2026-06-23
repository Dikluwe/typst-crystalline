# Prompt L0 — `rules/layout/link` — Layout de `Content::Link`
Hash do Código: 06da98c4

**Camada**: L1 · **Alvo**: `01_core/src/rules/layout/link.rs`
**Origem**: P422 (S) — atomização do layout de hiperligações.

---

## Decisão arquitetural (ADR-0109 forma B)

- `entities/elements/link.rs` — `LinkElem` struct puro (`url`, `body`).
- `rules/layout/link.rs` — free function `layout_link(layouter, &LinkElem)` que
  renderiza o body e envolve os itens resultantes em `FrameItem::Link`.
- `entities/layout_types.rs` — `FrameItem::Link { url: EcoString, items: Vec<FrameItem> }`.

## Algoritmo

1. Renderizar `body` normalmente através do `Layouter`.
2. Coletar os `FrameItem`s produzidos.
3. Emitir um único `FrameItem::Link { url, items }` no frame atual.

## Scope-out

- Cor azul / sublinhado — aguarda `FrameItem::Decoration`.
- Consumer PDF de annotation URI — aguarda passo dedicado do exportador.
- Links internos (`<label>`) e inter-página.

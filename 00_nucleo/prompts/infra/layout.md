# Prompt L0 — layout (infra)
Hash do Código: d7067fe4

## Módulo
`03_infra/src/layout.rs`

## Propósito
Ponte entre `FontBookMetrics` (L3) e `Layouter<M>` (L1 genérico).
Expõe `layout_with_font()` para uso por L4 ou exportadores.

## Interface
```rust
pub fn layout_with_font(
    content:   &Content,
    font_data: &[u8],
    font_size: f64,
) -> PagedDocument;
```

## Comportamento
- Se `font_data` é válido → usa `FontBookMetrics` (métricas reais)
- Se inválido → fallback para `typst_core::engine::layout::layout()`

## Critérios de verificação
- Bytes inválidos → não panic, retorna documento via fallback
- Bytes válidos (com fixture) → documento não vazio para texto não vazio

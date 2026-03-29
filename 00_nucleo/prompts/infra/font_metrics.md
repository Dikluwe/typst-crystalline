# Prompt L0 — font_metrics

## Módulo
`03_infra/src/font_metrics.rs`

## Propósito
Implementação de `FontMetrics` com métricas de fonte reais via `ttf-parser`.
`ttf-parser` confinado a L3 — não aparece em `01_core/Cargo.toml`.

## Interface
```rust
pub struct FontBookMetrics<'a> { face: Face<'a>, upem: f64 }

impl FontBookMetrics<'_> {
    pub fn from_bytes(data: &[u8]) -> Option<Self>;
}

impl FontMetrics for FontBookMetrics<'_> {
    fn advance(&self, text: &str, size: Pt) -> Pt;
    fn vertical_metrics(&self, size: Pt) -> (Pt, Pt);
}
```

## Invariantes
- `font_size` não armazenado — passado por chamada
- `upem == 0` → fallback `1000.0` em `from_bytes` (não panic)
- `descender()` pode ser negativo — usar `.abs()`
- Fallback advance para glifos ausentes: `upem * 0.6`

## Fórmulas
- `advance = size * (Σ glyph_units / upem)`
- `ascender_pt = size * (ascender / upem)`
- `line_height_pt = size * ((ascender + |descender| + line_gap) / upem)`

## Critérios de verificação
- `from_bytes(b"not a font")` → `None`
- `from_bytes(valid_ttf)` → `Some(...)`
- advance escala linearmente com size
- `vertical_metrics`: ascender < line_height, ambos positivos
- Com fonte proporcional: `advance("iiii") < advance("WWWW")`

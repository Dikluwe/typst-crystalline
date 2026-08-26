# Prompt L0 — `entities/elements/curve` — `CurveElem`
Hash do Código: ffffffff

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/curve.rs`
**Origem**: Passo 513 (curve elements: move/line/cubic/quad/close). **Não-locatável**
(definição em `entities/elements/_comum.md` §A.0.1).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CurveElem {
    pub segments: Vec<CurveSegment>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CurveSegment {
    Move(CurvePoint),
    Line(CurvePoint),
    Cubic(CurvePoint, CurvePoint, CurvePoint), // control1, control2, end
    Quad(CurvePoint, CurvePoint),              // control, end
    Close,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CurvePoint {
    pub x: Length,
    pub y: Length,
}
```

`Content::Curve(Arc<CurveElem>)`.
Construtores ergonómicos:
- `Content::curve_move(x, y)`
- `Content::curve_line(x, y)`
- `Content::curve_cubic(c1x, c1y, c2x, c2y, ex, ey)`
- `Content::curve_quad(cx, cy, ex, ey)`
- `Content::curve_close()`

## `impl Element for CurveElem`

| método | comportamento |
|---|---|
| `plain_text` | string vazia |
| `is_empty` | `false` |
| `map_content` | devolve `Content::Curve(Arc::new(self.clone()))` (terminal) |
| `map_text` | devolve `Content::Curve(Arc::new(self.clone()))` (terminal) |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `Hash`

Implementar manualmente via `format!("{:?}", self)` para contornar `f64` em `Length`.

## Critério

`plain_text` vazio; `map_*` terminal; segmentos preservam coordenadas.

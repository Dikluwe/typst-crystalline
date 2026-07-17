# Prompt L0 — `stdlib/curve` — namespace `curve`

**Camada**: L1 · **Alvo**: `01_core/src/engine/stdlib/shapes.rs` + `01_core/src/engine/eval/mod.rs`
**Origem**: Passo 513.

---

## Assinaturas

```typst
curve.move(point) -> content
curve.line(point) -> content
curve.cubic(control1, control2, end) -> content
curve.quad(control, end) -> content
curve.close() -> content
```

`point` é um array de 2 elementos (`(x, y)`) ou `Value::Array` de `Length`/`Float`/`Int`. Cada função devolve `Content::Curve` com um único segmento.

## Semântica

- `curve.move(p)`: move a caneta para `p`.
- `curve.line(p)`: linha recta até `p`.
- `curve.cubic(c1, c2, e)`: curva cúbica de Bézier com control points `c1`, `c2` e ponto final `e`.
- `curve.quad(c, e)`: curva quadrática de Bézier com control point `c` e ponto final `e`.
- `curve.close()`: fecha o path actual.

## Implementação

- `native_curve_move`, `native_curve_line`, `native_curve_cubic`, `native_curve_quad`, `native_curve_close` em `shapes.rs`.
- Helper `extract_point(val: &Value, fn_name, arg_name) -> SourceResult<CurvePoint>` aceita `Value::Array` com 2 elementos (`Length`, `Float` ou `Int` convertido para pt).
- Cada constructor devolve `Value::Content(Content::curve_*(...))`.

## Consumo

A função `curve(...)` existente (P293/P294) deve aceitar `Value::Content(Content::Curve(e))` como argumentos posicionais, concatenando `e.segments` ao path final. Preserva-se a sintaxe legada de tuplos (`curve(("move", (0,0)))`).

## Registo

O namespace `curve` é anexado à função `curve` existente em `eval/mod.rs`:
- `curve.move`
- `curve.line`
- `curve.cubic`
- `curve.quad`
- `curve.close`

## Casos de Aceitação

- `curve.move((0,0))` → `Content::Curve` com `Move`.
- `curve.line((100pt, 0pt))` → `Line`.
- `curve.cubic((0,0), (50,50), (100,0))` → `Cubic`.
- `curve.quad((0,0), (50,50))` → `Quad`.
- `curve.close()` → `Close`.
- `curve(curve.move((0,0)), curve.line((100,0)), curve.close())` produz path fechado.

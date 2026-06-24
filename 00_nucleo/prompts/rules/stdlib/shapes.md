# Prompt L0 — `stdlib/shapes` — módulo `shapes`
Hash do Código: ef3d2aa0

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/shapes.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com marcos
P76 (rect/ellipse/circle/line/polygon), P277 (`path_bbox`), P293-P294 (`curve`
c/ conversão quadratic→cubic), P396 (`parse_paint` com Tiling/Gradient).
**ADRs**: ADR-0033 (divergência intencional), ADR-0037 (coesão por domínio),
ADR-0054 (perfil graded).
**Convenções partilhadas**: ver `00_nucleo/prompts/rules/stdlib/_comum.md`.
**Prompt auxiliar**: `square.md` cobre o helper morfológico `square` sobre
`Rect`.

---

## Módulo `shapes` — funções nativas geométricas

Este módulo implementa as primitivas geométricas vetoriais de Typst expostas
no escopo global: rectângulos, elipses/círculos, linhas, polígonos e caminhos
de Bézier (`curve`). O helper `square` permanece no prompt dedicado
`square.md`.

Todas as funções partilham a assinatura padrão de `native_*`:

```rust
fn native_X(
    ctx: &mut EvalContext,
    args: &Args,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value>
```

Helpers partilhados no módulo:
- `parse_color` — converte `Value::Str` (nomes conhecidos) ou `Value::Color` em `Color`.
- `parse_paint` — converte `Color`, `Tiling`, `Gradient` ou string nomeada em `Paint`.

---

### `native_rect(width?, height?, fill?, stroke?)`

**Assinatura**: `rect(width: Length?, height: Length?, fill: Paint?, stroke: Color?) -> Content`

**Argumentos**:
- `width`, `height`: `Length` (ou `Float`/`Int` coagidos para pt) opcionais.
- `fill`: `Color`, `Tiling`, `Gradient` ou nome de cor (`Str`).
- `stroke`: `Color` ou nome de cor (`Str`). `stroke` em shapes usa apenas cor
  sólida com espessura 1pt (divergência vs vanilla `Stroke` rico).
- Sem argumentos posicionais.

**Semântica**: Cria `Content::Shape { kind: Rect, width, height, fill, stroke }`.
Fallback determinístico: se nem `fill` nem `stroke` forem fornecidos, aplica
stroke preta de 1pt.

**Paridade vanilla**: Equivalente a `#rect(width: 5cm, height: 3cm, fill: red)`.

**Limitações / scope-outs**:
- `stroke` só aceita cor sólida; espessura/overhang/dash scope-out.
- Cores hex (`#rrggbb`) ainda não parseadas via `parse_color`.

**Testes canónicos**:
```
rect(width: 5cm, height: 3cm) -> Shape Rect com stroke preto 1pt
rect(fill: red) -> Shape Rect fill red, sem stroke
rect(stroke: blue) -> Shape Rect stroke azul
rect(foo: 1) -> Err "argumento nomeado inesperado"
```

---

### `native_ellipse(width?, height?, fill?, stroke?)`

**Assinatura**: `ellipse(width: Length?, height: Length?, fill: Paint?, stroke: Color?) -> Content`

**Argumentos**: Idênticos a `rect`.

**Semântica**: Cria `Content::Shape { kind: Ellipse, width, height, fill, stroke }`.
Fallback de stroke preto 1pt quando `fill` e `stroke` ausentes.

**Paridade vanilla**: Equivalente a `#ellipse(width: 4cm, height: 2cm)`.

**Limitações / scope-outs**: Idênticas a `rect`.

**Testes canónicos**:
```
ellipse(width: 4cm, height: 2cm, fill: green) -> Shape Ellipse
ellipse() -> Shape Ellipse com stroke preto 1pt
```

---

### `native_circle(radius?, fill?, stroke?)`

**Assinatura**: `circle(radius: Length?, fill: Paint?, stroke: Color?) -> Content`

**Argumentos**:
- `radius`: `Length` (ou `Float`/`Int`) opcional.
- `fill`, `stroke`: idênticos a `rect`.

**Semântica**: Converte `radius` em `diameter = radius * 2` e cria
`Content::Shape { kind: Ellipse, width: diameter, height: diameter, fill, stroke }`.
Fallback de stroke preto 1pt quando necessário.

**Paridade vanilla**: Equivalente a `#circle(radius: 2cm)`.

**Limitações / scope-outs**: Idênticas a `rect`.

**Testes canónicos**:
```
circle(radius: 2cm) -> Shape Ellipse width=height=4cm
circle(fill: blue) -> Shape Ellipse fill blue
```

---

### `native_line(dx?, dy?, stroke?)`

**Assinatura**: `line(dx: Length?, dy: Length?, stroke: Color?) -> Content`

**Argumentos**:
- `dx`, `dy`: deslocamento em pt (`Length`, `Float`, `Int`). Default `0.0`.
- `stroke`: cor da linha. Default preto.

**Semântica**: Cria `Content::Shape { kind: Line { dx, dy }, stroke: preto default }`.
Linhas não têm fill.

**Paridade vanilla**: Equivalente a `#line(dx: 3cm, dy: 2cm)`.

**Limitações / scope-outs**: Stroke sólido 1pt; espessura/overhang scope-out.

**Testes canónicos**:
```
line(dx: 3cm, dy: 2cm) -> Shape Line dx=3cm dy=2cm stroke preto
line(stroke: red) -> Shape Line dx=0 dy=0 stroke vermelho
line(dx: -1cm) -> Shape Line dx=-1cm
```

---

### `native_polygon(pt1, pt2, ...; fill?, stroke?)`

**Assinatura**: `polygon(..vertices: Array[Float, Float], fill: Paint?, stroke: Color?) -> Content`

**Argumentos**:
- `vertices`: variádicos posicionais, cada um array `[x, y]` em pontos
  tipográficos.
- `fill`, `stroke`: opcionais.

**Semântica**: Constrói um path `MoveTo -> LineTo ... -> ClosePath`. Calcula
bbox via `geometry::path_bbox` para `width`/`height`. Cria
`Content::Shape { kind: Path, ... }`.

**Paridade vanilla**: Equivalente a `#polygon((0,0), (1,0), (0.5,1))`.

**Limitações / scope-outs**: Apenas vértices via `Array`; outras formas de
especificar pontos scope-out.

**Testes canónicos**:
```
polygon((0,0), (1,0), (0.5,1)) -> Shape Path fechado
triangle = polygon((0,0), (2,0), (1,1), fill: blue)
polygon() -> Err "pelo menos um ponto"
polygon((0,0), "x") -> Err "coordenada inválida"
```

---

### `native_curve(seg1, seg2, ...; fill?, stroke?)`

**Assinatura**: `curve(..segments: Array, fill: Paint?, stroke: Color?) -> Content`

**Argumentos**:
- `segments`: variádicos posicionais, cada segmento é um array cujo primeiro
  elemento é uma string `kind`:
  - `("move", [x, y])` → `MoveTo`.
  - `("line", [x, y])` → `LineTo`.
  - `("cubic", [c1x, c1y], [c2x, c2y], [ex, ey])` → `CubicTo`.
  - `("quadratic", [cx, cy], [ex, ey])` → convertido para `CubicTo` via
    fórmula q→c paridade vanilla (`C1 = (P0 + 2C)/3`, `C2 = (P2 + 2C)/3`).
  - `("close",)` → `ClosePath`.
- `fill`, `stroke`: opcionais.

**Semântica**: Constrói um path arbitrário, calcula bbox via `path_bbox`,
cria `Content::Shape { kind: Path, ... }`.

**Paridade vanilla**: Interface por tuples descritivos em vez dos métodos
`curve.move`/`curve.cubic` do vanilla (proc-macros de scope ainda não
materializados em L1).

**Limitações / scope-outs**:
- API de constructores por scope (`curve.move`, etc.) scope-out.
- `stroke` sólido 1pt.

**Testes canónicos**:
```
curve(("move", (0,0)), ("line", (1,0)), ("line", (1,1)), ("close",)) -> Shape Path
curve(("quadratic", (0.5, 0.5), (1,0))) -> CubicTo equivalente
 curve("x") -> Err "array de segmento válido"
```

---

## Nota sobre `square`

O helper `square(width, fill?, stroke?)` é um wrapper morfológico sobre
`rect` (`height = width` quando omitido). O seu contrato L0 mantém-se no
prompt dedicado `00_nucleo/prompts/rules/stdlib/square.md`.

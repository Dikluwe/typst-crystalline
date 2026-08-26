# Prompt L0 — `stdlib/shapes` — módulo `shapes`
Hash do Código: f965dd91

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/shapes.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com marcos
P76 (rect/ellipse/circle/line/polygon), P277 (`path_bbox`), P293-P294 (`curve`
c/ conversão quadratic→cubic), P396 (`parse_paint` com Tiling/Gradient),
P727/P732 (fallback de stroke `Smart::Auto` em `curve`/`polygon`; vértices
`Length` em `polygon`), P734 (`polygon` restringido a `Length` — rejeita
Int/Float como o vanilla), P1221 (preservação do `Stroke` rico já avaliado
em rectângulos, quadrados, elipses, círculos e linhas).
**ADRs**: ADR-0033 (divergência intencional), ADR-0037 (coesão por domínio),
ADR-0054 (perfil graded).
**Convenções partilhadas**: ver `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## Módulo `shapes` — funções nativas geométricas

Este módulo implementa as primitivas geométricas vetoriais de Typst expostas
no escopo global: rectângulos, elipses/círculos, linhas, polígonos, caminhos
de Bézier (`curve`) e o helper morfológico `square` sobre `Rect`.

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
  P477: 18 cores suportadas (5 originais + 13 novas CSS basic + 2 aliases gray/grey, aqua/cyan).
  Nomes: red, green, blue, black, white, yellow, cyan, magenta, orange, purple, gray, grey,
  silver, maroon, navy, olive, teal, lime, aqua.
- `parse_paint` — converte `Color`, `Tiling`, `Gradient` ou string nomeada em `Paint`.
- `parse_shape_stroke` — converte os shorthands de stroke da linguagem
  (`Length`, `Color` e `Stroke`) sem perder paint nem espessura; nomes de cor
  legados continuam aceites.

---

### `native_rect(width?, height?, fill?, stroke?)`

**Assinatura**: `rect(width: Length?, height: Length?, fill: Paint?, stroke: Stroke?) -> Content`

**Argumentos**:
- `width`, `height`: `Length` (ou `Float`/`Int` coagidos para pt) opcionais.
- `fill`: `Color`, `Tiling`, `Gradient` ou nome de cor (`Str`).
- `stroke`: shorthand `Length`, `Color`, nome de cor (`Str`) ou `Stroke` já
  avaliado. Paint e espessura são preservados até o `Content::Shape`.
- Sem argumentos posicionais.

**Semântica**: Cria `Content::Shape { kind: Rect, width, height, fill, stroke }`.
Fallback determinístico: se nem `fill` nem `stroke` forem fornecidos, aplica
stroke preta de 1pt.

**Paridade vanilla**: Equivalente a `#rect(width: 5cm, height: 3cm, fill: red)`.

**Limitações / scope-outs**:
- Campos de stroke ainda não representados por `entities::geometry::Stroke`
  (como dash/cap/join) permanecem scope-out.
- O dicionário por-lado de `rect(stroke:)` continua aceite pela superfície
  por compatibilidade, mas permanece scope-out na entidade uniforme.
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

**Assinatura**: `ellipse(width: Length?, height: Length?, fill: Paint?, stroke: Stroke?) -> Content`

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

**Assinatura**: `circle(radius: Length?, fill: Paint?, stroke: Stroke?) -> Content`

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

### `native_line(dx?, dy?, stroke?, start?, end?, length?, angle?)`

**Assinatura**: `line(dx: Length?, dy: Length?, stroke: Stroke?, start: Array?, end: Array?, length: Length?, angle: Angle?) -> Content`

**Argumentos**:
- `dx`, `dy`: deslocamento em pt (`Length`, `Float`, `Int`). Default `0.0`.
- `stroke`: shorthand `Length`, `Color`, nome de cor (`Str`) ou `Stroke` já
  avaliado. Default preto 1pt; paint e espessura explícitos são preservados.
- **`end`** (P739B): ponto final como array de 2 coordenadas, ex. `(50pt, 50pt)`
  (paridade vanilla — medido: `line(start: (0pt, 0pt), end: (50pt, 50pt))`
  compila). Alternativa a `dx`/`dy` — combinar `end` com `dx`/`dy` é erro.
- **`start`** (P739B): ponto inicial; default `(0pt, 0pt)`. Só admissível com
  `end` ou com `length`/`angle`. **Scope-out medido**: `start` ≠ `(0,0)` —
  `ShapeKind::Line` não carrega posição absoluta (a linha é relativa à posição
  corrente); erro explícito de scope-out em vez de desenhar deslocada para a origem.
- **`length`** (P804): comprimento da linha (`Length`, `Float`, `Int` em pt).
  Default `30pt` (paridade vanilla `#[default(Abs::pt(30.0))]`). **Só é
  respeitado se `end` for `none`** — medido no vanilla 0.15.0:
  `#line(length: 3cm, end: (1cm, 1cm))` compila e `length` é ignorado.
  `Ratio` (`length: 100%`) — **scope-out**: o native não tem a região para
  resolver percentagens; erro explícito.
- **`angle`** (P804): direcção da linha (`Value::Angle`). Default `0deg`.
  Só respeitado se `end` for `none` (como `length`). Tipo errado →
  `expected angle, found {type}` (padrão do projecto).

**Semântica**: Cria `Content::Shape { kind: Line { dx, dy }, stroke: preto default }`.
Linhas não têm fill. Com `end`: `dx = end.x − start.x`, `dy = end.y − start.y`.
Sem `end`, com `length`/`angle` (P804, paridade vanilla
`typst-layout/src/shapes.rs::layout_line`): `dx = cos(angle) · length`,
`dy = sin(angle) · length`. Combinar `length`/`angle` com o legado `dx`/`dy`
é erro. Sem `end` e sem `length`/`angle`: legado `dx`/`dy` (default 0,0 —
linha degenerada; NOTA: o vanilla desenharia 30pt a 0deg — divergência
registada da interface legada, mantida).

**Paridade vanilla**: Equivalente a `#line(dx: 3cm, dy: 2cm)`,
`#line(start: (0pt, 0pt), end: (50pt, 50pt))`, `#line(length: 3cm)` ou
`#line(length: 3cm, angle: 30deg)` (P804).

**Limitações / scope-outs**: campos de stroke não modelados pela entidade
cristalina (dash/cap/join) continuam scope-out.
`length:` como `Ratio` (percentagem) — scope-out (P804, sem região no native).
`start` ≠ `(0,0)` — scope-out (acima).

**Testes canónicos**:
```
line(dx: 3cm, dy: 2cm) -> Shape Line dx=3cm dy=2cm stroke preto
line(stroke: red) -> Shape Line dx=0 dy=0 stroke vermelho
line(dx: -1cm) -> Shape Line dx=-1cm
line(end: (50pt, 50pt)) -> Shape Line dx=50 dy=50  (P739B)
line(start: (0pt, 0pt), end: (50pt, 50pt)) -> Shape Line dx=50 dy=50  (P739B)
line(start: (10pt, 0pt), end: (50pt, 50pt)) -> Err scope-out  (P739B)
line(length: 3cm) -> Shape Line dx=3cm dy=0  (P804)
line(length: 4cm, angle: 90deg) -> Shape Line dx≈0 dy=4cm  (P804)
line(length: 3cm, end: (1cm, 1cm)) -> Shape Line dx=1cm dy=1cm (length ignorado)  (P804)
line(length: 3cm, dx: 1cm) -> Err (não combinável)  (P804)
```

---

### `native_polygon(pt1, pt2, ...; fill?, stroke?)`

**Assinatura**: `polygon(..vertices: Array[Length, Length], fill: Paint?, stroke: Color?) -> Content`

**Argumentos**:
- `vertices`: variádicos posicionais, cada um array `[x, y]` de `Length`
  (paridade vanilla — **P734**: o vanilla exige `Rel<Length>` e **rejeita**
  Int/Float com "expected relative length, found integer/float", medido;
  P732 tinha alargado a aceitação a `Length` mantendo números, P734
  restringe a `Length` via `extract_vertex`, helper próprio de `polygon`).
- `fill`, `stroke`: opcionais. **Fallback determinístico** (paridade vanilla
  `Smart::Auto`, `lab/typst-original/crates/typst-layout/src/shapes.rs:336-339`,
  corrigido no Passo 732): se nem `fill` nem `stroke` forem fornecidos,
  aplica stroke preta de 1pt; se `fill` for fornecido sem `stroke`, o
  polígono fica sem stroke — idêntico ao fallback de `native_rect` e de
  `native_curve` (P727).

**Semântica**: Constrói um path `MoveTo -> LineTo ... -> ClosePath`. Calcula
bbox via `geometry::path_bbox` para `width`/`height`. Cria
`Content::Shape { kind: Path, ... }`.

**Paridade vanilla**: Equivalente a `#polygon((0pt, 0pt), (50pt, 0pt), (25pt, 40pt))`.

**Limitações / scope-outs**:
- Vértices com componente `em` ou `Ratio` (`50%`): scope-out — o vanilla
  aceita ratio (resolve no layout contra o contentor; medido em P734:
  triângulo com `(50%, 0pt)` renderiza 2923 px), mas o constructor
  cristalino corre em tempo de eval, sem dimensão de referência (mesmo
  precedente de P513 em `curve`). Ratio produz erro explícito de
  scope-out, não a mensagem vanilla.
- **P741 — scope-out reforçado, com custo medido.** A sonda confirmou a
  dimensão de referência do vanilla: o ratio resolve contra o
  **contentor** em tempo de layout — `box(width: 200pt, height: 100pt,
  polygon((0pt, 50%), ...))` ≡ `(0pt, 50pt)` (diff 0.0000% medido); fora
  do box, contra a região de texto. O caminho real do utilizador chega
  como `Value::Relative` (abs zero), não `Value::Ratio` — pré-P741 a
  mensagem era absurda ("expected relative length, found relative
  length"); agora o braço `Value::Relative` produz a mensagem de
  scope-out. Custo da paridade completa (medido, não estimado): (1)
  `ShapeKind::Path` carrega `Point` absoluto — exigiria ponto relativo
  novo em `geometry.rs` + resolução em `layout/shape.rs` + alteração
  dos 3 braços de `03_infra/src/export/stream.rs` + construtores
  (`polygon` e `curve`); (2) **a altura de contentor inline não existe**
  — o `Boxed` faz layout do body com `unconstrained_height: true`
  (`layout/boxed.rs`), pelo que `50%` em y não teria referência sem
  estender o mecanismo de sub-frame inline (partilhado por todo o
  conteúdo em box). Sem consumidor em `cetz` (usa `Length` absolutos
  via `transform-point`, P734) — custo desproporcional.
- A restrição a `Length` **não** se aplica a `curve`: a interface por
  tuples do cristalino (`("move", (0, 0))`) é divergência intencional
  documentada (o vanilla rejeita tuples — "expected content, found array",
  medido em P734 — e usa `curve.move`/etc.); `extract_coordinate` mantém
  a aceitação de números para esse caminho. O `cetz` usa o caminho de
  conteúdo (`curve.move`/`curve.line`) com `Length` (`transform-point`
  aplica `* length`, `canvas.typ:141-156`) — confirmado sem consumidor de
  números nus no builtin `polygon`/`curve`.

**Testes canónicos**:
```
polygon((0pt, 0pt), (50pt, 0pt), (25pt, 40pt)) -> Shape Path fechado, stroke preto 1pt (fallback)
polygon((0pt,0pt), (2pt,0pt), (1pt,1pt), fill: blue) -> Shape Path fill blue, sem stroke
polygon((0pt,0pt), (1pt,0pt), stroke: blue) -> Shape Path stroke azul (sem regressão)
polygon((0, 0), (50, 0), (25, 40)) -> Err "expected relative length, found integer" (P734)
polygon((0.0, 0.0), (50.0, 0.0), (25.0, 40.0)) -> Err "expected relative length, found float" (P734)
polygon() -> Err "pelo menos um ponto"
polygon((0pt,0pt), "x") -> Err "coordenada inválida"
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
  Desde o Passo 513, um segmento posicional pode também ser
  `Value::Content(Content::Curve(e))` (produzido pelos constructores
  `curve.move`/`curve.line`/`curve.cubic`/`curve.quad`/`curve.close` —
  ver §"Namespace `curve`" abaixo); os seus
  segmentos são concatenados ao path final.
- `fill`, `stroke`: opcionais. **Fallback determinístico** (paridade vanilla
  `Smart::Auto`, `lab/typst-original/crates/typst-layout/src/shapes.rs:126-129`,
  corrigido no Passo 727): se nem `fill` nem `stroke` forem fornecidos,
  aplica stroke preta de 1pt; se `fill` for fornecido sem `stroke`, a curva
  fica sem stroke — idêntico ao fallback de `native_rect`.

**Semântica**: Constrói um path arbitrário, calcula bbox via `path_bbox`,
cria `Content::Shape { kind: Path, ... }`.

**Paridade vanilla**: Interface por tuples descritivos em vez dos métodos
`curve.move`/`curve.cubic` do vanilla (proc-macros de scope ainda não
materializados em L1); desde o Passo 513 aceita também `Content::Curve`
como segmento posicional.

**Limitações / scope-outs**:
- `stroke` sólido 1pt.

**Testes canónicos**:
```
curve(("move", (0,0)), ("line", (1,0)), ("line", (1,1)), ("close",)) -> Shape Path
curve(("quadratic", (0.5, 0.5), (1,0))) -> CubicTo equivalente
curve(("move", (0,0)), ("line", (1,0))) -> Shape Path com stroke preto 1pt (fallback)
curve(("move", (0,0)), ("line", (1,0)), fill: red) -> Shape Path fill red, sem stroke
curve(curve.move((0,0)), curve.line((100,0)), curve.close()) -> Shape Path fechado
curve("x") -> Err "array de segmento válido"
```

---

### Namespace `curve` (P513; absorvido de `compiler/stdlib/curve.md` em P847)

**Assinaturas**:

```typst
curve.move(point) -> content
curve.line(point) -> content
curve.cubic(control1, control2, end) -> content
curve.quad(control, end) -> content
curve.close() -> content
```

`point` é um array de 2 elementos (`(x, y)`) ou `Value::Array` de
`Length`/`Float`/`Int`. Cada função devolve `Content::Curve` com um único
segmento: `curve.move(p)` move a caneta para `p`; `curve.line(p)` linha
recta até `p`; `curve.cubic(c1, c2, e)` cúbica de Bézier com control
points `c1`, `c2` e ponto final `e`; `curve.quad(c, e)` quadrática com
control point `c` e ponto final `e`; `curve.close()` fecha o path actual.

**Implementação**: `native_curve_move`, `native_curve_line`,
`native_curve_cubic`, `native_curve_quad`, `native_curve_close` em
`shapes.rs`. Helper `extract_point(val, fn_name, arg_name)` aceita
`Value::Array` com 2 elementos (`Length`, `Float` ou `Int` convertido
para pt). O namespace é anexado à função `curve` em `eval/mod.rs`
(`curve.move`, `curve.line`, `curve.cubic`, `curve.quad`, `curve.close`).

**P803 — erro de tipo paridade vanilla**: argumento posicional que não é
`Content::Curve` nem tuplo legado (array com primeiro elemento string)
produz `expected content, found {type}` (padrão `type_name()`, paridade
vanilla — ex.: `#curve((0pt, 0pt))` → `expected content, found array`,
medido no vanilla 0.15.0). Os erros internos do tuplo legado (coordenada
inválida, aridade, kind desconhecido) mantêm-se.

**Casos de aceitação**:
- `curve.move((0,0))` → `Content::Curve` com `Move`; `curve.line((100pt, 0pt))` → `Line`.
- `curve.cubic((0,0), (50,50), (100,0))` → `Cubic`; `curve.quad((0,0), (50,50))` → `Quad`.
- `curve.close()` → `Close`; `curve(curve.move((0,0)), curve.line((100,0)), curve.close())` produz path fechado.

---

### `native_square(width, height?, fill?, stroke?)`

**Assinatura**: `square(width: Length, height: Length?, fill: Paint?, stroke: Color?) -> Content`

**Semântica**: helper sintático sobre `rect` — constrói `ShapeKind::Rect`
com `width == height` quando `height` é omitido. Sem tipo novo e sem
layout/render novo: a saída é idêntica à de `rect(width: w, height: w)`
por construção. Reutiliza `ShapeKind::Rect` e `Content::Shape` existentes.

**Argumentos**:
- `width`: primeiro argumento posicional ou nomeado `width`. Obrigatório.
- `height`: nomeado `height`; se omitido, assume o valor de `width`.
- `fill` / `stroke`: opcionais, mesmo parsing e fallback de `native_rect`.
- Argumentos nomeados desconhecidos → erro (padrão das nativas do cluster).

**Paridade vanilla**:
- `square(1cm)` ≡ `rect(width: 1cm, height: 1cm)` (morfologicamente — mesma
  forma `Rect`).
- `square(1cm, height: 2cm)` comporta-se como `rect(width: 1cm, height: 2cm)`
  (fallback aceite).

**Scope-outs explícitos**:
- Não criar `ShapeKind::Square`.
- Não adicionar variant `Value` ou `Content`.
- Não tocar em layout/render/export.

**Testes canónicos**:
```
square(w) -> Shape Rect com width == height == w
square(w, height: h) com h != w -> Rect genérico
square() sem width -> Err
square(w, named:{x:1}) -> Err "argumento nomeado inesperado"
square(w) sem cores -> stroke preta 1pt (paridade com rect)
```

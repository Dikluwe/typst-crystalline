# Prompt L0 — geometry
Hash do Código: 9475ace5

## Módulo
`01_core/src/entities/geometry.rs`

## Propósito
Tipos geométricos primitivos para o sistema de layout vectorial (Passo 76).
Puramente declarativos — sem I/O, sem métricas de fonte.
Depende de `Color` de `layout_types` — sem dependências externas adicionais.

## Tipos

### `Stroke`
Contorno de uma forma: cor, espessura em pontos, e overhang.
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub paint:     Color,
    pub thickness: f64,
    pub overhang:  bool,  // P252 — default false em construtor Rust
}
```
Usa `Color` de `layout_types` (já existente em L1) — sem tipo `RgbaColor` separado.

### P1223 — contrato proposto para stroke complexo (GATE ADR-0127)

> **Estado:** especificação L0 guardada; materialização produtiva bloqueada até
> confirmação humana porque adiciona campos à entidade pública compartilhada.

A superfície Typst ratificada aceita e preserva `cap`, `join`, `dash` e
`miter-limit`. A entidade não pode descartá-los entre eval, layout e exportação.
O modelo resolvido será:

```rust
pub enum LineCap { Butt, Round, Square }
pub enum LineJoin { Miter, Round, Bevel }
pub enum DashLength { Length(f64), LineWidth }
pub struct DashPattern {
    pub array: Vec<DashLength>,
    pub phase: f64,
}
pub struct Stroke {
    pub paint: Paint,
    pub thickness: f64,
    pub cap: LineCap,
    pub join: LineJoin,
    pub dash: Option<DashPattern>,
    pub miter_limit: f64,
    pub overhang: bool,
}
```

Defaults de linguagem: `cap = Butt`, `join = Miter`, `dash = None`,
`miter_limit = 4.0`. `DashLength::LineWidth` preserva o item público `"dot"`
até a espessura estar resolvida; comprimentos explícitos e `phase` usam pontos.
A ordem do array é semântica e nunca pode ser ordenada ou deduplicada.

`overhang` continua sendo extensão interna cristalina e não substitui nenhum
campo vanilla. Eval deve aceitar os nomes vanilla, validar seus domínios e
produzir `Value::Stroke` sem perda. Layout/`FrameItem::Shape` transporta o
mesmo `Stroke`; exportadores decidem apenas a serialização tardia. SVG emite
`stroke-linecap`, `stroke-linejoin`, `stroke-miterlimit`, `stroke-dasharray` e
`stroke-dashoffset` quando seus observáveis diferirem do default aplicável.

Critérios RED futuros: os quatro construtores aceitos pelo vanilla em P1223
(`cap: "round"`, `join: "bevel"`, `dash: "dashed"`, `miter-limit: 2`) devem
deixar de produzir “argumento nomeado inesperado”; o valor avaliado e o SVG
devem conservar cada dimensão separadamente. O trabalho deve começar pela
entidade e testes, e não pelo exportador, pois a primeira perda medida ocorre
antes de L3.

**P252 (M9d / M7+5; ADR-0079 Categoria A.4 Boxed COMPLETO 6/6;
cita ADR-0082 PROPOSTO N=3 terceira aplicação citante — limiar
atingido)** — field `overhang: bool` controla se o stroke se
sobrepõe ao corner da bounding box.

#### Default cristalino divergente P252

**Construtor Rust low-level**: `Stroke { paint, thickness,
overhang: false }`. Divergência consciente face vanilla
(`overhang: true` default). Justificações cumulativas:

1. **Backward compat literal estrita**: ~34 construtores literais
   pré-P252 preservam bounds Shape bit-equivalente.
2. **Anti-inflação 44ª**: defaults zero-impact em construtor
   low-level (paridade pattern P247 `fill: None` default
   construtor Rust).
3. **Paridade vanilla user-facing preservada via stdlib parse**:
   `extract_stroke` helper aplica `overhang: true` default
   quando input é `Length`/`Color` atalho ou `Dict` sem chave
   `overhang` explícita.

#### Activação semantic real Layouter (Block + Boxed)

Quando `stroke.overhang == true` em `FrameItem::Shape` emit:

```
shape_pos.x  = pos.x - thickness / 2
shape_pos.y  = pos.y - thickness / 2
shape_width  = width  + thickness    // +thickness/2 cada lado
shape_height = height + thickness
```

Quando `overhang == false`: bounds preservados literal (centered
stroke padrão PDF; backward compat).

**Limitações conscientes P252**:

- Aplicação activada em **Block + Boxed only** (Shape Rect/
  RoundedRect emit com stroke). Grid/Table cell borders são
  `FrameItem::Shape::Line` (4 linhas per cell) — overhang
  conceptualmente n/a (line cap distinct). Divergência
  consciente per ADR-0054 graded.
- PDF exporter intocado: bounds finais já calculados em
  Layouter (single source of truth).
- Round corners (RoundedRect P242) + overhang: bounds expandidos
  com radius preservado; visualmente paridade vanilla graded.

**Sub-padrão "Refactor cross-cutting entity primitivo" N=1
inaugurado P252** — novo módulo conceptual (Stroke é entity
primitivo cross-cutting em 6 variants Content + 4 caminhos PDF
exporter). Candidato a formalização N=3-4 futuro.

**Sub-padrão "Backward compat literal estrita" N=1 → N=2
cumulativo P252** (P251 cell tails preservam P248 clip para
Fixed rows + P252 stroke overhang preserva bounds via default
construtor `false`).

**Promoções reais scope-outs ADR-0054 graded granular**: N=13 →
**N=14 cumulativo P252** (P252 ×1: Boxed.stroke-overhang real;
**fecha Boxed A.4 COMPLETO 6/6** — segundo variant Content com
100% scope-outs originais fechados cumulativamente após Block
P250).

### `ShapeKind`
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeKind<T = Length> {
    Rect,
    Ellipse,
    Line { dx: f64, dy: f64 },
    RoundedRect { radii: Corners<T> },       // P242; Length no Content, Pt no Frame (P1133)
    Path(Vec<PathItem>),                      // P79+ (extended P277)
}
```
`Line::dx`/`dy`: deslocamentos no sistema de layout (Y positivo = baixo).
Bounding box de `Line` usa `abs()` dos deltas.

`Ellipse`: scaffolding presente; exportador PDF emite rectângulo placeholder
com comentário `TODO` apontando DEBT-31.

`RoundedRect`: P242 — rectângulo com cantos arredondados; radii por canto.
Ver §"ShapeKind::RoundedRect" abaixo.

### `ShapeKind::Path` — bbox analítica (P277, DEBT-33 fecho CLOSED)

`Path(Vec<PathItem>)` representa caminho geométrico livre composto por
segmentos `MoveTo`, `LineTo`, `CubicTo`, `ClosePath`. Materializado em
P79 (polígonos); CubicTo em P79+ enum (zero stdlib producers actuais
mas estruturalmente disponível para `curve()`/`path()` user-facing
futuro).

**Bbox calculada analíticamente** (P277 — DEBT-33 CLOSED):
- `MoveTo` / `LineTo`: endpoints contribuem directamente para min/max.
- `CubicTo(P₁, P₂, P₃)` a partir de `current_point P₀`: bbox via
  **raízes de B'(t)=0** em cada eixo. Candidatos: endpoints (P₀, P₃)
  + raízes em `t ∈ (0, 1)`. `B'(t) = 3·[a·t² + b·t + c]` onde
  `a = -P₀ + 3P₁ - 3P₂ + P₃`, `b = 2P₀ - 4P₁ + 2P₂`, `c = -P₀ + P₁`.

**Razão**: curva real de `CubicTo` pode exceder a bounding box dos
pontos de controlo. Cálculo analítico via raízes da derivada produz
AABB exacto (não conservador). Corrige vazamento visual subtil em
curvas que excedem control points.

**Complexidade**: O(1) por segmento CubicTo (≤6 candidatos a comparar).
**Pureza**: matemática `f64` pura; zero deps externas (ADR-0029
preserved absoluto).

**Helpers L1 em `geometry.rs`**:
- `bezier_cubic_bbox(p0, p1, p2, p3) -> (min_x, min_y, max_x, max_y)`.
- `path_bbox(items: &[PathItem]) -> (f64, f64, f64, f64)` — walker
  sobre PathItems com estado `current_point`.

`polygon()` (stdlib/shapes.rs) usa `path_bbox()` para consolidação;
LineTo-only paths preservam bit-exact min/max behavior.

## Exposição em `entities/mod.rs`

## P1225 — presença explícita dos campos (NOVO GATE ADR-0127)

Medição pública mostrou que valores resolvidos não bastam para `repr`:

```text
stroke()                    -> "1pt + black"
stroke(paint: red)          -> "rgb(\"#ff4136\")"
stroke(thickness: 1pt)      -> "1pt"
stroke(cap: "butt")         -> "(cap: \"butt\")"
stroke(miter-limit: 4)      -> "(miter-limit: 4.0)"
```

`Stroke` deve preservar quais campos foram explicitamente fornecidos, mesmo
quando iguais ao default resolvido:

```rust
pub struct StrokeFields {
    pub paint: bool,
    pub thickness: bool,
    pub cap: bool,
    pub join: bool,
    pub dash: bool,
    pub miter_limit: bool,
}
```

`Stroke` ganha `pub specified: StrokeFields`. Construtores internos usam todos
os bits `false`; `native_stroke` marca cada named presente. Layout/exportadores
consomem os valores resolvidos e ignoram `specified`; repr usa os bits para
distinguir ausência de valor explicitamente igual ao default.

> **Estado:** preparado em P1225 e aguarda confirmação humana, pois acrescenta
> estado a entidade pública compartilhada. A materialização P1225 para aqui.

## Exposição histórica em `entities/mod.rs`
```rust
pub mod geometry;
```
Tipos reutilizados nos módulos que os precisam via caminhos explícitos.

## Critérios de verificação
- Zero dependências externas além de `Color` de `layout_types`.
- `ShapeKind` e `Stroke` derivam `Debug`, `Clone`, `PartialEq`.

## `ShapeKind::RoundedRect` — Passo 242 (M9d/M7+5)

```rust
ShapeKind::RoundedRect { radii: Corners<T> }
```

Rectângulo com cantos arredondados (paridade vanilla
`layout/shape.rs::RoundedRect`). Co-existe com `Rect` / `Ellipse`
/ `Line` / `Path`. Consumer principal: `Content::Block.radius` +
`Content::Boxed.radius` (refino P231 → P242 `Option<Length>` →
`Corners<Length>`). P1133 distingue o valor de entrada do valor geométrico:
o `Content` conserva `ShapeKind<Length>`, mas o layout resolve cada canto com o
font-size vigente e só então constrói o `ShapeKind<Pt>` transportado pelo frame.

**Degeneração**: quando todos os 4 radii são zero, semantic é
equivalente a `Rect` mas a distinção estrutural é preservada (não
normaliza para `Rect` — preserva distinguibilidade). PDF exporter
P242 emite Bezier 4 corners path mesmo quando radii zero (output
correcto via geometria degenerada).

**Bezier kappa**: `0.552_284_749_831` (paridade `ShapeKind::Ellipse`
mesmo ficheiro). Quarto de círculo aproximado com 2 control points
por canto. Output PDF: `m` + `l` × 4 + `c` × N + `h` (N = #cantos
não-zero).

**Layout integration**: `Content::Block.clip == true` + `radius
!= zero` → Layouter emite `FrameItem::Group` com `clip_mask:
Some(ShapeKind::RoundedRect { radii: resolved_radius })`. PDF exporter
desenha clip path via `emit_rounded_rect_ops`.

**Bounding box**: radii não afecta bounding box (paridade Rect;
clamp interno em `emit_rounded_rect_ops` para `radii ≤ min(w,h)/2`
evita overflow geométrico). `measure_content` arms tratam
RoundedRect identicamente a Rect.

---

## Estado actual cumulativo (reconciliação P259 Cenário B2 — Fase A)

**Anotação documental**: esta secção reconcilia o prompt L0
com o estado real do código apurado em
`diagnostico-visualize-fase-a-passo-259.md`. Representação base
acima preservada como **histórico cumulativo** (paridade pattern
P258.B + ADR-0080 §"refactor aditivo"). **Não reconciliação
destructiva**.

### Estado real `ShapeKind` (5 variants)

```rust
pub enum ShapeKind<T = Length> {
    Rect,
    RoundedRect { radii: Corners<T> },        // P242; Length no Content, Pt no Frame (P1133)
    Ellipse,
    Line { dx: f64, dy: f64 },
    Path(Vec<PathItem>),                       // P79
}
```

### `PathItem` (P79) — não documentado anteriormente

```rust
pub enum PathItem {
    MoveTo(Point),
    LineTo(Point),
    CubicTo(Point, Point, Point),  // control1, control2, end
    ClosePath,
}
```

Path é container de sub-paths via `Vec<PathItem>`. PDF exporter
(P79+) emite `m` / `l` / `c` / `h` operators correspondentes.
DEBT-33 EM ABERTO preservado — bbox por min/max dos pontos de
controlo (não extremos paramétricos exactos).

### Ellipse — actualização P242 (linha 101-102 obsoleta)

A linha 101-102 acima ("scaffolding presente; exportador PDF
emite rectângulo placeholder TODO DEBT-31") está **factualmente
desactualizada per audit P259 Fase A**. Estado real:

```text
PDF exporter (03_infra/src/export.rs:875 +1037 +1137 +1383 +1565)
emite Ellipse via Bézier 4 corners com
kappa = 0.552_284_749_831  (paridade ShapeKind::Ellipse mesmo
ficheiro). Output PDF: m + c × 4 + h. Promoção parcial →
implementado⁺ via P242.
```

DEBT-31 (Transform afim) está **ENCERRADO em P78**; a nota
sobre Ellipse placeholder estava cruzada incorrectamente nessa
DEBT histórica.

### Polygon — promoção stdlib (não-documentada anteriormente)

`native_polygon` em `01_core/src/compiler/stdlib/shapes.rs:223`
materializa polygon via conversão para `ShapeKind::Path` com
sequência `MoveTo + N LineTo + ClosePath`. Não há `ShapeKind::
Polygon` estrutural separada (Path é representação canónica).

Stdlib: `scope.define("polygon", native_polygon)` em
`rules/eval/mod.rs:591`. Testes em `stdlib/mod.rs:1565-1599`.

Promoção `ausente → implementado` reconhecida em audit P259.A.

### Tabela B agregada Visualize (audit P259)

| Estado | Audit P259 |
|--------|------------|
| implementado | 10/27 (37%) |
| implementado⁺ | 4/27 (15%) — Color, Ellipse, Path, Stroke base |
| parcial | 0/27 (0%) |
| ausente | 13/27 (48%) |

**Cobertura ponderada linear**: 51.9%.
**Cobertura ponderada com bonus implementado⁺**: 54.8%.

Pendentes pós-P259 (Cenário B2 confirmado):

1. **Gradient subsistema** (F.1 Linear + F.2 Radial + F.3
   Conic) — ausentes; candidato P261.
2. **Paint wrapper** (G) — ausente; pré-requisito Gradient;
   candidato P260.
3. **Stroke refinos** (C.2 Stroke<T> Length + C.3 Dash +
   C.4 LineCap/Join) — ausentes; candidatos M.
4. **Image refinos** (D.3 SVG + D.4 metadata `alt`/`fit`) —
   ausentes; SVG L+ requer ADR crate `usvg`/`resvg`.
5. **Transform `origin` pivot** (E.5) — ausente com **scope-out
   documentado per ADR-0061** em `rules/stdlib/transforms.rs:
   104-105`. Materializar reverte scope-out arquitectural.
6. **Curve variant** (B.6) — ausente; refino bezier separado
   (não conversão Path stdlib).
7. **DEBT-33 Bézier bbox exacto** — EM ABERTO; refino qualitativo.

**Cenário B2 confirmado**: Opções 1-5 para P260+ dedicados.
P259.C saltado per decisão local (preservar política
administrativa documental + scope-out ADR-0061 sobre Opção 4).

## P1133 — raio resolvido na fase de layout

### Medição anterior à decisão

No estado `HEAD 781b207b4a5d` com working tree não commitado, os três
exportadores recebiam `Corners<Length>` sem o font-size que dá significado ao
componente `em`. `render.rs` e `svg.rs` chamavam `resolve_pt(0.0)`; `stream.rs`
lia somente `abs.0`. O caso `radius: 1em` produzia cantos quadrados no
cristalino e arredondados no vanilla ratificado. V23 `ContextErasure` confirmou
os três caminhos.

### Contrato

- `ShapeKind` é genérico em `T`, com default público `T = Length` para a árvore
  de conteúdo. `ShapeElem.kind` e os construtores user-facing continuam a usar
  `ShapeKind<Length>`; assim `Content::shape_with_radius` não precisa inventar
  font-size antes do layout.
- `Content::Block.radius`, `Content::Boxed.radius` e
  `TextStyle::highlight_radius` continuam a aceitar/armazenar `Length`, pois
  pertencem à linguagem e ainda dependem do estilo.
- A transição `Content/TextStyle → ShapeKind` é a fronteira proprietária da
  resolução. Cada canto usa `Length::resolve_pt(style.size.val())` do contexto
  que criou a forma.
- `FrameItem::Shape.kind` e `FrameItem::Group.clip_mask` usam explicitamente
  `ShapeKind<Pt>`; nessa especialização `RoundedRect` transporta
  `Corners<Pt>` e já está em coordenadas geométricas absolutas.
- A fronteira usa uma conversão exaustiva `ShapeKind<Length> → ShapeKind<Pt>`:
  variantes sem comprimento são preservadas; `RoundedRect` resolve os quatro
  cantos. Não usar cast, `Default` nem uma segunda variante ad hoc.
- L3 não chama `resolve_pt`, não acessa `Length.abs` e não infere font-size.
  PDF, SVG e raster consomem diretamente os quatro valores em pontos e aplicam
  apenas o clamp geométrico `min(width, height) / 2`.
- A resolução é por canto; não reduzir `Corners` ao `top_left` nos
  exportadores. Um backend que só ofereça raio uniforme deve degradar de forma
  explicitamente testada, nunca descartar silenciosamente três cantos.

### Critérios de verificação

- `1em` com estilo de 20pt chega ao frame como 20pt, sem constante empírica.
- comprimento misto `2pt + 0.5em` resolve para 12pt no mesmo estilo.
- quatro cantos distintos permanecem distintos até o emissor PDF.
- `radius: 0pt` conserva a degeneração para `Rect` decidida pelo layout.
- V23 não encontra `resolve_pt(0.0)` nem projeção `.abs` no transporte de raio.


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — identidade pública de `stroke`

`stroke` torna-se `Value::Type(Type::Stroke)` chamável. O valor construído
continua `Value::Stroke(Stroke)` com a mesma estrutura geométrica; nenhum campo
da entidade, igualdade ou comportamento de render é alterado.
# P1226 — regras de preenchimento de paths

Medição pública anterior à decisão: uma `curve` com `fill-rule: "even-odd"`
compila no vanilla ratificado e o SVG conserva `fill-rule="evenodd"`; o
cristalino aceita a chamada, mas emite `nonzero`. A geometria deve possuir o
enum público fechado `FillRule { NonZero, EvenOdd }`, com `NonZero` por
defeito. O valor é semântico: determina a região preenchida em paths com
subpaths sobrepostos. Não inferir a regra a partir de winding ou do SVG.

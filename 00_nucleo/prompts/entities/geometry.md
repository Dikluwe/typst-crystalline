# Prompt L0 — geometry
Hash do Código: 52271440

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
pub enum ShapeKind {
    Rect,
    Ellipse,
    Line { dx: f64, dy: f64 },
    RoundedRect { radii: Corners<Length> },  // P242
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
```rust
pub mod geometry;
```
Tipos reutilizados nos módulos que os precisam via caminhos explícitos.

## Critérios de verificação
- Zero dependências externas além de `Color` de `layout_types`.
- `ShapeKind` e `Stroke` derivam `Debug`, `Clone`, `PartialEq`.

## `ShapeKind::RoundedRect` — Passo 242 (M9d/M7+5)

```rust
ShapeKind::RoundedRect {
    radii: crate::entities::corners::Corners<Length>,
}
```

Rectângulo com cantos arredondados (paridade vanilla
`layout/shape.rs::RoundedRect`). Co-existe com `Rect` / `Ellipse`
/ `Line` / `Path`. Consumer principal: `Content::Block.radius` +
`Content::Boxed.radius` (refino P231 → P242 `Option<Length>` →
`Corners<Length>`).

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
Some(ShapeKind::RoundedRect { radii: radius })`. PDF exporter
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
pub enum ShapeKind {
    Rect,
    RoundedRect { radii: Corners<Length> },   // P242
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

`native_polygon` em `01_core/src/rules/stdlib/shapes.rs:223`
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

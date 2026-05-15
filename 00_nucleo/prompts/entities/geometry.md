# Prompt L0 — geometry
Hash do Código: 3f93a04c

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
}
```
`Line::dx`/`dy`: deslocamentos no sistema de layout (Y positivo = baixo).
Bounding box de `Line` usa `abs()` dos deltas.

`Ellipse`: scaffolding presente; exportador PDF emite rectângulo placeholder
com comentário `TODO` apontando DEBT-31.

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

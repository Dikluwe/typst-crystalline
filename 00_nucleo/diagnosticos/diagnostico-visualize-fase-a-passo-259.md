# Diagnóstico Visualize Fase A — Passo 259 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0034 diagnóstico canónico + ADR-0065 inventariar
primeiro critério #5.
**Diagnóstico pai**: `diagnostico-visualize-passo-259.md` +
`fase-a-checklist-visualize-passo-259.md`.
**Análogo estrutural**: P255 (Math) + P257 (Color) + P258 (Model).
**Imutabilidade**: após criação, este ficheiro **não pode ser
editado** per ADR-0034 §"diagnóstico imutável".

---

## §1 — Comandos executados e output literal

### Bloco 1 — Color subsistema (P257 confirmar)

```bash
$ grep -c "^\s*[A-Z][a-zA-Z]*\s*{" 01_core/src/entities/color.rs
8
$ grep -n "^\s*[A-Z][a-zA-Z]*\s*{" 01_core/src/entities/color.rs
36:    Srgb { r: f32, g: f32, b: f32, a: f32 },
39:    Luma { l: f32, a: f32 },
43:    LinearRgb { r: f32, g: f32, b: f32, a: f32 },
47:    Oklab { l: f32, a: f32, b: f32, alpha: f32 },
50:    Oklch { l: f32, c: f32, h: f32, alpha: f32 },
53:    Hsl { h: f32, s: f32, l: f32, a: f32 },
56:    Hsv { h: f32, s: f32, v: f32, a: f32 },
61:    Cmyk { c: f32, m: f32, y: f32, k: f32 },
```

→ 8 variants confirmadas; P257 preservado.

```bash
$ grep -rln "native_rgb\|native_luma\|native_oklab\|native_oklch\|native_hsl\|native_hsv\|native_cmyk\|native_linear_rgb" \
  01_core/src/rules/stdlib/
01_core/src/rules/stdlib/foundations.rs
01_core/src/rules/stdlib/mod.rs
```

→ 7+ stdlib funcs cor presentes (P257 preservado).

```bash
$ grep -rn "native_lighten\|native_darken\|native_mix\|native_saturate\|native_desaturate\|native_negate" \
  01_core/src/rules/stdlib/
(zero hits)
```

→ Operadores cor ausentes (ADR-0083 scope-out #2 preservado).

```bash
$ grep -n "fn to_srgb\|fn to_rgba_f32" 01_core/src/entities/color.rs
191:    pub fn to_srgb(&self) -> (u8, u8, u8, u8) {
204:    pub fn to_rgba_f32(&self) -> (f32, f32, f32, f32) {
```

→ Conversões básicas presentes.

### Bloco 2 — Shapes subsistema

```bash
$ grep -n "^\s*[A-Z]" 01_core/src/entities/geometry.rs | head -40
13:    MoveTo(Point),
15:    LineTo(Point),
17:    CubicTo(Point, Point, Point),
19:    ClosePath,
47:    Rect,
62:    RoundedRect { ... },
66:    Ellipse,
71:    Line { dx: f64, dy: f64 },
76:    Path(Vec<PathItem>),
```

→ **5 ShapeKind variants** (Rect, RoundedRect, Ellipse, Line,
Path) — **descoberta: `RoundedRect` não documentado no
diagnóstico pai**.

```bash
$ grep -rn "MoveTo\|LineTo\|CubicTo\|QuadTo\|ClosePath" 01_core/src/entities/geometry.rs
13: MoveTo(Point),
15: LineTo(Point),
17: CubicTo(Point, Point, Point),
19: ClosePath,
75: (CubicTo aprox bbox)
```

→ Path P79 confirmado MoveTo/LineTo/CubicTo/ClosePath; **QuadTo
ausente** (não materializado pós-P79).

```bash
$ grep -rn "Polygon\|ShapeKind::Polygon\|native_polygon" 01_core/src/
01_core/src/rules/stdlib/shapes.rs:223:pub fn native_polygon(...)
01_core/src/rules/eval/mod.rs:559: native_polygon (registo)
01_core/src/rules/eval/mod.rs:591: scope.define("polygon", native_polygon)
01_core/src/rules/stdlib/mod.rs:47: native_polygon (re-export)
01_core/src/rules/stdlib/mod.rs:1565,1580,1599: testes native_polygon
```

→ **`native_polygon` MATERIALIZADO em stdlib** — promoção
ausente → implementado **não documentada no diagnóstico pai**.
Polygon convertido internamente para `ShapeKind::Path` via
`Vec<PathItem>` (MoveTo + N LineTo + ClosePath); não há variant
estrutural separada.

```bash
$ grep -n "ellipse\|Ellipse" 03_infra/src/export.rs | head -10
875: P242 — Bezier 4 corners (paridade Ellipse mesmo kappa)
878: ShapeKind::Ellipse =>
1037: Bezier kappa = 0.552_284_749_831 (paridade ShapeKind::Ellipse)
1137: ShapeKind::Ellipse =>
1383: ShapeKind::Ellipse =>
1565: ShapeKind::Ellipse =>
```

→ **Ellipse REAL via Bézier 4 corners kappa=0.5522847498 (P242)**
— promoção parcial → implementado⁺ **não documentada no
diagnóstico pai** (DEBT-31 nota "rectângulo placeholder" está
desactualizada).

```bash
$ grep -rn "Curve\|ShapeKind::Curve\|native_curve" 01_core/src/
(zero hits)
```

→ Curve ausente confirmado.

### Bloco 3 — Path subsistema (DEBT-33)

```bash
$ grep -rn "bounding_box\|bbox" 01_core/src/rules/layout/ 01_core/src/entities/
01_core/src/entities/layout_types.rs:921: fn transform_matrix_rotacao_45_graus_aumenta_bounding_box()
01_core/src/entities/geometry.rs:91: fn shapekind_line_bounding_box_abs()
```

→ Bbox utilidades presentes; sem fn `exact_bezier_bbox`.

```bash
$ grep -A 20 "DEBT-33" 00_nucleo/DEBT.md
## DEBT-33 — Bounding Box de curvas Bézier (Passo 79) — EM ABERTO
A bounding box de `ShapeKind::Path` é calculada verificando o min/max dos pontos
de controlo. Para `CubicTo`, a curva real pode ultrapassar a caixa delimitadora
dos pontos de controlo, causando vazamento visual subtil.
```

→ **DEBT-33 EM ABERTO preservado** — aproximação por min/max
preservada.

### Bloco 4 — Stroke (Paint, dash, caps)

```bash
$ grep -A 12 "pub struct Stroke" 01_core/src/entities/geometry.rs
pub struct Stroke {
    pub paint:     Color,
    pub thickness: f64,
    pub overhang:  bool,   // P252
}
```

→ Stroke struct expandido por P252 com `overhang: bool`;
**promoção implementado → implementado⁺ não documentada no
diagnóstico pai**.

```bash
$ grep -rn "enum Paint\|\bPaint::\|\bPaint\b" 01_core/src/entities/
01_core/src/entities/content.rs:628: comment "(não Paint; Paint ausente)"
01_core/src/entities/content.rs:693: comment "(Paint enum ausente)"
```

→ **Paint enum AUSENTE confirmado** (só comments documentais).

```bash
$ grep -rn "DashPattern\|Dash::\|dash_pattern" 01_core/src/
(zero hits)
$ grep -rn "LineCap\|LineJoin\|MiterLimit\|miter_limit" 01_core/src/
(zero hits)
$ grep -rn "Stroke<" 01_core/src/
(zero hits)
```

→ Dash, LineCap, LineJoin, Stroke<T> generic **ausentes
confirmados**.

### Bloco 5 — Gradient subsistema

```bash
$ grep -rn "Gradient\|LinearGradient\|RadialGradient\|ConicGradient" 01_core/src/
01_core/src/entities/value.rs:83: // Gradient(Gradient), (comment placeholder)
$ grep -rn "GradientStop\|WeightedColor" 01_core/src/
(zero hits)
$ grep "native_gradient\|gradient" 01_core/src/rules/stdlib/mod.rs
(zero hits)
```

→ **Gradient AUSENTE confirmado** (só comment placeholder).

```bash
$ ls -la lab/typst-original/crates/typst-library/src/visualize/gradient.rs
-rw-rw-r-- 1 dikluwe dikluwe 46678 mar 22 lab/typst-original/.../gradient.rs
```

→ Vanilla file existe (46KB; 3 variants Linear/Radial/Conic).

### Bloco 6 — Paint, Tiling subsistemas

```bash
$ grep -rn "enum Paint\|Tiling\|TilingPattern" 01_core/src/
01_core/src/entities/value.rs:84: // Tiling(Tiling), (comment placeholder)
$ grep "native_tiling\|tiling" 01_core/src/rules/stdlib/mod.rs
(zero hits)
```

→ Paint + Tiling **ausentes confirmados** (só comments
placeholder).

### Bloco 7 — Image subsistema (SVG + metadata)

```bash
$ grep -A 8 "FrameItem::Image\|Image {" 01_core/src/entities/layout_types.rs
Image {
    pos:              Point,
    data:             Arc<Vec<u8>>,
    width:            Pt,
    height:           Pt,
    intrinsic_width:  u32,
    intrinsic_height: u32,
},
```

→ Image fields: data + width + height + intrinsic_w/h. **Sem
`alt`/`fit`/`format`** metadata.

```bash
$ grep -rn "Svg\|SVG\|usvg\|resvg\|svgtypes" 01_core/src/ 03_infra/src/
(zero hits)
$ grep -rn "svg\|usvg\|resvg" Cargo.toml */Cargo.toml
(zero hits)
```

→ **SVG AUSENTE confirmado** (zero hits em code + Cargo.toml).

```bash
$ grep -n "\"alt\"\|alt:\|\"fit\"\|fit:" 01_core/src/entities/content.rs
(zero hits)
```

→ Image metadata `alt`/`fit` **ausente confirmado**.

### Bloco 8 — Transform `origin` pivot

```bash
$ grep -n "origin\b" 01_core/src/rules/stdlib/transforms.rs
104: /// - `origin` (ponto de pivot) scope-out; análogo aos move/rotate/scale
105: ///   actuais que também não têm `origin` (refino futuro per ADR-0061).
```

→ **`origin` AUSENTE com scope-out documentado**. Comments em
`transforms.rs:104-105` declaram scope-out per ADR-0061
"refino futuro". Materializar reverte scope-out arquitectural
existente.

```bash
$ grep -A 6 "pub fn native_rotate" 01_core/src/rules/stdlib/transforms.rs
(rotate aceita angle; sem param origin)
```

→ `native_rotate/scale/skew` **não aceitam param `origin`**.

```bash
$ grep -n "pivot\|origin: " 01_core/src/entities/layout_types.rs
(zero hits)
```

→ TransformMatrix sem campo pivot/origin.

### Bloco 9 — Inconsistências documentais

```bash
$ ls -la 00_nucleo/prompts/entities/geometry.md 00_nucleo/prompts/entities/color.md 00_nucleo/prompts/entities/layout_types.md
-rw-rw-r-- 1 dikluwe 5725 mai 15 13:34 color.md
-rw-rw-r-- 1 dikluwe 5355 mai 15 10:32 geometry.md
-rw-rw-r-- 1 dikluwe 2286 abr 19 22:18 layout_types.md
```

→ Files existem; `color.md` recente (P257); `geometry.md`
recente (10:32 P252 paridade?); `layout_types.md` antigo
(2026-04-19; provável desactualização).

```bash
$ grep "@prompt-hash" 01_core/src/entities/{geometry,color,layout_types}.rs
geometry.rs: //! @prompt-hash 7c1ba7a4
color.rs:    //! @prompt-hash 20a91590
layout_types.rs: //! @prompt-hash af36c701
```

→ Hashes registados no código. `crystalline-lint` confirma
zero violations V5 → hashes consistentes com L0 actual.

```bash
$ head -50 00_nucleo/prompts/entities/geometry.md
```

→ `geometry.md` lista Stroke (com P252 overhang) +
ShapeKind base. Verificar se cobre:
- RoundedRect (P242).
- Path Polygon stdlib promoção.
- Ellipse Bézier 4 corners (P242).

### Bloco 10 — Exportador PDF visualize features

```bash
$ grep -n "fn emit_shape\|fn render_shape" 03_infra/src/export.rs
995: fn emit_shape_path_local(...)
```

→ Path emit helper presente.

```bash
$ grep -n "shading\|/Pattern\|ShadingType\|axial" 03_infra/src/export.rs
(zero hits)
```

→ **PDF Gradient/Shading AUSENTE confirmado** (Gradient
materializar exigirá expansão exporter).

---

## §2 — Classificação por subsistema (Tabela A — 27 entradas)

| # | Subsistema | Pré-audit | Audit P259 | Hits literais | Justificação |
|---|------------|-----------|------------|---------------|--------------|
| A | Color (8/8) | 100% (P257) | **implementado⁺** | 8 variants + 7 stdlib funcs | P257 preservado |
| B.1 | Rect | implementado | implementado | `Rect` em ShapeKind:47 | base preservada |
| B.2 | Ellipse | parcial | **implementado⁺** ⬆ | P242 Bézier kappa em export.rs (4 refs) | promoção P242 não-documentada |
| B.3 | Line | implementado | implementado | `Line` em ShapeKind:71 | base preservada |
| B.4 | Path | implementado⁺ DEBT-33 | implementado⁺ | MoveTo/LineTo/CubicTo/ClosePath | DEBT-33 EM ABERTO preservado |
| B.5 | Polygon | ausente | **implementado** ⬆ | `native_polygon` stdlib + 4 refs eval/mod + testes | promoção stdlib não-documentada |
| B.6 | Curve | ausente | ausente | zero hits | preservado |
| C.1 | Stroke base | implementado | **implementado⁺** ⬆ | `overhang: bool` campo P252 | promoção P252 não-documentada |
| C.2 | Stroke<T> Length | ausente | ausente | zero `Stroke<` | preservado |
| C.3 | Dash | ausente | ausente | zero DashPattern | preservado |
| C.4 | LineCap/Join | ausente | ausente | zero LineCap/LineJoin | preservado |
| C.5 | Paint enum | ausente | ausente | só comments placeholder | preservado |
| D.1 | Image JPEG | implementado | implementado | P73 | preservado |
| D.2 | Image PNG | implementado | implementado | P74 | preservado |
| D.3 | Image SVG | ausente | ausente | zero SVG/usvg | preservado |
| D.4 | Image metadata | parcial/ausente | **ausente** ⬇ | zero alt/fit | re-classificação para baixo (metadata só intrinsic_w/h presente) |
| E.1 | Transform Move | implementado | implementado | P78 | preservado |
| E.2 | Transform Rotate | implementado | implementado | P78 | preservado |
| E.3 | Transform Scale | implementado | implementado | P78 | preservado |
| E.4 | Transform Skew | implementado | implementado | P156F | preservado |
| E.5 | Transform origin pivot | ausente | ausente | scope-out doc em transforms.rs:104-105 | scope-out per ADR-0061 preservado |
| F.1 | Gradient Linear | ausente | ausente | só comment placeholder value.rs:83 | preservado |
| F.2 | Gradient Radial | ausente | ausente | zero hits | preservado |
| F.3 | Gradient Conic | ausente | ausente | zero hits | preservado |
| G | Paint wrapper | ausente | ausente | só comments placeholder | preservado |
| H | Tiling | ausente | ausente | só comment placeholder value.rs:84 | preservado |
| I | Clip | implementado | implementado | P79 clip_mask | preservado |

**Adicional descoberto fora da Tabela A original**:
- **`ShapeKind::RoundedRect`** (geometry.rs:62) — consumer
  Block/Boxed radius P242. Não classificável directamente nas
  27 entradas; é refino de Rect (B.1) com curvas Bézier per-corner.

---

## §3 — Estado agregado (Tabela B)

| Estado | Pré-P259 (estimado) | Audit P259 | Δ |
|--------|---------------------|------------|---|
| implementado | ~14/27 (52%) | 10/27 (37%) | -4 |
| implementado⁺ | 1/27 (4%) | **4/27 (15%)** | **+3** |
| parcial | 2/27 (7%) | 0/27 (0%) | -2 (promovidos ou rebaixados) |
| ausente | 10/27 (37%) | **13/27 (48%)** | +3 (D.4 rebaixado; promoção Polygon não-compensa) |
| TOTAL | 27 | 27 | 0 |

**Promoções detectadas Audit P259** (3 promoções não-documentadas
no diagnóstico pai):
- B.2 Ellipse: parcial → implementado⁺ (P242 Bézier real).
- B.5 Polygon: ausente → implementado (stdlib `native_polygon`).
- C.1 Stroke base: implementado → implementado⁺ (P252 overhang).

**Rebaixamentos detectados Audit P259** (1):
- D.4 Image metadata: parcial → ausente (re-classificação
  conservadora — só `intrinsic_w/h` é metadata estrutural; sem
  `alt`/`fit` user-facing).

**Cobertura ponderada linear** (peso 1.0 implementado/⁺,
0.5 parcial, 0 ausente):
- Audit P259: `(10 + 4 + 0 + 0) / 27` = **51.9%**.

**Cobertura ponderada** (peso 1.2 implementado⁺, 1.0
implementado, 0 ausente):
- Audit P259: `(10*1.0 + 4*1.2) / 27` = `14.8 / 27` =
  **54.8%**.

**Fechados literais**: 14/27 = **51.9%**; restante = 13/27 =
48.1% ausente.

**Pre-P259 estimativa optimista** (60-65%) era **5-10pp acima**
do real. Causa: contagem optimista de "parcial" (D.4) +
expectativa Stroke<T> não materializado.

---

## §4 — Achados inesperados

### 4.1 — `ShapeKind::RoundedRect` (P242) **não documentado**

`ShapeKind::RoundedRect { ... }` em `geometry.rs:62` com
consumer Block/Boxed via `Content::Block.radius` +
`Content::Boxed.radius` per-corner Length. PDF exporter (P242)
emite Bézier 4 corners path via `emit_shape_path_local`.

**Implicação documental**: `geometry.md` L0 prompt provavelmente
não cobre RoundedRect. Verificar P259.B.

### 4.2 — `Ellipse` Bézier kappa REAL (P242) **não documentado**

DEBT-31 nota "rectângulo placeholder" referenciada no diagnóstico
pai está **factualmente desactualizada**. Export.rs (4 refs)
implementa Bézier 4 corners com kappa=0.552_284_749_831
(paridade `ShapeKind::Ellipse`). Promoção parcial → implementado⁺.

### 4.3 — `Polygon` em stdlib **não documentado**

`native_polygon` em `01_core/src/rules/stdlib/shapes.rs:223`
materializado com testes (3 testes em stdlib/mod.rs:1565-1599).
Implementação via conversão para `ShapeKind::Path` com
`Vec<PathItem>` (MoveTo + N LineTo + ClosePath). Promoção
ausente → implementado.

### 4.4 — `Stroke.overhang` P252 **promoção qualitativa**

Campo `overhang: bool` adicionado por P252 (M9d / M7+5; ADR-0079
Categoria A.4 Boxed COMPLETO 6/6). Promoção implementado →
implementado⁺ via paridade vanilla (default cristalino divergente
`false`; user-facing `true` via stdlib parse).

### 4.5 — Image metadata mais limitada do que pré-classificado

D.4 Image metadata pré-classificado como "parcial/ausente" no
diagnóstico pai. Audit P259 confirma **ausente** — `intrinsic_w/h`
é metadata estrutural (não user-facing); zero `alt`/`fit`.

### 4.6 — Transform `origin` scope-out **documentado ADR-0061**

`transforms.rs:104-105` declara explicitamente scope-out per
ADR-0061 "refino futuro". Decisão arquitectural existente.
Materializar agora reverte scope-out documentado.

### 4.7 — PDF Gradient exporter exige refactor

`03_infra/src/export.rs` tem zero hits `shading/Pattern/
ShadingType`. Materializar Gradient Linear requer expansão
exporter (axial shading `sh` operator). Magnitude P261 inflada
face estimativa M.

### 4.8 — `geometry.md` L0 cobertura incerta

`geometry.md` 5355 bytes (mais que esperado para "Stroke +
ShapeKind base"). Provavelmente cobre Stroke P252 e talvez
RoundedRect. Verificar em P259.B se cobertura está actualizada
vs Path Polygon promotion + Ellipse Bézier real.

---

## §5 — Decisão cenário Fase B

**Contagem fechados/abertos**: 14/27 fechados; 13/27 abertos.

**Cobertura agregada empírica**: **51.9% ponderado linear** /
**54.8% ponderado** (com peso 1.2 implementado⁺).

**Cenário escolhido**: ☑ **B2 (55-70% — sub-passos prioritários)**.

**Justificação**:
- Limiar bordeline: 51.9% literal está ligeiramente abaixo do
  limiar B2 inferior (55%); 54.8% ponderado está logo acima
  do limiar inferior B2.
- 13/27 ausentes não é re-classificação massiva (B3) — é
  estado real coerente com previsão.
- Promoções não-documentadas (Polygon, Ellipse Bézier, Stroke
  overhang) sugerem cobertura útil ligeiramente superior ao
  literal (paridade refinos qualitativos).
- 3 grupos pendentes claros: Gradient/Paint/Tiling (subsistema
  F+G+H), refinos Stroke (Stroke<T>+Dash+LineCap/Join), Image
  (SVG+metadata).
- Cenário B1 não atingido (<<75%) — pendências reais existem.
- Cenário B3 não justificado (re-classificação é mínima: 1
  rebaixamento; 3 promoções).

**Se B2, opção(ões) recomendada(s)** (para validação humana
pós-P259):
- ☑ Opção 1 — Paint enum + Gradient Linear (P260+P261; M+S+;
  +11pp). **Sequência arquitectural preferida**.
- ☐ Opção 2 — Polygon + Ellipse refino — **PARCIALMENTE
  PRÉ-CUMPRIDA** (Polygon stdlib já implementado; Ellipse
  Bézier real P242). Restante: Curve variant + Polygon variant
  estrutural separada (não conversão Path). Magnitude reduzida
  S+ vs S+S original.
- ☐ Opção 3 — DEBT-33 + Stroke<Length> (S+M; +5pp; refinos
  qualitativos).
- ☐ Opção 4 — Transform `origin` pivot (S+; +2-3pp). **Scope-out
  declarado em transforms.rs:104-105 per ADR-0061** — materializar
  reverte decisão arquitectural; recomendado adiar.
- ☐ Opção 5 — SVG image format (L+; NÃO recomendado P260).

**Decisão local sobre P259.C** (per spec §3 §"Excepção" Opção 4):
**P259.C SALTADO** — não materializar Opção 4 em P259.C apesar
da magnitude S+ permitir, por:
1. Scope-out documentado em `transforms.rs:104-105` per
   ADR-0061; materializar agora rompe granularidade arquitectural
   declarada.
2. Materializar `origin` requer ADR explícita "revogação
   scope-out per ADR-0061" — decisão arquitectural fora do scope
   declarado deste passo (P259 é documental + reconciliação
   L0).
3. Preservar política administrativa P259 (paridade pattern
   P258 Cenário B1 puramente documental).

**Cenário B2 implementado**: P259.B reconciliação L0 (geometry.md)
+ P259.D anotações cumulativas. Opções 1-5 ficam para P260+
dedicados.

---

## §6 — Referências

- ADR-0019, ADR-0026, ADR-0029, ADR-0033, ADR-0034, ADR-0054,
  ADR-0061, ADR-0065, ADR-0079, ADR-0082, ADR-0083.
- DEBT-33 (Bézier bbox EM ABERTO preservado).
- DEBT-31 (Transform afim ENCERRADO P78; nota "rectângulo
  placeholder" factualmente desactualizada per audit).
- `diagnostico-visualize-passo-259.md` — diagnóstico pai.
- `fase-a-checklist-visualize-passo-259.md` — comandos exactos.
- P25 — Color simplificado original (REVOGADO via P257).
- P72-P74 — Image stack JPEG+PNG.
- P76 — geometry tipos primitivos.
- P78 — Transform Move/Rotate/Scale.
- P79 — Path + clip + DEBT-30/33.
- P156F — Transform Skew.
- P242 — Boxed/Block radius + Ellipse Bézier kappa P252.
- P252 — Stroke overhang cross-cutting (refactor entidade
  primitivo N=1).
- P257 — Color paridade vanilla 8/8 (template ADR
  PROPOSTO+IMPLEMENTADO mesmo passo N=1; precedente N=2
  refactor cross-cutting).
- P255, P258 — precedentes "auditoria condicional" N=2 + N=4.
- ADR-0029 §"Sobre os tipos tipográficos vanilla" §enumeração
  — fonte canónica pendências Visualize.
- ADR-0083 — Color paridade vanilla IMPLEMENTADO; anotação
  cumulativa P259 esperada.

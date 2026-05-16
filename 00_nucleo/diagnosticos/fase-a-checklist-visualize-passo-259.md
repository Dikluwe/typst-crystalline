# Fase A — Checklist empírico Visualize + Template Fase B

**Companheiro de**: `diagnostico-visualize-passo-259.md`
**Função**: lista executável de comandos `grep`/`view` para
produzir evidência factual sobre subsistemas Visualize.
**Análogo a**: `fase-a-checklist-math-passo-254B.md` (P255
executou) e `fase-a-checklist-model-passo-256.md` (P258
executou).

---

## Comandos Fase A (executáveis em sequência)

### Bloco 1 — Color subsistema (P257 confirmar)

```bash
# Variants Color
grep -c "^\s*[A-Z][a-zA-Z]*\s*{" 01_core/src/entities/color.rs
grep -n "^\s*[A-Z][a-zA-Z]*\s*{" 01_core/src/entities/color.rs

# Stdlib funcs registadas
grep -rn "native_rgb\|native_luma\|native_oklab\|native_oklch\|native_hsl\|native_hsv\|native_cmyk\|native_linear_rgb" \
  01_core/src/rules/stdlib/

# Operadores cor (scope-out ADR-0083 #2)
grep -rn "native_lighten\|native_darken\|native_mix\|native_saturate\|native_desaturate\|native_negate" \
  01_core/src/rules/stdlib/

# Conversões implementadas
grep -n "fn to_srgb\|fn to_rgba_f32" 01_core/src/entities/color.rs
```

**Critério**:
- 8 variants Color confirmados → P257 status preservado.
- 7 stdlib funcs cor → P257 status preservado.
- Operadores cor zero hits → ADR-0083 scope-out #2 confirmado.

### Bloco 2 — Shapes subsistema

```bash
# ShapeKind variants
grep -n "^\s*[A-Z]" 01_core/src/entities/geometry.rs

# Path P79 — MoveTo/LineTo/CubicTo/QuadTo/Close
grep -n "MoveTo\|LineTo\|CubicTo\|QuadTo\|ClosePath\|Close\b" \
  01_core/src/entities/geometry.rs 01_core/src/entities/

# Polygon ausente?
grep -rn "Polygon\|ShapeKind::Polygon\|native_polygon" 01_core/src/

# Ellipse real (vs placeholder rect)
grep -n "ellipse\|Ellipse" 03_infra/src/export.rs | head -20
grep -A 5 "ShapeKind::Ellipse" 03_infra/src/export.rs

# Curve ausente?
grep -rn "Curve\|ShapeKind::Curve\|native_curve" 01_core/src/
```

**Critério**:
- 4 ShapeKind variants base (Rect/Ellipse/Line/Path) → confirmado.
- QuadTo/Close hits → expansão Path pós-P79; documentar.
- Polygon hits → re-classificar como implementado.
- Ellipse export usa `c` operator → real; emite rect TODO →
  placeholder per DEBT-31 nota.

### Bloco 3 — Path subsistema (DEBT-33)

```bash
# Path bbox cálculo
grep -rn "bounding_box\|bbox" 01_core/src/rules/layout/
grep -rn "bounding_box" 01_core/src/entities/

# DEBT-33 status
grep -A 20 "DEBT-33" 00_nucleo/DEBT.md
```

**Critério**: DEBT-33 EM ABERTO confirmar; aproximação por
min/max dos pontos de controlo preservada.

### Bloco 4 — Stroke (Paint, dash, caps)

```bash
# Stroke struct fields actuais
grep -A 10 "pub struct Stroke" 01_core/src/entities/geometry.rs

# Paint enum
grep -rn "enum Paint\|Paint::\|\bPaint\b" 01_core/src/entities/

# Dash patterns
grep -rn "DashPattern\|Dash::\|dash_pattern" 01_core/src/

# Line cap / line join / miter
grep -rn "LineCap\|LineJoin\|MiterLimit\|miter_limit" 01_core/src/

# Stroke<T> generic
grep -rn "Stroke<" 01_core/src/
```

**Critério**:
- Stroke struct campos: confirmar `paint: Color, thickness:
  f64` vs expansão pós-P252.
- Paint enum zero hits → ausente confirmado.
- Dash/LineCap/LineJoin zero hits → ausentes confirmados.
- Stroke<T> generic hits → expansão Length unit.

### Bloco 5 — Gradient subsistema (ausente esperado)

```bash
# Gradient ausente?
grep -rn "Gradient\|LinearGradient\|RadialGradient\|ConicGradient" \
  01_core/src/
grep -rn "GradientStop\|WeightedColor" 01_core/src/
grep "native_gradient\|gradient" 01_core/src/rules/stdlib/

# Vanilla Gradient enum
grep -n "Linear\|Radial\|Conic" lab/typst-original/crates/typst-library/src/visualize/gradient.rs 2>/dev/null | head -20
```

**Critério esperado**: zero hits cristalino → confirmar
ausência total. Vanilla file existe e tem 3 variants.

### Bloco 6 — Paint, Tiling subsistemas (ausente esperado)

```bash
grep -rn "enum Paint\|Tiling\|TilingPattern" 01_core/src/
grep "native_tiling\|tiling" 01_core/src/rules/stdlib/
```

**Critério esperado**: zero hits.

### Bloco 7 — Image subsistema (SVG + metadata)

```bash
# Variants Image
grep -A 10 "Content::Image\b" 01_core/src/entities/content.rs
grep -A 10 "FrameItem::Image" 01_core/src/entities/layout_types.rs

# SVG ausente?
grep -rn "Svg\|SVG\|usvg\|resvg\|svgtypes" 01_core/ 03_infra/
grep "svg\|usvg\|resvg" Cargo.toml */Cargo.toml

# Metadata alt/fit
grep -n "alt\|fit\|alt_text" 01_core/src/entities/content.rs
```

**Critério**:
- SVG zero hits → ausente confirmado.
- alt/fit zero hits → metadata ausente; ou hits → parcial.

### Bloco 8 — Transform `origin` pivot

```bash
grep -rn "origin\b" 01_core/src/rules/stdlib/transforms.rs
grep -A 8 "native_rotate\|native_scale\|native_skew" 01_core/src/rules/stdlib/transforms.rs
grep -n "pivot\|origin: " 01_core/src/entities/layout_types.rs
```

**Critério**: zero hits `origin` em transforms → confirmar
ausência cross-tipo.

### Bloco 9 — Inconsistências documentais

```bash
# L0 prompts existentes
ls -la 00_nucleo/prompts/entities/geometry.md
ls -la 00_nucleo/prompts/entities/color.md
ls -la 00_nucleo/prompts/entities/layout_types.md

# Conteúdo geometry.md vs código real
view 00_nucleo/prompts/entities/geometry.md

# Hash drift detection
grep "@prompt-hash" 01_core/src/entities/geometry.rs
grep "@prompt-hash" 01_core/src/entities/color.rs
grep "@prompt-hash" 01_core/src/entities/layout_types.rs

# Verificar hashes do prompt
grep "Hash do Código" 00_nucleo/prompts/entities/geometry.md
grep "Hash do Código" 00_nucleo/prompts/entities/color.md
```

**Esperado** (per precedente P255/P257/P258):
- `geometry.md` provavelmente lista apenas Stroke + ShapeKind
  base — desactualizado vs Path P79 + clip P79 + P252
  refactor.
- `color.md` actualizado P257 (`c120d66c`-like).
- Possíveis hash drifts.

### Bloco 10 — Exportador PDF visualize features

```bash
# Operators emitidos por shape
grep -n "fn emit_shape\|fn render_shape" 03_infra/src/export.rs
grep -n " m \\| l \\| c " 03_infra/src/export.rs | head -20

# Gradient PDF support (shading patterns)
grep -n "shading\|sh\\b\|axialShading\|radialShading" 03_infra/src/export.rs
```

**Critério**:
- Path emit usa `m`/`l`/`c` PDF operators (MoveTo/LineTo/
  CubicTo) → consumer real.
- Shading PDF support zero hits → confirma Gradient ausente
  no exporter (não materializável sem refactor exporter).

---

## Tabela de classificação Fase A (preencher após executar)

### Tabela A — Subsistemas Visualize

| # | Subsistema | Pré-audit | Audit P259 | Hits literais | Justificação |
|---|------------|-----------|------------|---------------|--------------|
| A | Color | 100% (P257) | _ | _ | _ |
| B.1 | Rect | implementado | _ | _ | _ |
| B.2 | Ellipse | parcial | _ | _ | _ |
| B.3 | Line | implementado | _ | _ | _ |
| B.4 | Path | implementado⁺ (DEBT-33) | _ | _ | _ |
| B.5 | Polygon | ausente | _ | _ | _ |
| B.6 | Curve | ausente | _ | _ | _ |
| C.1 | Stroke base | implementado | _ | _ | _ |
| C.2 | Stroke<T> | ausente | _ | _ | _ |
| C.3 | Dash | ausente | _ | _ | _ |
| C.4 | LineCap/Join | ausente | _ | _ | _ |
| C.5 | Paint enum | ausente | _ | _ | _ |
| D.1 | Image JPEG | implementado | _ | _ | _ |
| D.2 | Image PNG | implementado | _ | _ | _ |
| D.3 | Image SVG | ausente | _ | _ | _ |
| D.4 | Image metadata | parcial/ausente | _ | _ | _ |
| E.1 | Transform Move | implementado | _ | _ | _ |
| E.2 | Transform Rotate | implementado | _ | _ | _ |
| E.3 | Transform Scale | implementado | _ | _ | _ |
| E.4 | Transform Skew | implementado | _ | _ | _ |
| E.5 | Transform origin pivot | ausente | _ | _ | _ |
| F.1 | Gradient Linear | ausente | _ | _ | _ |
| F.2 | Gradient Radial | ausente | _ | _ | _ |
| F.3 | Gradient Conic | ausente | _ | _ | _ |
| G | Paint wrapper | ausente | _ | _ | _ |
| H | Tiling | ausente | _ | _ | _ |
| I | Clip | implementado | _ | _ | _ |

### Tabela B — Estado agregado

| Estado | Pré-P259 (estimado) | Audit P259 | Δ |
|--------|---------------------|------------|---|
| implementado | ~14/27 (52%) | _ | _ |
| implementado⁺ | 1/27 (4%) | _ | _ |
| parcial | 2/27 (7%) | _ | _ |
| ausente | 10/27 (37%) | _ | _ |
| TOTAL | 27 | _ | _ |
| Cobertura ponderada | ~60-65% (estimativa) | _% | _pp |

### Decisão cenário Fase B

☐ **B1** (≥75% — fecho conceptual).
☐ **B2** (55-70% — sub-passos prioritários).
☐ **B3** (≤50% — re-classificação primeiro).

---

## Templates Fase B por cenário

### Cenário B1 — Fecho conceptual

**Improvável**. Se materializar:

1. ADR-0083 anotação cumulativa "Visualize fecho conceptual
   cumulativo".
2. Actualizar L0 prompts obsoletos descobertos.
3. Relatório de fecho conceptual.

**Magnitude**: XS-S documental.

### Cenário B2 — Sub-passos prioritários

**Mais provável**. Opções não exclusivas:

#### Opção 1: Sequência minimal arquitectural (Paint + Gradient Linear)

**P260 — Paint enum**:
- Pré-requisitos: ADR-0083 anotação cumulativa OU ADR nova
  (decisão por magnitude). Prompt L0 novo
  `00_nucleo/prompts/entities/paint.md` com 3 variants
  Color/Gradient/Tiling.
- Materialização (testes primeiro):
  - `enum Paint { Color(Color), Gradient(Gradient), Tiling(Tiling) }`
    em `entities/paint.rs`. Inicialmente Gradient e Tiling
    podem ser unit variants placeholder (preserva enum shape
    enquanto subsistemas não materializam).
  - **Alternativa minimalista**: Paint só com Color variant
    inicialmente; Gradient/Tiling adicionados quando
    materializados. Menos overhead arquitectural.
- Adaptar Stroke.paint: Color → Paint; Style::Fill: Color →
  Paint (cross-cutting análogo P252).
- Magnitude: S+ (~1-2h; +8-12 tests).

**P261 — Gradient Linear**:
- Pré-requisitos: P260 Paint enum completo.
- Diagnóstico vanilla obrigatório per ADR-0029
  §"Diagnosticar primeiro": leitura literal
  `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`.
- Materialização (testes primeiro):
  - `enum Gradient { Linear { stops, angle, ... }, ... }`.
  - `struct GradientStop { offset: f64, color: Color }`.
  - Stdlib `native_gradient_linear(stops, angle)`.
  - PDF exporter: emit `/Pattern /Shading` com `sh` operator
    (axial shading).
- Magnitude: M (~3-4h; +15-20 tests).

**Cobertura cumulativa**: ~+11pp Visualize agregada.

#### Opção 2: Sequência shapes (Polygon + Ellipse refino)

**P260 — Polygon**:
- Pré-requisitos: L0 prompt `entities/geometry.md` actualizado.
- Materialização (testes primeiro):
  - `ShapeKind::Polygon { points: Arc<[Point]> }` ou similar.
  - Stdlib `native_polygon(points: array)`.
  - PDF exporter: emit Path com MoveTo + N LineTo + ClosePath.
- Magnitude: S+ (~1-2h; +5-8 tests).

**P261 — Ellipse real**:
- Pré-requisitos: L0 prompt actualizado.
- Materialização (testes primeiro):
  - PDF exporter: substituir rectângulo placeholder por 4
    arcos Bézier (constante mágica `0.5522847498`).
  - Geometry helper `ellipse_to_cubic_segments(rx, ry) ->
    [CubicSegment; 4]`.
- Magnitude: S (~1h; +3-5 tests).

**Cobertura cumulativa**: ~+6pp.

#### Opção 3: DEBT-33 + Stroke<T>

**P260 — DEBT-33 Bézier bbox exacto**:
- Materialização: cálculo analítico extremos B(t) (raízes da
  derivada `B'(t) = 0` em [0,1]).
- Magnitude: S+ (~1-2h; +5 tests; matemática paramétrica).
- DEBT-33 transita CLOSED.

**P261 — Stroke<Length>**:
- Pré-requisitos: L0 prompt `entities/geometry.md`
  actualizado; possível ADR para "Length unit em vez de
  f64".
- Materialização (testes primeiro):
  - `struct Stroke { paint, thickness: Length }` em vez de
    `f64`.
  - Adaptar consumers (geometry, exporter, layouter).
- Magnitude: M (~2-3h; +10-15 tests; refactor cross-cutting).

**Cobertura cumulativa**: ~+5pp; DEBT activo fechado.

#### Opção 4: Refinos qualitativos

- Transform `origin` pivot point (cross-tipo).
- Magnitude: S+ (~1h; +5 tests).

#### Opção 5: SVG image format (L+ adiar)

- Pré-requisitos: ADR nova `usvg + resvg` autorização (L3);
  prompt L0 novo.
- Magnitude: L+ (~6-10h; +20-30 tests).
- **Não recomendado para P260** — magnitude grande; deixar
  para passo dedicado pós-P259+P260+P261.

### Cenário B3 — Re-classificação primeiro

**Improvável**. Se materializar:

1. Re-classificar Tabela A conservadoramente.
2. Identificar entradas que foram superestimadas.
3. Sub-passos de elevação prioritários como B2.

---

## Notas metodológicas

1. **Honesty rule** (precedente P255/P257/P258): classificações
   Fase A literais (`grep` hits/no-hits), não interpretativas.

2. **Diagnóstico imutável**: ficheiro
   `diagnostico-visualize-fase-a-passo-259.md` em
   `00_nucleo/diagnosticos/` marcado "Imutável após criação per
   ADR-0034". Subpadrão N=4 (após P255/P257/P258).

3. **Pré-execução**: Claude Code deve **ler CLAUDE.md primeiro**
   (Regra de Ouro). Para qualquer materialização P260+, prompts
   L0 actualizados são pré-requisito.

4. **Sequência arquitectural preferida** (se B2):
   - **Paint primeiro** (P260) → **Gradient depois** (P261).
   - Razão: Gradient sem Paint wrapper consumer não tem
     consumidor real (Stroke.paint continua Color literal).

5. **Tempo estimado**:
   - Fase A audit: 30-45 min.
   - Cenário B1 fecho: 30-60 min documental.
   - Cenário B2 Opção 1 (Paint+Gradient): 4-6h total (2 passos).
   - Cenário B2 Opção 2 (shapes): 2-3h total.
   - Cenário B2 Opção 3 (DEBT-33 + Stroke<T>): 3-5h total.

# Diagnóstico Visualize — Passo 259 (preparatório)

**Data**: 2026-05-15
**Tipo**: passo arquitectural de diagnóstico (não materializa código)
**Estrutura**: duas fases registadas (precedente N=4 de
"auditoria condicional" P192A/P255/P257/P258):
- **Fase A** — inventário empírico das entradas Visualize;
  classificar cada uma como `implementado/implementado⁺/parcial/
  ausente/scope-out`.
- **Fase B** — decisão condicional sobre próximos passos
  baseada em cobertura real apurada.

**Análogo estrutural**: P254A (Introspection actualizado),
P254B+P255 (Math), P256+P258 (Model), P257 (Color paridade
vanilla).

**Motivação**: utilizador escolheu pivot para Visualize após
P258 fechar Model conceptualmente. Visualize tem múltiplos
subsistemas independentes — Color (P257 fechou 8/8 estructural)
+ shapes (P79) + paths (P79) + images (P73/74) + transforms
(P78+P156F) + Gradient/Paint/Tiling **ausentes** + refinos
qualitativos abertos (DEBT-33 Bézier bbox).

**Cobertura citada pré-P259**: ~54% pré-P257; pós-P257 sobe
alguns pp (Color subsistema 25% → 100% estrutural). Audit
empírico vai produzir número real.

---

## §1 — ADRs e DEBTs relevantes

### ADRs activos para Visualize

| ADR | Status | Relevância |
|-----|--------|------------|
| ADR-0019 | IMPLEMENTADO | `ttf-parser` + `rustybuzz` (toca imagens via TTF) |
| ADR-0026 + R1 | EM VIGOR | Content enum + `Arc<[T]>` em Sequence |
| ADR-0028 | REVOGADA | Color simplificado P25 (substituída por ADR-0083) |
| ADR-0029 | EM VIGOR | Pureza física L1 + diagnóstico vanilla + ADR explícita para scope-outs |
| ADR-0033 | EM VIGOR | Paridade observable vanilla |
| ADR-0034 | EM VIGOR | Diagnóstico canónico |
| ADR-0054 | EM VIGOR | Perfil graded |
| ADR-0065 | EM VIGOR | Inventariar primeiro |
| ADR-0083 | IMPLEMENTADO (P257) | Color paridade vanilla 8/8 espaços; 4 scope-outs documentados |

### DEBTs activos para Visualize

| DEBT | Status | Relevância |
|------|--------|------------|
| DEBT-33 | EM ABERTO (P79) | Bézier bbox exacto — `ShapeKind::Path` |
| DEBT-31 | ENCERRADO (P78) | Transformações afins (rotate/scale) |
| DEBT-30 | ENCERRADO (P79) | Clipping paths |
| DEBT-27 | ENCERRADO (P74) | PNG transparência |
| DEBT-29 | ENCERRADO (P74) | JPEG ColorSpace |

---

## §2 — Inventário declarado pré-P259 (não auditado)

Visualize agrega múltiplos subsistemas. Lista parcial baseada
em referências cumulativas cruzadas:

### Subsistema A — Color (P257 IMPLEMENTADO)

| Espaço | Estado | Origem |
|--------|--------|--------|
| Srgb (Rgb/Rgba) | implementado | P25 → P257 expansão |
| Luma | implementado | P25 alias → P257 variant nativo |
| LinearRgb | implementado | P257 |
| Oklab | implementado | P257 |
| Oklch | implementado | P257 |
| Hsl | implementado | P257 |
| Hsv | implementado | P257 |
| Cmyk | implementado | P257 |

**Cobertura Color**: **100% estrutural** (8/8 espaços
materializados; 4 scope-outs documentados em ADR-0083).

### Subsistema B — Shapes

| Forma | Estado | Origem |
|-------|--------|--------|
| Rect | implementado | P76 |
| Ellipse | **parcial** (placeholder retangular em export per DEBT-31 nota) | P76 |
| Line { dx, dy } | implementado | P76 |
| Path (CubicTo) | implementado⁺ (DEBT-33 bbox aproximado) | P79 |
| Polygon | **ausente** | — |
| Curve | **ausente** | — |

### Subsistema C — Stroke

| Aspecto | Estado | Origem |
|---------|--------|--------|
| `Stroke { paint, thickness }` base | implementado | P76 |
| Refactor cross-cutting (P252) | implementado | P252 |
| `Stroke<T>` (Length unit em vez de f64) | **ausente** | ADR-0029 §enumeração |
| Dash patterns | **ausente** | — |
| Line cap / line join / miter limit | **ausente** | — |
| Paint enum (Color | Gradient | Tiling) | **ausente** | ADR-0029 §enumeração |

### Subsistema D — Image

| Aspecto | Estado | Origem |
|---------|--------|--------|
| JPEG (`/DCTDecode`) | implementado | P73 |
| PNG (RGBA + `/SMask`) | implementado | P74 |
| JPEG ColorSpace detection | implementado | P74 |
| Deduplicação por `Arc::as_ptr` | implementado | P73 |
| Dimensões reais via `imagesize` | implementado | P72 |
| SVG | **ausente** | — |
| Refino metadata (`alt`, `fit`) | **ausente** ou parcial | — |

### Subsistema E — Transform

| Tipo | Estado | Origem |
|------|--------|--------|
| Move | implementado | P78 |
| Rotate | implementado | P78 |
| Scale | implementado | P78 |
| Skew | implementado | P156F |
| `origin` pivot point (cross-tipo) | **ausente** | — |

### Subsistema F — Gradient (ADR-0029 §enumeração)

| Tipo | Estado | Notas |
|------|--------|-------|
| Linear | **ausente** | — |
| Radial | **ausente** | — |
| Conic | **ausente** | — |
| GradientStop sub-componente | **ausente** | per ADR-0029 §exclusões — sub-componente |

### Subsistema G — Paint enum

| Aspecto | Estado |
|---------|--------|
| `enum Paint { Color, Gradient, Tiling }` | **ausente** |
| Stroke uses Paint instead of Color directly | **ausente** |
| Fill uses Paint instead of Color directly | **ausente** |

### Subsistema H — Tiling

| Aspecto | Estado |
|---------|--------|
| Tiling pattern (vanilla feature) | **ausente** |

### Subsistema I — Clip

| Aspecto | Estado | Origem |
|---------|--------|--------|
| `FrameItem::Group.clip_mask: Option<ShapeKind>` | implementado | P79 |
| PDF `W n` operator | implementado | P79 |
| q/Q state push/pop | implementado | P79 |

### Cobertura agregada pré-P259 (estimativa)

| Subsistema | Implementado | Pendente |
|------------|-------------|----------|
| Color | 100% estrutural | 0 espaços (4 scope-outs ADR-0083) |
| Shapes | 4/6 ≈ 67% | Polygon, Curve, refino Ellipse |
| Stroke | base + cross-cutting | refinos Stroke<T>, dash, caps, joins, Paint |
| Image | 2/3 formatos | SVG, refinos metadata |
| Transform | 4/4 tipos + sem origin | origin pivot |
| **Gradient** | **0/3 tipos** | Linear/Radial/Conic |
| **Paint** | **ausente** | enum wrapper |
| **Tiling** | **ausente** | pattern |
| Clip | 100% | — |

**Cobertura agregada estimativa**: ~60-65% (sobe de ~54%
citada pré-P257 via Color expansion). Audit empírico
produzirá número real.

---

## §3 — Fase A: Inventário empírico (a executar)

Análogo a P255/P257/P258 §1. Comandos `grep`/`view` que
produzem evidência factual antes de decisão.

### Bloco 1 — Color subsistema (P257 confirmar)

```bash
# Variant Color confirmar 8/8
grep -c "^\s*[A-Z][a-zA-Z]*\s*{" 01_core/src/entities/color.rs
grep -n "^\s*[A-Z][a-zA-Z]*\s*{" 01_core/src/entities/color.rs

# Stdlib funcs registadas
grep -rn "native_rgb\|native_luma\|native_oklab\|native_oklch\|native_hsl\|native_hsv\|native_cmyk\|native_linear_rgb" \
  01_core/src/rules/stdlib/
```

**Esperado**: 8 variants Color; 7+ stdlib funcs (confirmar
P257 relatório §4).

### Bloco 2 — Shapes subsistema

```bash
# ShapeKind variants
grep -n "^\s*[A-Z]" 01_core/src/entities/geometry.rs
grep -n "ShapeKind::" 01_core/src/rules/stdlib/shapes.rs

# Polygon / Curve ausentes?
grep -rn "Polygon\|ShapeKind::Polygon" 01_core/src/

# Ellipse real (vs placeholder rect)
grep -n "ellipse\|Ellipse" 03_infra/src/export.rs
```

**Critério**:
- Polygon hits → re-classificar como implementado.
- Ellipse export usa `c` operator (Bézier 4 arcs) → real;
  emite rect TODO → placeholder.

### Bloco 3 — Path subsistema (DEBT-33)

```bash
# Path variants (MoveTo/LineTo/CubicTo/QuadTo/Close)
grep -n "MoveTo\|LineTo\|CubicTo\|QuadTo\|Close" 01_core/src/entities/

# Bounding box exacto vs aproximado
grep -n "bounding_box\|bbox" 01_core/src/rules/layout/
grep -n "DEBT-33" 00_nucleo/DEBT.md
```

**Critério**:
- QuadTo/Close hits → expansão pós-P79.
- DEBT-33 status atual.

### Bloco 4 — Stroke (Paint enum, dash, caps)

```bash
# Stroke struct fields
grep -A 5 "pub struct Stroke" 01_core/src/entities/geometry.rs

# Paint enum
grep -rn "enum Paint\|Paint::" 01_core/src/entities/

# Dash / LineCap / LineJoin
grep -rn "Dash\|LineCap\|LineJoin\|MiterLimit" 01_core/src/
```

**Critério**:
- Stroke campos extra → refactor pós-P252.
- Paint enum hits → implementado.
- Dash/Cap/Join hits → expandido.

### Bloco 5 — Gradient subsistema (ausente esperado)

```bash
# Gradient ausente?
grep -rn "Gradient\|LinearGradient\|RadialGradient\|ConicGradient" 01_core/src/
grep -rn "GradientStop\|WeightedColor" 01_core/src/
grep "native_gradient\|gradient" 01_core/src/rules/stdlib/
```

**Critério esperado**: zero hits → confirmar ausência.

### Bloco 6 — Paint, Tiling subsistemas (ausente esperado)

```bash
grep -rn "enum Paint\|Tiling\|TilingPattern" 01_core/src/
grep "native_tiling\|tiling" 01_core/src/rules/stdlib/
```

**Critério esperado**: zero hits.

### Bloco 7 — Image subsistema (SVG)

```bash
# SVG ausente?
grep -rn "Svg\|SVG\|usvg\|resvg" 01_core/ 03_infra/
grep "svg" Cargo.toml */Cargo.toml

# Image variants metadata
grep -A 10 "Content::Image\|FrameItem::Image" 01_core/src/entities/
```

### Bloco 8 — Transform `origin`

```bash
grep -rn "origin\b.*Point\|pivot\|origin: " 01_core/src/rules/stdlib/transforms.rs
grep -A 5 "native_rotate\|native_scale\|native_skew" 01_core/src/rules/stdlib/transforms.rs
```

**Critério**: zero hits `origin` → confirmar ausência.

### Bloco 9 — Inconsistências documentais

```bash
# L0 prompts vs código real
view 00_nucleo/prompts/entities/geometry.md | head -50
ls -la 00_nucleo/prompts/entities/geometry.md
ls -la 00_nucleo/prompts/entities/color.md

# Hash drift
grep "@prompt-hash" 01_core/src/entities/geometry.rs
grep "@prompt-hash" 01_core/src/entities/color.rs
```

**Esperado** (per precedente P255/P257/P258):
- `geometry.md` provavelmente lista apenas Rect/Ellipse/Line
  base — desactualizado vs Path P79.
- `color.md` actualizado P257 (`c120d66c`-like).

---

## §4 — Cenários Fase B

### Cenário B1: Cobertura ≥75% confirmada (improvável)

Implicaria que múltiplos subsistemas Gradient/Paint/Tiling
foram materializados fora do que vejo no contexto. Pouco
provável.

Acção: relatório de fecho conceptual Visualize análogo a P258.
ADR-0083 anotação cumulativa.

### Cenário B2: Cobertura 55-70% (provável)

Cenário **mais provável**. Reflecte:
- Color expandido P257 (8/8 estructural).
- Shapes/Stroke/Image/Transform com bases sólidas.
- Gradient/Paint/Tiling ausentes — gap real.
- Ellipse refino e DEBT-33 candidatos.

Acção: documentar estado factual; identificar 1-3 sub-passos
prioritários:
- **Opção 1**: Gradient Linear materialização (M; ~+15-20
  tests; +8pp).
- **Opção 2**: Paint enum + adaptar Stroke/Fill (S+; ~+10
  tests; +3pp). Pré-requisito para Gradient real consumer.
- **Opção 3**: Stroke<Length> + dash + caps (M; ~+15 tests;
  +5pp).
- **Opção 4**: Polygon shape (S+; ~+5 tests; +3pp).
- **Opção 5**: Ellipse real (Bézier 4 arcs em PDF; S; ~+3
  tests; refino qualitativo).
- **Opção 6**: SVG image format (L+; depende crate `usvg` +
  ADR autorização; ~+15-20 tests; +5pp).
- **Opção 7**: DEBT-33 Bézier bbox exacto (S+; matemática
  paramétrica; ~+5 tests; refino qualitativo).

### Cenário B3: Cobertura ≤50% (regressão)

Implicaria que classificações declaradas estavam optimistas.
Improvável dado Color P257 confirmado.

Acção: re-classificação conservadora; sub-passos de elevação
prioritários.

---

## §5 — Recomendação concreta

### Recomendação primária

**P259-aud — Fase A audit empírico** (XS; ~30 min de leitura
de código). Output: diagnóstico imutável análogo a
`diagnostico-math-fase-a-passo-255.md` /
`diagnostico-color-vanilla-passo-257.md` /
`diagnostico-model-fase-a-passo-258.md` com classificação
literal de cada subsistema.

### Recomendação secundária (pós-audit)

Depende do cenário Fase B confirmado:

- **B1 raro**: fecho conceptual; passar a outro módulo.
- **B2 provável**:

#### Sub-recomendação 1: Sequência minimal (Paint + Gradient Linear)

1. **P260 — Paint enum** (S+ pré-requisito; ADR nova ou
   anotação ADR-0083).
2. **P261 — Gradient Linear** (M).
3. Cobertura cumulativa: **~+11pp**.

Vantagem: desbloqueia consumer chain Gradient → Paint → Stroke/Fill.

#### Sub-recomendação 2: Sequência shapes (Polygon + Ellipse refino)

1. **P260 — Polygon** (S+).
2. **P261 — Ellipse real** (S; refino).
3. Cobertura cumulativa: **~+6pp**.

Vantagem: trabalho granular puro sem dependências cruzadas.

#### Sub-recomendação 3: DEBT-33 + Stroke<T>

1. **P260 — DEBT-33 Bézier bbox** (S+; matemática
   paramétrica).
2. **P261 — Stroke<Length>** (M; toca Stroke struct).
3. Cobertura cumulativa: **~+5pp**; refinos qualitativos.

Vantagem: fecha DEBT activo; refino observable.

### Recomendação terciária

**SVG image format** (L+; ADR crate `usvg` + `resvg` ou
similar). Magnitude grande; não recomendado neste passo.

### Não recomendado

- Atacar **Tiling pattern** sem audit prévio confirmar
  prioridade.
- Materializar **Gradient Linear/Radial/Conic** todos juntos
  sem Paint enum primeiro (sequência arquitectural quebrada).

---

## §6 — Padrões metodológicos aplicados

### ADR-0065 critério #5 — scope determinado por inventário

Aplicação directa. Este passo é diagnóstico-de-diagnóstico
análogo a P256/P254B.

### Subpadrão "auditoria condicional" N=4 → N=5

Cumulativo:
- N=1 P192A (M7 fixpoint).
- N=2 P255 (DEBT-8 Math).
- N=3 P257 (Color Fase A vanilla).
- N=4 P258 (Model Fase A).
- **N=5 P259** (Visualize Fase A; este passo recomenda).

**Patamar N=5 atinge limiar formalização clara**. Decisão de
formalizar pode ser tomada em passo administrativo XS
dedicado (análogo P156K para ADR-0064/0065).

### Subpadrão "diagnóstico imutável precedente à acção"

Cumulativo:
- N=1 P255.
- N=2 P257.
- N=3 P258.
- **N=4 P259** (se Fase A materializar).

**Patamar N=4 reforça pattern**.

### Política "sem novas reservas"

Preservada. Recomendações §5 são para validação humana, não
compromissos.

---

## §7 — Limitações deste diagnóstico

1. **Cobertura agregada Visualize não auditada empíricamente**
   — estimativa ~60-65% pós-P257 baseada em referências
   cruzadas; audit empírico produzirá número real.

2. **Subsistemas Gradient/Paint/Tiling assumidos ausentes**
   — sem evidência directa de materialização no contexto.
   Audit Fase A vai confirmar.

3. **Ellipse refino estado real incerto** — DEBT-31 nota
   referenciou "rectângulo placeholder" mas pode ter sido
   materializado em passo intermediário não auditado.

4. **SVG status incerto** — sem evidência de materialização;
   considerado ausente.

5. **Cross-references Visualize ↔ Layout** não auditadas —
   Ellipse refino e Bézier bbox afectam Layouter
   (`measure_content_constrained` / placement).

---

## §8 — Referências

- ADR-0019, ADR-0026, ADR-0029, ADR-0033, ADR-0034,
  ADR-0054, ADR-0065, ADR-0083.
- DEBT-33 (Bézier bbox aberto).
- DEBT-31, DEBT-30, DEBT-27, DEBT-29 (encerrados —
  contexto histórico Visualize).
- P25 — Color simplificado original (REVOGADO).
- P72-P74 — Image stack (JPEG + PNG + dimensões reais).
- P76 — geometry tipos primitivos (Stroke + ShapeKind).
- P78 — Transform (Move/Rotate/Scale).
- P79 — Path + clip + DEBT-30/33.
- P156F — Skew.
- P252 — Refactor cross-cutting Stroke (precedente N=1
  "Refactor cross-cutting entity primitivo").
- P257 — Color paridade vanilla 8/8 (precedente N=2 mesmo
  subpadrão).
- P254A — precedente "actualização cumulativa de módulo".
- P255 — DEBT-8 Math ENCERRADO (subpadrão "auditoria
  condicional" N=2).
- P258 — Model fecho conceptual cumulativo (subpadrão N=4).
- ADR-0029 §"Sobre os tipos tipográficos vanilla" §enumeração
  — fonte canónica das pendências Visualize.

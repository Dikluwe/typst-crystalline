# Diagnóstico Fase A2 — P282 Estado percentual do projecto

**Data**: 2026-05-18
**Tipo**: ADR-0085 diagnóstico imutável; **37º consumo**.
**Método**: empírico (grep estructural cristalino vs vanilla
`lab/typst-original/crates/typst-library/`); NÃO declarativo.

---

## §A2.1 — Inventário factual cross-codebase

### Cristalino (`01_core/`)

- **`Content` enum variants**: **61** distintos
  (de `Text` a `CounterDisplayCallback`).
- **Stdlib `native_*` functions**: **88** distintos (estimativa
  inferior; exclui testes).
- **Stdlib categorias**: foundations, calc, gradients, shapes,
  transforms, layout, text, figure_image, structural.

### Vanilla (`lab/typst-original/crates/typst-library/`)

- **`#[elem]` definitions**: **168** distintos.
- **`#[func]` definitions**: **159** distintos.
- Distribuição `#[elem]` por categoria:

| Categoria | Vanilla #[elem] |
|---|---|
| model | 54 |
| layout | 33 |
| math | 29 |
| visualize | 18 |
| text | 14 |
| introspection | 5 |
| (foundations) | 15 |

- Vanilla calc functions: **41** (#[func]).
- Cristalino calc functions: **9** (`calc_abs/ceil/clamp/floor/max/min/pow/round/sqrt`).

---

## §A2.2 — Cobertura por categoria

### Model — ~55% cobertura

**Cristalino tem** (12+ variants em `Content`):
- `Heading { level, body }` ✓
- `Raw { ... }` ✓
- `ListItem(Box<Content>)` ✓
- `EnumItem { number, body }` ✓
- `Link { url, body }` ✓
- `Figure { ... }` + `SetFigureNumbering { pattern }` ✓
- `Image { ... }` ✓ (compartilhado com visualize)
- `Ref { ... }` ✓ + `Labelled` ✓
- `Bibliography { ... }` (parcial — DEBT-55) ✓
- `Cite { ... }` (parcial — DEBT-55) ✓
- `Table/TableHeader/TableCell/TableFooter` ✓
- `Quote { ... }` ✓
- `Terms/TermItem` ✓
- `Metadata` ✓ (P150)
- `Divider` ✓
- `SetHeadingNumbering` ✓ + `SetEquationNumbering` ✓

**Cristalino MISSING (vanilla `model/`)**:
- ❌ `Footnote` — sem variant ou stdlib fn.
- ❌ `Outline` — sem variant ou stdlib fn.
- ❌ `Title` — sem variant.
- ❌ `Par` (paragraph as element) — text fluído mas sem elem distinto.
- ❌ `Emph` / `Strong` como **elementos** (existem como `native_emph` /
  `native_strong` mas não como variants em `Content`; presumivelmente
  via `Styled(Box, Styles)` wrapper).
- ❌ `Footnote` numbering / display.

**% cobertura**: 12 features core implementadas / ~20 features
vanilla = **~60%**. Hayagriva externo (bibliography backend) é
bloqueador parcial (ADR-0062 — DEBT-55 aguarda passo dedicado).

**Próximo accionável**: Footnote (M-magnitude; requer
introspection + layout reservation).

### Layout — ~70% cobertura

**Cristalino tem** (18 variants):
- `Shape` ✓
- `Transform` ✓ + `native_move/rotate/scale/skew` ✓
- `Grid/GridHeader/GridFooter/GridCell` ✓ (cluster completo P273)
- `SetPage` ✓
- `Align` ✓ + `native_align` ✓
- `Place` ✓
- `Pad` ✓ + `native_pad` ✓
- `Hide` ✓ + `native_hide` ✓
- `HSpace` / `VSpace` ✓ + `native_h` / `native_v` ✓
- `Pagebreak` ✓ + `native_pagebreak` ✓
- `Colbreak` ✓ + `native_colbreak` ✓
- `Stack` ✓ + `native_stack` ✓
- `Boxed` ✓ + `native_box` ✓
- `Block` ✓ + `native_block` ✓
- `Repeat` ✓ + `native_repeat` ✓
- `Columns` ✓ + `native_columns` ✓ (parcial — full multi-region
  bloqueado por ADR-0078 / ex-DEBT-56 — Fase 3 multi-region pendente)

**Cristalino MISSING**:
- ❌ Multi-region column flow real (encerrado P221 com nota
  arquitectural; ADR-0078 documenta).
- ❌ `Container` (compositional wrapper similar a Boxed mas com
  semântica diferente).
- ❌ Measure/fragment APIs avançadas (DEBT-pending).
- Layout primitives auxiliares: `axes`, `corners`, `sides` — existem
  estructuralmente mas não como elems user-facing.

**% cobertura**: 18 features core / ~25 features vanilla = **~72%**.

**Próximo accionável**: Multi-region column flow (M+ — Fase 3
ADR-0078); `Footnote` precisa de layout integration.

### Visualize — ~50% cobertura

**Cristalino tem**:
- `Shape` (Rect/Ellipse/Line variant via `ShapeKind` + RoundedRect
  P242 + Path P277) ✓
- Color types: `Color::rgb/luma/cmyk/oklab/oklch/hsl/hsv/linear_rgb` ✓
  (`native_rgb/luma/cmyk/oklab/oklch/hsl/hsv/linear_rgb` 8 stdlib fns)
- Gradient: Linear ✓ (P263) + Radial ✓ (P265) + Conic ✓ (P268+)
- Stroke (field em Shape) ✓
- Path com bbox analítica (P277) ✓

**Cristalino MISSING**:
- ❌ `Curve` (vanilla `curve.rs`: Move/Line/Quad/Cubic/Close
  imperativos; DEBT pendente — ADR-0078 §sub-fase b).
- ❌ `Tiling` (pattern fill via tiling; vanilla `tiling.rs`).
- ❌ `Polygon` como elem distinto (existe `native_polygon` mas
  emit interno via Path P277).
- ❌ Pattern stroke (apenas pattern fill emit; P273 cluster
  só cobre stroke gradients).
- ❌ Transparency vectorial (alpha em `rg`/`RG`; ca/CA operators PDF 1.4).
- ❌ Image transformations (rotate/clip em imagens — emit P273.13 cobre Group + clip mas Image individual sem clip).

**% cobertura**: 6 features / ~12 features vanilla = **~50%**.

**Próximo accionável**: Curve (S-M — geometry primitive base) +
Tiling (M — requer pattern dict expandido).

### Math — ~40% cobertura

**Cristalino tem**:
- `Equation` + `SetEquationNumbering` ✓
- `MathSequence` ✓
- `MathIdent` ✓ + `MathText` ✓
- `MathFrac` ✓
- `MathAttach` (super/subscript) ✓
- `MathRoot` ✓
- `MathDelimited` ✓
- `MathMatrix` ✓
- `MathCases` ✓
- Math symbols module (P40 — DEBT-8 fechado P255) ✓

**Cristalino MISSING** (vanilla `math/`):
- ❌ `Accent` (mathematical accents — circumflex, tilde, vec, etc.).
- ❌ `Cancel` (strike-through cancellation).
- ❌ `Op` (mathematical operators: lim, sin, cos, etc.; DEBT-pending).
- ❌ `Lr` (left/right auto-sized delimiters — diferente de
  `MathDelimited` que é manual).
- ❌ `Underover` (underbrace/overbrace).
- ❌ `Style` (math style variant — display/inline/script/scriptscript).

**% cobertura**: 9/15+ math features = **~40-60%**. Math é
intrinsecamente complexa; cobertura "satisfatória" para casos comuns
(equação inline + fração + matriz + cases) mas falta refino para
casos avançados (notação científica completa).

**Próximo accionável**: Accent + Cancel (XS-S cada); Op (M com
sistema de operadores nomeados); Lr (S — auto-sized delimiters
detectam contexto).

### Text — ~30% cobertura

**Cristalino tem**:
- `Text(EcoString, TextStyle)` ✓ — variant principal
- `TextStyle { size, font, bold, italic, tracking, ... }` ✓
- `native_emph` / `native_strong` (via Styled wrapper) ✓
- `native_lower` / `native_upper` (case via Styled?) ✓
- `native_raw` ✓
- Faux-bold (P139) ✓
- Tracking (P137) ✓
- Lang/lyfen (P101+) parcial

**Cristalino MISSING** (vanilla `text/`):
- ❌ `Underline` — DEBT-pending.
- ❌ `Overline` — DEBT-pending.
- ❌ `Strikethrough` — DEBT-pending.
- ❌ `SmartQuote` — typographic smart quotes.
- ❌ `Subscript` / `Superscript` (text mode, não math).
- ❌ `SmallCaps` — small capitals variant.
- ❌ `Linebreak` como elem (\ explícito).
- ❌ `Space` como elem (space character as element).
- ❌ `Lorem` — stdlib placeholder text generator.
- ❌ Bold/italic CIDFont real (DEBT — current usa apenas /F1 em
  CIDFont; type1 tem faux-bold).
- ❌ Hyphenation completa (parcial em P101+).

**% cobertura**: 4-5 features / ~14 features vanilla = **~30%**.
Text é provavelmente a categoria mais sub-implementada após Math.

**Próximo accionável**: Underline/Overline/Strikethrough (XS each
— P-text-deco-emit em L3 + style flags em L1); SmartQuote
(S — typographic pre-processing).

### Introspection — ~70% cobertura

**Cristalino tem**:
- `Labelled { label, ... }` ✓
- `Ref { ... }` ✓
- `CounterDisplay`, `CounterUpdate`, `native_counter_at`,
  `native_counter_final`, `native_counter_display`,
  `native_counter_step` ✓
- `State`, `StateUpdate`, `StateDisplay`, `native_state`,
  `native_state_at`, `native_state_final` ✓
- `native_locate`, `native_here`, `native_query`, `native_measure`,
  `native_replace` ✓
- `Metadata` (P150) ✓
- Fixpoint convergence (P176-177) ✓
- Locatable trait (introspection L1 puro) ✓
- Tag extraction (P150+) ✓

**Cristalino MISSING**:
- ❌ `Outline` (depende de Heading detection + numbering — DEBT-pending).
- ❌ `Footnote` introspection (depende de Footnote elem).
- ❌ `Title` introspection.
- ❌ Cross-document linking (out-of-scope; vanilla também parcial).

**% cobertura**: 10/13+ features vanilla = **~70%**. Introspection
é uma das categorias mais robustas em cristalino (cluster cumprido
em passos 150-180 + fixpoint convergence).

**Próximo accionável**: Outline (M — requer Heading scan + numbering);
Footnote integration.

### Export (PDF) — ~85% cobertura

**Cristalino tem** (`03_infra/src/export.rs`):
- Helvetica Type1 path ✓
- CIDFont Identity-H path (ADR-0027) ✓
- Multifont path (P146) ✓
- Image XObject (JPEG P73 + PNG P74+) ✓
- Gradient patterns Linear (P263) + Radial (P265) + Conic (P268+) ✓
- Shape paths (Rect, Ellipse, Line, RoundedRect P242, Path P277) ✓
- Group + clip_mask (P79+P273) ✓
- Render real Groups N=3 (Shape+Image+Text/Glyph/Line — P281) ✓
- Faux-bold P139 ✓
- Tracking P137 ✓
- Unified pipeline `PageContext` (P281) ✓
- Walkers recursivos em Group (P273.10 / P279 / P280) ✓

**Cristalino MISSING**:
- ❌ Stroke colour (`RG`) em `FrameItem::Line` (limitação simétrica
  P282 §A1.3 — pendência `P-line-color-rg-emit`).
- ❌ Font subsetting (ADR-0027 Opção A: fonte completa embebida).
- ❌ Bold/italic CIDFont real (apenas /F1 em CIDFont; type1 tem faux-bold).
- ❌ PDF transparency (`ca`/`CA` PDF 1.4).
- ❌ Tagged PDF (acessibilidade).
- ❌ PDF/A (variant arquivística).
- ❌ Pattern fill (apenas pattern stroke — P263).

**% cobertura**: 11 features core / ~13 features vanilla = **~85%**.
Export é a categoria mais avançada — cluster Gradient + Group emit
fechado em P281.

**Próximo accionável**: `RG` em Line (S — requer mudança L1 +
emit); transparency vectorial (M — requer expansion do model
de cor + Page resources).

### Stdlib — ~50% cobertura

**Cristalino tem**: 88 `native_*` funções (alguns dos quais são
agrupadores test-name; reais ~50-60).

**Vanilla tem**: 159 `#[func]` + 41 calc fns = ~200 user-facing fns.

**% cobertura calc**: 9/41 = **~22%** (trig, hyperbolic, decimal,
fact, gcd/lcm, log/ln/exp, fract, round-mode, etc. todos pendentes).

**% cobertura geral stdlib**: ~50%.

**Próximo accionável**: Calc trigonometric (XS-S — sin/cos/tan via
f64::sin/cos/tan); Calc hyperbolic; Calc log/exp.

### Eval — ~80% cobertura

**Cristalino tem**:
- Expr → Value evaluation ✓
- Closure capture (P31) ✓ — eager snapshot (DEBT-2 documentado)
- Show rule (P-many; DEBT-50 parcial) ✓
- Set rule ✓
- Module imports ✓
- Field access / method calls ✓
- Pattern matching ✓
- Bindings (let / let-mut) ✓

**Cristalino MISSING**:
- ❌ Lazy closure capture (DEBT-2 — `comemo` integration adiada).
- ❌ Show selector completo (DEBT-50 parcial).

**% cobertura**: ~80%.

### CLI / Wiring — ~75% cobertura

**Cristalino tem**:
- `typst compile` CLI ✓
- Source loading com early-hash (ADR-0031 — P+) ✓
- World trait + injection ✓
- Engine<'a> (ADR-0044) ✓
- Font loading + matching ✓
- Sink<'a> diagnostics (P106+) ✓

**Missing**:
- ❌ `typst watch` ou modo incremental real.
- ❌ Package management (`typst init` etc.).
- ❌ Compile flags avançados (output formats além PDF).

**% cobertura**: ~75%.

---

## §A2.3 — Síntese percentual

| Categoria | Cobertura empírica | Direcção principal |
|---|---|---|
| Model | ~60% | Footnote (M); Outline (M); Title (XS); Par-as-elem (?) |
| Layout | ~72% | Multi-region columns (M+; bloqueado ADR-0078 sub-fase b); Container |
| Visualize | ~50% | **Curve** (S-M; DEBT-pending); Tiling (M); Transparency (M+); polygon-as-elem |
| Math | ~40% | Accent (XS); Cancel (XS); Op (M); Lr auto-delimiters (S); Underover (S); Style |
| Text | ~30% | **Underline/Overline/Strikethrough** (XS each); SmartQuote (S); Sub/Superscript (S); SmallCaps (S); CIDFont bold/italic real (M) |
| Introspection | ~70% | Outline (M); Footnote integration (depende Footnote elem) |
| Export | ~85% | `RG` em Line (S); transparency vectorial (M); font subsetting (M+) |
| Stdlib | ~50% | Calc trig+hyperbolic+log (~20 fns; XS each); demais funções foundations |
| Eval | ~80% | DEBT-2 lazy capture (S+M; comemo integration); DEBT-50 Show selector |
| CLI | ~75% | typst watch incremental (M+); package management (M+) |

**Cobertura agregada estimada**: ~63% (média ponderada).

**Insights**:
- **Categoria mais robusta**: Export (~85%; cluster Gradient + Group
  emit fechado em P281).
- **Categoria mais sub-implementada**: Text (~30%; cobertura cosmética
  básica mas falta decorações típicas + bold/italic real).
- **Math é uma frente concentrada**: 6 features XS-M cada permitiriam
  saltar 40% → 75%.
- **Stdlib calc tem ROI alto**: XS por função, ~20 funções, todas
  triviais (`f64::sin/cos/...`).

---

## §A2.4 — Top 10 frentes de trabalho accionáveis

Ordenadas por valor user-facing × ausência de bloqueador × continuidade
com trabalho recente.

### Rank 1 — Stdlib calc trigonometric/hyperbolic/log
- **Magnitude**: XS × ~20 fns.
- **Valor**: user-facing alto (qualquer documento com cálculo).
- **Bloqueador**: nenhum — `f64::sin/cos/tan/...` directo em L1.
- **Cobertura**: salta calc 22% → 70%.
- **Próximo passo**: `P-stdlib-calc-trig` (XS).

### Rank 2 — Curve (geometry primitive)
- **Magnitude**: S-M.
- **Valor**: alto (desbloqueia desenho avançado, ADR-0078 §sub-fase b).
- **Bloqueador**: nenhum — ADR-0078 já planeia.
- **Cobertura**: Visualize 50% → 60%; desbloqueia DEBT-pendente.
- **Próximo passo**: `P-curve-geometry` (S-M).

### Rank 3 — Text decorações (Underline/Overline/Strikethrough)
- **Magnitude**: XS each (~XS×3).
- **Valor**: alto (formatação básica esperada).
- **Bloqueador**: nenhum — requer apenas style flags + L3 emit
  (linhas em paralelo ao baseline).
- **Cobertura**: Text 30% → 45%.
- **Próximo passo**: `P-text-deco-emit` (S — combinação dos 3).

### Rank 4 — Math Accent + Cancel
- **Magnitude**: XS × 2.
- **Valor**: médio (math avançado).
- **Bloqueador**: nenhum.
- **Cobertura**: Math 40% → 50%.
- **Próximo passo**: `P-math-accent-cancel` (XS+S).

### Rank 5 — SmartQuote
- **Magnitude**: S.
- **Valor**: médio (qualidade tipográfica).
- **Bloqueador**: nenhum — pre-processing de Text em layout.
- **Cobertura**: Text 30% → 40%.
- **Próximo passo**: `P-smartquote` (S).

### Rank 6 — RG em Line (paridade emit P282 §A1.3)
- **Magnitude**: S (L1 + L3).
- **Valor**: médio (cor de linhas user-facing).
- **Bloqueador**: nenhum.
- **Cobertura**: Export 85% → 87%.
- **Próximo passo**: `P-line-color-rg-emit` (S).

### Rank 7 — Footnote
- **Magnitude**: M.
- **Valor**: alto (feature canónica de documentos académicos).
- **Bloqueador**: requer Layout reservation + Introspection numbering.
- **Cobertura**: Model 60% → 70%; Introspection +5%.
- **Próximo passo**: `P-footnote-cluster` (M).

### Rank 8 — Outline
- **Magnitude**: M.
- **Valor**: alto (TOC para documentos longos).
- **Bloqueador**: depende de Heading scan + numbering (ambos têm).
- **Cobertura**: Model +5%; Introspection 70% → 80%.
- **Próximo passo**: `P-outline-cluster` (M).

### Rank 9 — Math Op (operadores nomeados) + Lr auto-delimiters
- **Magnitude**: M + S.
- **Valor**: médio-alto (math user-friendly).
- **Bloqueador**: nenhum.
- **Cobertura**: Math 40% → 60%.
- **Próximo passo**: `P-math-op-lr` (M).

### Rank 10 — Multi-region column flow (Fase 3 ADR-0078)
- **Magnitude**: M+.
- **Valor**: alto (refluxo de páginas longas).
- **Bloqueador**: ex-DEBT-56 (encerrado P221 com nota arquitectural;
  ADR-0078 §sub-fase b documenta).
- **Cobertura**: Layout 72% → 85%.
- **Próximo passo**: `P-columns-multi-region` (M+).

---

## §A2.5 — Cobertura agregada e direcção próxima

**Estimativa global**: cristalino implementa **~63%** das features
vanilla user-facing. O resto distribui-se entre:

- **Quick wins** (XS-S): Text decorações, Math accent/cancel, calc
  trig/hyperbolic/log, RG em Line, SmartQuote. **~15% cobertura
  ganha por ~25 passos XS-S.**
- **M passos consolidatórios**: Curve, Outline, Footnote, Math Op,
  Tiling. **~10% cobertura ganha por 5 passos M.**
- **Bloqueado ou M+**: Multi-region columns, Bold/italic CIDFont
  real, Hayagriva integration, package management, typst watch.
  **~12% cobertura latent.**

**Direcção recomendada P283+**:

Opção **horizontal** (rampa de cobertura): bater stdlib calc + text
decorações + math accent/cancel em série XS-S → ganha ~15% cobertura
em ~8-10 passos de baixa magnitude.

Opção **vertical** (frente concentrada): atacar Curve + Footnote +
Outline em série M → ganha ~10% mas desbloqueia features de alto valor.

Opção **híbrida**: 1-2 quick wins (calc trig + text deco) seguidos
de Curve ou Footnote.

Decisão humana.

---

## §A2.6 — Conclusão Fase A2

- **Cobertura agregada**: ~63%.
- **Categoria mais robusta**: Export (85%; cluster fechado P281).
- **Categoria mais sub-implementada**: Text (30%; quick wins
  disponíveis).
- **Direcções desbloqueadas pelo P281**: emit Text/Glyph/Line em
  Group desbloqueia testes de paridade vanilla para casos complexos
  (cluster Gradient + Group fechado em todos os planos).
- **Próximo passo decisão humana**: 10 frentes accionáveis listadas
  §A2.4 ordenadas por valor / ausência de bloqueador / continuidade.

Fase A2 imutável a partir de 2026-05-18.

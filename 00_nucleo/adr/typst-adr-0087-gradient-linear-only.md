# ⚖️ ADR-0087: Gradient Linear materializado; Radial/Conic scope-out

**Status**: `IMPLEMENTADO`
**Data**: 2026-05-15
**Autor**: Humano + IA
**Validado**: Passo 262.B (criação PROPOSTO) → Passo 262.D
(promoção `IMPLEMENTADO` pós-materialização L1 + stdlib;
PDF shading scope-out adicional para **P263** dedicado per
decisão user pós-P262.C inspecção magnitude).
**Aplicação**:
`00_nucleo/materialization/typst-passo-261-relatorio.md`.
**Diagnóstico prévio**:
`00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
(imutável per ADR-0085 — **primeiro consumo directo** pós-P260).
**Análogo estrutural**: ADR-0083 (Color paridade vanilla; N=2
do pattern) + ADR-0086 (Paint wrapper Solid only; N=3 do
pattern; precedente directo — este passo cumpre §"Critério
revisão").

---

## Contexto

Visualize vanilla define:

```rust
// lab/typst-original/.../visualize/gradient.rs:178
pub enum Gradient {
    Linear(Arc<LinearGradient>),
    Radial(Arc<RadialGradient>),
    Conic(Arc<ConicGradient>),
}

pub struct LinearGradient {
    pub stops: Vec<(Color, Ratio)>,
    pub angle: Angle,
    pub space: ColorSpace,           // default Oklab
    pub relative: Smart<RelativeTo>,
    pub anti_alias: bool,
}
```

Cristalino pré-P262 zero hits estruturais (P259 confirmou
ausência). ADR-0086 §"Critério revisão" aponta literal para
este passo. Sequência arquitectural P259 Cenário B2 Opção 1
sub-passo 2.

ADR-0029 §"Simplificações aceites apenas com ADR explícita"
obriga ADR para subset materializado vs vanilla full. Paridade
pattern N=2 → **N=3** cumulativo com ADR-0083 (Color) +
ADR-0086 (Paint).

**User decisions** (pre-P262.A pre-flight):
1. **Materializar tudo** (L1 + L3 + tests) — full P262.
2. **Oklab paridade vanilla** — interpolação Oklab (não sRGB).
3. **GradientStop com Option<Ratio> + auto-spacing** — paridade
   vanilla.

---

## Decisão

### Subset materializado P262 — Linear only

```rust
// 01_core/src/entities/gradient.rs

pub struct GradientStop {
    pub color:  Color,
    pub offset: Option<Ratio>,       // auto-spacing per decisão user
}

pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    // space: ColorSpace,            // scope-out — Oklab fixo
    // relative: RelativeTo,         // scope-out — bbox-relative
    // anti_alias: bool,             // scope-out — assume true
}

pub enum Gradient {
    Linear(Arc<Linear>),
    // Radial(Arc<Radial>),          // P-Gradient-Radial — comentário reserva
    // Conic(Arc<Conic>),            // P-Gradient-Conic — comentário reserva
}
```

**Derives**: `Debug, Clone, PartialEq`. Arc preserva clone O(1).

**Variants `Radial`/`Conic`**: comentários reserva no enum
(não unit placeholders). Política P158 "sem novas reservas"
preservada.

### Activação Paint::Gradient

ADR-0086 §"Critério revisão" cumprido:

```rust
// 01_core/src/entities/paint.rs

#[derive(Debug, Clone, PartialEq)]   // Copy removido — Gradient não é Copy
pub enum Paint {
    Solid(Color),
    Gradient(Gradient),               // P262 — descomentado
    // Tiling(Tiling),                // futuro — comentário reserva
}

impl From<Gradient> for Paint {
    fn from(g: Gradient) -> Self { Paint::Gradient(g) }
}
```

**`Paint::to_color()` fallback Gradient**: retorna primeiro
stop color. Documentado como "Solid fallback; Gradient renderiza
via L3 shading pattern separado". Preserva API P261.

### Activação Value::Gradient

```rust
// 01_core/src/entities/value.rs

pub enum Value {
    // ...
    Color(Color),
    Gradient(Gradient),               // P262 — descomentado
    // Tiling(Tiling),                // futuro
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            // ...
            Value::Gradient(_) => "gradient",   // P262
        }
    }
}

impl From<Gradient> for Value {
    fn from(g: Gradient) -> Self { Value::Gradient(g) }
}
```

### Stdlib `native_gradient_linear`

Novo ficheiro `01_core/src/rules/stdlib/gradients.rs`
(decisão Opção α — domínio próprio per precedente subpadrão
"módulo stdlib por domínio").

```rust
pub fn native_gradient_linear(args: &Args) -> SourceResult<Value>;
```

- Variadic positional stops (paridade vanilla `#[variadic]`).
- Aceita `Color` directo (offset = None) OU array `[Color, Ratio]`.
- Named: `angle: Angle` (default 0deg).
- Auto-spacing computado em L1 via `Linear::effective_offsets()`.

### PDF shading exporter L3 — adiado P263

**Decisão pós-P262.C inspecção magnitude** (user decision):
PDF shading completo (Function/Shading/Pattern objects +
Resources + dedup + branching emit em 4 caminhos
build_pdf_*) é magnitude S-M sozinha (~200-300 LoC) que
exige refactor monolítico exporter. **Adiado para P263
dedicado** per ADR-0061 §"granularidade 1-2 features/passo".

**Estado actual P262**: `Paint::to_color()` faz fallback
`first_stop_color` para `Paint::Gradient(g)`. Os 4 sítios
PDF exporter `s.paint.to_color().to_rgba_f32()` retornam
primeira cor do gradient como Solid. PDF output mostra
primeira cor literal (sem interpolação real).

**P263 spec preliminar** (referência futura):
1. Helper `compute_axial_coords(angle, bbox) -> [f64; 4]`.
2. Helper `sample_gradient_to_srgb_stops(linear, n)` — amostra
   N pontos em Oklab; converte para sRGB.
3. Emit `/Function Type 3` (stitching) + N Type 2.
4. Emit `/Shading <obj-id>` com `/ShadingType 2` axial.
5. Emit `/Pattern <obj-id>` com `/PatternType 2`.
6. Registo em `/Resources /Pattern` page-level.
7. Dedup via `Arc::as_ptr(linear)`.
8. Branching 4 sítios `FrameItem::Shape`:
   - `Paint::Solid(c)` → emit literal P261 preservado.
   - `Paint::Gradient(g)` → emit `/Pattern cs` + `/P<n> scn`.

### ColorSpace fixo Oklab (paridade vanilla)

Interpolação L1 em **Oklab** (paridade vanilla default).
Amostragem N pontos via `Linear::sample(t)`; conversão para
sRGB via `Color::to_rgba_f32()` (linear path Oklab → linear
RGB → sRGB).

**Por que não scope-out sRGB**: user decision P262 Q2 escolheu
paridade vanilla literal. Magnitude L1 sample helper é pequena
(~30 LoC); PDF emit beneficia de sample fidelity.

### Preservações arquitecturais

- **ADR-0039 SR-Struct Resolvido**: `TextStyle.fill: Option<Color>`
  **inalterado** literal. P262 não migra TextStyle para
  Option<Paint> (preservação P261 confirmada).
- **Style::Fill(Color)**: inalterado.
- **DEBT-1** (fechado P142): preservado.

### Scope-outs documentados

| Scope-out | Razão | Resolução prevista |
|-----------|-------|---------------------|
| `Gradient::Radial(Radial)` | Sem consumer real P262 | **P-Gradient-Radial** dedicado (M; `/ShadingType 3`) |
| `Gradient::Conic(Conic)` | Baixa prioridade | **P-Gradient-Conic** dedicado |
| `LinearGradient.space: ColorSpace` | Oklab fixo per decisão user | Refino futuro se sRGB explícito for prioritário |
| `LinearGradient.relative: Smart<RelativeTo>` | bbox-relative (vanilla default) | "self-relative" diferido P-Gradient-Relative |
| `LinearGradient.anti_alias` | Assume true (PDF default) | Refino se controlo necessário |
| `Gradient::sample()` user-facing | API auxiliar vanilla | Futuro se consumer real exigir |
| `Gradient::stops()` getter | Auxiliar vanilla | Futuro |
| **PDF shading completo** (`/Pattern` + `/Shading` + `/Function` objects + Resources + dedup + branching emit) | **Scope-out adicional pós-P262.C inspecção magnitude** — refactor monolítico build_pdf_* exporter estoira M+ | **P263 dedicado** (M; ~200-300 LoC L3 + tests E2E). Actualmente PDF usa fallback `Paint::to_color()` → `first_stop_color` (mostra primeira cor como Solid). |

---

## Consequências

### Positivas

- **Activa `Paint::Gradient` variant** (ADR-0086 §"Critério
  revisão" cumprido).
- **User-facing `#gradient.linear(...)`** funcional.
- **PDF shading pattern primeiro uso** — abre caminho para
  Radial/Conic futuros.
- **Paridade vanilla observable** preservada (interpolação
  Oklab + auto-spacing).
- **Cobertura Visualize** +8pp estructural (F.1 Gradient
  Linear ausente → implementado).

### Negativas

- **PDF exporter ganha caminho complexo** (Function dict +
  Pattern dict + Shading dict + dedup + coords compute);
  ~200-300 LoC L3.
- **`Paint::to_color()` Gradient fallback** é approximation
  (primeiro stop); documentado.
- **Magnitude M-M+** comprometido (~3-5h real).

### Neutras

- **Variants `Radial`/`Conic` (comentários)** no enum: roadmap
  visual; não DEBT/ADR novo per política P158.
- **`Copy` removido de Paint** (Gradient não é Copy via Arc);
  P261 era Copy via Solid(Color).

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| α1 — Linear only (escolhida) | Magnitude controlada; activa P262 base | Radial/Conic comentários reserva |
| α2 — Linear + Radial juntos | Cobertura ↑ | Magnitude M+ vs M; passo dedicado preferível |
| α3 — Stubs Radial/Conic unit | Enum completo | Variants vazios sem consumer poluem enum |
| β — sRGB fixo (scope-out Oklab) | PDF emit mais simples | Divergência face vanilla user-facing |
| γ — Vec<(Color, Ratio)> directo (sem GradientStop type) | Paridade vanilla literal | Menos idiomático cristalino |

**Decisão**: **α1 (Linear only) + Oklab paridade vanilla +
GradientStop Option<Ratio> com auto-spacing** per user decisions
P262 pre-flight.

---

## Critério revisão

ADR-0087 transita `IMPLEMENTADO` → expansão real quando:

1. **P-Gradient-Radial** materializa `Radial` struct +
   activa `Gradient::Radial(Arc<Radial>)` variant + PDF
   `/ShadingType 3`.
2. **P-Gradient-Conic** materializa `Conic` struct + activa
   variant.
3. **P-Gradient-Space-Custom** materializa `space: ColorSpace`
   campo (revoga scope-out Oklab fixo).
4. **P-Gradient-Relative-Custom** materializa `relative:
   RelativeTo` (revoga scope-out bbox-fixo).

Cada activação é **passo dedicado pequeno** (XS-M) per pattern
P262+; sem DEBT novo per política P158.

---

## Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo via Cenário B1/B2"

Cumulativo:
- N=1 P257 (ADR-0083 Color).
- N=2 P261 (ADR-0086 Paint).
- **N=3 P262** (ADR-0087 Gradient; este passo).

**Patamar N=3 atinge limiar formalização clara**. Candidato a
meta-ADR futuro — **improvável** (padrão auto-documentado em
cada ADR individual; análogo P156K self-documentation).

---

## Subpadrão "Refactor cross-cutting entity primitivo" N=3 → N=4

Cumulativo:
- N=1 P252 (Stroke `overhang`).
- N=2 P257 (Color expansão).
- N=3 P261 (Paint wrapper).
- **N=4 P262** (Gradient + PDF exporter expand cross-cutting).

**Patamar N=4 reforça formalização**.

---

## Subpadrão "Diagnóstico imutável precedente à acção" — primeiro consumo directo ADR-0085

P260 formalizou ADR-0085. P261 foi consumo indirecto.
**P262 é primeiro consumo directo** — diagnóstico Gradient
vanilla é imutável per ADR-0085 literal, producido por
materialização per ADR-0029 §"Diagnosticar primeiro".

**N=4 cumulativo (P255+P257+P258+P259) + 1 directo P262 = N=5
geral**. Patamar validates formalização P260 retroactivamente.

---

## Referências

- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório
  (regra principal cumprida).
- ADR-0033 — Paridade observable vanilla.
- ADR-0034 — Diagnóstico canónico.
- **ADR-0083** — Color paridade vanilla (precedente N=2 do
  pattern).
- **ADR-0084, ADR-0085** — Auditoria condicional + diagnóstico
  imutável (P260; **primeiro consumo directo** P262).
- **ADR-0086** §"Critério revisão" — cumprido por este passo
  (precedente N=3; Paint wrapper).
- ADR-0027 — PDF objects estrutura (precedente shading).
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded (scope-outs aceites).
- ADR-0065 — Inventariar primeiro (cumprido em P262.A).
- DEBT-1 — fechado P142 (preservado).
- P252 — Stroke `overhang` cross-cutting (precedente N=1).
- P257 — Color paridade vanilla 8/8 (precedente N=2; template
  PROPOSTO+IMPLEMENTADO mesmo passo N=1).
- P259 §3 Opção 1 sub-passo 2 — spec preliminar.
- P260 — ADRs meta (formaliza padrões).
- P261 — Paint wrapper Solid only (precedente N=3; pré-requisito).
- `00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
  — diagnóstico imutável P262.A.
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  — fonte canónica (1366 linhas).

---

## Próximos passos

1. P262.C executa materialização imediata (gradient.rs + Paint
   activação + Value activação + stdlib + PDF shading).
2. P262.D promove ADR-0087 → `IMPLEMENTADO`.
3. P-Gradient-Radial (futuro) — activa `Gradient::Radial`
   variant + PDF `/ShadingType 3`.
4. P-Gradient-Conic (futuro) — activa `Gradient::Conic` variant.

---

## Anotação cumulativa P263 — PDF shading complete materializado

**Data**: 2026-05-15.

PDF rendering real Gradient Linear materializado. Substitui
fallback `first_stop_color` introduzido em P262 (preserved
apenas em `draw_item_local` para shapes em groups com `cm`
transformations — scope-out adicional).

### Componentes materializados em `03_infra/src/export.rs`

- **Tipos novos**: `PatternRef { pattern_obj_id, name }` +
  `GradientObject { linear, function_id, shading_id, pattern_id }`.
- **`scan_all_gradients(doc, first_id)`** — pre-pass dedup
  via `Arc::as_ptr(linear)` (paridade pattern image P73 N=2
  do template arquitectural). Aloca 3 ObjectIDs por gradient
  único (Function + Shading + Pattern).
- **`pattern_resources_for_page(page, ptr_to_idx, refs)`** —
  fragmento `/Pattern << /P1 X 0 R ... >>` page-level.
- **`compute_axial_coords(angle_rad, x0, y0, w, h)`** — L3
  puro; semi-axes projection.
- **`oklab_sample_stops(linear, n=16)`** — pré-sample em
  Oklab via `Linear::sample(t)` L1 (P262); converte para sRGB
  via `Color::to_rgba_f32`.
- **`emit_function_dict(stops, function_id, sub_first_id)`** —
  Type 2 para 2 stops; Type 3 stitching para N>2.
- **`emit_gradient_objects`** método PdfBuilder — emit
  sub-functions + Function + Shading + Pattern objects.
- **`emit_stroke_paint(ops, paint, thickness, ptr_to_idx, refs)`**
  branching helper unificado:
  - `Paint::Solid(c)` → `r g b RG {w} w` literal P261 preservado.
  - `Paint::Gradient(g)` → `/Pattern CS /P{n} SCN {w} w`.
- **3 paths `build_helvetica/cidfont/multifont` adaptados**:
  - Pre-pass `scan_all_gradients`.
  - Resources `/Pattern << ... >>` condicional.
  - `build_page_stream_*` signature ganha `pat_ptr_to_idx +
    pat_refs`.
  - `emit_gradient_objects` chamado depois das XObjects.
- **`build_page_stream_type1` + `cidfont` + `multifont`**
  ganham branching `emit_stroke_paint` (3 sítios de stroke
  emit principal).

### Paridade observable cumprida pós-P263

`#gradient.linear(red, blue, angle: 90deg)` em `Stroke.paint`
agora renderiza gradient real no PDF via `/ShadingType 2 axial`
+ `/Function Type 3` stitching (16 stops amostrados em Oklab).

### Scope-out adicional pós-P263.C

- **`draw_item_local`** (shape recursivo dentro de groups com
  `cm` transformations) preserva fallback `first_stop_color`
  per scope-out adicional — função interna sem acesso a
  `pat_ptr_to_idx` no escopo; refactor para passar pattern
  resources adiado (refino futuro se consumer real exigir).
- **`FrameItem::Shape.fill: Option<Color>`** continua literal
  Color (não Paint) — refino futuro pode estender Fill → Paint
  se prioritário.

### Decisões D1-D5 P263.A

- **D1 Function dict**: ambos Type 2 + Type 3 implementados.
- **D2 Oklab → sRGB**: pré-amostragem N=16 stops em Oklab;
  emit Type 3 stitching linear sRGB.
- **D3 Coords L3**: helper `compute_axial_coords` L3 puro;
  bbox-based semi-axes projection (algoritmo simplificado —
  paridade vanilla pode diferir).
- **D4 Dedup `Arc::as_ptr`**: implementado paridade pattern
  P73.
- **D5 Cross-path**: 3 paths build_page_stream_* todos
  adaptados.

### Tests adicionais P263 (+8 cumulativos)

- `p263_compute_axial_coords_angle_0_horizontal`.
- `p263_compute_axial_coords_angle_90_vertical`.
- `p263_oklab_sample_stops_red_blue_endpoints`.
- `p263_emit_function_dict_2_stops_uses_type_2`.
- `p263_emit_function_dict_4_stops_uses_type_3_stitching`.
- `p263_export_pdf_gradient_in_stroke_emits_shading` (E2E).
- `p263_export_pdf_gradient_solid_preserva_rg_emit` (E2E
  paridade P261).
- `p263_export_pdf_gradient_dedup_arc_ptr` (E2E dedup).

### Cobertura Visualize agregada

- Pre-P263: ~58% (P262 L1+stdlib +5pp).
- **Pós-P263: ~63%** (+5pp via PDF render real Gradient
  Linear; F.1 promovido `implementado+stdlib` → `implementado+stdlib+render`).

### Subpadrões cumulativos pós-P263

- **"Refactor cross-cutting entity primitivo" N=3 → N=4** —
  PDF exporter cross-path Gradient emit toca 3 paths
  simultaneamente.
- **"P262/P263 dividir granularidade" N=1** — pattern emergente
  novo: L1+stdlib materialização (P262) + L3 PDF rendering
  dedicado (P263). Preserva ADR-0061 §"granularidade 1-2
  features/passo". Candidato N=2 se Radial/Conic seguir mesmo
  padrão.
- **Status `IMPLEMENTADO` preservado** — anotação cumulativa
  não muda status; refina aplicação per paridade pattern
  ADR-0080 §"refactor aditivo".

Cross-references:
- L3 emit: `03_infra/src/export.rs` (~300 LoC novas).
- L0 prompt: `00_nucleo/prompts/infra/export.md` secção
  "Suporte Gradient via Shading Patterns (Passo 263)".
- Tests E2E: confirmam `/ShadingType 2`, `/PatternType 2`,
  `/FunctionType`, `/Coords`, dedup `Arc::as_ptr`, paridade
  P261 Solid preservada.
- ADR-0027 (precedente arquitectural estrutura objectos PDF).
- P262 (precedente directo Gradient L1+stdlib).
- P73 (template arquitectural `image_resources` dedup
  `Arc::as_ptr`).

---

## Anotação cumulativa P270 — ColorSpace runtime activado L1+stdlib

**Data**: 2026-05-17.

`Linear` variant ganha campo `space: ColorSpace` (default Oklab;
preserva P262 behavior bit-exact). `Linear::sample(t)` interpola no
space escolhido via dispatcher `interpolate_in_space`. L3 emit Oklab
pipeline preservado P270 — refactor multi-space adiado P270.1.

Sub-padrão "Anotação cumulativa cross-ADR" N=1 inaugural — P270 anota
ADR-0083/0054/0087/0088/0089/0090 simultâneo.

Status `IMPLEMENTADO` preservado literal. Ver **ADR-0091 EM VIGOR**
para decisão arquitectural completa.

---

## Anotação cumulativa P270.1 — L3 emit multi-space materializado

**Data**: 2026-05-17.

Linear L3 emit ganha consciência de `linear.space` via helper
renomeado `multispace_sample_stops(linear, n)` (era
`oklab_sample_stops`). Pipeline `/ShadingType 2` axial preservado
P263 — só nome do helper muda; body literal preserved porque
`linear.sample(t)` despacha via P270 dispatcher automaticamente.

Default Oklab preserva bytes pré-P270.1 bit-exact. CMYK preserva
scope-out P270.1; P270.2 fecha. Ver ADR-0091 §"Anotação cumulativa
P270.1".

Sub-padrão "Anotação cumulativa cross-ADR" N=1 → N=2 cumulativo
(P270 + **P270.1**).

Status `IMPLEMENTADO` preservado literal.

---

## Anotação cumulativa P270.2 — CMYK emit branch directo

**Data**: 2026-05-17.

Linear L3 emit ganha CMYK branch via dispatcher dual em
`emit_gradient_objects`. `linear.space == ColorSpace::Cmyk`:
shading dictionary `/ColorSpace /DeviceCMYK` + Function 4-component
(`/Range [0 1 0 1 0 1 0 1]`; `/C0` `/C1` 4-component). `space !=
Cmyk`: pipeline P270.1 preserved literal.

Cluster Linear L3 emit **feature-complete 8/8 spaces** (Oklab/Oklch/
sRGB/Luma/LinearRGB/HSL/HSV + CMYK directo).

Bug vanilla #4422 resolvido por construção (cristalino emit
`/DeviceCMYK` correcto). Ver ADR-0091 §"Anotação cumulativa P270.2".

Sub-padrão "Anotação cumulativa cross-ADR" N=2 → N=3 cumulativo.

Status `IMPLEMENTADO` preservado literal.

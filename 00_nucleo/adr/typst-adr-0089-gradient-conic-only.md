# ⚖️ ADR-0089: Gradient Conic-only L1+stdlib (fecha cluster Gradient 3/3 variants)

**Status**: `IMPLEMENTADO`
**Data**: 2026-05-15
**Autor**: Humano + IA
**Validado**: Passo 267.B (criação PROPOSTO) → Passo 267.D
(promoção `IMPLEMENTADO` pós-materialização L1+stdlib;
PDF Conic shading adiado P268 dedicado paridade P264/P265).
**Aplicação**:
`00_nucleo/materialization/typst-passo-267-relatorio.md`.
**Diagnóstico prévio**:
`00_nucleo/diagnosticos/diagnostico-gradient-conic-passo-267.md`
(imutável per ADR-0085 — **terceiro consumo directo vanilla**
pós-P262/P264).
**Análogo estrutural directo**: ADR-0088 (Gradient Radial-only;
N=4 do pattern PROPOSTO+IMPLEMENTADO mesmo passo; este ADR
é N=5 cumulativo — pattern sólido estabelecido).

---

## Contexto

Visualize vanilla define:

```rust
// lab/typst-original/.../visualize/gradient.rs:1145
pub struct ConicGradient {
    pub stops: Vec<(Color, Ratio)>,
    pub angle: Angle,
    pub center: Axes<Ratio>,
    pub space: ColorSpace,
    pub relative: Smart<RelativeTo>,
    pub anti_alias: bool,
}
```

6 campos vanilla (sem `focal_*` — esses são exclusivos
Radial). P262 materializou Linear; P264 materializou Radial;
**Conic restante para fechar cluster Gradient L1+stdlib 3/3
variants**. ADR-0088 §"variants não materializados" Conic
preservava scope-out; este passo **revoga parcialmente**
ADR-0088 (Conic sai; `focal_*` Radial preservado).

P259 Cenário B2 Opção 1 sub-passo 2 (extensão Gradient cluster)
e §"Variants vanilla a materializar" — spec preliminar deste
passo. P267 executa Gradient Conic L1+stdlib; P268 candidato
L3 PDF Conic shading dedicado.

ADR-0029 §"Simplificações aceites apenas com ADR explícita"
obriga ADR para subset materializado vs vanilla full. Paridade
pattern N=4 → **N=5** cumulativo com ADR-0083 (Color) +
ADR-0086 (Paint) + ADR-0087 (Gradient Linear) + ADR-0088
(Gradient Radial).

---

## Decisão

### Subset materializado P267 — Conic 3 campos

```rust
// 01_core/src/entities/gradient.rs (delta sobre P262+P264)

#[derive(Debug, Clone, PartialEq)]
pub struct Conic {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub angle:  Angle,
    // space: ColorSpace,            // scope-out — Oklab fixo
    // relative: Smart<RelativeTo>,  // scope-out — bbox-local
    // anti_alias: bool,             // scope-out — true assumed
}

pub enum Gradient {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),
    Conic(Arc<Conic>),    // P267 — descomentado
}

impl Gradient {
    pub fn conic(
        stops: impl Into<Arc<[GradientStop]>>,
        center: Axes<Ratio>,
        angle: Angle,
    ) -> Self;
}

impl Conic {
    pub fn effective_offsets(&self) -> Vec<f32>;  // paridade Linear/Radial
    pub fn sample(&self, t: f32) -> Color;        // paridade Linear/Radial (Oklab)
}
```

**Subset 3 campos** (stops + center + angle). Magnitude
controlada.

### Reutilização literal helpers Oklab P262/P264

`Conic::sample(t)` reutiliza literal helpers privados de
`gradient.rs` (P262):
- `interpolate_oklab`.
- `color_to_oklab_with_alpha`.
- `srgb_to_linear`.
- `linear_rgb_to_oklab`.

**Zero código novo helpers**. Subpadrão "Reutilização literal
de helpers cross-passos" (inaugurado P265) cresce **N=1 → N=2**.

### Stdlib `native_gradient_conic`

```rust
// 01_core/src/rules/stdlib/gradients.rs

pub fn native_gradient_conic(args, ...) -> SourceResult<Value>;
```

- Variadic positional stops (paridade `native_gradient_radial`).
- Named: `center: Axes<Ratio>` (default `(50%, 50%)`); `angle:
  Angle` (default `0deg`).
- Auto-spacing via `effective_offsets` paridade Linear/Radial.

`make_gradient_module()` ganha entrada `conic`. User-facing:
`#gradient.conic(red, blue, center: (40%, 60%), angle: 90deg)`.

### Activação Gradient::Conic em consumers existentes

`Paint::Gradient(Gradient)` (P261/P262) e
`Value::Gradient(Gradient)` (P262) são enum wrappers
**indiferentes a variant interno**. Aceitam Conic
automaticamente. **Zero cascade refactor**.

`Gradient::first_stop_color()` pattern-match expande para cobrir
Conic:

```rust
pub fn first_stop_color(&self) -> Color {
    match self {
        Gradient::Linear(l) => l.stops.first().map(|s| s.color)
            .unwrap_or(Color::rgb(0, 0, 0)),
        Gradient::Radial(r) => r.stops.first().map(|s| s.color)
            .unwrap_or(Color::rgb(0, 0, 0)),
        Gradient::Conic(c) => c.stops.first().map(|s| s.color)
            .unwrap_or(Color::rgb(0, 0, 0)),
    }
}
```

PDF exporter (P263+P265) pattern-match em
`scan_all_gradients` / `pattern_resources_for_page` /
`emit_stroke_paint` / `emit_gradient_objects`: 3+1 sítios
adaptados para tratar Conic. **Decisão P267**: Conic fallback
Solid no PDF (paridade Radial pré-P265 state) — PDF shading
Conic completo adiado para **P268** dedicado.

### Preservações arquitecturais

- **ADR-0039 SR-Struct Resolvido**: `TextStyle.fill:
  Option<Color>` **inalterado** literal.
- **ADR-0086 Paint wrapper Solid only**: status `IMPLEMENTADO`
  preservado.
- **ADR-0087 Gradient Linear-only**: status `IMPLEMENTADO`
  preservado (§"Critério revisão" Linear preservado).
- **ADR-0088 Gradient Radial-only**: status `IMPLEMENTADO`
  preservado; **§"variants não materializados" parcialmente
  revogado** — Conic activado; `focal_*` (Radial-only)
  preserva scope-out.
- **DEBT-1** (fechado P142): preservado.

### Scope-outs documentados

| Scope-out | Razão | Resolução prevista |
|-----------|-------|---------------------|
| `ConicGradient.space: ColorSpace` | Oklab fixo (paridade ADR-0087/0088) | Refino futuro se sRGB explícito for prioritário |
| `ConicGradient.relative: Smart<RelativeTo>` | bbox-local (paridade ADR-0087/0088) | "self-relative" diferido |
| `ConicGradient.anti_alias` | true assumed (PDF default) | Refino se controlo necessário |
| **PDF Conic shading** | Pattern P262/P263 + P264/P265 dividir granularidade N=3 cumulativo | **P268 dedicado** (S-M; replica P263/P265 template; `/ShadingType` custom function ou Type 4-7 lattice) |
| `Gradient::sample()` user-facing Conic | API auxiliar vanilla | Futuro se consumer real exigir |

**Não há scope-out `focal_*`** para Conic — `focal_*` é
exclusivo Radial (ADR-0088 §scope-out preservado).

---

## Consequências

### Positivas

- **Cluster Gradient L1+stdlib 3/3 completo** (Linear +
  Radial + Conic).
- **User-facing `gradient.conic(...)` funcional** parcialmente
  (parsing + L1; PDF render Solid fallback até P268).
- **Zero cascade refactor consumers** — Paint/Value já
  preparados P261/P262.
- **Cobertura Visualize** +2-3pp estructural (F.3 Conic
  ausente → implementado L1+stdlib).
- **Helpers Oklab reutilizados** literal de P262 (zero código
  duplicado).
- **Subpadrão N=2 "Reutilização literal de helpers
  cross-passos"** confirma pattern emergente.

### Negativas

- **PDF render Conic inicialmente fallback Solid** — paridade
  Radial pre-P265 state; P268 fecha promessa.
- **Magnitude controlada M-** (~2h) — extensão minimal P264
  pattern.

### Neutras

- **ADR-0088 §"variants não materializados" parcialmente
  revogado** (Conic sai; `focal_*` Radial preservado).
- **Axes<T> minimal P264 reutilizado** sem alteração.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| α1 — Conic completo (com space/relative/anti_alias) | Paridade vanilla 100% | Magnitude M+ vs M-; raramente usado user-facing |
| **α2 — Conic subset 3 campos (escolhida)** | **Magnitude controlada; helpers L1 reutilizados** | **3 scope-outs preservados** |
| α3 — Linear+Radial+Conic unificados sem struct dedicado | -1 struct | Paridade pattern Linear/Radial structs dedicados quebrada |
| β — Anotação cumulativa ADR-0087 ou 0088 sem ADR nova | -1 ADR | Mistura âmbitos (Linear/Radial/Conic scope-outs distintos) |

**Decisão**: **α2 (Conic subset 3 campos) + Opção α (ADR-0089
nova)** per paridade pattern ADR-0083/0086/0087/0088.

---

## Critério revisão

ADR-0089 transita `IMPLEMENTADO` → expansão real quando:

1. **P268 PDF Conic shading** materializa shading real;
   substitui fallback `first_stop_color` por render real
   (paridade pattern P262 → P263 + P264 → P265).
2. **P-Gradient-Space-Custom** materializa `space: ColorSpace`
   campo (revoga scope-out Oklab fixo).
3. **P-Gradient-Relative-Custom** materializa `relative:
   RelativeTo` (revoga scope-out bbox-fixo).

Cada activação é **passo dedicado pequeno** (XS-M) per pattern
P262+; sem DEBT novo per política P158.

---

## Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo" N=4 → N=5 cumulativo

Cumulativo:
- N=1 P257 (ADR-0083 Color).
- N=2 P261 (ADR-0086 Paint).
- N=3 P262 (ADR-0087 Gradient Linear).
- N=4 P264 (ADR-0088 Gradient Radial).
- **N=5 P267** (ADR-0089 Gradient Conic-only; este passo).

**Patamar N=5 excede limiar formalização clara**. Candidato a
meta-ADR — **improvável e desnecessário** (padrão
auto-documentado em cada ADR individual).

---

## Subpadrão "Dividir granularidade L1+stdlib / L3" N=2 → N=3 cumulativo

Cumulativo:
- N=1 P262/P263 (Linear).
- N=2 P264/P265 (Radial).
- **N=3 P267/P268** (Conic; P268 futuro).

**Cluster Gradient completa 3 divisões** quando P268 materializar.

---

## Subpadrão "Decisão minimalista (subset materializado)" N=4 → N=5 cumulativo

Cumulativo:
- N=1 P257 Color (8/8 + 4 scope-outs).
- N=2 P261 Paint (Solid only).
- N=3 P262 Gradient Linear only.
- N=4 P264 Gradient Radial subset.
- **N=5 P267 Gradient Conic subset** (3 campos; 3 scope-outs).

**Pattern emergente sólido**.

---

## Subpadrão "Reutilização literal de helpers cross-passos" N=1 → N=2

Cumulativo:
- N=1 P265 (PDF Radial reutiliza helpers P263).
- **N=2 P267** (Conic L1 reutiliza helpers Oklab P262 literal).

**Pattern emergente sólido** — confirma overhead arquitectural
mínimo para extensões cluster.

---

## Referências

- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório.
- ADR-0033 — Paridade observable vanilla.
- ADR-0034 — Diagnóstico canónico.
- **ADR-0083** — Color paridade vanilla (precedente N=2 pattern).
- **ADR-0084, ADR-0085** — Auditoria condicional + diagnóstico
  imutável (P260; consumido directamente).
- **ADR-0086** — Paint wrapper (Paint::Gradient absorve Conic).
- **ADR-0087** — Gradient Linear-only (precedente N=3 pattern;
  helpers Oklab reutilizados).
- **ADR-0088** — Gradient Radial-only (precedente directo N=4
  pattern; §"variants não materializados" Conic parcialmente
  revogado por este passo).
- ADR-0027 — PDF objects estrutura (precedente P268 futuro).
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded (anotação cumulativa P267).
- ADR-0061 — Granularidade 1-2 features/passo (cumprido via
  divisão P267/P268).
- ADR-0065 — Inventariar primeiro.
- ADR-0080 — L0 minimal para refactors aditivos.
- DEBT-1 — Fechado P142 (preservado).
- Aplicações precedentes do pattern:
  - P252 — Stroke `overhang` (precedente N=1 cross-cutting).
  - P257 — Color 8/8 (precedente N=2 pattern PROPOSTO+IMPL).
  - P261 — Paint wrapper (precedente N=3).
  - P262 — Gradient Linear L1+stdlib (precedente N=4; helpers
    Oklab reutilizados).
  - P263 — Gradient Linear PDF (template P268 futuro).
  - **P264** — Gradient Radial L1+stdlib (**template literal
    P267**; N=5).
  - P265 — Gradient Radial PDF (template P268 futuro; divisão
    N=2).
  - P266 — Text audit Fase A (primeiro consumo directo formal
    ADR-0084/0085 pós-P260).
- `00_nucleo/diagnosticos/diagnostico-gradient-conic-passo-267.md`
  — diagnóstico imutável P267.A (terceiro consumo directo
  vanilla).
- Vanilla `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  (1366 linhas; ConicGradient §1145-1158; 6 campos).

---

## Próximos passos

1. P267.C executa materialização imediata (gradient.rs + Conic
   struct + Gradient::Conic activado + stdlib + 3 sítios
   pattern-match adaptados).
2. P267.D promove ADR-0089 → `IMPLEMENTADO`.
3. **P268** (futuro) — PDF Conic shading dedicado (replica
   P263/P265 template; ~200-250 LoC L3).
4. **P-Gradient-Focal** (futuro) — activa `focal_center` +
   `focal_radius` Radial (revoga ADR-0088 §focal scope-out).

---

## Anotação cumulativa P268 — PDF Conic shading complete

**Data**: 2026-05-15.

**Promessa P267 fechada**: scope-out §"PDF emit adiado P268"
→ **IMPLEMENTADO P268**. 3 sítios pattern-match `Gradient::Conic(_)
=> continue/fallback Solid` substituídos por emit real.

### Estratégia decidida P268.A: Type 4 Gouraud manual

User pre-flight escolheu Type 4 (Free-Form Gouraud) em vez
de scope-out preserved ou fallback aproximação. Vanilla usa
crate externa `krilla::SweepGradient` (não autorizada
cristalino per ADR-0018); P268 implementa Type 4 manual.

PDF Spec ISO 32000 §7.5.7:
- `/ShadingType 4`.
- Triangulação disco em N=32 fatias (compromisso fidelidade/LOC).
- 96 vertices × (1 flag + 1 x + 1 y + 3 RGB) = 576 bytes stream.
- BitsPerCoordinate/Component/Flag = 8.

### Helpers materializados P268

- **`oklab_sample_stops_conic(conic, n)`** — paridade literal
  `oklab_sample_stops_radial` P265.
- **`emit_conic_gouraud_stream(conic, w, h, n_slices)`** —
  produz bytes binary Type 4.

### Reutilização literal helpers (sub-padrão N=2 → N=3)

- `interpolate_oklab` (P262 helper privado).
- Logic `Conic::sample(t)` Oklab L1 (P267 helper).
- Estrutura `emit_gradient_objects` (P263/P265 padrão).

### Pattern-match cluster Gradient L1+stdlib+PDF 3/3 completo

| Variant | L1 | Stdlib | PDF |
|---------|----|----|-----|
| Linear | P262 ✓ | P262 ✓ | P263 ✓ /ShadingType 2 |
| Radial | P264 ✓ | P264 ✓ | P265 ✓ /ShadingType 3 |
| **Conic** | **P267 ✓** | **P267 ✓** | **P268 ✓ /ShadingType 4** |

### Scope-outs preservados pós-P268

- `focal_*` (exclusivo Radial; ADR-0088 §scope-out).
- `space` ColorSpace custom (Oklab fixo paridade ADR-0087/0088/0089).
- `relative` custom (bbox-local paridade).
- `anti_alias` (PDF default true).
- Spread mode repeat (Type 4 não suporta nativamente; Pad implícito).

### Sub-padrões cumulativos

- **"Anotação cumulativa em vez de ADR nova" N=4 → N=5**
  (P258.B/P259.B/P263/P265/**P268**).
- **"Reutilização literal helpers cross-passos" N=2 → N=3**
  (P265 + P267 + **P268**).
- **"Dividir granularidade L1+stdlib / L3" N=3 completo**
  (cluster Gradient encerrado quanto a 3 variants base).
- **"Diagnóstico imutável precedente à acção" N=8 → N=9
  cumulativo** (P268.A quarto consumo directo vanilla
  pós-P262/P264/P267).

Status `IMPLEMENTADO` preservado literal (anotação cumulativa
não muda status; refina aplicação paridade pattern P263
anotação ADR-0087).

---

## Anotação cumulativa P268.1 — Cross-reference ADR-0090

**Data**: 2026-05-15.

**Motivo**: Divergência arquitectural Type 4 cristalino vs estratégia
vanilla actual desconhecida (krilla `SweepGradient` interno opaco;
Typst original pré-krilla era Type 6 Coons per blog 2023) formalizada
em ADR-0090 dedicada (EM VIGOR) após pesquisa empírica industry
(Cairo/Inkscape/Skia/pdf.js/Typst original/krilla).

**Conclusão factual**: cristalino Type 4 alinhado com industry
mesh-based standard (Cairo Type 6/7, Inkscape Type 7, Typst original
Type 6 Coons). Divergência intra-família mesh (Type 4 vs Type 6),
não entre famílias. Krilla actual produce strategy desconhecida.

Ver ADR-0090 §"Pesquisa empírica industry" + §"Decisão" + §"Convenção
cor central" + §"Nota metodológica de proveniência" para
justificação completa.

**Cor central = primeiro stop** confirmada como convenção PDF mesh
shading (não decisão arbitrária P268).

**Refino qualidade visual** pendente P268.2 (adaptive N hybrid).

Status `IMPLEMENTADO` preservado literal (ADR-0089 cobre L1+stdlib
+ decisão estratégia PDF materializada; ADR-0090 nova **complementa**
formalizando justificativa empírica; sem revogação).

---

## Anotação cumulativa P268.2 — Refino adaptive N hybrid 1+2 (qualidade visual)

**Data**: 2026-05-15.

**Motivo**: P268.1 ADR-0090 formalizou Type 4 Gouraud com N=32 fixo;
casos extremos (muitos stops ou contraste cromático alto) podem
apresentar banding visível. P268.2 refina N adaptive sem mudar
estratégia Type 4.

### Fórmula adaptive N hybrid 1+2

Critério 1 (número de stops) + Critério 2 (Oklab ΔE) combinados:

```rust
const N_BASE: usize = 32;
const N_MIN: usize = 8;
const N_MAX: usize = 128;
const FACTOR_DELTA: f32 = 256.0;  // Oklab canónico — ver §A.5 diagnóstico

fn compute_adaptive_n_conic(conic: &Conic) -> usize {
    let num_stops = conic.stops.len();
    if num_stops < 2 { return N_MIN; }
    let n_stops = num_stops.saturating_sub(2) * 8;
    let sum_delta_e: f32 = conic.stops.windows(2)
        .map(|p| oklab_delta_e(p[0].color, p[1].color)).sum();
    let n_delta = (sum_delta_e * FACTOR_DELTA) as usize;
    N_BASE.max(n_stops + n_delta).clamp(N_MIN, N_MAX)
}
```

### factor_delta = 256.0 (recalibrado vs spec original 2.0)

Spec P268.2 §A.5 propunha `factor_delta = 2.0` com base na suposição
de ΔE em escala CIELab (~0-100). Verificação empírica cristalino in-situ
(`00_nucleo/diagnosticos/diagnostico-adaptive-n-passo-268-2.md` §A.4)
confirma que helpers Oklab L1 implementam **Oklab canónico** (Björn
Ottosson + W3C CSS Color 4): ΔE_OK ∈ [0, ~1.2]; ΔE_OK(red, blue) ≈ 0.537
(não ~70 como spec assumia).

Recalibração `factor_delta = 256.0`:
- preserva fórmula estrutural intacta (apenas constante muda);
- magnitude LOC preservada (§política condição 2 não accionada — não
  há mudança estrutural nem estouro);
- produz N=128 para contraste máximo (red↔blue, black↔white); N=32 para
  pastel (2 stops ΔE ≤ ~0.1); N intermediário para casos mistos;
- intuição: `256 ≈ 1 / threshold perceptual Oklab` (~0.004 unidades por
  fatia em contraste máximo).

### N_max = 128 — stream PDF tolerável

128 fatias × 18 bytes/triangle = 2304 bytes stream vs 576 actual P268
(4×); tolerável para qualidade visual industry-grade.

### ADR-0090 preservada literal

Estratégia Type 4 Gouraud intocada; P268.2 só refina parâmetro N.
Decisão arquitectural ADR-0090 §"Decisão" (Type 4 vs Type 1 vanilla)
permanece em vigor sem modificação.

### Helpers Oklab P262 reutilizados literal

`color_to_oklab_with_alpha` (L1 `gradient.rs`) promovido a `pub fn`
(mudança 4 caracteres) para acessibilidade cross-crate. Function body
preservada literal. Wrapper L3 `oklab_delta_e` é ~8 LOC.

Subpadrão **"Reutilização literal helpers cross-passos" N=3 → N=4
cumulativo**:
- N=1 P265 (PDF Radial reutiliza helpers P263).
- N=2 P267 (Conic L1 reutiliza helpers Oklab P262).
- N=3 P268 (PDF Conic reutiliza helpers Oklab P262/P265).
- **N=4 P268.2** (compute_adaptive_n_conic reutiliza
  `color_to_oklab_with_alpha` P262).

### Regressão tests P268 originais — proibida e preservada

Assinatura `emit_conic_gouraud_stream(conic, n_slices)` mantida literal
(não removido o parâmetro `n_slices`). **6 tests P268 originais
permanecem verdes literal** — `p268_emit_conic_gouraud_stream_n32_size`,
`_min_8_slices`, `_oklab_sample_stops_conic_red_blue_endpoints`,
`p268_export_pdf_conic_emits_shading_type_4`, `_dedup_arc_ptr`,
`_cluster_3_variants_coexistem`. Adaptive N entra apenas no callsite
production (linha 1175 em `03_infra/src/export.rs`).

### Cluster Gradient PDF qualidade visual industry-grade

Adaptive N elimina banding observable em casos extremos (muitos stops
ou contraste alto); cristalino Type 4 qualitativamente competitivo com
Cairo Type 6/7 sem aumentar magnitude implementação.

### Justificação over-engineering deliberado

Critério 4 hybrid 1+2 é mais complexo que crit 1 isolado (~5 LOC) ou
crit 2 isolado (~30-40 LOC). Utilizador escolheu hybrid explicitamente
para cobrir ambos casos extremos; cap S (200 LOC) acomoda (~30-40 LOC
totais); refino vale a complexidade.

### Subpadrão "Anotação cumulativa em vez de ADR nova" N=5 → N=6 cumulativo

- N=1 P258.B.
- N=2 P259.B.
- N=3 P263.
- N=4 P265.
- N=5 P268.
- **N=6 P268.2** (esta anotação — não cria ADR-0091).

### Subpadrão "Refino paramétrico preservando ADR estratégica" — N=1 inaugural

P268.2 inaugura padrão: refino que ajusta constante/parâmetro
melhorando comportamento sem revogar ADR estratégica subjacente.
Aqui: `factor_delta` calibrado, `N` adaptive vs fixo; ADR-0090 (Type 4
strategy) intocada.

Status `IMPLEMENTADO` preservado literal (anotação cumulativa não muda
status; refina aplicação paridade pattern P263 anotação ADR-0087 +
P265 anotação ADR-0088 + P268 anotação esta ADR).

---

## Anotação cumulativa P270 — ColorSpace runtime activado L1+stdlib

**Data**: 2026-05-17.

`Conic` variant ganha campo `space: ColorSpace` (default Oklab;
preserva P267 behavior bit-exact). `Conic::sample(t)` interpola no
space escolhido via dispatcher `interpolate_in_space`. L3 emit Type 4
Gouraud preservado P270 (P268 + P268.2 adaptive N intactos) — refactor
multi-space adiado P270.1.

Sub-padrão "Anotação cumulativa cross-ADR" N=1 inaugural — P270 anota
ADR-0083/0054/0087/0088/0089/0090 simultâneo.

Status `IMPLEMENTADO` preservado literal. Ver **ADR-0091 EM VIGOR**
para decisão arquitectural completa.

---

## Anotação cumulativa P270.1 — L3 emit multi-space materializado

**Data**: 2026-05-17.

Conic L3 emit ganha consciência de `conic.space` via helper
renomeado `multispace_sample_stops_conic(conic, n)` (era
`oklab_sample_stops_conic`). Pipeline `/ShadingType 4` Type 4 Gouraud
+ adaptive N hybrid P268.2 preservados — só nome do helper muda;
body literal preserved porque `conic.sample(t)` despacha via P270
dispatcher automaticamente.

Default Oklab preserva bytes pré-P270.1 bit-exact. CMYK preserva
scope-out P270.1; P270.2 fecha. Ver ADR-0091 §"Anotação cumulativa
P270.1".

Sub-padrão "Anotação cumulativa cross-ADR" N=1 → N=2 cumulativo
(P270 + **P270.1**).

Status `IMPLEMENTADO` preservado literal.

---

## Anotação cumulativa P270.2 — Conic CMYK scope-out preserved (Cenário B)

**Data**: 2026-05-17.

P270.2 materializa CMYK emit directo para Linear+Radial.
**Conic CMYK preserved scope-out P270.2** (§A.8 diagnóstico Cenário
B; ADR-0091 §"Anotação cumulativa P270.2"):

Razões:
- Vanilla Conic CMYK suporte incerto (krilla opaco).
- PDF reader compatibility Type 4 Gouraud + CMYK incerto.
- Complexidade extra (stream binary 4 bytes/vertex; `/Decode` 5
  pares) adiciona ~50 LOC L3.
- Linear + Radial cobrem maioria dos use cases user-facing.

**Conic com `space: Cmyk` em P270.2**: pipeline P270.1 fallback
preservado (sample CMYK convert para sRGB via `Conic::sample(t)` +
`to_rgba_f32()`). Funcional mas **gama CMYK perdida no emit**
(sub-óptimo).

Cluster Conic L3 emit **7/8 spaces full + CMYK fallback** —
candidato refino futuro **P-Gradient-Conic-CMYK** ao revogar
definitivamente.

Sub-padrão "Anotação cumulativa cross-ADR" N=2 → N=3 cumulativo.

Status `IMPLEMENTADO` preservado literal. Estratégia Type 4 Gouraud
ADR-0090 intocada.

---

## Anotação cumulativa P270.3 — Conic 2 emit paths coexistem (Type 4 Gouraud + Type 6 Coons)

**Data**: 2026-05-17.

P270.3 materializa **infra-estrutura Type 6 Coons Patch Mesh** como
estratégia adicional Conic L3 emit (preparação CMYK P270.4). **Conic
ganha 2 emit paths coexistentes**:

- **Type 4 Gouraud (P268+P268.2 preserved)** — usado para 7 spaces
  RGB-family + perceptual (sRGB, LinearRGB, Luma, Oklab, Oklch, HSL,
  HSV); P270.1 pipeline preservado literal.
- **Type 6 Coons (P270.3 infra + P270.4 activação)** — usado para
  CMYK em P270.4; reservado para futuro converge Conic RGB
  (P-Gradient-Coons-RGB-Final candidato).

### Dispatcher opt-in flag interno (não user-facing)

- **P270.3**: flag OFF default → `emit_conic_gouraud_stream` literal
  preserved.
- **P270.4**: flag ON para `space == Cmyk` → `emit_conic_coons_stream`
  activado.
- Flag interno cristalino; decisão arquitectural não exposta API.

### Strategy "1 patch per stop"

Paridade Typst original blog 2023: N stops → N patches angulares; cada
patch cobre 360°/N graus. Matemática Bezier cúbico arc círculo
(Stanislaw Adaszewski): offset = r·(4/3)·tan(angle/4).

### Marco arquitectural P270.3

**Primeiro caso "2 estratégias L3 emit coexistem para mesmo variant"**
em cristalino — Conic ganha Type 4 Gouraud (RGB) + Type 6 Coons (CMYK
preparação). Estabelece precedente para futuras divergências intra-emit
fundamentadas em reader-compatibility.

Sub-padrão "Anotação cumulativa cross-ADR" N=3 → N=4 cumulativo.

Status `IMPLEMENTADO` preservado literal. Estratégia Type 4 ADR-0090
intocada. Ver ADR-0092 EM VIGOR.

---

## Anotação cumulativa P270.4 — Conic 2 emit paths AMBOS ACTIVOS (cluster 8/8 absoluto)

**Data**: 2026-05-17.

P270.4 activa opt-in flag ON para `space == Cmyk` via ADR-0092
§"Anotação cumulativa P270.4". **Conic L3 emit dispatcher dual ACTIVO**:

| Strategy | Spaces | Shading |
|---|---|---|
| **Type 4 Gouraud** (P268+P268.2 preserved) | 7 spaces RGB-family + perceptual | `/ShadingType 4 /ColorSpace /DeviceRGB` |
| **Type 6 Coons** (P270.3 infra + P270.4 activação) | CMYK | `/ShadingType 6 /ColorSpace /DeviceCMYK` |

Estratégia dispatcher por `conic.space`:
- `space != Cmyk` → Type 4 Gouraud (preserved literal P268+P268.2).
- **`space == Cmyk` → Type 6 Coons** (P270.4 activação).

### Cluster Conic L3 emit feature-complete 8/8 spaces absoluto

Conic L3 emit cobre 8/8 spaces absoluto pós-P270.4 via 2 estratégias
coexistentes ambos activos. Marco final série P270 — cluster Gradient
24/24 absoluto. Ver ADR-0092 §"Anotação cumulativa P270.4".

Sub-padrão "Anotação cumulativa cross-ADR" N=4 → N=5 cumulativo.

Status `IMPLEMENTADO` preservado literal. Estratégia Type 4 ADR-0090
intocada.

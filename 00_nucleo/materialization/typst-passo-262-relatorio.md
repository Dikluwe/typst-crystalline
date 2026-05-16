# Relatório do passo P262 — Gradient Linear-only via ADR-0087

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-262.md`.
**Tipo**: passo composto sequencial; ADR PROPOSTO+IMPLEMENTADO
mesmo passo (paridade P257/P261 pattern N=3 cumulativo —
limiar formalização atingido).
**Análogo estrutural canónico**: P257 (Color paridade) + P261
(Paint wrapper) — sequência arquitectural completa Opção 1
P259 Cenário B2 sub-passos 1+2.
**Magnitude planeada**: M (M+ cap).
**Magnitude real**: **M-** (~2-3h) — PDF shading completo
scope-out adicional pós-P262.C inspecção magnitude (refactor
exporter estoira M+); dividido em P262 (L1+stdlib) + P263 (PDF
shading) dedicado per ADR-0061.

---

## §1 — Sumário executivo

**Fase A confirmada**: vanilla Gradient 3 variants (Linear/
Radial/Conic) com Arc<T> wrapper; LinearGradient com 5 campos
(`stops: Vec<(Color, Ratio)>` tuple + angle + space + relative +
anti_alias); GradientStop separado com `Option<Ratio>` para
auto-spacing.

**ADR criada/promovida**: **ADR-0087** "Gradient Linear
materializado; Radial/Conic scope-out":
- Criada `PROPOSTO` em P262.B.
- Promovida `IMPLEMENTADO` em P262.D pós-materialização.
- Paridade pattern P257 ADR-0083 + P261 ADR-0086 (**N=3
  cumulativo** atinge limiar formalização clara).

**Tests delta**: **2341 → 2361** (+13 gradient.rs + 7 stdlib
gradients; zero regressões).

**ADRs distribuição**:
- PROPOSTO 11 (inalterado — ADR-0087 entra e sai no mesmo passo).
- EM VIGOR 32 (inalterado).
- **IMPLEMENTADO 26 → 27** (+0087 P262).
- **Total 73 → 74**.

**Ficheiros criados**:
- `00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
  (imutável per ADR-0085 — **primeiro consumo directo** pós-P260).
- `00_nucleo/adr/typst-adr-0087-gradient-linear-only.md`
  (criado PROPOSTO P262.B → promovido IMPLEMENTADO P262.D).
- `00_nucleo/prompts/entities/gradient.md` (L0 prompt novo;
  hash `391208e2`).
- `01_core/src/entities/gradient.rs` (Gradient enum + Linear
  struct + GradientStop sub-comp + sample(t) Oklab + 13 tests).
- `01_core/src/rules/stdlib/gradients.rs` (novo módulo dedicado
  per Opção α; native_gradient_linear + make_gradient_module).
- `00_nucleo/materialization/typst-passo-262-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `01_core/src/entities/mod.rs` (re-export `pub mod gradient`).
- `01_core/src/entities/paint.rs` (`Paint::Gradient(Gradient)`
  variant activada; `Copy` removido; `From<Gradient> for Paint`
  adicionado; `Paint::to_color()` fallback first_stop_color
  para Gradient).
- `01_core/src/entities/value.rs` (`Value::Gradient(Gradient)`
  variant activada; `type_name() => "gradient"`).
- `01_core/src/rules/stdlib/mod.rs` (mod gradients + 7 tests
  P262 inline).
- `01_core/src/rules/eval/mod.rs` (registo
  `scope.define("gradient", make_gradient_module())` + use
  make_gradient_module).
- `00_nucleo/adr/README.md` (tabela + distribuição + entrada
  P262 ~50 linhas).

**~16 ficheiros tocados; ~750 LoC adicionados em L1+stdlib**.

---

## §2 — Sub-passo P262.A — Fase A diagnóstico Gradient vanilla

### A.1 — Estrutura vanilla literal

```rust
// lab/typst-original/.../visualize/gradient.rs:178
pub enum Gradient {
    Linear(Arc<LinearGradient>),
    Radial(Arc<RadialGradient>),
    Conic(Arc<ConicGradient>),
}

// gradient.rs:1001
pub struct LinearGradient {
    pub stops: Vec<(Color, Ratio)>,    // tuple, não GradientStop
    pub angle: Angle,
    pub space: ColorSpace,             // default Oklab
    pub relative: Smart<RelativeTo>,
    pub anti_alias: bool,
}

// gradient.rs:1217
pub struct GradientStop {
    pub color: Color,
    pub offset: Option<Ratio>,         // Option auto-spacing
}
```

3 variants vanilla; LinearGradient com 5 campos; GradientStop
sub-comp com Option<Ratio>.

### A.2 — Consumers cristalino (impacto P262)

- **Stroke.paint** (P261) — `Paint::Gradient(g)` via From
  automatic; sem refactor cascade adicional.
- **PDF exporter** — 4 sítios `s.paint.to_color().to_rgba_f32()`;
  fallback `first_stop_color` via `Paint::to_color()` actualizado.
- **Value enum** — `Value::Gradient` comentado pré-P262; activado.
- **Color::to_rgba_f32** reutilizado para amostragem
  Oklab → sRGB.

### A.3 — PDF shading arquitectura

**Decisão local**: Coords L3 (exporter conhece bbox); L1 puro
em representação angular. **Implementação real adiada P263**
per decisão user pós-P262.C inspecção magnitude.

### A.4 — Decisão forma cristalina

**User decisions pre-flight** (3 questões P262):
1. **Q1 — Materializar tudo**: L1 + L3 + tests. **P263 dedicado
   para PDF shading complete** per decisão pós-P262.C.
2. **Q2 — Oklab paridade vanilla**: interpolação Oklab (não
   sRGB fixo).
3. **Q3 — GradientStop com Option<Ratio> + auto-spacing**:
   paridade vanilla.

### A.5 — Validações stdlib

`native_gradient_linear`:
- Stops vazios → erro hard.
- Stop com offset fora de [0, 1] → erro hard.
- Aceita `Color` directo OR array `[Color, Ratio]`.
- Named `angle`; rejeita outros named.

---

## §3 — Sub-passo P262.B — ADR-0087 criada PROPOSTO

Ficheiro novo
`00_nucleo/adr/typst-adr-0087-gradient-linear-only.md`:

- **Status**: `PROPOSTO` (em P262.B).
- **Estrutura**: contexto + subset materializado Linear only +
  adaptação Stroke.paint via From<Gradient> + activação
  Paint::Gradient + activação Value::Gradient + stdlib module
  novo + PDF shading L3 (adiado P263) + ColorSpace Oklab fixo
  + preservações ADR-0039 + scope-outs (7 incluindo PDF
  shading complete adicional) + consequências + alternativas +
  critério revisão + subpadrões + referências.

---

## §4 — Sub-passo P262.C — Materialização L1 + Stdlib

### C.1 — L0 prompt `entities/gradient.md`

Ficheiro novo análogo `entities/color.md` + `entities/paint.md`.
Estrutura: módulo + camada + propósito + Gradient enum +
GradientStop sub-comp + Linear struct + impl + critérios
verificação + notas paridade vanilla + ADR-0086/0039 +
exposição mod.rs + cross-references.

Hash inicial `00000000`; propagado para `391208e2` via
`crystalline-lint --fix-hashes`.

### C.2 — `entities/gradient.rs` materializado

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientStop {
    pub color:  Color,
    pub offset: Option<Ratio>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
}

impl Linear {
    pub fn effective_offsets(&self) -> Vec<f32>;
    pub fn sample(&self, t: f32) -> Color;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(Arc<Linear>),
    // Radial(Arc<Radial>),  // P-Gradient-Radial — comentário reserva
    // Conic(Arc<Conic>),    // P-Gradient-Conic — comentário reserva
}

impl Gradient {
    pub fn linear(stops: impl Into<Arc<[GradientStop]>>, angle: Angle) -> Self;
    pub fn first_stop_color(&self) -> Color;
}
```

**Helpers privados**:
- `interpolate_oklab(c0, c1, t)` — interpolação linear em Oklab.
- `color_to_oklab_with_alpha(c)` — converte qualquer Color para
  Oklab via sRGB → linear → Oklab.
- `srgb_to_linear(c)` — gamma 2.2 inversa.
- `linear_rgb_to_oklab(r, g, b)` — constantes Björn Ottosson 2020.

**13 tests gradient::**:
- `gradient_stop_new_com_offset`.
- `gradient_stop_unspaced`.
- `gradient_linear_construcao_2_stops`.
- `gradient_first_stop_color`.
- `linear_effective_offsets_explicit`.
- `linear_effective_offsets_auto_spacing_all_none`.
- `linear_effective_offsets_auto_spacing_middle_explicit`.
- `linear_sample_extremos_returns_stops`.
- `linear_sample_meio_interpola`.
- `gradient_clone_arc_o1`.
- `gradient_partial_eq`.
- `linear_effective_offsets_1_stop`.
- `linear_sample_clamp_above_1`.

### C.3 — `entities/mod.rs` re-export

```rust
// P262 — Gradient Linear-only per ADR-0087; activa Paint::Gradient.
pub mod gradient;
```

### C.4 — `entities/paint.rs` activações

- `Paint::Gradient(Gradient)` variant **activada** (era
  comentário reserva).
- `Copy` removido (Gradient não é Copy via Arc).
- `From<Gradient> for Paint` adicionado.
- `Paint::to_color()` actualizado: `Paint::Gradient(g) =>
  g.first_stop_color()` (fallback Solid documentado).
- Test `paint_copy_clone` renomeado `paint_clone` (Copy
  removido).

### C.5 — `entities/value.rs` activação

- `Value::Gradient(Gradient)` variant **activada** (era
  comentário pré-P262).
- `type_name()` adicionado `Self::Gradient(_) => "gradient"`.
- Total 24 variants (era 23).

### C.6 — Stdlib novo módulo `gradients.rs`

Decisão **Opção α** (módulo dedicado por domínio per precedente
calc.rs / structural.rs / transforms.rs).

```rust
pub fn make_gradient_module() -> Value {
    // Dict { "linear" => native_gradient_linear }
}

pub fn native_gradient_linear(
    _ctx, args, _world, _file, _figure_numbering
) -> SourceResult<Value> {
    // Parse stops variadic positional + named angle.
    // Returns Value::Gradient(Gradient::linear(stops, angle)).
}

fn parse_stops(items: &[Value]) -> SourceResult<Vec<GradientStop>>;
```

**7 stdlib tests P262** em `stdlib/mod.rs` tests module:
- `p262_gradient_linear_2_color_stops_no_offset`.
- `p262_gradient_linear_explicit_offsets`.
- `p262_gradient_linear_angle_named`.
- `p262_gradient_linear_zero_stops_erro`.
- `p262_gradient_linear_offset_out_of_range_erro`.
- `p262_gradient_linear_named_invalido_erro`.
- `p262_gradient_linear_value_type_name`.

### C.7 — Registo scope eval/mod.rs

```rust
scope.define("gradient", make_gradient_module());
```

User-facing `#gradient.linear(red, blue)` funcional.

### C.8 — PDF exporter — fallback Solid (P263 dedicado)

**Decisão user pós-P262.C** (questão dedicada): PDF shading
completo (Function/Shading/Pattern objects + Resources +
dedup + branching emit em 4 caminhos) é ~200-300 LoC L3 +
refactor monolítico build_pdf_*. **Adiado para P263** per
ADR-0061 §"granularidade 1-2 features/passo".

**Estado actual P262**: `Paint::to_color()` retorna
`first_stop_color` para Gradient → os 4 sítios PDF exporter
mostram primeira cor literal como Solid. User-facing parsing
funciona; tests L1+Stdlib passam.

### C.9 — Verificação intermediária

- `cargo build --workspace` → verde.
- `cargo test --workspace --release` → **2341 → 2361** (+20
  tests: 13 gradient.rs + 7 stdlib; zero regressões).
- `crystalline-lint --fix-hashes .` → `entities/gradient.md`
  hash propagado para `391208e2`.
- `crystalline-lint .` → `✓ No violations found`.

---

## §5 — Sub-passo P262.D — Promoção ADR + README + relatório

### D.1 — ADR-0087 PROPOSTO → IMPLEMENTADO

Status actualizado em
`typst-adr-0087-gradient-linear-only.md`:
- Status: PROPOSTO → **IMPLEMENTADO**.
- Linha **Validado** + **Aplicação** apontando para este
  relatório.
- Scope-out adicional "PDF shading completo → P263 dedicado"
  documentado.

### D.2 — README ADRs

- Tabela: entrada ADR-0087 adicionada com status `IMPLEMENTADO`.
- Distribuição: **IMPLEMENTADO 26 → 27**; PROPOSTO 11
  preservado (entra/sai); EM VIGOR 32 preservado; **total 73
  → 74**.
- Passos-chave: entrada P262 ~60 linhas descritivas paridade
  P257/P259/P260/P261.

### D.3 — Relatório (este ficheiro)

Estrutura canónica §1-§10.

---

## §6 — Padrões metodológicos

### Primeiro consumo directo ADR-0085 pós-P260

P260 formalizou ADR-0085 (diagnóstico imutável). P261 foi
**consumo indirecto** (diagnóstico Paint vanilla cumpre forma
análoga).

**P262 é primeiro consumo directo** — diagnóstico Gradient
vanilla é imutável per ADR-0085 literal, producido por
materialização per ADR-0029 §"Diagnosticar primeiro". **Valida
formalização P260 retroactivamente**.

Cumulativo (Fase A audit + 1 directo):
- N=4 audits Fase A (P255 + P257 + P258 + P259).
- **N=5 geral pós-P262** (audits + directo P262).

### Subpadrão "Refactor cross-cutting entity primitivo" N=3 → N=4

Cumulativo:
- N=1 P252 (Stroke `overhang` cross-cutting).
- N=2 P257 (Color expansão cross-cutting).
- N=3 P261 (Paint wrapper cross-cutting Stroke.paint).
- **N=4 P262** (Gradient + Paint::Gradient activação +
  Value::Gradient activação + stdlib novo módulo).

**Patamar N=4 reforça formalização**.

### Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo via Cenário B1/B2" N=2 → N=3

Cumulativo:
- N=1 P257 (ADR-0083 Color paridade vanilla).
- N=2 P261 (ADR-0086 Paint wrapper Solid only).
- **N=3 P262** (ADR-0087 Gradient Linear-only; este passo).

**Patamar N=3 atinge limiar formalização clara**. Candidato a
meta-ADR futuro — **improvável** (padrão auto-documentado em
cada ADR individual; análogo P156K self-documentation).

### Subpadrão "Decisão minimalista (subset materializado) com variants comentário reserva" N=2 → N=3

Cumulativo:
- N=1 P25 → P257 Color (8 espaços materializados; 4 scope-outs).
- N=2 P261 Paint (Solid only; Gradient/Tiling comentários).
- **N=3 P262** Gradient (Linear only; Radial/Conic comentários).

**Pattern emergente sólido**: cada tipo wrapper materializa
1 variant base + comentários reserva activáveis em passos
dedicados futuros per ADR §"Critério revisão".

### Granularidade ADR-0061 preservada via divisão P262/P263

Magnitude PDF shading completo (~200-300 LoC L3) estoira M+
sozinha. Decisão local pós-P262.C: **dividir** em P262
(L1+stdlib; M) + P263 (PDF shading; S-M dedicado). Preserva
ADR-0061 §"granularidade 1-2 features/passo".

---

## §7 — Cobertura

**Visualize agregado** (Tabela B P259):
- Pre-P261: ~51.9% ponderado linear (P259 audit).
- Pre-P262: ~53% (P261 +Paint wrapper +1pp).
- **Pós-P262: ~58%** (+5pp via Gradient Linear; entradas P259
  Tabela A F.1 Gradient Linear promovida ausente →
  implementado L1+stdlib; G Paint preservado).

**Entradas P259 Tabela A actualizadas pós-P262**:
- F.1 Gradient Linear: ausente → **implementado L1+stdlib**
  (PDF render Solid fallback até P263).
- F.2 Gradient Radial / F.3 Gradient Conic: ausentes (scope-out
  ADR-0087 → P-Gradient-Radial/Conic dedicados).
- G Paint wrapper: implementado (P261; preservado).

**Cobertura agregada user-facing total**: ~75-76% preservado.
**Layout Fase 5**: ~98-99% preservado.
**Math**: DEBT-8 ENCERRADO P255 preservado.
**Model**: ~73% pós-P258 preservado.
**Color** subsistema: 100% estrutural preservado.
**Visualize agregado**: **~58% pós-P262** (+6pp pós-P259
empírico).

---

## §8 — Limitações e trabalho futuro

### Pendências residuais P262 (não-bloqueantes; candidatos passos dedicados)

**PDF shading completo (P263 dedicado)**:
1. **PDF `/ShadingType 2` axial emit** — magnitude S-M (~200-300
   LoC L3); refactor build_pdf_* exporter; tests E2E. **Marco
   P263**: PDF output real Gradient Linear (não fallback Solid).

**Cluster Gradient extensões (P-Gradient-Radial/Conic)**:
2. **Gradient Radial** (F.2) — `/ShadingType 3` radial; P-Gradient-Radial
   dedicado M.
3. **Gradient Conic** (F.3) — P-Gradient-Conic dedicado.

**Cluster Stroke refinos (Opção 3 P259)**:
4. **DEBT-33 Bézier bbox exacto** (S+; ~+5 tests).
5. **Stroke<Length>** (M; ~+10-15 tests).
6. **Dash patterns**.
7. **LineCap/LineJoin/MiterLimit**.

**Cluster Shapes refinos**:
8. **Polygon variant estrutural separada**.
9. **Curve variant**.

**Cluster Image (Opção 5)**:
10. **SVG image format** (L+; requer ADR `usvg`/`resvg`).
11. **Image metadata** `alt`/`fit` (S).

**Transform**:
12. **`origin` pivot** (scope-out ADR-0061 preservado).

**Tiling**:
13. **Tiling pattern** — pré-requisito Paint::Tiling activar.

### Possível migração futura TextStyle.fill → Option<Paint>

Refino futuro pode migrar `TextStyle.fill` para `Option<Paint>`
se **Gradient para texto** for prioritário (ADR-0039 SR
preserved actualmente; preservação P261 confirmada P262).

### Sem ADR nova aberta

Política P158 "sem novas reservas" preservada. ADR-0087
criada e promovida no mesmo passo (PROPOSTO transitório).

### Sem DEBT novo aberto

Saldo DEBTs preservado.

---

## §9 — Critério de aceitação global P262 — Checklist final

- [x] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [x] `cargo test --workspace --release` retorna **2361 verdes**
  (+20 vs baseline 2341; sem regressão).
- [x] `diagnostico-gradient-vanilla-passo-262.md` existe com
  §1-§8 preenchidos.
- [x] ADR-0087 criada PROPOSTO P262.B → IMPLEMENTADO P262.D.
- [x] `00_nucleo/prompts/entities/gradient.md` criado.
- [x] `01_core/src/entities/gradient.rs` materializado com 13
  tests verdes.
- [x] `01_core/src/entities/paint.rs` `Paint::Gradient(Gradient)`
  activado; `Copy` removido; `From<Gradient>` adicionado.
- [x] `01_core/src/entities/value.rs` `Value::Gradient(Gradient)`
  activado; `type_name() => "gradient"`.
- [x] `01_core/src/entities/mod.rs` re-export Gradient adicionado.
- [x] Stdlib `native_gradient_linear` + `make_gradient_module`
  registado em `01_core/src/rules/stdlib/gradients.rs` novo
  módulo; 7 tests stdlib em mod.rs.
- [x] `scope.define("gradient", make_gradient_module())`
  registado em `eval/mod.rs`.
- [x] **ADR-0039 preservado literal** (TextStyle.fill: Color
  inalterado).
- [x] **Paint::Solid + Paint::Gradient ambos funcionais** em
  Stroke.paint via From<Color>/From<Gradient>.
- [ ] **PDF shading completo** — **scope-out adicional P263
  dedicado** per decisão user pós-P262.C inspecção magnitude.
- [x] Hashes propagados (`entities/gradient.md` → `391208e2`).
- [x] README ADRs actualizado (distribuição 73 → 74; entrada
  P262 ~60 linhas).
- [x] Relatório do passo criado.
- [x] Paridade observable parcial preservada: user-facing
  `gradient.linear(...)` funciona; PDF mostra fallback
  first_stop_color até P263.

**Estado pós-P262**:
- Tests workspace: **2361 verdes** (+20 vs baseline 2341).
- Hash drift: zero.
- Lint: zero violations.
- DEBTs saldo: **10 preservado**.
- ADRs distribuição: PROPOSTO 11; IDEIA 2; EM VIGOR 32;
  **IMPLEMENTADO 27**; REVOGADO 2; ADIADO 1; **total 74**.
- Prompts L0 criados/editados: 2 (gradient.md novo + paint.md
  via fix-hashes anterior).
- Diagnóstico imutável criado: 1 (**primeiro consumo directo
  ADR-0085**).
- ADRs criadas: 1 (`IMPLEMENTADO` mesmo passo via paridade
  P257/P261).
- **45 aplicações cumulativas anti-inflação** pós-P205D
  preservadas.

**Marco P262**: **Gradient Linear-only L1+stdlib materializado**;
user-facing `gradient.linear(...)` funcional; activa
`Paint::Gradient` variant (ADR-0086 §"Critério revisão"
cumprido); PDF shading completo adiado **P263 dedicado** per
granularidade ADR-0061.

**Recomendação subjectiva pós-P262**:

- **P263 PDF shading complete** (S-M dedicado; refactor
  exporter; tests E2E real Gradient render).
- **OU P-Gradient-Radial** (M; activa `Gradient::Radial`
  variant + PDF `/ShadingType 3`).
- **OU outras Opções P259 alternativas**:
  - DEBT-33 Bézier bbox + Stroke<Length> Opção 3.
  - Curve variant + Polygon estrutural separada Opção 2.
- **OU Text audit** (consumo directo ADR-0084 + 0085 — segundo
  audit pós-formalização).
- **OU P-Footnote-N** refino M (P258 pendência residual).

**Decisão humana fica em aberto literal** pós-P262.

---

## §10 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- **ADR-0087** (criada PROPOSTO P262.B → IMPLEMENTADO P262.D) —
  Gradient Linear-only.
- **ADR-0086** §"Critério revisão" — **cumprido por este
  passo** (precedente N=3; Paint wrapper Solid only IMPLEMENTADO
  P261).
- **ADR-0085** — Diagnóstico imutável (**primeiro consumo
  directo** P262; valida formalização P260 retroactivamente).
- **ADR-0083** — Color paridade vanilla (precedente N=2 do
  pattern PROPOSTO+IMPLEMENTADO mesmo passo).
- **ADR-0084** — Auditoria condicional (P260; consumido
  metodologicamente).
- ADR-0027 — PDF objects estrutura (precedente shading;
  futuro P263).
- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório
  (regra principal cumprida).
- ADR-0033 — Paridade observable vanilla.
- ADR-0034 — Diagnóstico canónico (estendido por ADR-0085).
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded.
- ADR-0061 — Granularidade 1-2 features/passo (cumprido via
  divisão P262/P263).
- ADR-0065 — Inventariar primeiro.
- DEBT-1 — Fechado P142 (preservado).
- Aplicações precedentes do pattern:
  - P25 → P257 — Color subset (precedente N=2).
  - P252 — Stroke `overhang` (precedente N=1 cross-cutting).
  - P261 — Paint wrapper Solid only (precedente N=3 do pattern
    PROPOSTO+IMPLEMENTADO; pré-requisito P262).
- P259 §3 Opção 1 sub-passo 2 — spec preliminar deste passo.
- P260 — ADRs meta (formaliza ADR-0085 consumido directamente).
- `00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
  — diagnóstico imutável P262.A (primeiro consumo directo).
- `00_nucleo/diagnosticos/diagnostico-paint-vanilla-passo-261.md`
  — diagnóstico imutável precedente.
- `00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md`
  — audit Visualize cobertura agregada.
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  — fonte canónica (1366 linhas; 3 variants).

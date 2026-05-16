# Relatório do passo P267 — Gradient Conic L1+stdlib via ADR-0089 (fecha cluster Gradient 3/3 variants)

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-267.md`.
**Tipo**: passo composto sequencial; ADR PROPOSTO+IMPLEMENTADO
mesmo passo (paridade P257/P261/P262/P264 pattern **N=5
cumulativo**).
**Análogo estrutural canónico directo**: P264 (Gradient Radial
L1+stdlib).
**Magnitude planeada**: M (cap 350 LOC L1 + 80 stdlib + 40
tests).
**Magnitude real**: **M- (~2h)** — ~200 LOC L1 + ~80 stdlib;
14 tests; extensão minimal P264 template.

---

## §1 — Sumário executivo

**Fase A confirmada**: ConicGradient vanilla 6 campos (stops +
angle + center + space + relative + anti_alias); **NÃO tem
`focal_*`** (correção factual spec §1 — focal_* é exclusivo
Radial).

**ADR criada/promovida**: **ADR-0089** "Gradient Conic-only
L1+stdlib (fecha cluster Gradient 3/3 variants)":
- Criada `PROPOSTO` em P267.B.
- Promovida `IMPLEMENTADO` em P267.D pós-materialização.
- Paridade pattern N=5 cumulativo (P257+P261+P262+P264+P267).

**Tests delta**: **2393 → 2407** (+14 P267: 9 gradient.rs Conic
+ 5 stdlib conic; zero regressões).

**ADRs distribuição**:
- PROPOSTO 11 (preservado — ADR-0089 entra/sai mesmo passo).
- EM VIGOR 32 (preservado).
- **IMPLEMENTADO 28 → 29** (+0089 P267).
- **Total 75 → 76**.

**Ficheiros criados**:
- `00_nucleo/diagnosticos/diagnostico-gradient-conic-passo-267.md`
  (imutável per ADR-0085 — **terceiro consumo directo vanilla**).
- `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md`
  (PROPOSTO P267.B → IMPLEMENTADO P267.D).
- `00_nucleo/materialization/typst-passo-267-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `00_nucleo/prompts/entities/gradient.md` (secção cumulativa
  P267 anotada; hash propagado código `3354fb75`).
- `01_core/src/entities/gradient.rs` (Conic struct +
  effective_offsets + sample Oklab + Gradient::Conic variant
  activada + conic() construtor + first_stop_color 3-arm match
  + 9 tests; hash `3354fb75`).
- `01_core/src/rules/stdlib/gradients.rs` (native_gradient_conic
  + make_gradient_module entrada conic).
- `01_core/src/rules/stdlib/mod.rs` (re-export
  native_gradient_conic + 5 stdlib tests P267).
- `03_infra/src/export.rs` (3 sítios pattern-match adaptados
  Gradient::Conic → fallback Solid até P268).
- `00_nucleo/adr/README.md` (entrada P267 + distribuição
  actualizada + ADR-0088 cross-reference §parcial revogação).

**~7 ficheiros tocados; ~200 LoC L1 + ~80 stdlib + 14 tests**.

---

## §2 — Sub-passo P267.A — Diagnóstico Conic vanilla

### Estrutura vanilla literal

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

6 campos vanilla. **Correção factual spec P267 §1**:
ConicGradient **NÃO tem `focal_*`** — esses são exclusivos
RadialGradient (`gradient.rs:1063`).

### Decisões Q1-Q5

- Q1 L1+stdlib only (PDF P268 dedicado).
- Q2 Interpolação Oklab (paridade P262/P264).
- Q3 GradientStop Option<Ratio> (paridade).
- Q4 focal_* N/A (não existe em Conic vanilla).
- Q5 Axes<T> reutilizado (P264 criado).

### Decisão arquitectural

Subset 3 campos materializados: stops + center + angle.
Scope-outs: space (Oklab fixo); relative (bbox-local);
anti_alias (true).

---

## §3 — Sub-passo P267.B — ADR-0089 criada PROPOSTO

Ficheiro novo
`00_nucleo/adr/typst-adr-0089-gradient-conic-only.md`:

- **Status**: `PROPOSTO` (em P267.B).
- **Estrutura**: contexto + subset materializado Conic 3
  campos + activação Gradient::Conic + stdlib `native_gradient_conic`
  + PDF emit adiado P268 + reutilização literal helpers Oklab
  P262 + preservações ADR-0039/0086/0087/0088 + scope-outs
  (5 incluindo PDF P268) + consequências + alternativas +
  critério revisão + subpadrões + referências.

---

## §4 — Sub-passo P267.C — Materialização L1+stdlib

### C.1 — `entities/gradient.md` L0 secção P267

Secção "Anotação cumulativa P267 — Conic variant materializada
(cluster Gradient 3/3 completo)" anotada com tipos + enum
expandido + scope-outs + cross-references.

### C.2 — `entities/gradient.rs` actualizado

**Conic struct + impl**:
```rust
pub struct Conic {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub angle:  Angle,
}

impl Conic {
    pub fn effective_offsets(&self) -> Vec<f32>;  // paridade Linear/Radial
    pub fn sample(&self, t: f32) -> Color;        // paridade (Oklab)
}
```

**`Gradient::Conic(Arc<Conic>)` variant** activada (era
comentário reserva).

**`Gradient::conic(stops, center, angle)`** construtor novo.

**`Gradient::first_stop_color`** pattern-match 3-arm expand.

**9 tests P267 Conic**:
- `p267_conic_construcao_2_stops`.
- `p267_conic_first_stop_color`.
- `p267_conic_clone_arc_o1`.
- `p267_conic_partial_eq`.
- `p267_conic_effective_offsets_auto_spacing`.
- `p267_conic_sample_extremos`.
- `p267_conic_sample_clamp_above_1`.
- `p267_gradient_conic_to_paint_via_from`.
- `p267_conic_angle_non_default`.

**Helpers Oklab reutilizados literal** de P262 (interpolate_oklab,
color_to_oklab_with_alpha, srgb_to_linear, linear_rgb_to_oklab).
**Subpadrão "Reutilização literal helpers cross-passos" N=1 → N=2**.

### C.3 — Stdlib `native_gradient_conic`

```rust
pub fn native_gradient_conic(args, ...) -> SourceResult<Value>;
```

- Stops parsing reutiliza `parse_stops`.
- Named `center: [Ratio, Ratio]` (default `(50%, 50%)`).
- Named `angle: Angle` (default `0deg`).
- Validações: stops vazios → erro; named desconhecido → erro.

`make_gradient_module()` ganha entrada `conic`.

**5 stdlib tests P267**:
- `p267_gradient_conic_2_color_stops_defaults`.
- `p267_gradient_conic_custom_center_angle`.
- `p267_gradient_conic_zero_stops_erro`.
- `p267_gradient_conic_named_invalido_erro`.
- `p267_gradient_conic_value_type_name`.

### C.4 — 3 sítios pattern-match `03_infra/src/export.rs`

- `scan_all_gradients` linha 364-373: `Gradient::Conic(_) =>
  continue` (não regista resource).
- `pattern_resources_for_page` linha 414: `Gradient::Conic(_)
  => continue`.
- `emit_stroke_paint` linha 1140-1146: `Gradient::Conic(_) =>
  fallback Solid emit via first_stop_color` (return).

**PDF emit Conic adiado P268 dedicado**.

### C.5 — Verificação intermediária

- `cargo build --workspace` → verde.
- `cargo test --workspace --release` → **2393 → 2407** (+14;
  zero regressões).
- `crystalline-lint --fix-hashes .` → 1 hash propagado
  (`gradient.md` → código `3354fb75`).
- `crystalline-lint .` → `✓ No violations found`.

---

## §5 — Sub-passo P267.D — Promoção ADR + relatório

### D.1 — ADR-0089 PROPOSTO → IMPLEMENTADO

Status atualizado em
`typst-adr-0089-gradient-conic-only.md`:
- Status: PROPOSTO → **IMPLEMENTADO**.
- Linha **Validado** + **Aplicação** apontando para este
  relatório.

### D.2 — README ADRs

- Tabela: entrada ADR-0089 adicionada com status `IMPLEMENTADO`.
- ADR-0088 cross-reference §"variants não materializados"
  parcialmente revogado P267 (Conic activado; focal_* preserved).
- Distribuição: **IMPLEMENTADO 28 → 29**; PROPOSTO 11
  preservado; EM VIGOR 32 preservado; **total 75 → 76**.
- Passos-chave: entrada P267 ~50 linhas (paridade entrada
  P264).

### D.3 — Relatório (este ficheiro)

Estrutura canónica §1-§10.

---

## §6 — Padrões metodológicos

### Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo" N=4 → N=5

Cumulativo:
- N=1 P257 ADR-0083.
- N=2 P261 ADR-0086.
- N=3 P262 ADR-0087.
- N=4 P264 ADR-0088.
- **N=5 P267 ADR-0089**.

**Patamar N=5 excede limiar formalização clara**.

### Subpadrão "Dividir granularidade L1+stdlib / L3" N=2 → N=3

Cumulativo:
- N=1 P262/P263 (Linear).
- N=2 P264/P265 (Radial).
- **N=3 P267/P268** (Conic; P268 futuro).

**Cluster Gradient completa 3 divisões** quando P268 materializar.

### Subpadrão "Decisão minimalista (subset materializado)" N=4 → N=5

Cumulativo: P257 Color + P261 Paint + P262 Linear + P264 Radial
+ **P267 Conic**.

### Subpadrão "Reutilização literal de helpers cross-passos" N=1 → N=2

- N=1 P265 (PDF Linear reutiliza helpers P263).
- **N=2 P267** (Conic L1 reutiliza helpers Oklab P262 literal).

### Subpadrão "Diagnóstico imutável precedente à acção" — terceiro consumo directo vanilla

Cumulativo pós-P267:
- N=1-4 P255/P257/P258/P259 (audit Fase A).
- N=5 P262 (Linear vanilla — primeiro consumo directo).
- N=6 P264 (Radial vanilla — segundo).
- N=7 P266 (Text audit Fase A formal).
- **N=8 P267** (Conic vanilla — **terceiro consumo directo
  vanilla**; diagnóstico imutável P267.A).

**Patamar N=8 reforça pattern sólido**.

---

## §7 — Cobertura

**Cluster Gradient** (pós-P267):
- Linear: implementado+stdlib+render (P262+P263).
- Radial: implementado+stdlib+render (P264+P265).
- **Conic: implementado+stdlib** (P267; PDF P268 dedicado).

**Cluster L1+stdlib**: **3/3 completo**.
**Cluster com PDF render**: 2/3 (Conic falta P268).

**Cobertura Visualize agregada**:
- Pre-P265: ~58%.
- Pre-P266 (P265): ~73%.
- Pre-P267 (P266 audit): ~73% (audit Visualize não tocou).
- **Pós-P267: ~75%** (+2pp via Conic L1+stdlib; F.3 Gradient
  Conic promovido ausente → `implementado+stdlib`).

---

## §8 — Limitações e trabalho futuro

### Pendências residuais P267 (não-bloqueantes)

**PDF Conic dedicado**:
1. **P268 — Gradient Conic PDF shading** (S-M dedicado; replica
   P263/P265 template; provavelmente custom shading function
   ou Type 4-7 lattice — decisão local P268).

**Cluster Gradient extensões adicionais**:
2. **P-Gradient-Focal** (M; activa `focal_center` + `focal_radius`
   Radial; revoga ADR-0088 §focal scope-out).
3. **P-Gradient-Space-Custom** (S+; activa `space: ColorSpace`
   campo cross-variant; revoga Oklab fixo).
4. **P-Gradient-Relative-Custom** (M; activa `relative:
   RelativeTo`).

**Cluster Visualize outros**:
5. DEBT-33 Bézier bbox.
6. Stroke<Length>.
7. Curve variant + Polygon estrutural.
8. SVG image format.
9. Tiling pattern.

**Cluster Text refinos (P266 pendências)**:
10. C.5 Variant-aware font selection (P267 Opção 1 spec; M).
11. C.6 Font subsetting (M-L).

### Sem ADR nova além de ADR-0089

Política P158 "sem novas reservas" preservada.

### Sem DEBT novo aberto

Saldo DEBTs preservado.

---

## §9 — Critério de aceitação global P267 — Checklist final

- [x] `crystalline-lint .` retorna `✓ No violations found`.
- [x] `cargo test --workspace --release` retorna **2407 verdes**
  (+14 vs baseline 2393; sem regressão).
- [x] `diagnostico-gradient-conic-passo-267.md` existe com §A.1-§A.11
  preenchidos.
- [x] ADR-0089 criada PROPOSTO P267.B → IMPLEMENTADO P267.D.
- [x] `entities/gradient.md` L0 anotado secção P267.
- [x] `entities/gradient.rs` `Gradient::Conic(Arc<Conic>)`
  activado; struct Conic materializado; 9 tests verdes.
- [x] Stdlib `native_gradient_conic` + `make_gradient_module`
  expandido com `conic`; 5 stdlib tests P267.
- [x] **ADR-0039 preservada literal**.
- [x] **Paint::Gradient + Value::Gradient absorvem Conic
  automaticamente**.
- [x] **PDF render Conic fallback Solid** até P268 (3 sítios
  pattern-match em export.rs adaptados).
- [x] Hashes propagados (`gradient.md` → código `3354fb75`).
- [x] README ADRs actualizado (distribuição 75 → 76; entrada
  P267; ADR-0088 cross-reference §parcial revogação).
- [x] Relatório criado.
- [x] Cluster Gradient L1+stdlib 3/3 completo.

**Estado pós-P267**:
- Tests workspace: **2407 verdes** (+14 vs baseline 2393).
- Hash drift: zero.
- Lint: zero violations.
- DEBTs saldo: **10 preservado**.
- ADRs distribuição: PROPOSTO 11; EM VIGOR 32; **IMPLEMENTADO
  29**; total **76**.
- Prompts L0 editados: 1 (`entities/gradient.md` anotado).
- Diagnóstico imutável criado: 1 (**terceiro consumo directo
  vanilla**).
- ADRs criadas: 1 (`IMPLEMENTADO` mesmo passo via paridade
  P257/P261/P262/P264).

**Marco P267**: **cluster Gradient L1+stdlib 3/3 completo**
(Linear P262 + Radial P264 + Conic P267); user-facing
`gradient.conic(red, blue, center: (50%, 50%), angle: 90deg)`
funcional via parsing; activa `Gradient::Conic` variant
fechando ADR-0088 §scope-out Conic.

**Recomendação subjectiva pós-P267**:

- **P268 PDF Conic shading** (S-M dedicado; fecha promessa
  P267; cluster Gradient L1+stdlib+PDF **3/3 completo**).
- **OU P-Gradient-Focal** (M; activa focal_* Radial; revoga
  ADR-0088 §focal scope-out).
- **OU outras Opções P259/P266 alternativas**:
  - DEBT-33 Bézier bbox + Stroke<Length>.
  - Variant-aware fonts P266 Opção 1.
  - Curve variant + Polygon estrutural.

**Decisão humana fica em aberto literal** pós-P267.

---

## §10 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- **ADR-0089** (criada PROPOSTO P267.B → IMPLEMENTADO P267.D).
- **ADR-0088** §"variants não materializados" parcialmente
  revogado por este passo (Conic activado; focal_* preserved).
- **ADR-0087** — Gradient Linear (precedente N=3).
- **ADR-0086** — Paint wrapper (absorve Conic).
- ADR-0029, ADR-0033, ADR-0034, ADR-0039, ADR-0054, ADR-0061,
  ADR-0065, ADR-0080, ADR-0083.
- **ADR-0084 + ADR-0085** — Auditoria condicional + diagnóstico
  imutável (consumidos via diagnóstico vanilla Conic).
- DEBT-1 (fechado P142; preservado).
- Aplicações precedentes:
  - P252, P257, P261, P262, **P264** (template literal),
    P265 (helpers reutilizados), P266.
- `00_nucleo/diagnosticos/diagnostico-gradient-conic-passo-267.md`
  — diagnóstico imutável P267.A.
- Vanilla `lab/typst-original/.../visualize/gradient.rs`
  (ConicGradient §1145-1158; 6 campos).

# Relatório do passo P264 — Gradient Radial L1+stdlib via ADR-0088

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-264.md`.
**Tipo**: passo composto sequencial; ADR PROPOSTO+IMPLEMENTADO
mesmo passo (paridade P257/P261/P262 pattern **N=4 cumulativo —
limiar formalização clara excedido**).
**Análogo estrutural canónico directo**: P262 (Gradient Linear
L1+stdlib).
**Magnitude planeada**: M (M+ cap; ~2-3h).
**Magnitude real**: **M-** (~2h) — extensão minimal P262;
helpers Oklab reutilizados literal; zero cascade refactor
consumers; +17 tests.

---

## §1 — Sumário executivo

**Fase A confirmada**: RadialGradient vanilla 8 campos
(`stops`, `center: Axes<Ratio>`, `radius: Ratio`, `focal_center:
Axes<Ratio>`, `focal_radius: Ratio`, `space: ColorSpace`,
`relative: Smart<RelativeTo>`, `anti_alias: bool`).
**Correcção palpite spec preliminar**: `focal_center`/`focal_radius`
**NÃO são Smart** — directos vanilla. `Axes<T>` ausente
cristalino → criado minimal.

**ADR criada/promovida**: **ADR-0088** "Gradient Radial
materializado; Conic scope-out preservado":
- Criada `PROPOSTO` em P264.B.
- Promovida `IMPLEMENTADO` em P264.D pós-materialização.
- Paridade pattern P257/P261/P262 N=4 cumulativo (**limiar
  formalização clara excedido**).

**Tests delta**: **2369 → 2386** (+17 P264 tests: 3 axes + 9
gradient.rs Radial + 5 stdlib radial; zero regressões).

**ADRs distribuição**:
- PROPOSTO 11 (preservado — ADR-0088 entra/sai mesmo passo).
- EM VIGOR 32 (preservado).
- **IMPLEMENTADO 27 → 28** (+0088 P264).
- **Total 74 → 75**.

**Ficheiros criados**:
- `00_nucleo/diagnosticos/diagnostico-gradient-radial-vanilla-passo-264.md`
  (imutável per ADR-0085 — **segundo consumo directo** pós-P262).
- `00_nucleo/adr/typst-adr-0088-gradient-radial-only.md`
  (PROPOSTO P264.B → IMPLEMENTADO P264.D).
- `00_nucleo/prompts/entities/axes.md` (L0 prompt novo;
  hash `c942a18a`).
- `01_core/src/entities/axes.rs` (Axes<T> minimal; 3 tests;
  hash `9b5d3f18`).
- `00_nucleo/materialization/typst-passo-264-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `00_nucleo/prompts/entities/gradient.md` (secção cumulativa
  P264 anotada; hash propagado).
- `01_core/src/entities/gradient.rs` (Radial struct +
  effective_offsets + sample Oklab + Gradient::Radial variant
  activada + radial() construtor + first_stop_color expand +
  9 tests; hash `911125dd`).
- `01_core/src/entities/mod.rs` (re-export `pub mod axes`).
- `01_core/src/rules/stdlib/gradients.rs` (native_gradient_radial
  + make_gradient_module entrada radial + parse_ratio helper).
- `01_core/src/rules/stdlib/mod.rs` (re-export
  native_gradient_radial + 5 stdlib tests P264).
- `03_infra/src/export.rs` (3 sítios pattern-match adaptados
  match Linear/Radial → fallback Solid Radial até P265).
- `00_nucleo/adr/typst-adr-0087-gradient-linear-only.md`
  (cross-reference cumulativa P264 anotada).
- `00_nucleo/adr/README.md` (entrada P264 + distribuição
  actualizada).

**~10 ficheiros tocados; ~250 LoC novas L1 + 17 tests**.

---

## §2 — Sub-passo P264.A — Diagnóstico Radial vanilla

### A.1 — Estrutura vanilla literal

```rust
// lab/typst-original/.../visualize/gradient.rs:1063
pub struct RadialGradient {
    pub stops: Vec<(Color, Ratio)>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    pub focal_center: Axes<Ratio>,    // NÃO Smart (palpite spec corrigido)
    pub focal_radius: Ratio,           // NÃO Smart
    pub space: ColorSpace,
    pub relative: Smart<RelativeTo>,
    pub anti_alias: bool,
}
```

8 campos vanilla. Wrapper `Gradient::Radial(Arc<RadialGradient>)`
paridade Linear P262.

### A.2 — Comparação Linear vs Radial

| Campo | Linear | Radial | Notas |
|-------|--------|--------|-------|
| `stops` | ✓ | ✓ | comum |
| `angle` | ✓ | — | Linear-specific |
| `center` | — | ✓ | Radial-specific |
| `radius` | — | ✓ | Radial-specific |
| `focal_center` | — | ✓ | Radial-specific (default = center) |
| `focal_radius` | — | ✓ | Radial-specific (default 0%) |
| `space/relative/anti_alias` | ✓ | ✓ | comuns |

### A.3 — Zero cascade refactor consumers

`Paint::Gradient(Gradient)` (P262) e `Value::Gradient(Gradient)`
(P262) são enum wrappers indiferentes ao variant interno.
Aceitam Radial automaticamente. **Sem cascade adicional**.

3 sítios pattern-match `let Gradient::Linear(_) = g`
identificados em `03_infra/src/export.rs` (scan_all_gradients,
pattern_resources_for_page, emit_stroke_paint) — exigem branch
Radial.

### A.4 — Decisões Q1-Q5

| Q | Decisão | Justificação |
|---|---------|--------------|
| Q1 — Materializar tudo? | **L1+stdlib only** | Pattern P262/P263 dividir granularidade N=2 (P265 PDF Radial dedicado) |
| Q2 — Interpolação Oklab? | **Sim** | Paridade P262; reutiliza helpers literal |
| Q3 — GradientStop Option<Ratio>? | **Sim** | Paridade P262 |
| Q4 — Focal point? | **Scope-out** | Default center + 0% radius; consumer raro |
| Q5 — Axes<T>? | **Criar minimal** | Vanilla usa amplamente; tuple perde semântica |

### A.5 — Decisão granularidade ADR

☑ **Opção α — ADR-0088 nova** (paridade pattern N=2
P261/P262 cada subset materializado tem ADR própria).

### A.6 — Axes<Ratio> ausente

```bash
$ grep -rn "pub struct Axes\b" 01_core/src/entities/
(zero hits)
```

**Decisão**: criar `entities/axes.rs` minimal (~25 LoC) +
`entities/axes.md` L0 + re-export em `entities/mod.rs`.

---

## §3 — Sub-passo P264.B — ADR-0088 criada PROPOSTO

Ficheiro novo
`00_nucleo/adr/typst-adr-0088-gradient-radial-only.md`:

- **Status**: `PROPOSTO` (em P264.B).
- **Estrutura**: contexto + subset materializado Radial 3
  campos + Axes<T> minimal criado + activação
  Gradient::Radial + stdlib `native_gradient_radial` + PDF
  shading adiado P265 + preservações ADR-0039 + scope-outs
  (7 incluindo PDF shading P265 dedicado) + consequências +
  alternativas + critério revisão + subpadrões + referências.

---

## §4 — Sub-passo P264.C — Materialização L1+stdlib

### C.1 — `entities/axes.md` L0 + `entities/axes.rs`

`Axes<T>` genérico minimal:
```rust
pub struct Axes<T> { pub x: T, pub y: T }
impl<T> Axes<T> { pub fn new(x: T, y: T) -> Self { ... } }
impl<T: Eq> Eq for Axes<T> {}
```

3 tests (`axes_new_armazena_x_y`, `axes_copy_clone`,
`axes_partial_eq`).

### C.2 — `entities/gradient.md` L0 secção P264

Secção "Anotação cumulativa P264 — Radial variant materializada"
adicionada ao fim com:
- Tipos adicionados (Radial struct).
- Enum Gradient expandido.
- Scope-outs P264 enumerados.
- Cross-references P264.

### C.3 — `entities/gradient.rs` actualizado

**Radial struct + impl**:
```rust
pub struct Radial {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
}

impl Radial {
    pub fn effective_offsets(&self) -> Vec<f32>;  // paridade Linear
    pub fn sample(&self, t: f32) -> Color;        // paridade Linear (Oklab)
}
```

**Gradient::Radial(Arc<Radial>) variant** activada (era
comentário reserva).

**`Gradient::radial(stops, center, radius)`** construtor novo.

**`Gradient::first_stop_color`** pattern-match expand para
cobrir Radial.

**9 tests P264 Radial**:
- `p264_radial_construcao_2_stops`.
- `p264_radial_first_stop_color`.
- `p264_radial_clone_arc_o1`.
- `p264_radial_partial_eq`.
- `p264_radial_effective_offsets_auto_spacing`.
- `p264_radial_sample_extremos`.
- `p264_radial_sample_clamp_above_1`.
- `p264_gradient_radial_to_paint_via_from`.
- `p264_radial_center_non_default`.

**Helpers Oklab reutilizados literal** de P262 (zero código
duplicado): `interpolate_oklab`, `color_to_oklab_with_alpha`,
`srgb_to_linear`, `linear_rgb_to_oklab`.

### C.4 — `entities/mod.rs` re-export

```rust
// P264 — Axes<T> minimal per ADR-0088 + ADR-0080.
pub mod axes;
```

### C.5 — Stdlib `native_gradient_radial`

```rust
pub fn native_gradient_radial(args, ...) -> SourceResult<Value>;
```

- Stops parsing reutiliza `parse_stops` paridade
  `native_gradient_linear`.
- Named `center: [Ratio, Ratio]` (default `(50%, 50%)`).
- Named `radius: Ratio` (default 50%; validação out-of-range
  → erro).
- Validações: stops vazios → erro; named desconhecido → erro.

`make_gradient_module()` ganha entrada `radial`.

Helper privado `parse_ratio` extraído (paridade pattern
`parse_stops` privado).

**5 stdlib tests P264** em `01_core/src/rules/stdlib/mod.rs`:
- `p264_gradient_radial_2_color_stops_defaults`.
- `p264_gradient_radial_custom_center_radius`.
- `p264_gradient_radial_zero_stops_erro`.
- `p264_gradient_radial_radius_out_of_range_erro`.
- `p264_gradient_radial_value_type_name`.

### C.6 — 3 sítios pattern-match `03_infra/src/export.rs` adaptados

Padrão `let Gradient::Linear(linear) = g;` (refutable agora
que Radial existe) → `match { Linear => ..., Radial =>
continue/fallback }`:

- **`scan_all_gradients`** (linha ~357): Radial → `continue`
  (não regista resource).
- **`pattern_resources_for_page`** (linha ~400): Radial →
  `continue` (não emit entry).
- **`emit_stroke_paint`** (linha ~1070): Radial → fallback
  Solid emit via `first_stop_color`.

**Resultado**: PDF mostra Radial como primeira cor literal
(paridade pré-P263 state Linear); shading real `/ShadingType 3`
adiado P265.

### C.7 — Verificação intermediária

- `cargo build --workspace` → verde.
- `cargo test --workspace --release` → **2369 → 2386** (+17;
  zero regressões).
- `crystalline-lint --fix-hashes .` → 2 hashes propagados
  (`axes.md` → `c942a18a` + código `9b5d3f18`; `gradient.md` →
  `391208e2` + código `911125dd`).
- `crystalline-lint .` → `✓ No violations found`.

---

## §5 — Sub-passo P264.D — Promoção ADR + relatório

### D.1 — ADR-0088 PROPOSTO → IMPLEMENTADO

Status actualizado em
`typst-adr-0088-gradient-radial-only.md`:
- Status: PROPOSTO → **IMPLEMENTADO**.
- Linha **Validado** + **Aplicação** apontando para este
  relatório.

### D.2 — README ADRs

- Tabela: entrada ADR-0088 adicionada com status `IMPLEMENTADO`.
- ADR-0087 ganha cross-reference cumulativa P264 (anotação
  documental).
- Distribuição: **IMPLEMENTADO 27 → 28**; PROPOSTO 11
  preservado (entra/sai); EM VIGOR 32 preservado; **total 74
  → 75**.
- Passos-chave: entrada P264 ~60 linhas descritivas paridade
  P262/P263 entradas.

### D.3 — Relatório (este ficheiro)

Estrutura canónica §1-§10.

---

## §6 — Padrões metodológicos

### Subpadrão "P262/P263 dividir granularidade L1+stdlib / L3" N=1 → N=2

Cumulativo:
- N=1 P262 (Linear L1+stdlib) → P263 (Linear PDF).
- **N=2 P264** (Radial L1+stdlib) → P265 (Radial PDF; futuro).

**Patamar N=2 reforça pattern**. Próxima aplicação candidata:
P-Gradient-Conic L1+stdlib + L3 PDF se materializar.

### Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo" N=3 → N=4

Cumulativo:
- N=1 P257 (ADR-0083 Color).
- N=2 P261 (ADR-0086 Paint).
- N=3 P262 (ADR-0087 Gradient Linear).
- **N=4 P264** (ADR-0088 Gradient Radial-only).

**Patamar N=4 excede limiar formalização clara**. Candidato a
meta-ADR — **improvável e desnecessário** (padrão
auto-documentado em cada ADR individual; análogo P156K
self-documentation).

### Subpadrão "Decisão minimalista (subset materializado)" N=3 → N=4

Cumulativo:
- N=1 P257 Color (8/8 + 4 scope-outs).
- N=2 P261 Paint (Solid only).
- N=3 P262 Gradient Linear only.
- **N=4 P264 Gradient Radial subset** (3 campos materializados;
  5 scope-outs incluindo focal_*).

**Pattern emergente sólido** confirma. Cada tipo wrapper
materializa subset minimal + comentários reserva activáveis em
passos dedicados futuros.

### Subpadrão "Diagnóstico imutável precedente à acção" — segundo consumo directo ADR-0085

- N=1-4 P255/P257/P258/P259 (audit Fase A).
- N=5 P262 (diagnóstico Gradient Linear vanilla — primeiro
  consumo directo pós-P260).
- **N=6 P264** (diagnóstico Gradient Radial vanilla —
  **segundo consumo directo** pós-P260).

**Patamar N=6 valida formalização P260 ADR-0085** retroactivamente
e reforça pattern.

### Anti-pattern: confiar em palpites informados na spec

Spec P264 §A.1 tinha palpite `focal_center: Smart<Axes<Ratio>>`
e `focal_radius: Smart<Ratio>`. **Vanilla literal** confirmou
**ambos NÃO Smart** — directos. P264.A registou correcção e
ajustou decisão arquitectural. **Vanilla read-first
explicitamente autorizado** per spec §0 cumprido — palpite
substituído por evidência factual.

---

## §7 — Cobertura

**Visualize agregado**:
- Pre-P262: ~52% (P259 audit).
- Pre-P263: ~58% (P262 Linear L1+stdlib +5pp).
- Pre-P264: ~63% (P263 Linear PDF +5pp).
- **Pós-P264: ~68%** (+5pp via Radial L1+stdlib; F.2 Radial
  promovido ausente → implementado L1+stdlib; PDF render
  Radial fica fallback até P265).

**Entradas P259 Tabela A actualizadas pós-P264**:
- F.1 Gradient Linear: `implementado+stdlib+render` (P262+P263).
- F.2 Gradient Radial: ausente → **`implementado+stdlib`**
  (P264; PDF render fica P265).
- F.3 Gradient Conic: ausente (scope-out ADR-0088 →
  P-Gradient-Conic dedicado).

**Cobertura agregada user-facing total**: ~75-76% preservado.
**Layout Fase 5**: ~98-99% preservado.
**Math**: DEBT-8 ENCERRADO P255 preservado.
**Model**: ~73% pós-P258 preservado.
**Color** subsistema: 100% estrutural preservado.
**Visualize agregado**: **~68% pós-P264** (+10pp pós-P262
cumulativo).

---

## §8 — Limitações e trabalho futuro

### Pendências residuais P264 (não-bloqueantes; candidatos passos dedicados)

**PDF render Radial dedicado**:
1. **P265 — Gradient Radial PDF shading complete** (S-M
   dedicado; replica P263 template; `/ShadingType 3` radial;
   fecha promessa P264; ~250 LoC L3).

**Cluster Gradient extensões adicionais**:
2. **P-Gradient-Conic L1+stdlib** (M; replica P264 pattern;
   activa `Gradient::Conic` variant).
3. **P-Gradient-Conic PDF** (S-M; replica P263/P265 template).
4. **P-Gradient-Focal** (M; activa `focal_center` +
   `focal_radius` campos Radial).

**Cluster Stroke refinos (Opção 3 P259)**:
5. **DEBT-33 Bézier bbox exacto** (S+).
6. **Stroke<Length>** (M).
7. **Dash patterns**.
8. **LineCap/LineJoin/MiterLimit**.

**Cluster Shapes**:
9. **Polygon variant estrutural separada**.
10. **Curve variant**.

**Cluster Image (Opção 5)**:
11. **SVG image format** (L+; requer ADR `usvg`/`resvg`).
12. **Image metadata** `alt`/`fit` (S).

**Transform**:
13. **`origin` pivot** (scope-out ADR-0061 preservado).

**Tiling**:
14. **Tiling pattern** — pré-requisito Paint::Tiling activar.

### Sem ADR nova além de ADR-0088

Política P158 "sem novas reservas" preservada. ADR-0088 criada
e promovida no mesmo passo (PROPOSTO transitório).

### Sem DEBT novo aberto

Saldo DEBTs preservado.

---

## §9 — Critério de aceitação global P264 — Checklist final

- [x] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [x] `cargo test --workspace --release` retorna **2386 verdes**
  (+17 vs baseline 2369; sem regressão).
- [x] `diagnostico-gradient-radial-vanilla-passo-264.md` existe
  com §1-§9 preenchidos.
- [x] ADR-0088 criada PROPOSTO P264.B → IMPLEMENTADO P264.D
  (Opção α).
- [x] `entities/gradient.md` L0 actualizado com secção P264.
- [x] `entities/axes.md` L0 criado.
- [x] `entities/gradient.rs` `Gradient::Radial(Arc<Radial>)`
  activado; struct Radial materializado; 9 tests verdes.
- [x] `entities/axes.rs` Axes<T> minimal; 3 tests verdes.
- [x] `entities/mod.rs` re-export Axes adicionado.
- [x] Stdlib `native_gradient_radial` + `make_gradient_module`
  expandido com `radial`; 5 tests stdlib P264.
- [x] `scope.define("gradient", ...)` em eval/mod.rs cobre
  Radial via namespace.
- [x] **ADR-0039 preservada literal** (TextStyle.fill: Color
  inalterado).
- [x] **Paint::Gradient + Value::Gradient absorvem Radial
  automaticamente** (zero cascade refactor).
- [x] **PDF render Radial fallback Solid** até P265 (paridade
  pré-P263 state Linear; 3 sítios pattern-match em export.rs
  adaptados).
- [x] Hashes propagados (`axes.md` → `c942a18a` + código
  `9b5d3f18`; `gradient.md` → `391208e2` + código `911125dd`).
- [x] README ADRs actualizado (distribuição 74 → 75; entrada
  P264; ADR-0088 IMPLEMENTADO).
- [x] Relatório criado.
- [x] Paridade observable parcial: user-facing
  `gradient.radial(...)` funcional via parsing; PDF mostra
  fallback primeira cor até P265.

**Estado pós-P264**:
- Tests workspace: **2386 verdes** (+17 vs baseline 2369).
- Hash drift: zero.
- Lint: zero violations.
- DEBTs saldo: **10 preservado**.
- ADRs distribuição: PROPOSTO 11; IDEIA 2; EM VIGOR 32;
  **IMPLEMENTADO 28**; REVOGADO 2; ADIADO 1; **total 75**.
- Prompts L0 criados/editados: 2 (axes.md novo + gradient.md
  anotado).
- Diagnóstico imutável criado: 1 (**segundo consumo directo
  ADR-0085**).
- ADRs criadas: 1 (`IMPLEMENTADO` mesmo passo via paridade
  P257/P261/P262).
- **45 aplicações cumulativas anti-inflação** pós-P205D
  preservadas.

**Marco P264**: **Gradient Radial L1+stdlib materializado**;
user-facing `gradient.radial(red, blue, center: (40%, 60%),
radius: 70%)` funcional via parsing; activa `Gradient::Radial`
variant; **PDF shading Radial real adiado P265 dedicado**
preservando pattern "P262/P263 dividir granularidade" N=2.

**Recomendação subjectiva pós-P264**:

- **P265 PDF Radial shading** (S-M dedicado; replica P263
  template; `/ShadingType 3` radial; fecha promessa P264 +
  paridade sequência P262→P263 bem-sucedida).
- **OU P-Gradient-Conic L1+stdlib** (M; replica P264 pattern;
  activa último variant Gradient).
- **OU outras Opções P259 alternativas**:
  - DEBT-33 Bézier bbox + Stroke<Length> Opção 3.
  - Curve variant + Polygon estrutural separada Opção 2.
- **OU Text audit** (segundo audit pós-formalização P260).
- **OU P-Footnote-N** refino M (P258 pendência residual).

**Decisão humana fica em aberto literal** pós-P264.

---

## §10 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- **ADR-0088** (criada PROPOSTO P264.B → IMPLEMENTADO P264.D) —
  Gradient Radial-only.
- **ADR-0087** §"Critério revisão" — **cumprido parcialmente**
  por este passo (Radial activado; Conic continua scope-out);
  cross-reference cumulativa P264 anotada.
- **ADR-0086** — Paint wrapper (`Paint::Gradient` activa P261/P262
  absorve Radial sem cascade).
- **ADR-0085** — Diagnóstico imutável (**segundo consumo
  directo** P264; valida formalização P260 retroactivamente).
- **ADR-0083** — Color paridade vanilla (precedente N=2 pattern).
- **ADR-0084** — Auditoria condicional (P260; consumido
  metodologicamente via inventário inline P264.A).
- ADR-0080 — L0 minimal para refactors aditivos (cumprido em
  `axes.md` + `gradient.md` cumulativo).
- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório.
- ADR-0033 — Paridade observable vanilla.
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded (scope-outs aceites).
- ADR-0061 — Granularidade 1-2 features/passo (cumprido via
  divisão P264/P265).
- ADR-0065 — Inventariar primeiro.
- DEBT-1 — Fechado P142 (preservado).
- Aplicações precedentes do pattern:
  - P252 — Stroke `overhang` (precedente N=1 cross-cutting).
  - P257 — Color 8/8 (precedente N=2 pattern PROPOSTO+IMPL).
  - P261 — Paint wrapper Solid only (precedente N=3).
  - **P262** — Gradient L1+stdlib (precedente directo N=4;
    **template literal P264** — helpers Oklab reutilizados).
  - **P263** — Gradient Linear PDF (template literal P265
    futuro; subpadrão dividir granularidade N=1).
- P259 §3 Opção 1 sub-passo 2 — spec preliminar extensão.
- P260 — ADRs meta (formaliza ADR-0085 consumido directamente).
- `00_nucleo/diagnosticos/diagnostico-gradient-radial-vanilla-passo-264.md`
  — diagnóstico imutável P264.A (segundo consumo directo).
- `00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
  — diagnóstico Linear precedente directo.
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  — fonte canónica (1366 linhas; RadialGradient §1063-1080;
  8 campos).

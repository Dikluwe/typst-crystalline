# Relatório do passo P261 — Paint wrapper enum (Solid only) via ADR-0086

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-261.md`.
**Tipo**: passo composto sequencial; ADR PROPOSTO+IMPLEMENTADO
mesmo passo (paridade P257 pattern N=2 cumulativo).
**Análogo estrutural canónico**: P257 (Color paridade vanilla
com subset materializado P25 → P257).
**Magnitude planeada**: S+ (M cap).
**Magnitude real**: **S+ (~1-1.5h)** — cascade ~30 sítios via
sed batch + 4 PDF exporter + 7 paint tests; magnitude controlada.

---

## §1 — Sumário executivo

**Fase A confirmada**: vanilla Paint enum 3 variants (Solid/
Gradient/Tiling); cristalino pré-P261 usa `Stroke.paint: Color`
directo; consumers ~30 sítios identificados.

**ADR criada/promovida**: **ADR-0086** "Paint wrapper enum com
subset materializado (Solid only)":
- Criada `PROPOSTO` em P261.B.
- Promovida `IMPLEMENTADO` em P261.D pós-materialização.
- Paridade pattern P257 ADR-0083 (N=2 cumulativo).

**Tests delta**: **2334 → 2341** (+7 paint tests; zero
regressões).

**ADRs distribuição**:
- PROPOSTO 11 (inalterado — ADR-0086 entra e sai no mesmo passo).
- EM VIGOR 32 (inalterado).
- **IMPLEMENTADO 25 → 26** (+0086 P261).
- **Total 72 → 73**.

**Ficheiros criados**:
- `00_nucleo/diagnosticos/diagnostico-paint-vanilla-passo-261.md`
  (imutável per ADR-0085; §1-§7 preenchidos).
- `00_nucleo/adr/typst-adr-0086-paint-wrapper-solid-only.md`
  (criado PROPOSTO P261.B → promovido IMPLEMENTADO P261.D).
- `00_nucleo/prompts/entities/paint.md` (L0 prompt novo;
  hash `f9855284`).
- `01_core/src/entities/paint.rs` (enum Paint + impl + From +
  7 tests).
- `00_nucleo/materialization/typst-passo-261-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `01_core/src/entities/mod.rs` (re-export `pub mod paint;`).
- `01_core/src/entities/geometry.rs` (Stroke.paint Color → Paint;
  import `use Paint`; testes adaptados).
- `01_core/src/entities/content.rs` (~9 sítios construção
  Stroke + import Paint).
- `01_core/src/rules/layout/mod.rs` (1 sítio Divider stroke +
  import Paint).
- `01_core/src/rules/layout/tests.rs` (~21 sítios tests +
  import Paint).
- `01_core/src/rules/stdlib/layout.rs` (4 sítios — extract_stroke
  + native_stroke + 2 imports Paint).
- `01_core/src/rules/stdlib/shapes.rs` (~8 sítios + import Paint).
- `01_core/src/rules/stdlib/mod.rs` (5 sítios tests + 3 imports
  Paint inline).
- `03_infra/src/export.rs` (4 sítios `s.paint.to_rgba_f32()` →
  `s.paint.to_color().to_rgba_f32()`).
- `00_nucleo/adr/typst-adr-0086-...md` (status PROPOSTO →
  IMPLEMENTADO P261.D).
- `00_nucleo/adr/README.md` (tabela + distribuição + entrada
  P261).

**Total ~52 sítios edits cross-cutting**.

---

## §2 — Sub-passo P261.A — Fase A diagnóstico Paint vanilla

### A.1 — Estrutura vanilla literal

```rust
// lab/typst-original/.../visualize/paint.rs:10
pub enum Paint {
    Solid(Color),
    Gradient(Gradient),
    Tiling(Tiling),
}
```

5 métodos + 4 conversões (blanket `From<T: Into<Color>>` +
`From<Tiling>` + `From<Gradient>` + `unwrap_solid()`).

### A.2 — Consumers cristalino (impacto)

- **Stroke construções ~22 sítios literais** + ~8 em tests
  layout/tests.rs = ~30 sítios total.
- **PDF exporter 4 sítios** (export.rs:863/1125/1371/1553).
- **Style::Fill(Color)** + **TextStyle.fill: Option<Color>**:
  NÃO TOCA P261 (ADR-0039 preservado).
- **Stdlib native_rgb**: NÃO TOCA (continua retornar
  `Value::Color`).

### A.3 — Decisão Solid only confirmada

| Variant | Status P261 | Razão |
|---------|-------------|-------|
| Solid(Color) | Materializar | wrapper Stroke.paint |
| Gradient | Comentário reserva | Sem Gradient L1; P262 |
| Tiling | Comentário reserva | Sem Tiling L1 |

### A.4 — Decisão granularidade ADR

☑ **Opção α — ADR-0086 nova**. Paridade ADR-0083 (cada tipo
vanilla com ADR próprio).

---

## §3 — Sub-passo P261.B — ADR-0086 criada PROPOSTO

Ficheiro novo
`00_nucleo/adr/typst-adr-0086-paint-wrapper-solid-only.md`:

- **Status**: `PROPOSTO` (em P261.B).
- **Estrutura**: contexto + decisão subset Solid only + adaptação
  Stroke.paint + preservações ADR-0039/Style::Fill/Stdlib +
  scope-outs documentados + compatibilidade pré-existente +
  consequências + alternativas + critério revisão + subpadrões
  + referências.

---

## §4 — Sub-passo P261.C — Materialização

### C.1 — L0 prompt `entities/paint.md`

Ficheiro novo com estrutura análoga `entities/color.md` (5725
bytes color.md vs 2500 bytes paint.md — minimal). Cobertura:
estrutura enum + impl + From<Color> + critérios verificação +
notas paridade vanilla + ADR-0039 preservado + Style::Fill
preservado + stdlib intocado + cross-references.

Hash inicial `00000000`; propagado para `f9855284` via
`crystalline-lint --fix-hashes`.

### C.2 — `entities/paint.rs` materializado

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Paint {
    Solid(Color),
    // Gradient(Gradient),  // P262 — comentário reserva
    // Tiling(Tiling),      // futuro — comentário reserva
}

impl Paint {
    pub fn solid(c: Color) -> Self { Paint::Solid(c) }
    pub fn to_color(&self) -> Color { match self { Paint::Solid(c) => *c } }
}

impl From<Color> for Paint {
    fn from(c: Color) -> Self { Paint::Solid(c) }
}
```

**7 tests**:
1. `paint_solid_construcao`.
2. `paint_to_color_solid`.
3. `paint_to_color_solid_via_solid_helper`.
4. `paint_from_color`.
5. `paint_partial_eq`.
6. `paint_copy_clone`.
7. `paint_debug_format`.

### C.3 — `entities/mod.rs` re-export

```rust
// P261 — Paint wrapper enum (Solid only) per ADR-0086.
pub mod paint;
```

### C.4 — `entities/geometry.rs` Stroke.paint Color → Paint

```rust
pub struct Stroke {
    /// **P261** — Paint wrapper enum (Solid only) per ADR-0086.
    pub paint:     Paint,
    pub thickness: f64,
    pub overhang:  bool,
}
```

Import `use crate::entities::paint::Paint;` adicionado. Test
interno adaptado.

### C.5 — Cascade ~30 sítios construção `Stroke { paint: ... }`

Pattern aplicado via `sed` batch + `perl` regex:
- `paint: Color::rgb(N, N, N)` → `paint: Paint::Solid(Color::rgb(N, N, N))`.
- `paint: c,` (variável) → `paint: Paint::Solid(c),`.
- `paint: *c,` (deref) → `paint: Paint::Solid(*c),`.
- `paint: stroke_color,` → `paint: Paint::Solid(stroke_color),`.
- `Stroke { paint, thickness, overhang }` (shorthand) →
  `Stroke { paint: Paint::Solid(paint), thickness, overhang }`.

**Imports `use Paint` adicionados** em 8 sítios (content.rs +
layout/mod.rs + layout/tests.rs + stdlib/layout.rs ×2 +
stdlib/shapes.rs + stdlib/mod.rs ×3).

### C.6 — 4 sítios PDF exporter

```bash
$ sed -i 's/s.paint.to_rgba_f32/s.paint.to_color().to_rgba_f32/g' \
  03_infra/src/export.rs
```

Substituição literal 4×. Wrapper transparente `Paint::to_color()`
encadeia com `Color::to_rgba_f32()`.

### C.7 — Verificação intermediária

- `cargo build --workspace` → verde após `use Paint` adicionado
  em todos os ficheiros consumidores.
- `cargo test --workspace --release` → 2334 → **2341 (+7
  tests paint; zero regressões)**.
- `crystalline-lint --fix-hashes .` → `entities/paint.md` hash
  propagado para `f9855284`.
- `crystalline-lint .` → `✓ No violations found`.

---

## §5 — Sub-passo P261.D — Promoção ADR + README + relatório

### D.1 — ADR-0086 PROPOSTO → IMPLEMENTADO

Status atualizado em
`typst-adr-0086-paint-wrapper-solid-only.md`:
- Status: PROPOSTO → **IMPLEMENTADO**.
- Linha **Validado** + **Aplicação** apontando para este
  relatório.

### D.2 — README ADRs

- Tabela: entrada ADR-0086 adicionada com status `IMPLEMENTADO`.
- Distribuição: **IMPLEMENTADO 25 → 26**; PROPOSTO 11 preservado
  (entra/sai); EM VIGOR 32 preservado; **total 72 → 73**.
- Passos-chave: entrada P261 ~50 linhas descritivas paridade
  P257/P259/P260.

### D.3 — Relatório (este ficheiro)

Estrutura canónica §1-§10.

---

## §6 — Padrões metodológicos

### Primeiro consumo indirecto ADR-0085 pós-P260

P260 formalizou ADR-0084 (auditoria condicional) + ADR-0085
(diagnóstico imutável). P261 **não** é audit Fase A (é
materialização arquitectural cumprindo ADR-0029 §"Diagnosticar
primeiro"), mas o diagnóstico Paint vanilla
(`diagnostico-paint-vanilla-passo-261.md`) **cumpre forma
análoga ADR-0085** — imutável, em
`00_nucleo/diagnosticos/`, com cabeçalho canónico.

Próximo passo (P262 Gradient Linear) consumirá ADR-0085
explicitamente ao produzir diagnóstico Gradient vanilla imutável.

### Subpadrão "Refactor cross-cutting entity primitivo" N=2 → N=3

Cumulativo:
- N=1 P252 (Stroke `overhang` cross-cutting).
- N=2 P257 (Color expansão cross-cutting; toca exporter +
  stdlib + variants).
- **N=3 P261** (Paint cross-cutting Stroke.paint via
  Paint::Solid wrapper).

**Patamar N=3 atinge limiar formalização sólida**. Candidato
meta-ADR (improvável; padrão auto-documentado em ADR individual
de cada aplicação).

### Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo via Cenário B1/B2" N=1 → N=2

Cumulativo:
- N=1 P257 (ADR-0083 Color paridade vanilla).
- **N=2 P261** (ADR-0086 Paint wrapper Solid only).

**Patamar N=2 reforça pattern**. N=3 começará a ser candidato
formalização meta (improvável).

### Decisão minimalista (paridade P25 → P257)

Subset Solid only com variants Gradient/Tiling comentários
reserva no enum (não unit placeholders). Expansão consumer-
driven em P262+. Política P158 "sem novas reservas" preservada.

### Preservações arquitecturais críticas

- **ADR-0039 SR-Struct Resolvido** preservado literal
  (TextStyle.fill: Option<Color>).
- **DEBT-1** fechado P142 preservado.
- **Stdlib `native_rgb`/etc.** intocado (continua
  `Value::Color`).
- **Style::Fill(Color)** StyleChain variant inalterado.

---

## §7 — Cobertura

**Visualize agregado** (Tabela B P259):
- Pre-P261: ~51.9% ponderado linear (P259 audit).
- **Pós-P261: ~53% ponderado linear** (+1pp estructural).
- Entrada G "Paint wrapper" P259 promovida ausente →
  implementado.

**Cobertura agregada user-facing total**: ~75-76% preservado.
**Layout Fase 5**: ~98-99% preservado.
**Math**: DEBT-8 ENCERRADO P255 preservado.
**Model**: ~73% pós-P258 preservado.
**Color** subsistema: 100% estrutural preservado.

---

## §8 — Limitações e trabalho futuro

### Pendências residuais P261 (não-bloqueantes)

**Cluster Gradient (sequência preferida P262)**:
1. **Gradient Linear** (F.1) — após Paint enum; P262 candidato
   M (~3-4h; +15-20 tests; +8pp). Activa `Paint::Gradient`
   variant. Exige expansão PDF exporter (shading patterns
   `/Pattern /Shading /ShadingType 2`).
2. **Gradient Radial/Conic** (F.2/F.3) — deferidos pós-Linear.

**Cluster Stroke refinos (Opção 3 P259)**:
3. **DEBT-33 Bézier bbox exacto** (S+; ~+5 tests).
4. **Stroke<Length>** (M; ~+10-15 tests; refactor
   cross-cutting).
5. **Dash patterns**.
6. **LineCap/LineJoin/MiterLimit**.

**Cluster Shapes (parcial pré-cumprida)**:
7. **Polygon variant estrutural separada**.
8. **Curve variant**.

**Cluster Image (Opção 5)**:
9. **SVG image format** (L+; requer ADR `usvg`/`resvg`).
10. **Image metadata** `alt`/`fit` (S).

**Transform**:
11. **`origin` pivot** (scope-out ADR-0061 preservado).

**Tiling**:
12. **Tiling pattern** — vanilla feature; baixa prioridade;
    pré-requisito Paint::Tiling activar.

### Possível migração futura TextStyle.fill → Option<Paint>

Refino futuro pode migrar `TextStyle.fill` para `Option<Paint>`
se **Gradient para texto** for prioritário (ADR-0039 SR
preserved actualmente). Decisão fora do scope P261.

### Sem ADR nova aberta

Política P158 "sem novas reservas" preservada. ADR-0086 criada
e promovida no mesmo passo (PROPOSTO transitório).

### Sem DEBT novo aberto

Saldo DEBTs preservado.

---

## §9 — Critério de aceitação global P261 — Checklist final

- [x] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [x] `cargo test --workspace --release` retorna **2341 verdes**
  (+7 vs baseline 2334; sem regressão).
- [x] `diagnostico-paint-vanilla-passo-261.md` existe com
  §1-§7 preenchidos.
- [x] ADR-0086 criada PROPOSTO P261.B → IMPLEMENTADO P261.D
  (Opção α).
- [x] `00_nucleo/prompts/entities/paint.md` criado.
- [x] `01_core/src/entities/paint.rs` materializado com 7 tests
  verdes.
- [x] `01_core/src/entities/geometry.rs` `Stroke.paint: Paint`.
- [x] `entities/mod.rs` re-export Paint adicionado.
- [x] Consumers Stroke.paint adaptados (~30 sítios cascade
  via sed batch).
- [x] **TextStyle.fill: Option<Color> preservado literal**
  (ADR-0039 intacto).
- [x] **Stdlib native_rgb continua retornar Value::Color**
  (sem alteração user-facing).
- [x] Exportador PDF 4 sítios adaptados (`s.paint.to_color().to_rgba_f32()`).
- [x] Hashes propagados (`entities/paint.md` → `f9855284`).
- [x] README ADRs actualizado (distribuição 72 → 73; entrada
  P261; ADR-0086 IMPLEMENTADO).
- [x] Relatório do passo criado.
- [x] Paridade observable preservada (Paint::Solid(c)
  transparente para PDF output — wrapper sem mudança semântica).

**Estado pós-P261**:
- Tests workspace: **2341 verdes** (+7 vs baseline 2334).
- Hash drift: zero.
- Lint: zero violations.
- DEBTs saldo: **10 preservado**.
- ADRs distribuição: PROPOSTO 11; IDEIA 2; EM VIGOR 32;
  **IMPLEMENTADO 26**; REVOGADO 2; ADIADO 1; **total 73**.
- Prompts L0 criados/editados: 2 (paint.md novo + geometry.md
  via fix-hashes anterior).
- Diagnóstico imutável criado: 1.
- ADRs criadas: 1 (`IMPLEMENTADO` mesmo passo via paridade
  P257).
- **45 aplicações cumulativas anti-inflação** pós-P205D
  preservadas.

**Marco P261**: **Paint wrapper Solid only materializado**;
abre caminho arquitectural para P262 Gradient Linear (sequência
preferida P259 Cenário B2 Opção 1 sub-passo 2). Paridade
vanilla observable preservada via `Paint::Solid` transparente.

**Recomendação subjectiva pós-P261**:

- **P262 Gradient Linear** (M; sequência preferida; activa
  `Paint::Gradient` variant; +15-20 tests; +8pp Visualize;
  exige expansão PDF exporter shading patterns).
- **OU Opções P259 alternativas**:
  - DEBT-33 Bézier bbox + Stroke<Length> Opção 3.
  - Curve variant + Polygon estrutural separada Opção 2.
- **OU Text audit** (consumo directo ADR-0084 + 0085 —
  primeiro audit pós-formalização).
- **OU P-Footnote-N** refino M (P258 pendência residual).

**Decisão humana fica em aberto literal** pós-P261.

---

## §10 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- **ADR-0086** (criada PROPOSTO P261.B → IMPLEMENTADO P261.D) —
  Paint wrapper Solid only.
- **ADR-0083** — Color paridade vanilla (precedente N=2 do
  pattern; análogo estrutural).
- **ADR-0084, ADR-0085** — Auditoria condicional + diagnóstico
  imutável (P260; primeiro consumo indirecto via diagnóstico
  Paint vanilla cumprir forma análoga).
- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório
  (regra principal cumprida).
- ADR-0033 — Paridade observable vanilla.
- ADR-0034 — Diagnóstico canónico (estendido por ADR-0085).
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded.
- ADR-0065 — Inventariar primeiro.
- DEBT-1 — Fechado P142 (preservado).
- Aplicações precedentes do pattern:
  - P25 — Color simplificado original (REVOGADO via P257).
  - P252 — Stroke `overhang` cross-cutting (precedente N=1 do
    refactor).
  - P257 — Color paridade vanilla 8/8 (precedente N=2;
    template PROPOSTO+IMPLEMENTADO mesmo passo).
- P259 §3 Opção 1 — spec preliminar Paint enum + Gradient
  Linear.
- P260 — ADRs meta (formaliza padrões consumidos).
- `00_nucleo/diagnosticos/diagnostico-paint-vanilla-passo-261.md`
  — diagnóstico imutável P261.A.
- `00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md`
  — audit precedente Visualize.
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/paint.rs`
  — fonte canónica (3 variants + 5 métodos + 4 conversões).

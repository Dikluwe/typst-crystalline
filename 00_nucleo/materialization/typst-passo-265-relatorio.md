# Relatório do passo P265 — PDF Radial shading complete (fecha promessa P264)

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-265.md`.
**Tipo**: passo composto sequencial L3 cross-cutting;
**subpadrão "P262/P263 dividir granularidade" N=2 → N=3
cumulativo atinge limiar formalização clara**.
**Análogo estrutural canónico directo**: P263 (Gradient Linear
PDF shading complete).
**Magnitude planeada**: S-M (M cap; ~2-4h).
**Magnitude real**: **S-M (~2h)** — ~150 LoC L3 novas; ~70%
código reutilizado de P263 (helpers + 3 paths
build_page_stream_* sem modificação); 7 tests P265.

---

## §1 — Sumário executivo

**Fase A confirmada inline**: 5 helpers P263 presentes;
3 sítios pattern-match P264 com fallback Solid (linhas 360,
403, 1073 em `03_infra/src/export.rs`); `GradientObject`
precisa generalizar de `linear: Arc<Linear>` para enum
Linear/Radial.

**Sem ADR nova** — ADR-0088 já cobre Gradient Radial; P265
materializa o backend PDF via **anotação cumulativa**
(paridade pattern P263 anotação ADR-0087 + ADR-0080
§"refactor aditivo"). Status `IMPLEMENTADO` preservado.

**Tests delta**: **2386 → 2393** (+7 P265; zero regressões).

**ADRs distribuição**:
- PROPOSTO 11 (preservado).
- EM VIGOR 32 (preservado).
- IMPLEMENTADO 28 (preservado — anotação cumulativa não muda
  status).
- **Total 75** preservado.

**Ficheiros criados**:
- `00_nucleo/materialization/typst-passo-265-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `00_nucleo/prompts/infra/export.md` (secção P265 anotada;
  hash `bf71181c`).
- `03_infra/src/export.rs` (~150 LoC novas: enum local
  `GradientObjectKind` + 2 helpers novos + emit_gradient_objects
  expandido branching + 3 sítios pattern-match substituídos +
  7 tests P265).
- `00_nucleo/adr/typst-adr-0088-gradient-radial-only.md`
  (secção "Anotação cumulativa P265" adicionada).
- `00_nucleo/adr/README.md` (entrada P265 + cross-reference
  cumulativa P265 em ADR-0088 linha).

**~4 ficheiros tocados; ~150 LoC L3 + 7 tests + L0 + ADR
anotação**.

---

## §2 — Sub-passo P265.A — Inventário inline (auto-aplicação ADR-0065)

### A.1 — Helpers P263 + sítios P264

```bash
$ grep -n "fn compute_axial_coords\|fn oklab_sample_stops\|..." 03_infra/src/export.rs
339: fn scan_all_gradients
386: fn pattern_resources_for_page
427: fn compute_axial_coords
443: fn oklab_sample_stops
465: fn emit_function_dict
955: emit_gradient_objects (método PdfBuilder)
1055: fn emit_stroke_paint
```

3 sítios pattern-match P264 com fallback Solid:
- 360: `scan_all_gradients` — `Gradient::Radial(_) => continue`.
- 403: `pattern_resources_for_page` — `Gradient::Radial(_) => continue`.
- 1073: `emit_stroke_paint` — Radial → fallback Solid emit.

`GradientObject` actualmente `linear: Arc<Linear>` (apenas
Linear). Precisa generalizar.

### A.2 — Decisões D1-D5

**D1 Generalização `oklab_sample_stops`**: ☑ **Opção α
duplicação explícita** — criar `oklab_sample_stops_radial`
paridade literal. Sem trait machinery; mais legível.

**D2 `compute_radial_coords` algoritmo**: 6 valores Coords
`[x0 y0 r0 x1 y1 r1]`; círculos concêntricos (foco pontual
no center, target radius):
- `(x0, y0, r0) = (cx, cy, 0.0)`.
- `(x1, y1, r1) = (cx, cy, r)`.
- Subset materializado P264 (focal_* scope-out).

**D3 Pattern_resources chave**: HashMap genérico
`<usize, PatternRef>` preservado P263; `Arc::as_ptr(radial)`
funciona idêntico a `Arc::as_ptr(linear)`.

**D4 Cross-path cobertura**: branching unificado em
`emit_stroke_paint` (helper P263 ganha branch Radial);
**zero modificação em 3 paths build_page_stream_***.

**D5 Function reutilizada**: `emit_function_dict` aceita
stops genéricos `&[(f32, f32, f32)]` — idêntico para Linear
e Radial.

---

## §3 — Sub-passo P265.B — L0 export.md actualizado

Secção nova "Suporte Gradient Radial via Shading Patterns
(Passo 265)" adicionada ao fim de
`00_nucleo/prompts/infra/export.md`:

- Pattern resources reutilizado P263 (HashMap genérico).
- GradientObject expandido para enum local
  `GradientObjectKind`.
- Shading Type 3 (radial) — materializado.
- Radial subset materializado P264 (círculos concêntricos).
- Conversão Ratio → pontos absolutos (Coords radial).
- 3 helpers internos novos enumerados.
- Helpers reutilizados P263 (sem alteração).
- 3 sítios pattern-match P264 substituídos.
- Limitações P265.

Hash propagado via `--fix-hashes`: `export.md` → `bf71181c`.

---

## §4 — Sub-passo P265.C — Materialização L3

### C.1 — Tipo `GradientObjectKind` enum local

```rust
enum GradientObjectKind {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),
}

struct GradientObject {
    kind: GradientObjectKind,          // antes: linear: Arc<Linear>
    function_id, shading_id, pattern_id,
}
```

### C.2 — Helpers novos

**`compute_radial_coords`**:
```rust
fn compute_radial_coords(
    center: Axes<Ratio>,
    radius: Ratio,
    w: f64,
    h: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let cx = center.x.0 * w;
    let cy = center.y.0 * h;
    let r = radius.0 * w.min(h);
    (cx, cy, 0.0, cx, cy, r)   // círculos concêntricos
}
```

**`oklab_sample_stops_radial`**:
```rust
fn oklab_sample_stops_radial(radial: &Radial, n_samples: usize)
    -> Vec<(f32, f32, f32)>
{
    let n = n_samples.max(2);
    (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let c = radial.sample(t);  // L1 helper P264 reutilizado
            let (r, g, b, _) = c.to_rgba_f32();
            (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
        })
        .collect()
}
```

**Paridade literal `oklab_sample_stops` (P263)** — apenas
diferindo no tipo Radial vs Linear.

### C.3 — `emit_gradient_objects` expandido branching

```rust
let (stops, shading_dict) = match &kind {
    GradientObjectKind::Linear(linear) => {
        let stops = oklab_sample_stops(linear, 16);
        let (x0, y0, x1, y1) = compute_axial_coords(...);
        let shading = format!("/ShadingType 2 ... /Extend [false false]");
        (stops, shading)
    }
    GradientObjectKind::Radial(radial) => {
        let stops = oklab_sample_stops_radial(radial, 16);
        let (x0, y0, r0, x1, y1, r1) = compute_radial_coords(...);
        let shading = format!("/ShadingType 3 ... /Extend [true true]");
        (stops, shading)
    }
};
// Restante emit (Function + Pattern dicts) idêntico P263.
```

### C.4 — `emit_stroke_paint` branching unificado

```rust
Paint::Gradient(g) => {
    let ptr = match g {
        Gradient::Linear(l) => Arc::as_ptr(l) as usize,
        Gradient::Radial(r) => Arc::as_ptr(r) as usize,
    };
    if let Some(&idx) = pat_ptr_to_idx.get(&ptr) {
        let r = &pat_refs[idx];
        ops.push_str(&format!("/Pattern CS\n/{} SCN\n{:.2} w\n", r.name, thickness));
    } else {
        // Fallback paranóide.
    }
}
```

Fallback Solid P264 substituído por lookup unificado.

### C.5 — `scan_all_gradients` + `pattern_resources_for_page`

```rust
// scan_all_gradients (linha 356 area):
let (ptr, kind) = match g {
    Gradient::Linear(l) => (Arc::as_ptr(l) as usize,
                            GradientObjectKind::Linear(Arc::clone(l))),
    Gradient::Radial(r) => (Arc::as_ptr(r) as usize,
                            GradientObjectKind::Radial(Arc::clone(r))),
};

// pattern_resources_for_page (linha 408 area):
let ptr = match g {
    Gradient::Linear(l) => Arc::as_ptr(l) as usize,
    Gradient::Radial(r) => Arc::as_ptr(r) as usize,
};
```

### C.6 — Reutilizações literais P263 (sem modificação)

- **`emit_function_dict`** (Type 2 / Type 3 stitching) —
  idêntico; aceita stops genéricos.
- **`emit_pattern_dict`** inline em `emit_gradient_objects` —
  idêntico (PatternType 2 wrapper).
- **`pattern_resources: HashMap<usize, PatternRef>`** —
  estrutura genérica preservada.
- **3 paths `build_helvetica/cidfont/multifont`** — zero
  modificação (branching unificado via `emit_stroke_paint`
  helper P263).
- **`Radial::sample(t)`** L1 P264 — reutilizado em
  `oklab_sample_stops_radial`.

### C.7 — 7 Tests P265

**Unit helpers** (4):
- `p265_compute_radial_coords_center_default` — center (0.5,
  0.5) + radius 0.5 + 100x100 bbox → coords corretas.
- `p265_compute_radial_coords_center_offset` — center off-center.
- `p265_compute_radial_coords_non_square_uses_min_dim` —
  300x50 bbox → radius * min.
- `p265_oklab_sample_stops_radial_red_blue_endpoints` —
  sampling endpoints.

**E2E PDF** (3):
- `p265_export_pdf_radial_emits_shading_type_3` — confirma
  `/ShadingType 3` + `/PatternType 2` + `/FunctionType` +
  `/Coords` + `/Extend [true true]` + `SCN`.
- `p265_export_pdf_radial_dedup_arc_ptr` — 3 shapes mesmo
  Arc<Radial> → 1 Shading dedup.
- `p265_export_pdf_linear_e_radial_coexistem` — 1 Linear + 1
  Radial → ambos ShadingType 2 + 3 distintos.

### C.8 — Verificação intermediária

- `cargo build --workspace` → verde.
- `cargo test --workspace --release` → **2386 → 2393** (+7
  P265; zero regressões).
- `crystalline-lint --fix-hashes .` → `export.md` hash
  propagado `bf71181c`.
- `crystalline-lint .` → `✓ No violations found`.

---

## §5 — Sub-passo P265.D — Anotação ADR-0088 + relatório

### D.1 — Anotação cumulativa ADR-0088

Secção nova "Anotação cumulativa P265 — PDF Radial shading
complete materializado" adicionada após "Próximos passos":

- Componentes materializados em `03_infra/src/export.rs`
  (enum GradientObjectKind + 2 helpers + emit_gradient_objects
  branching + emit_stroke_paint unificado + 3 sítios
  pattern-match substituídos).
- Reutilização literal de P263 (sem alteração).
- Paridade observable cumprida pós-P265.
- Decisões D1-D5 documentadas.
- 7 tests adicionais P265.
- Cobertura Visualize agregada (~68% → ~73%).
- Subpadrões cumulativos.

**Status `IMPLEMENTADO` preservado** — anotação cumulativa
não muda status (paridade pattern P263 anotação ADR-0087 +
ADR-0080 §"refactor aditivo").

### D.2 — README ADRs

- Sem alteração de contagens (anotação cumulativa).
- Linha ADR-0088 ganha cross-reference cumulativa P265.
- Entrada P265 ~70 linhas nos passos-chave (paridade entrada
  P263/P264).
- Distribuição preservada: PROPOSTO 11, EM VIGOR 32,
  IMPLEMENTADO 28, total 75.

### D.3 — Relatório (este ficheiro)

Estrutura canónica §1-§10.

---

## §6 — Padrões metodológicos

### Subpadrão "P262/P263 dividir granularidade" N=2 → N=3 cumulativo

Cumulativo:
- N=1 P262 (Linear L1+stdlib) → P263 (Linear PDF).
- N=2 P264 (Radial L1+stdlib) → P265 (Radial PDF).
- **N=3 atingido cumulativamente** com este passo — **cluster
  Gradient completa duas divisões L1/L3**.

**Patamar N=3 atinge limiar formalização clara**. Candidato a
ADR meta — **improvável** (auto-documentado por cada aplicação).

### Subpadrão emergente "Reutilização literal de helpers cross-passos" N=1 inaugurado

P265 reutiliza inalterados:
- `emit_function_dict` (P263) — interpolação stops genérica.
- `emit_pattern_dict` inline em emit_gradient_objects (P263).
- `Radial::sample` (P264) — Oklab L1.
- Helpers Oklab privados gradient.rs (P262).

**~70% do código L3 P265 é wiring + helpers específicos**.
Magnitude S-M reflecte herança arquitectural sólida.

Pattern emergente inaugurado **N=1**. Candidato a formalização
N=3-4 cumulativo se P-Gradient-Conic + outros passos seguirem.

### Subpadrão "Anotação cumulativa em vez de ADR nova" reaplicada

Paridade pattern P263 anotação ADR-0087. ADR-0088 já cobre
Gradient Radial globalmente; P265 materializa backend PDF que
ADR-0088 implicitamente apontava. **Anotação cumulativa**
documenta; status `IMPLEMENTADO` preservado literal.

Cumulativo aplicações:
- N=1 P263 (anotação ADR-0087 PDF Linear).
- **N=2 P265** (anotação ADR-0088 PDF Radial).

Pattern emergente sólido — N=3 cumulativo se P-Gradient-Conic
seguir.

### Subpadrão "Auto-aplicação ADR-0065 critério #5 inline"

P265.A inventário inline (sem ficheiro separado). Paridade
pattern P260.A + P263.A — quando inventário trivial (mag
XS), inline aceitável per ADR-0065 §"Neutras".

Cumulativo:
- N=1 P156K (ADR-0064 + ADR-0065).
- N=2 P160A (ADR-0066).
- N=3 P260 (ADR-0084 + ADR-0085).
- N=4 P263.
- **N=5 P265**.

**Patamar N=5 reforça pattern** — inventário inline é prática
estabelecida.

---

## §7 — Cobertura

**Visualize agregado**:
- Pre-P262: ~52% (P259 audit).
- Pre-P263: ~58% (P262 Linear L1+stdlib +5pp).
- Pre-P264: ~63% (P263 Linear PDF +5pp).
- Pre-P265: ~68% (P264 Radial L1+stdlib +5pp).
- **Pós-P265: ~73%** (+5pp via PDF Radial real; F.2 promovido
  `implementado+stdlib` → `implementado+stdlib+render`
  paridade Linear).

**Entradas P259 Tabela A actualizadas pós-P265**:
- F.1 Gradient Linear: `implementado+stdlib+render` (P262+P263).
- F.2 Gradient Radial: **`implementado+stdlib+render`** (P264+P265
  — promovido P265).
- F.3 Gradient Conic: ausente (scope-out ADR-0088).

**Cobertura agregada user-facing total**: ~75-76% preservado.
**Layout Fase 5**: ~98-99% preservado.
**Math**: DEBT-8 ENCERRADO P255 preservado.
**Model**: ~73% pós-P258 preservado.
**Color** subsistema: 100% estrutural preservado.
**Visualize agregado**: **~73% pós-P265** (+21pp pós-P262
cumulativo — cluster Gradient completo L1+stdlib+PDF Linear +
Radial).

---

## §8 — Limitações e trabalho futuro

### Pendências residuais P265 (não-bloqueantes; candidatos passos dedicados)

**Cluster Gradient final**:
1. **P-Gradient-Conic L1+stdlib** (M; replica P262/P264 pattern;
   activa último Gradient variant).
2. **P-Gradient-Conic PDF** (S-M; replica P263/P265 template;
   `/ShadingType 6` ou similar conic).
3. **P-Gradient-Focal** (M; activa `focal_center` +
   `focal_radius` Radial campos; revoga scope-out ADR-0088).

**Refinos qualitativos P265**:
4. **`draw_item_local` Gradient support** — refactor para
   passar pattern resources no escopo recursivo (preserved
   scope-out P263 + P265).
5. **`FrameItem::Shape.fill: Option<Paint>`** — refino futuro
   se Fill Gradient prioritário (preserved scope-out P263).
6. **Coords algoritmo Radial vs vanilla** — refino se
   divergência empírica detectada (algoritmo simplificado
   círculos concêntricos; vanilla pode ter behaviour
   ligeiramente diferente para non-square bbox).

**Cluster Stroke refinos (Opção 3 P259)**:
7. **DEBT-33 Bézier bbox exacto** (S+).
8. **Stroke<Length>** (M).
9. **Dash patterns**.
10. **LineCap/LineJoin/MiterLimit**.

**Cluster Shapes**:
11. **Polygon variant estrutural separada**.
12. **Curve variant**.

**Cluster Image (Opção 5)**:
13. **SVG image format** (L+; requer ADR `usvg`/`resvg`).
14. **Image metadata** `alt`/`fit` (S).

**Transform**:
15. **`origin` pivot** (scope-out ADR-0061 preservado).

**Tiling**:
16. **Tiling pattern** — pré-requisito Paint::Tiling activar.

### Sem ADR nova aberta

Política P158 "sem novas reservas" preservada. P265 usa
anotação cumulativa ADR-0088.

### Sem DEBT novo aberto

Saldo DEBTs preservado.

---

## §9 — Critério de aceitação global P265 — Checklist final

- [x] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [x] `cargo test --workspace --release` retorna **2393 verdes**
  (+7 vs baseline 2386; sem regressão).
- [x] Inventário P265.A inline no relatório §2 com decisões
  D1-D5 explicitadas.
- [x] `00_nucleo/prompts/infra/export.md` actualizado com
  secção P265; hash propagado (`bf71181c`).
- [x] `03_infra/src/export.rs` ganha **3 helpers** novos
  (`compute_radial_coords`, `oklab_sample_stops_radial`,
  `emit_radial` inline) + **enum local `GradientObjectKind`**.
- [x] `emit_stroke_paint` branch unificado substitui fallback
  Solid P264.
- [x] `scan_all_gradients` + `pattern_resources_for_page`
  adaptados via match enum kind.
- [x] `pattern_resources` HashMap estruturalmente preservado
  (reutilizado P263).
- [x] 3 sítios pattern-match fallback P264 substituídos.
- [x] E2E tests confirmam bytes PDF (`/ShadingType 3`, Coords
  6 valores, `/Extend [true true]`, dedup `Arc::as_ptr<Radial>`,
  coexistência Linear+Radial).
- [x] Tests unit confirmam helpers (compute_radial_coords
  default/offset/non-square; oklab_sample_stops_radial).
- [x] ADR-0088 ganha secção "Anotação cumulativa P265".
- [x] **Paridade observable cumprida pós-P265**: PDFs com
  `#gradient.radial(...)` renderizam radial real via
  `/ShadingType 3` (não fallback).
- [x] Cross-path testado: 3 paths build_page_stream_*
  funcionais com Radial via branching unificado.
- [x] **Stroke + Fill** — só Stroke ganha Paint::Gradient
  branching; Fill continua Color literal (scope-out preserved
  P263 + P265).

**Estado pós-P265**:
- Tests workspace: **2393 verdes** (+7 vs baseline 2386).
- Hash drift: zero.
- Lint: zero violations.
- DEBTs saldo: **10 preservado**.
- ADRs distribuição preservada: PROPOSTO 11; IDEIA 2; EM VIGOR
  32; **IMPLEMENTADO 28**; REVOGADO 2; ADIADO 1; **total 75
  preservado**.
- Prompts L0 editados: 1 (`infra/export.md`).
- Diagnóstico imutável criado: 0 (P265 não é audit).
- ADRs anotadas cumulativamente: 1 (ADR-0088).
- **45 aplicações cumulativas anti-inflação** pós-P205D
  preservadas.

**Marco P265**: **PDF Radial shading complete materializado**;
**promessa P264 cumprida**; **cluster Gradient (Linear +
Radial) L1+stdlib+PDF 100% completo** pós-P265 (Conic continua
scope-out P-Gradient-Conic dedicado futuro); granularidade
ADR-0061 preservada via divisões P262/P263 + P264/P265.

**Recomendação subjectiva pós-P265**:

- **P-Gradient-Conic L1+stdlib + PDF** (M+S-M cumulativo;
  replica templates P262+P263/P264+P265; activa último
  Gradient variant; +3pp Visualize cobertura cumulativa).
- **OU P-Gradient-Focal** (M; activa `focal_center` +
  `focal_radius` Radial; revoga scope-out ADR-0088).
- **OU outras Opções P259 alternativas**:
  - DEBT-33 Bézier bbox + Stroke<Length> Opção 3.
  - Curve variant + Polygon estrutural separada Opção 2.
- **OU Text audit** (segundo audit pós-formalização P260).
- **OU P-Footnote-N** refino M (P258 pendência residual).
- **OU Tiling activação** (Paint::Tiling — análogo P262/P263
  estrutural).

**Decisão humana fica em aberto literal** pós-P265.

---

## §10 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- **ADR-0088** §"Anotação cumulativa P265" — PDF Radial
  shading complete materializado (criada por este passo).
- **ADR-0087** §"Anotação cumulativa P263" — precedente directo
  template (PDF Linear shading complete).
- **ADR-0086** — Paint wrapper (`Paint::Gradient` activa P261/P262
  absorve Radial sem cascade).
- **ADR-0085** — Diagnóstico imutável (P260; consumido
  metodologicamente em P264.A precedente — Radial vanilla
  diagnóstico).
- **ADR-0084** — Auditoria condicional (P260; consumido
  metodologicamente via inventário inline P265.A).
- **ADR-0083** — Color paridade vanilla (precedente N=2 pattern;
  `to_srgb`/`to_rgba_f32` reutilizados).
- ADR-0027 — CIDFont/Identity-H (precedente arquitectural
  estrutura objectos PDF).
- ADR-0029 — Pureza física L1 (regra geral; P265 é L3).
- ADR-0033 — Paridade observable vanilla.
- ADR-0034 — Diagnóstico canónico.
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded (Oklab interpolação pre-render
  aceite).
- ADR-0061 — Granularidade 1-2 features/passo (cumprido via
  divisão P264/P265 paridade P262/P263).
- ADR-0065 — Inventariar primeiro (cumprido inline P265.A).
- ADR-0080 — L0 minimal para refactors aditivos (cumprido via
  anotação cumulativa).
- DEBT-1 — Fechado P142 (preservado).
- ISO 32000-1 §7.5.7 — Shading patterns (referência canónica
  spec PDF; `/ShadingType 3` radial).
- **P73** — Image stack dedup `Arc::as_ptr` (template
  arquitectural N=1; **P263 N=2; P265 N=3 cumulativo**).
- P74 — PNG `/SMask` (precedente cross-path resource cascade).
- P257 — Color paridade 8/8 (helpers Color reutilizados).
- P261 — Paint wrapper (Paint::Gradient absorve Radial sem
  cascade).
- P262 — Gradient L1+stdlib (precedente N=1 dividir
  granularidade).
- **P263** — Gradient Linear PDF (**template literal P265**;
  paridade quase 1-para-1; ~70% código reutilizado).
- **P264** — Gradient Radial L1+stdlib (origem promessa fechada
  por este passo; 3 sítios pattern-match preparados).
- `00_nucleo/diagnosticos/diagnostico-gradient-radial-vanilla-passo-264.md`
  — diagnóstico imutável precedente.
- `00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
  — diagnóstico Linear precedente.
- `00_nucleo/prompts/infra/export.md` — L0 prompt actualizado
  por este passo (secção P265 anotada; hash `bf71181c`).
- `03_infra/src/export.rs` — código L3 alterado por este
  passo (~150 LoC novas: enum GradientObjectKind + 2 helpers
  + branching emit + 3 sítios pattern-match + 7 tests).

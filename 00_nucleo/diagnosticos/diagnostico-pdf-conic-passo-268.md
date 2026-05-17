# Diagnóstico PDF Conic vanilla — Passo 268 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0029 §"Diagnosticar primeiro" + ADR-0085
diagnóstico imutável (**quarto consumo directo vanilla** pós-
P262/P264/P267; **nono N=9 cumulativo geral** incluindo audit
Fase A) + ADR-0065 inventariar primeiro.
**Diagnóstico pai**: `typst-passo-268.md` (spec).
**Análogo estrutural directo**: P263 (Linear /ShadingType 2) +
P265 (Radial /ShadingType 3) + P267 (Conic L1+stdlib).
**Imutabilidade**: após criação, este ficheiro **não pode ser
editado** per ADR-0085.

---

## §A.1 — Vanilla typst-pdf paths

```bash
$ ls lab/typst-original/crates/typst-pdf/src/
attach.rs  convert.rs  image.rs  lib.rs  link.rs  metadata.rs
outline.rs  page.rs  paint.rs  shape.rs  tags  text.rs  util.rs
```

`paint.rs` contém logic Gradient rendering PDF.

---

## §A.2 — ESTRATÉGIA VANILLA CONIC PDF (decisão estratégica central)

**Vanilla usa crate externa `krilla::SweepGradient`** —
`paint.rs:255`:

```rust
let sweep = SweepGradient {
    cx, cy,
    start_angle: 0.0,
    end_angle: 360.0,
    transform: actual_transform.to_krilla(),
    spread_method: SpreadMethod::Pad,
    stops,
    anti_alias: gradient.anti_alias(),
};
```

**`krilla` não está autorizada em cristalino** (ADR-0018
whitelist). Vanilla também emite warning em `convert.rs:514`:
`"conic gradients are not supported in this PDF standard"` —
paridade observable krilla-dependente.

### Pre-amostragem densa (vanilla)

Vanilla pre-amostra densamente stops conic via `convert_gradient_stops`
(`paint.rs:320-369`):
- `max_dt = 0.05` para non-hue spaces (20 stops mínimos).
- `max_dt = 0.005` para hue spaces (200 stops mínimos).
- `max_dt = 0.25` para stops com cor igual.

Logic mistura `WeightedColor` cross-space (linha 355-361) via
`Color::mix_iter`.

### Decisão pré-flight (user P268)

User escolheu **Type 4 Gouraud manual** (triangulação do
disco; ~150-200 LOC; cap 250 LOC apertado mas viável).

PDF Shading Type 4 (ISO 32000 §7.5.7) — Free-Form Gouraud-
Shaded Triangle Mesh:
- `/ShadingType 4`.
- `/ColorSpace /DeviceRGB`.
- `/BitsPerCoordinate 8` (suficiente para 256 níveis em
  bbox unit space).
- `/BitsPerComponent 8` (RGB per vertex).
- `/BitsPerFlag 8` (continuation flag por vertex).
- `/Decode [xmin xmax ymin ymax 0 1 0 1 0 1]`.
- Stream binary: para cada vertex: flag + x + y + R + G + B.

### Triangulação proposta

Disco unit (center, edge points):
- N=32 fatias do disco (compromisso fidelidade vs LOC).
- Cada triangulação: (center, edge[i], edge[i+1]).
- Cor no centro: primeiro stop color (paridade fallback).
- Cor em edge[i] = `Conic::sample(i / N)`.

Total N=32 triângulos × 3 vértices = 96 vértices binary.

---

## §A.3 — Vanilla parâmetros emit

Krilla esconde detalhes. Pre-amostragem stops é literal:
- 20 stops min (non-hue).
- 200 stops min (hue spaces).
- 256 níveis bit-resolution PDF (paridade Type 4 8-bit).

---

## §A.4 — Cristalino emit_linear (P263) — template

```bash
$ grep -n "emit_gradient_objects\|/ShadingType 2" 03_infra/src/export.rs
```

`emit_gradient_objects` (linha ~1014) branching `GradientObjectKind`:
- Linear → `/ShadingType 2` + `/Coords [4]` + `/Extend [false false]`.
- Radial → `/ShadingType 3` + `/Coords [6]` + `/Extend [true true]`.

P268: adiciona **Conic → `/ShadingType 4` Type 4 Gouraud**.

---

## §A.5 — Cristalino emit_radial (P265) — template

Idêntico estrutura ao Linear, mudando ShadingType + Coords.

P268 Conic: diverge de Linear/Radial (Function dict não usado —
cores ficam directas no Vertex stream).

---

## §A.6 — Helpers Oklab N=16 existentes (reutilização viável)

```bash
$ grep "fn oklab_sample_stops\|fn oklab_sample_stops_radial\|fn interpolate_oklab" 03_infra/src/export.rs
```

- `oklab_sample_stops(linear, n)` — P263.
- `oklab_sample_stops_radial(radial, n)` — P265.
- `interpolate_oklab` em `01_core/src/entities/gradient.rs`
  (helper privado P262).

P268: criar **`oklab_sample_stops_conic(conic, n)`** paridade
N=2 → N=3 reutilização literal.

---

## §A.7 — Gap a fechar P268

1. **Helper `oklab_sample_stops_conic(conic, n)`** paridade
   P263/P265.
2. **Helper `emit_conic_gouraud_stream(conic, w, h, n_slices)`**
   produz bytes binary Type 4.
3. **`emit_gradient_objects` expand** — branch
   `GradientObjectKind::Conic` para emit Type 4.
4. **`GradientObjectKind` enum** ganha variant `Conic(Arc<Conic>)`.
5. **3 sítios pattern-match em export.rs** substituem `continue/fallback`
   por emit real:
   - `scan_all_gradients` — regista Conic.
   - `pattern_resources_for_page` — emit resource entry.
   - `emit_stroke_paint` — emit `/Pattern CS /Pn SCN`.
6. **Tests E2E**: emit `/ShadingType 4` + binary stream;
   helper unit tests.
7. **L0 anotação** `entities/gradient.md` secção P268.
8. **ADR-0089 anotação cumulativa P268**.

---

## §A.8 — Cenário detectado

☑ **B2 sub-passo dedicado** — materialização L3 PDF Conic.

Não é audit Fase A clássico (módulo grande). P268 é
**materialização L3 PDF dedicado** análoga P263/P265.

---

## §A.9 — Magnitude empírica revisada

**Estimativa**: S-M confirmado (~150-200 LOC L3; cap 250
apertado mas viável):
- Helper `oklab_sample_stops_conic`: ~15 LOC (paridade
  literal P263).
- Helper `emit_conic_gouraud_stream`: ~80-100 LOC (binary
  stream + triangulação).
- Branching `emit_gradient_objects`: ~30-40 LOC.
- 3 sítios pattern-match: ~10-15 LOC.
- Total LOC: ~150-180.
- Tests: ~10-15 (4 unit helpers + 3 E2E PDF + sítios
  pattern-match smoke).

---

## §A.10 — Cobertura empírica pré-P268 Visualize PDF

- Linear: implementado+stdlib+render (P262+P263).
- Radial: implementado+stdlib+render (P264+P265).
- **Conic: implementado+stdlib (P267); PDF fallback Solid**.

Visualize PDF cobertura: **~75% pós-P267 → ~78% esperado
pós-P268** (+3pp via PDF Conic real).

---

## §A.11 — Decisão arquitectural

☑ **Estratégia B — Type 4 Gouraud manual** (user pre-flight
decisão).

**Decisão local**:
- N=32 fatias do disco (compromisso fidelidade/LOC).
- N=16 stops Oklab pre-sample paridade P263/P265.
- Cor central = primeiro stop (fallback simples).
- BitsPerCoordinate = 8 (256 níveis bbox unit).
- BitsPerComponent = 8 (RGB sRGB).
- BitsPerFlag = 8 (only flag=0 used — todos os triângulos
  novos; sem continuation optimization).

**Subset implementação P268**:
- Triangulação simples disc fan (sem optimizações).
- Flag=0 para cada triangle (3 vértices/triângulo; 96 vertices
  total).
- Cores em edges via `Conic::sample(i/N)`.

**Scope-outs preservados**:
- Anti-aliasing (PDF default).
- Spread mode (Pad implícito; PDF não suporta repeat directo
  Type 4).
- focal_* (Radial-only; preservado).
- space/relative custom (preservado).

---

## §A.12 — Referências

- ADR-0027 (PDF objects estrutura).
- ADR-0085 (diagnóstico imutável).
- ADR-0089 (Gradient Conic-only; promessa P267 fechada por P268).
- ADR-0087/0088 anotações cumulativas P263/P265 (templates emit).
- ISO 32000 §7.5.7 — Shading Patterns Type 4 (Free-Form Gouraud).
- Vanilla `lab/typst-original/crates/typst-pdf/src/paint.rs:242-267`
  (Conic via krilla::SweepGradient; não autorizada cristalino).
- P263 (Linear PDF emit template).
- P265 (Radial PDF emit template + helpers Oklab N=16).
- P267 (Conic L1+stdlib precedente directo; 3 sítios pattern-match
  fallback a substituir).

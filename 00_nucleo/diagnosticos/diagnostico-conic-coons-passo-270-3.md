# Diagnóstico — Conic Type 6 Coons Patch Mesh L3 emit infra-estrutura (P270.3.A)

**Status**: imutável após criação (per ADR-0085).
**Data**: 2026-05-17.
**Passo**: P270.3 (Coons RGB infra-estrutura; preparação CMYK P270.4).
**Décimo primeiro consumo directo de fonte** vanilla (P262-P270.2 + **P270.3 vanilla Coons patches + Cairo precedente + ISO 32000-1 §7.5.7**).
**Origem**: spec `00_nucleo/materialization/typst-passo-270.3.md` §1.

---

## §A.1 — Cristalino emit_conic_gouraud_stream actual (P268+P268.2 baseline)

`03_infra/src/export.rs`:
- `emit_conic_gouraud_stream(conic, n_slices) -> Vec<u8>`: Type 4
  Gouraud, 18N bytes (N=32 default; adaptive via P268.2 hybrid).
- `compute_adaptive_n_conic(conic) -> usize`: factor_delta=256 + N_BASE/
  N_MIN/N_MAX clamps.
- Shading dict: `/ShadingType 4 /ColorSpace /DeviceRGB ... /Decode
  [0 1 0 1 0 1 0 1 0 1]`.
- Callsite production em `emit_gradient_objects`:
  `let n_adaptive = compute_adaptive_n_conic(conic);
   let stream = emit_conic_gouraud_stream(conic, n_adaptive);`

**P270.3 preserva tudo isto literal** — Type 6 Coons é estratégia
adicional paralela, não substituição. Flag opt-in default OFF.

---

## §A.2 — Cristalino L1 helpers reutilizáveis

- `multispace_sample_stops_conic` (P270.1): pré-amostragem N=16 em
  RGB via `conic.sample(t)` + `to_rgba_f32()`.
- `interpolate_in_space` (P270 dispatcher): arm RGB-family (sRGB,
  LinearRGB, Luma, Oklab, Oklch, HSL, HSV).
- `Color::to_rgba_f32()` (P257): conversion centralizada.

**P270.3 reutiliza para corner colors** dos patches Coons (cada corner
= stop color convertido para sRGB normalizado).

Sub-padrão "Reutilização literal helpers cross-passos" N=8 → **N=9
cumulativo**.

---

## §A.3 — Vanilla pré-krilla Coons emit (blog 2023 strategy literal)

Blog Typst 2023 "Color gradients and my gradual descent into madness"
documenta strategy literal:

> "we can still use Coons patches, but we need to create at least as
> many patches as there are stops in the gradient."

**Estratégia "1 patch per stop"** — paridade cristalina P270.3.

Motivação histórica vanilla: Apple PDF reader não suporta shading
function nos patches (forçaria interpolation interna); força usar 1
patch por stop com control points + corner colors explícitos.

---

## §A.4 — Vanilla krilla actual Coons emit

Krilla actual estratégia interna opaca (per ADR-0090 §"Pesquisa
empírica industry"). Sem documentação pública sobre Conic emit
post-transição P5420.

**Cristalino não tem acesso para verificar literal** — blog 2023
strategy é referência canónica (pré-krilla); cristalino converge a
isso via P270.3+P270.4.

---

## §A.5 — PDF spec ISO 32000-1 §7.5.7.4 Type 6 estrutura

**Coons Patch Mesh `/ShadingType 6`**:

- Stream binary contém sequência de patches.
- Por patch (flag = 0, "new patch"):
  - 1 byte flag.
  - 12 control points (24 bytes — 2 bytes per coord).
  - 4 corner colors (cada corner = N_components × bits_per_component
    bytes; RGB = 3 bytes; CMYK = 4 bytes).
- Por patch (flag ∈ {1, 2, 3}, "continuation"):
  - 1 byte flag.
  - 8 control points (16 bytes; shares 4 control points com adjacent).
  - 2 corner colors (shares 2 corners).

**Decode array** especifica range mapping para coord + color
componentes. Cristalino: `/Decode [0 1 0 1 0 1 0 1 0 1]` (5 pares:
x, y + 3 RGB) para Type 6 RGB.

**Para P270.3**: usar flag = 0 em todos os patches (simpler; sem
optimization de continuation — paridade Typst original P268
gouraud strategy).

---

## §A.6 — PROPOSTA Cristalino Coons emit para Conic

**Strategy "1 patch per stop"** (paridade Typst original blog 2023):

- N stops → N patches angulares.
- Cada patch cobre 360°/N graus angulares (assumindo stops uniformemente
  espaçados; cristalino também suporta non-uniform via `effective_offsets`,
  mas P270.3 simplifica para uniform per spec).

**Layout 12 control points per patch**:

PDF Type 6 Coons usa 12 control points formando 4 curvas Bezier
cúbicas:

```
        P0 ────P1────P2─── P3
        │                   │
        P11                 P4
        │                   │
        P10                 P5
        │                   │
        P9 ────P8────P7─── P6
```

- P0 = corner topo-esq (centro do disco).
- P3 = corner topo-dir (edge_start; arc inicio).
- P6 = corner baixo-dir (edge_end; arc fim).
- P9 = corner baixo-esq (centro do disco; mesmo que P0 — singularidade).
- P1, P2 = control points entre P0 e P3 (centro → edge_start; linear).
- P4, P5 = control points entre P3 e P6 (arc do círculo).
- P7, P8 = control points entre P6 e P9 (edge_end → centro; linear).
- P10, P11 = control points entre P9 e P0 (centro → centro; degenerate).

**Corner colors** (4 cores):
- corner0 (P0/centro topo) = stop_curr.color.
- corner1 (P3/edge_start) = stop_curr.color (gradient começa neste
  stop).
- corner2 (P6/edge_end) = stop_next.color.
- corner3 (P9/centro baixo) = stop_next.color (gradient termina neste
  stop).

Notar: P0 e P9 são fisicamente o centro do disco (mesmo ponto), mas
em Coons a topologia trata-os como 4 corners distintos. PDF reader
interpola entre cores baseado em parametric U-V coordinates.

**Convenção cor central preservada** (P268+P268.1-correção+P270.2):
para evitar gradient strange near centro singularity, ambos corners
P0 e P9 do patch i têm cor stop_curr.color (corner-pair inicial), e
gradient "flui" para edge externo. Patch i+1 começa com stop_next.
Isto preserva primeiro stop como cor central para Conic gradient.

**Decisão arquitectural P270.3**: simplificar — convention cor central
= primeiro stop (paridade P268.1-correção). Patches começam onde
stops começam; transição entre patches forma o gradient.

---

## §A.7 — Control points matemática Bezier cúbico arc círculo

**Standard approximation** (Stanislaw Adaszewski "Drawing a Circle
with Bezier Curves"):

Para arc de start_angle até end_angle, com angle_delta = end_angle -
start_angle:

```
offset = radius * (4.0 / 3.0) * (angle_delta / 4.0).tan()
```

Control points:
- cp1 = start_point + tangent_start * offset (tangent normal ao
  radial em start_angle).
- cp2 = end_point - tangent_end * offset (tangent normal ao radial
  em end_angle, "para trás").

```rust
fn bezier_control_points_for_arc(
    center: (f32, f32),
    radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> [(f32, f32); 2] {
    let angle_delta = end_angle - start_angle;
    let offset = radius * (4.0 / 3.0) * (angle_delta / 4.0).tan();

    let (sin_s, cos_s) = start_angle.sin_cos();
    let (sin_e, cos_e) = end_angle.sin_cos();

    let cp1 = (
        center.0 + radius * cos_s - offset * sin_s,
        center.1 + radius * sin_s + offset * cos_s,
    );
    let cp2 = (
        center.0 + radius * cos_e + offset * sin_e,
        center.1 + radius * sin_e - offset * cos_e,
    );

    [cp1, cp2]
}
```

**Erro máximo** ~0.0003 do círculo verdadeiro para 4 patches cobrindo
360° (quartos). Para N>4 patches angulares, erro ainda menor.

**Para N=2** (2 stops): angle_delta = π por patch (90°*2 = 180°). Erro
permanece pequeno (~0.027 max para half-circle approximation; aceitável
para Conic infra-estrutura visual).

---

## §A.8 — PROPOSTA stream binary Type 6 cristalino

**Por patch** (flag = 0):
- 1 byte flag (0).
- 12 control points × 2 coord bytes = 24 bytes.
- 4 corner colors × 3 RGB bytes = 12 bytes.
- **Total: 37 bytes per patch**.

**N patches** (paridade stops count): **37N bytes total**.

Comparação P268 Type 4 Gouraud:
- 18N_adaptive bytes (N_adaptive ~32 default; ~576 bytes típico).
- Para 2-stop conic: P268 ~576 bytes vs P270.3 Coons 74 bytes.
- Coons mais eficiente bytes-per-stop, mas Type 4 oferece resolução
  maior via N adaptive.

Trade-off: P270.3 Coons CMYK (P270.4) ~37·stops bytes (typically pequeno);
P268 Type 4 RGB ~32·18=576 bytes (resolução visual N=32).

---

## §A.9 — PROPOSTA dispatcher cristalino

```rust
GradientObjectKind::Conic(conic) => {
    // P270.3: flag opt-in interno default OFF.
    // P270.4: ON para space == Cmyk.
    let use_coons_emit = false;  // P270.3 reservado; P270.4 conecta.

    if use_coons_emit {
        // P270.3 infra-estrutura — não chamada P270.3 (default OFF).
        // P270.4 conecta para space == Cmyk.
        let stream = emit_conic_coons_stream(conic);
        let shading_dict = format!(
            "<< /ShadingType 6 /ColorSpace /DeviceRGB \
               /BitsPerCoordinate 8 /BitsPerComponent 8 \
               /BitsPerFlag 8 \
               /Decode [0 1 0 1 0 1 0 1 0 1] \
               /Length {} >>",
            stream.len(),
        );
        // ... emit shading dict + stream.
    } else {
        // P268+P268.2 literal preserved (default).
        let _ = multispace_sample_stops_conic(conic, 16);
        let n_adaptive = compute_adaptive_n_conic(conic);
        let stream = emit_conic_gouraud_stream(conic, n_adaptive);
        // ...
    }
}
```

**Flag opt-in não exposto user-facing** — decisão interna cristalino.

**P270.3 default OFF** preserva 2545 baseline bit-exact.

---

## §A.10 — PROPOSTA helpers L3 novos P270.3

1. `bezier_control_points_for_arc(center, radius, start_angle, end_angle)
   -> [(f32, f32); 2]` — matemática Stanislaw Adaszewski.

2. `compute_coons_patches_n_stops(conic: &Conic) -> usize` — retorna
   `conic.stops.len()`.

3. `emit_conic_coons_stream(conic: &Conic) -> Vec<u8>` — emit
   N patches; cada patch 37 bytes (flag + 12 coord + 4 colors).

Helpers privadas via `#[allow(dead_code)]` em P270.3 (default OFF;
P270.4 activa). Marca explícita preserve clean lint output.

---

## §A.11 — Estimativa cap LOC P270.3

| Componente | LOC estimado | Cap hard | Cap soft |
|---|---|---|---|
| `bezier_control_points_for_arc` | ~25 | 350 | 250 |
| `compute_coons_patches_n_stops` | ~5 | | |
| `emit_conic_coons_stream` | ~80-100 | | |
| Helper `coons_corner_colors` | ~15-20 | | |
| Dispatcher branch minimal | ~10-15 | | |
| `#[allow(dead_code)]` marcações | ~5 | | |
| **L3 total** | **~140-170** | **350 (folga ~52-60%)** | **250 (folga ~32-44%)** |
| Tests (15) | ~220-280 | 25 testes | 18 testes |

**L3 production ~150 LOC**. Cap hard 350 respeitado (folga ~57%).
Cap soft 250 respeitado (folga ~40%).

Tests 15. Cap hard 25 respeitado (folga 40%). Cap soft 18 respeitado
(folga 17%).

---

## §A.12 — Defaults preservam P270.2 bit-exact

- Flag opt-in OFF default → arm "else" dispatcher → `emit_conic_gouraud_stream`
  literal preserved.
- 2545 baseline tests P262-P270.2 preservados.

§política condições 4 + 7 satisfeitas.

---

## §A.13 — Industry research consolidada (factual)

| Fonte | Conclusão factual |
|---|---|
| Cairo Igalia blog (2020) | Type 6/7 mesh patches for conic gradients (20+ anos maturidade) |
| Inkscape | Type 7 Tensor patches (follower Cairo) |
| Typst original blog 2023 | Type 6 Coons "1 patch per stop" (paridade strategy cristalino) |
| W3C CSS-Color-4 Workshop 2021 (Mike Bremford bfo) | "the only way we can render conic gradients" em PDF é Coons Patch shading |
| pdf.js issue #6283 | Type 4 Gouraud "not supported" (cristalino diverge intencionalmente per ADR-0090) |
| Apache PDFBOX-2100 | Type 4 historical broken (Adobe Reader rejected `unknown imaging construct`) |
| matplotlib issue #18034 | Type 4 + Adobe Illustrator "unknown imaging construct" |
| Stanislaw Adaszewski "Drawing a Circle with Bezier Curves" | offset = r·(4/3)·tan(angle/4); 4 Bezier cúbicos cobrem 360° |
| ISO 32000-1 §7.5.7.4 | Type 6 Coons Patch Mesh structure literal |

**Cristalino P270.3+P270.4 converge industry**:
- Cairo/Inkscape/Typst original mesh-based para conic.
- Cristalino retém Type 4 Gouraud RGB (ADR-0090 strategy) para 2545
  baseline; Type 6 Coons CMYK (P270.4) resolve reader compatibility.

Sub-padrão "Fase A com industry research proactiva" N=2 → **N=3
cumulativo (limiar formalização clara)** — candidato meta-ADR.

---

## §A.14 — Cenário detectado: B1 fecho conceptual

**B1 confirmado**: infra-estrutura Coons RGB OFF preparação P270.4
materializável em L3 hard 350 LOC.

§política condição 1 não accionada (matemática Coons trivial via
Bezier standard approximation; corner colors convenção clara).

---

## §A.15 — Decisão arquitectural — Cenário A revisado P270.3

Cluster Gradient L3 emit pós-P270.3+P270.4:

| Variant | RGB-family + perceptual (7 spaces) | CMYK |
|---------|-----|-----|
| Linear | P270.1 ✓ /DeviceRGB Function 3-comp | **P270.2 ✓** /DeviceCMYK Function 4-comp |
| Radial | P270.1 ✓ /DeviceRGB | **P270.2 ✓** /DeviceCMYK |
| Conic Type 4 Gouraud | **P268+P268.2 preserved** ✓ /ShadingType 4 /DeviceRGB | (Type 4 + CMYK reader inconsistente) |
| Conic Type 6 Coons | **P270.3 infra-estrutura** (default OFF P270.3) | **P270.4 ON** ✓ /ShadingType 6 /DeviceCMYK |

**Cluster L3 emit 24/24 absoluto pós-P270.4** — 3 variants × 8 spaces
totalmente materializados.

---

## §A.16 — Pendência P270.4

P270.4 fecha cluster L3:
1. Dispatcher flag opt-in ON para `conic.space == ColorSpace::Cmyk`.
2. Emit `/ShadingType 6 /ColorSpace /DeviceCMYK`.
3. Coons stream binary com 4 corner colors CMYK (16 bytes per patch
   total adicionais; 41 bytes per patch CMYK).
4. Revoga ADR-0091 §"Conic CMYK scope-out preserved" definitivamente.
5. Cluster L3 emit feature-complete 24/24 absoluto.

Magnitude P270.4 esperada: **S** (~80-100 LOC L3 extensão CMYK Coons
+ ~5-8 testes).

---

## §A.17 — Sumário decisões diagnóstico

| Item | Decisão |
|---|---|
| §A.1 Type 4 Gouraud preserved | Literal P268+P268.2; default OFF P270.3 |
| §A.6 Strategy 1 patch per stop | Paridade Typst original blog 2023 |
| §A.7 Bezier control points | Stanislaw Adaszewski standard |
| §A.8 Stream binary | 37 bytes per patch (flag + 12 coord + 4 RGB) |
| §A.9 Dispatcher branching | Flag opt-in default OFF P270.3 |
| §A.10 Helpers L3 | 3 helpers novos com `#[allow(dead_code)]` |
| §A.11 Cap LOC | ~150 LOC (cap hard 350 folga 57%) |
| §A.12 Defaults preservam P270.2 | 2545 baseline bit-exact |
| §A.13 Industry research factual | 9 fontes confirmadas |
| §A.14 Cenário B1 | Confirmado |
| §A.15 Cluster pós-P270.4 24/24 | Preparação P270.3 + P270.4 fecha |

**Diagnóstico aprovado para passagem a sub-passo P270.3.B (ADR-0092
criação + anotações cumulativas).**

---

## §A.18 — Referências

- Spec P270.3: `00_nucleo/materialization/typst-passo-270.3.md`.
- Cristalino L3 P268+P268.2: `03_infra/src/export.rs` (`emit_conic_gouraud_stream`,
  `compute_adaptive_n_conic`).
- Cristalino L3 P270.1: `multispace_sample_stops_conic`,
  `interpolate_in_space` arm RGB-family.
- ADR-0089 — Gradient Conic-only (2 emit paths pós-P270.3).
- ADR-0090 — Type 4 strategy (§Type 6 scope-out revogado parcialmente).
- ADR-0091 — ColorSpace runtime + CMYK strategy (P270.3 preparação
  P270.4).
- ADR-0092 — Conic Coons Patches (criada P270.3).
- ADR-0085 — Diagnóstico imutável (décimo primeiro consumo).
- Vanilla `lab/typst-original/.../typst-pdf/src/` (krilla actual;
  estratégia interna opaca).
- Typst blog 2023 (Coons 1 patch per stop).
- W3C CSS-Color-4 Workshop 2021 Mike Bremford bfo.
- Igalia blog 2020 (Cairo Type 6/7).
- pdf.js issue #6283 (Type 4 reader compatibility).
- Apache PDFBOX-2100 (Type 4 historical broken).
- matplotlib issue #18034 (Type 4 + Adobe Illustrator).
- Stanislaw Adaszewski "Drawing a Circle with Bezier Curves".
- ISO 32000-1 §7.5.7.4 (Type 6 Coons Patch Mesh spec).

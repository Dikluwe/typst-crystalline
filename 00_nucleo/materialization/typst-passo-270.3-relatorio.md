# Relatório P270.3 — Conic Type 6 Coons Patch Mesh L3 emit infra-estrutura RGB (preparação CMYK P270.4)

**Data**: 2026-05-17.
**Magnitude**: M (real ~250 LOC L3 + 15 testes).
**Cluster**: Visualize / Gradient / PDF export (refino L3 estratégia adicional).
**Tipo**: sub-passo .3 da série P270.
**Spec**: `00_nucleo/materialization/typst-passo-270.3.md`.

---

## §1 — Sumário executivo

Infra-estrutura **Type 6 Coons Patch Mesh** materializada como
estratégia adicional Conic L3 emit (preparação CMYK P270.4 via
ADR-0092 EM VIGOR). **Cenário A revisado confirmado** — cristalino
tem 2 estratégias Conic L3 emit coexistentes:
- **Type 4 Gouraud (P268+P268.2 preserved)** — usado para 7 spaces
  RGB-family + perceptual.
- **Type 6 Coons (P270.3 infra + P270.4 activação)** — usado para
  CMYK em P270.4.

### Marcos arquiteturais P270.3

**(1) Primeiro caso "2 estratégias L3 emit coexistem para mesmo
variant"** em cristalino — Conic ganha Type 4 Gouraud (RGB; ADR-0090
preserved) + Type 6 Coons (CMYK preparação; ADR-0092 novo).
Estabelece precedente para futuras divergências intra-emit
fundamentadas em reader-compatibility.

**(2) Sub-padrão "ADR scope-out revogado parcialmente" N=5 limiar
formalização clara muito ultrapassado** — candidato meta-ADR
URGENTE. Pattern claro: P267 Conic + P269 focal_* + P270 ColorSpace
+ P270.2 DeviceCMYK + **P270.3 Type 6 Coons**.

**(3) Sub-padrão "Fase A com industry research proactiva" N=3
atinge limiar formalização clara** — P270 + P270.2 + P270.3.

**(4) Sub-padrão "Cap LOC hard vs soft explícito" N=3 consolidação
clara** — P270.1 + P270.2 + P270.3 pattern estabelecido.

### Industry research P270.3 (9 fontes consolidadas)

- **Cairo Igalia blog** (2020) — Type 6/7 mesh patches.
- **Inkscape** — Type 7 Tensor patches (follower Cairo).
- **Typst original blog** (2023) — Type 6 Coons "1 patch per stop"
  literal.
- **W3C CSS-Color-4 Workshop** (2021, Mike Bremford bfo) — Coons
  como única forma render conic em PDF.
- **pdf.js issue #6283** — Type 4 Gouraud reader compatibility.
- **Apache PDFBOX-2100** — Type 4 historical broken.
- **matplotlib issue #18034** — Type 4 + Adobe Illustrator "unknown
  imaging construct".
- **Stanislaw Adaszewski "Drawing a Circle with Bezier Curves"** —
  offset = r·(4/3)·tan(angle/4) standard approximation.
- **ISO 32000-1 §7.5.7.4** — Type 6 Coons Patch Mesh spec literal.

### Decisão Cenário A revisado (vs spec original P270.2 Cenário B)

P270.2 confirmou Cenário B (Conic CMYK scope-out preserved).
Pesquisa industry preventiva P270.3 revelou Type 6 Coons como
industry-standard mesh-based para conic — cristalino converge via
**Cenário A revisado**:
- Type 4 Gouraud RGB **preservado** (ADR-0090 estratégia perceptível
  intocada; 2545 baseline bit-exact).
- Type 6 Coons CMYK **materializado** (P270.4 activação opt-in).

Esta decisão arquitectural justifica ADR-0092 dedicada (vs apenas
anotação cumulativa ADR-0090) — escala 2 estratégias L3 emit
coexistentes é decisão estrutural distinta.

### Defaults preservam P270.2 — zero regressão

Flag opt-in default OFF P270.3 → arm "else" dispatcher →
`emit_conic_gouraud_stream` literal preserved. **2545 baseline tests
preservados bit-exact**. §política condições 4 + 7 + 9 satisfeitas
absolutas.

---

## §2 — Diff L3 antes/depois

### §2.1 — 3 helpers Coons novos (~250 LOC)

```rust
// 1. Matemática Bezier cúbico arc círculo (Stanislaw Adaszewski; ~25 LOC)
#[allow(dead_code)]
fn bezier_control_points_for_arc(
    center: (f32, f32),
    radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> [(f32, f32); 2] {
    let angle_delta = end_angle - start_angle;
    let offset = radius * (4.0 / 3.0) * (angle_delta / 4.0).tan();
    // ... (sin_cos + control point construction)
}

// 2. Strategy "1 patch per stop" (~5 LOC)
#[allow(dead_code)]
fn compute_coons_patches_n_stops(conic: &Conic) -> usize {
    conic.stops.len()
}

// 3. Emit Coons Patch Mesh stream binary RGB (~200 LOC)
#[allow(dead_code)]
fn emit_conic_coons_stream(conic: &Conic) -> Vec<u8> {
    // N stops → N patches angulares
    // 37 bytes per patch: 1 flag + 12 control points × 2 coord + 4 corners × 3 RGB
    // Layout 12 control points: P0..P11 (4 corners + 8 control points)
    // 4 corner colors: corner0/corner1 = stop_curr; corner2/corner3 = stop_next
}
```

### §2.2 — Dispatcher branching (default OFF P270.3)

P270.3 **não toca** o dispatcher Conic em `emit_gradient_objects` —
helpers Coons reservados para P270.4 via `#[allow(dead_code)]`.
Conic emit pipeline P268+P268.2+P270.1 preservado literal.

P270.4 adicionará dispatcher branching:
```rust
GradientObjectKind::Conic(conic) => {
    if conic.space == ColorSpace::Cmyk {
        // P270.4 — Type 6 Coons CMYK
        let stream = emit_conic_coons_stream(conic);
        // ... emit /ShadingType 6 /ColorSpace /DeviceCMYK
    } else {
        // P268+P268.2 preserved (default).
        // ...
    }
}
```

### §2.3 — Convention cor central preservada

Em Coons Patch Mesh, P0 (corner topo-esq) e P9 (corner baixo-esq)
de cada patch são **mesmo ponto físico** (centro do disco;
singularidade topológica). PDF reader interpola entre cores baseado
em parametric U-V coordinates.

**Cristalino convention P270.3** (paridade P268+P268.1-correção+P270.2):
ambos P0/P9 do patch i têm cor `stop_curr.color`; P3/P6 (edge_start/
edge_end) têm `stop_curr/stop_next` respectivamente. Gradient flui
radialmente centro→edge e angularmente stop_i→stop_i+1.

### §2.4 — Stream binary layout 37 bytes per patch

```
Byte 0:       flag (0 = new patch)
Bytes 1-2:    P0.x, P0.y     (corner centro topo)
Bytes 3-4:    P1.x, P1.y     (interior centro→edge_start, 1/3)
Bytes 5-6:    P2.x, P2.y     (interior centro→edge_start, 2/3)
Bytes 7-8:    P3.x, P3.y     (corner edge_start)
Bytes 9-10:   P4.x, P4.y     (arc Bezier control point 1)
Bytes 11-12:  P5.x, P5.y     (arc Bezier control point 2)
Bytes 13-14:  P6.x, P6.y     (corner edge_end)
Bytes 15-16:  P7.x, P7.y     (interior edge_end→centro, 1/3)
Bytes 17-18:  P8.x, P8.y     (interior edge_end→centro, 2/3)
Bytes 19-20:  P9.x, P9.y     (corner centro baixo = singularidade)
Bytes 21-22:  P10.x, P10.y   (degenerate centro→centro)
Bytes 23-24:  P11.x, P11.y   (degenerate centro→centro)
Bytes 25-27:  corner0 RGB    (stop_curr.color)
Bytes 28-30:  corner1 RGB    (stop_curr.color)
Bytes 31-33:  corner2 RGB    (stop_next.color)
Bytes 34-36:  corner3 RGB    (stop_next.color)

Total: 37 bytes per patch.
N stops → N patches → 37N bytes.
```

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P270.3 | Nota |
|---|---|---|
| **ADR PROPOSTO+IMPLEMENTADO mesmo passo** | **N=6 → N=7 cumulativo** | + P270.3 ADR-0092 |
| **ADR scope-out revogado parcialmente** | **N=4 → N=5 cumulativo (limiar muito ultrapassado)** | + P270.3 ADR-0090 §Type 6 — **candidato meta-ADR URGENTE** |
| **Fase A com industry research proactiva** | **N=2 → N=3 cumulativo (limiar formalização clara)** | + P270.3 (9 fontes industry consolidadas) |
| **Cap LOC hard vs soft explícito** | **N=2 → N=3 cumulativo (consolidação clara)** | + P270.3 |
| Anotação cumulativa cross-ADR | **N=3 → N=4 cumulativo** | + P270.3 (4 anotações cumulativas paralelas) |
| Reutilização literal helpers cross-passos | **N=8 → N=9 cumulativo** | + P270.3 (dispatcher P270 + helpers L3 templates) |
| Diagnóstico imutável (décimo primeiro consumo) | **N=15 → N=16 cumulativo** | + P270.3 (vanilla + Cairo + Typst blog 2023 + W3C + Adaszewski + ISO 32000-1) |
| Auditoria condicional (ADR-0084) | **N=14 → N=15 cumulativo** | + P270.3 |
| Auto-aplicação ADR-0065 inline | **N=13 → N=14 cumulativo** | + P270.3 |

**3 sub-padrões atingem limiar formalização clara** (N=5/N=3/N=3) —
candidato meta-ADR formalização futura paridade P260 ADR-0084/0085.

---

## §4 — Métricas finais

| Métrica | Pré-P270.3 | Pós-P270.3 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2545 | **2560** | +15 |
| Tests P270.3 novos | — | 15 | 8 unit + 4 E2E + 3 snapshot |
| Tests P262-P270.2 originais (verdes) | 2545 | 2545 | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Hashes propagados | — | 1 (L0 gradient.md) | +1 |
| ADRs totais | 78 | **79** | **+1 (ADR-0092 IMPLEMENTADO)** |
| ADRs IMPLEMENTADO | 30 | **31** | +1 |
| LOC L3 adicionado | — | ~250 | cap hard 350 (folga 28%); cap soft 250 ligeiramente acima |

### §política condições verificadas

- 1 (Coons matemática trivial via Bezier Adaszewski standard; corner
  colors convention clara per §A.6 diagnóstico). ✓
- 2 (Cap L3 hard 350 — real ~250; folga 28%). ✓
- 3 (Cap testes hard 25 — real 15; folga 40%). ✓
- 4 (Defaults flag opt-in OFF preservam 2545 baseline — verificado via
  workspace test). ✓
- 5 (Snapshot bytes reproduzíveis — 3 tests determinísticos). ✓
- 6 (Lint zero pós `--fix-hashes`). ✓
- 7 (Zero regressão P262-P270.2 — 2545 baseline preserved). ✓
- 8 (Corner colors convention preservada — corner0/corner1 = stop_curr;
  corner2/corner3 = stop_next per §A.6). ✓
- 9 (PDF spec ISO 32000-1 §7.5.7.4 stream binary detalhes
  documentados §2.4). ✓
- 10 (Cluster Gradient marco preservado — test
  `cluster_3_variants_opt_in_off_preserved` passa). ✓
- 11 (Dispatcher branching minimal — apenas adiciona helpers
  reservados P270.4; sem call sites externos afectados). ✓
- 12 (Industry research §A.13 factualmente verificada — 9 fontes
  documentadas em diagnóstico). ✓
- 13 (ADR-0090 revogação parcial — apenas §Type 6 sai; Type 7
  preserved). ✓

**13 condições §política verificadas — todas satisfeitas**.

---

## §5 — Verificação regressão zero P262-P270.2

**2545 tests preservados literal** (baseline P262-P270.2):

- typst-core: 2162 preserved.
- typst-infra: 336 → 351 (+15 P270.3 tests).
- Outros: preserved.

Mecânica: flag opt-in default OFF P270.3 → dispatcher Conic em
`emit_gradient_objects` não é tocado P270.3; helpers Coons reservados
via `#[allow(dead_code)]`. Pipeline P268+P268.2+P270.1+P270.2
preservado bit-exact.

§política condições 4 + 7 + 9 satisfeitas absolutas.

---

## §6 — ADR-0092 criada + 4 anotações cumulativas + L0

### §6.1 — ADR-0092 EM VIGOR criada

`00_nucleo/adr/typst-adr-0092-conic-coons-patches-rgb-cmyk.md` — ADR
dedicada documentando:
- Decisão Cenário A revisado (Type 4 RGB + Type 6 CMYK).
- Strategy "1 patch per stop" (paridade Typst original blog 2023).
- Matemática Bezier cúbico arc círculo (Stanislaw Adaszewski).
- Dispatcher opt-in flag interno default OFF P270.3 / ON P270.4 para
  CMYK.
- Convention cor central preservada (paridade P268+P268.1-correção+
  P270.2).
- 9 fontes industry consolidadas (Cairo/Inkscape/Typst blog 2023/W3C/
  pdf.js/PDFBOX/matplotlib/Adaszewski/ISO 32000-1).
- 9 alternativas consideradas (α1-α5 + decisão α3).
- Subpadrões aplicados (7 cumulativos avançados; 3 atingem limiar
  formalização clara).

Status PROPOSTO P270.3.B → IMPLEMENTADO P270.3.D mesmo passo (sub-padrão
N=7 cumulativo).

### §6.2 — ADR-0090 anotação cumulativa P270.3 (revogação parcial)

§"Scope-outs preserved" §"Type 6/7 Coons/Tensor patches" revogado
**parcialmente**: Type 6 sai (materialização cristalina P270.3);
Type 7 preserved (refino futuro candidato).

**ADR-0090 decisão de fundo (Type 4 Gouraud RGB cristalino)
preservada literal** — Type 6 é estratégia adicional, não substituição.

Sub-padrão "ADR scope-out revogado parcialmente" **N=5 cumulativo
limiar formalização clara muito ultrapassado** — candidato meta-ADR
URGENTE.

### §6.3 — ADR-0089 anotação cumulativa P270.3 (2 emit paths)

Conic ganha 2 emit paths coexistentes — Type 4 Gouraud (P268+P268.2
preserved; RGB default) + Type 6 Coons (P270.3 infra; P270.4 activa
para CMYK). Dispatcher opt-in flag interno (não user-facing).

**Marco P270.3**: Primeiro caso "2 estratégias L3 emit coexistem para
mesmo variant" em cristalino.

### §6.4 — ADR-0091 anotação cumulativa P270.3 (preparação P270.4)

ADR-0091 §"Conic CMYK scope-out preserved" P270.2 fica preparado para
revogação P270.4 via infra-estrutura Type 6 Coons materializada P270.3.
P270.4 fecha cluster L3 24/24 absoluto.

### §6.5 — ADR-0054 anotação cumulativa P270.3

Infra-estrutura Type 6 Coons Patch Mesh materializada como estratégia
adicional Conic L3 emit. Cluster Gradient ganha industry-aligned
mesh-based para conic CMYK (Cairo/Inkscape/Typst original Type 6
precedent).

### §6.6 — L0 `entities/gradient.md` anotação P270.3

Adicionada após anotação P270.2. Documenta:
- 2 emit paths Conic coexistentes.
- Helpers Coons (Bezier control points; "1 patch per stop"; 37 bytes
  per patch).
- Dispatcher opt-in flag default OFF P270.3.
- `#[allow(dead_code)]` marcações.
- 2545 baseline preserved bit-exact.

Hash propagado via `crystalline-lint --fix-hashes` (1 ficheiro
afectado).

---

## §7 — README ADRs distribuição

### Total ADRs

**78 → 79** (+ADR-0092 IMPLEMENTADO P270.3 — Conic Coons Patches;
criada PROPOSTO+IMPLEMENTADO mesmo passo via Cenário B1; sub-padrão
N=6 → N=7 cumulativo).

### Distribuição

| Status | Pré-P270.3 | Pós-P270.3 | Delta |
|---|---|---|---|
| `PROPOSTO` | 11 | 11 | 0 (ADR-0092 entra e sai PROPOSTO no mesmo passo) |
| `IDEIA` | 2 | 2 | 0 |
| `EM VIGOR` | 33 | 33 | 0 |
| `IMPLEMENTADO` | 30 | **31** | **+1 (ADR-0092)** |
| `REVOGADO` | 2 | 2 | 0 |
| `ADIADO` | 1 | 1 | 0 |
| **Total** | **78** | **79** | **+1** |

### Passos-chave

Nova entrada `- **Passo 270.3**` adicionada após Passo 270.2.
~135 linhas (infra-estrutura + ADR-0092 dedicada + 4 anotações
cumulativas paralelas + 3 sub-padrões atingem limiar formalização
clara).

### Cobertura Visualize agregada

~83-85% (P270.2) → **~83-85% preservada pós-P270.3** (P270.3 é
infra-estrutura; cobertura efectiva inalterada até P270.4 activar
CMYK).

---

## §8 — Pendências preservadas pós-P270.3

### Próximo passo na série P270

- **P270.4** (S futuro) — Coons CMYK activação opt-in flag ON para
  `space == Cmyk`. Revoga ADR-0091 §"Conic CMYK scope-out preserved"
  **definitivamente**. Cluster L3 emit **24/24 absoluto**. Magnitude
  S esperada (~80-100 LOC L3 extensão CMYK Coons + ~5-8 testes).

### Refinos Conic candidatos

- **P-Gradient-Coons-RGB-Final** (M futuro) — converge Conic RGB de
  Type 4 Gouraud para Type 6 Coons; elimina 2 estratégias
  coexistentes. Não bloqueante — preserved candidato.

### Meta-ADR formalização sub-padrões

**Passo administrativo XS candidato futuro** paridade P260
ADR-0084/0085:
- "ADR scope-out revogado parcialmente" N=5 (P267+P269+P270+P270.2+
  P270.3) — **URGENTE** limiar muito ultrapassado.
- "Fase A com industry research proactiva" N=3 (P270+P270.2+P270.3) —
  limiar formalização clara.
- "Cap LOC hard vs soft explícito" N=3 (P270.1+P270.2+P270.3) —
  consolidação clara.

### Demais pendências

- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266 Opção 1).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 Bézier bbox** + outros Visualize.
- **Tiling activação** (Paint::Tiling).
- **P-Gradient-CMYK-ICC** (S-M futuro; krilla paridade ICC profiles).
- **P-Gradient-Adaptive-Multispace** (S futuro; N adaptive HSL/Oklch
  hue diff alto).

Decisão humana fica em aberto literal pós-P270.3.

---

## §9 — Critério aceitação checklist

- [x] **Fase A diagnóstico** `diagnostico-conic-coons-passo-270-3.md`
      criado (§A.1-§A.18; imutável per ADR-0085).
- [x] **9 fontes industry consolidadas** (Cairo Igalia 2020 + Typst
      blog 2023 + W3C 2021 + pdf.js #6283 + PDFBOX-2100 + matplotlib
      #18034 + Stanislaw Adaszewski + ISO 32000-1 §7.5.7.4).
- [x] **Cenário B1 confirmado** §A.14 (infra-estrutura Coons RGB
      materializável em L3 hard 350 LOC).
- [x] **ADR-0092 criada PROPOSTO+IMPLEMENTADO** mesmo passo (sub-padrão
      N=7 cumulativo).
- [x] **ADR-0090 anotada P270.3** (§Type 6 scope-out revogado
      parcialmente; sub-padrão N=5 limiar muito ultrapassado).
- [x] **ADR-0089 anotada P270.3** (2 emit paths Conic).
- [x] **ADR-0091 anotada P270.3** (preparação P270.4).
- [x] **ADR-0054 anotada P270.3** (cluster L3 status).
- [x] **L0 `entities/gradient.md` anotada P270.3** após anotação
      P270.2; hash propagado.
- [x] **15 tests-primeiro** adicionados antes do código L3.
- [x] **L3 helpers Coons**: `bezier_control_points_for_arc`,
      `compute_coons_patches_n_stops`, `emit_conic_coons_stream` com
      `#[allow(dead_code)]` marcações.
- [x] **Dispatcher Conic preservado P268+P268.2** literal (default
      OFF P270.3; helpers reservados P270.4).
- [x] **README ADRs** linha tabela ADR-0092 nova; passo 270.3
      §"passos-chave"; total 78→79; IMPLEMENTADO 30→31.
- [x] **Tests workspace** 2545 → 2560 (+15; **zero regressões**).
- [x] **Lint zero violations** pós `--fix-hashes`.
- [x] **Build cargo** exit 0.
- [x] **Snapshot bytes reproduzíveis** (3 tests determinísticos).

**13 condições §política verificadas — todas satisfeitas**.

---

## §10 — Referências

### Cross-passos

- **P268** — PDF Conic Type 4 Gouraud (preserved; opt-in flag default
  OFF P270.3 + P270.4 ON CMYK).
- **P268.1-correção** — ADR-0090 correcção factual (preserved).
- **P268.2** — Adaptive N hybrid (preserved; aplicado a Type 4
  Gouraud).
- **P270/P270.1/P270.2** — ColorSpace runtime + L3 emit RGB + L3
  emit CMYK Linear+Radial (preservados; Conic CMYK scope-out
  preserved P270.2 revogado por P270.3+P270.4).
- **P270.4** — Coons CMYK activação (próximo passo).

### ADRs

- **ADR-0092** — Conic Coons Patches (criada PROPOSTO+IMPLEMENTADO
  P270.3; este passo).
- **ADR-0089** — Gradient Conic-only (anotada cumulativa P270.3).
- **ADR-0090** — Type 4 strategy (anotada cumulativa P270.3 §Type 6
  revogado parcialmente).
- **ADR-0091** — ColorSpace runtime + CMYK strategy (anotada
  cumulativa P270.3 preparação P270.4).
- **ADR-0054** — Perfil graded (anotada cumulativa P270.3).
- **ADR-0018** — Whitelist crates (preservada).
- **ADR-0085** — Diagnóstico imutável (décimo primeiro consumo
  directo de fonte).

### Documentos cristalinos editados

- `03_infra/src/export.rs` (~250 LOC L3: 3 helpers Coons novos com
  `#[allow(dead_code)]` + 15 tests P270.3 novos).
- `00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md`
  (anotação P270.3 §Type 6 revogado parcialmente).
- `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md` (anotação
  P270.3 2 emit paths).
- `00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md`
  (anotação P270.3 preparação P270.4).
- `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` (anotação
  P270.3).
- `00_nucleo/prompts/entities/gradient.md` (anotação P270.3; hash
  propagado).
- `00_nucleo/adr/README.md` (tabela ADR-0092 nova + total 78→79 +
  IMPLEMENTADO 30→31 + passos-chave P270.3).

### Documentos criados

- `00_nucleo/adr/typst-adr-0092-conic-coons-patches-rgb-cmyk.md`
  (ADR-0092 IMPLEMENTADO).
- `00_nucleo/diagnosticos/diagnostico-conic-coons-passo-270-3.md`
  (imutável; décimo primeiro consumo directo de fonte vanilla +
  industry).
- `00_nucleo/materialization/typst-passo-270-3-relatorio.md` (este
  relatório).

### Vanilla literal (verificável)

- `lab/typst-original/.../typst-pdf/src/paint.rs` — vanilla krilla
  SweepGradient (estratégia interna opaca; cristalino diverge
  intencionalmente).

### Fontes industry consolidadas (9; verificáveis via web)

- **Igalia blog "Renderization of Conic gradients"** (2020) — Cairo
  Type 6/7.
- **Typst blog "Color gradients and my gradual descent into madness"**
  (2023) — Type 6 1 patch per stop literal.
- **W3C CSS-Color-4 Workshop** (2021, Mike Bremford bfo) — Coons
  como única forma render conic em PDF.
- **pdf.js issue #6283** — Type 4 Gouraud not supported.
- **Apache PDFBOX-2100** — Type 4 historical broken.
- **matplotlib issue #18034** — Type 4 + Adobe Illustrator "unknown
  imaging construct".
- **Stanislaw Adaszewski "Drawing a Circle with Bezier Curves"** —
  offset formula.
- **ISO 32000-1 §7.5.7.4** — Type 6 Coons Patch Mesh spec literal.
- **Inkscape** — Type 7 Tensor patches (follower Cairo).

### Marco arquitectural

**Primeiro caso "2 estratégias L3 emit coexistem para mesmo variant"**
em cristalino:
- Conic Type 4 Gouraud (RGB; ADR-0090 preserved).
- Conic Type 6 Coons (CMYK preparação; ADR-0092 novo).

**3 sub-padrões atingem limiar formalização clara** — candidato
meta-ADR formalização futura:
- "ADR scope-out revogado parcialmente" **N=5 (URGENTE)**.
- "Fase A com industry research proactiva" **N=3**.
- "Cap LOC hard vs soft explícito" **N=3 (consolidação)**.

**Pendência P270.4**: activação opt-in flag ON para CMYK; revoga
ADR-0091 §"Conic CMYK scope-out preserved" definitivamente; cluster
L3 24/24 absoluto. Magnitude S esperada.

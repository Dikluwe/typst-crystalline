# ⚖️ ADR-0092: Conic Type 6 Coons Patch Mesh L3 emit (preparação CMYK + RGB futuro)

**Status**: **`IMPLEMENTADO`** (PROPOSTO P270.3.B → **IMPLEMENTADO
P270.3.D** — materialização infra-estrutura Coons RGB L3 +
4 anotações cumulativas paralelas ADR-0089/0090/0091/0054;
flag opt-in interno default OFF preserva 2545 baseline bit-exact;
P270.4 activa para CMYK)
**Data**: 2026-05-17
**Autor**: Humano + IA
**Passo origem**: P270.3 (preparação infra-estrutura RGB; P270.4 activa CMYK)
**Cluster**: Visualize / Gradient / PDF export
**Tipo**: estratégia adicional Conic L3 emit (Cenário A revisado)
**Validado**: Passo 270.3 (criação PROPOSTO → promoção IMPLEMENTADO
mesmo passo via Cenário B1; sub-padrão N=6 → N=7 cumulativo
ADR-0083/0086/0087/0088/0089/0091/0092).
**Diagnóstico prévio**:
`00_nucleo/diagnosticos/diagnostico-conic-coons-passo-270-3.md`
(imutável per ADR-0085; **décimo primeiro consumo directo de fonte**
vanilla + Cairo + Typst blog 2023 + ISO 32000-1 §7.5.7.4 +
Stanislaw Adaszewski).

---

## Contexto

ADR-0089 + ADR-0090 estabeleceram Type 4 Gouraud como estratégia
cristalina Conic L3 emit (P268 + P268.2 adaptive N hybrid). ADR-0091
materializou ColorSpace runtime cross-variant (P270 + P270.1 + P270.2);
Conic CMYK ficou scope-out preserved P270.2 (Cenário B) por
reader-compatibility incerto Type 4 Gouraud + DeviceCMYK.

**Pesquisa industry preventiva P270.3** (Igalia blog 2020 + Typst blog
2023 + W3C Workshop 2021 + pdf.js #6283 + PDFBOX-2100 + matplotlib
#18034) revelou:

- Type 4 Gouraud + CMYK reader compatibility problemático (Adobe
  Illustrator "unknown imaging construct"; pdf.js Type 4 inconsistente).
- Industry mesh-based para conic = Type 6 Coons Patches (Cairo,
  Inkscape, Typst original pré-krilla).
- Typst original blog 2023 strategy literal: "1 patch per stop".
- W3C 2021: "the only way we can render conic gradients" em PDF é
  Coons.

Cristalino diverge intra-família mesh (Type 4 vs Type 6 industry).
ADR-0090 §"Scope-outs preserved" §"Type 6/7 Coons/Tensor patches"
listou Type 6 como candidato futuro; este passo revoga **parcialmente**
para materializar via "P-Gradient-Conic-CMYK" Cenário A revisado.

Sub-padrão **"ADR PROPOSTO+IMPLEMENTADO mesmo passo"** N=6 → **N=7
cumulativo** (P257/P261/P262/P264/P267/P270/**P270.3**).

---

## Decisão (Cenário A revisado)

1. **Cristalino tem 2 estratégias Conic L3 emit coexistentes**:
   - **Type 4 Gouraud (P268+P268.2 preservado)** — usado para 7
     spaces RGB-family + perceptual (sRGB, LinearRGB, Luma, Oklab,
     Oklch, HSL, HSV); P270.1 pipeline preservado literal.
   - **Type 6 Coons (P270.3 infra-estrutura + P270.4 activação CMYK)** —
     usado para CMYK em P270.4; reservado para futuro converge
     Conic RGB (P-Gradient-Coons-RGB-Final candidato).

2. **Strategy Coons**: 1 patch per stop (paridade Typst original blog
   2023). N stops → N patches angulares; cada patch cobre 360°/N graus.

3. **Matemática Bezier cúbico arc círculo** (Stanislaw Adaszewski):
   ```
   offset = radius * (4.0 / 3.0) * (angle_delta / 4.0).tan()
   ```
   Standard approximation; erro máximo ~0.0003 do círculo verdadeiro
   para quartos (90°). Para N>4 patches angulares, erro menor.

4. **Dispatcher opt-in flag interno** (não user-facing):
   - **P270.3**: flag OFF default → `emit_conic_gouraud_stream`
     literal preserved.
   - **P270.4**: flag ON para `space == Cmyk` → `emit_conic_coons_stream`
     activado.
   - Flag interno cristalino; decisão arquitectural não exposta API.

5. **Type 7 Tensor patches preserved scope-out** — apenas Type 6 sai
   do scope-out ADR-0090. Type 7 candidato futuro adicional se
   necessário.

---

## Pesquisa empírica industry consolidada

| Projecto | Estratégia Conic PDF | Tipo PDF | Reader compatibility |
|---|---|---|---|
| Cairo | Coons/Tensor mesh | `/ShadingType 6` ou `7` | universal (20+ anos) |
| Inkscape | Tensor patches | `/ShadingType 7` | universal |
| Typst original pré-krilla | Coons 1 patch per stop | `/ShadingType 6` | universal (Apple Preview confirmed) |
| Typst actual via krilla | desconhecido publicamente | abstracção | ? |
| **Cristalino actual P270.2** | **Type 4 Gouraud + CMYK scope-out** | **`/ShadingType 4` + RGB only** | Type 4 problemático em readers minoritários |
| **Cristalino P270.3+P270.4** | **Type 4 RGB + Type 6 CMYK** | **dispatcher dual** | universal para CMYK; preservado P268 para RGB |

Sub-padrão **"Fase A com industry research proactiva"** N=2 → **N=3
cumulativo (limiar formalização clara)** — candidato meta-ADR
junto com "ADR scope-out revogado parcialmente".

---

## Convenção cor central preservada

Cor central = primeiro stop (paridade P268+P268.1-correção+P270.2).

Em Coons Patch Mesh, P0 (corner topo-esq) e P9 (corner baixo-esq) de
cada patch são **mesmo ponto físico** (centro do disco; singularidade
topológica). PDF reader interpola entre cores baseado em parametric
U-V coordinates.

**Cristalino convenção P270.3**: ambos P0/P9 do patch_i têm cor
`stop_curr.color` (corner-pair inicial); P3/P6 (edge_start/edge_end)
têm cores stop_curr/stop_next respectivamente. Patch i+1 começa onde
patch i termina. Gradient "flui" radialmente centro→edge e
angularmente stop_i→stop_i+1.

---

## Consequências

### Positivas

- **Cluster Gradient L3 emit Cenário A revisado preparado para 24/24
  absoluto** (P270.4 fecha definitivamente).
- **Industry alignment forte** (Cairo/Inkscape/Typst original Type 6
  precedent).
- **Reader compatibility Conic CMYK fica universal** via Type 6.
- **Cristalino preserva Type 4 RGB** (2545 baseline; ADR-0090
  estratégia perceptível preservada).

### Negativas

- **Cristalino tem 2 estratégias Conic emit coexistentes** —
  complexidade arquitectural adicional.
- **ADR-0090 §"Type 6/7 Coons/Tensor patches scope-out" revogado
  parcialmente** (Type 6; Type 7 preserved).

### Neutras

- Infra-estrutura Coons RGB pronta para futuro (P-Gradient-Coons-RGB-Final
  candidato se ganho factual demonstrado).
- Tests P262-P270.2 preservados literal (flag default OFF P270.3).

---

## Scope-outs preserved

- **Type 7 Tensor patches**: scope-out preserved. Cairo/Inkscape usam
  para conic complexo; cristalino Type 6 1-patch-per-stop suficiente
  para Conic CMYK.
- **P-Gradient-Coons-RGB-Final**: candidato futuro converger Conic RGB
  de Type 4 Gouraud para Type 6 Coons (eliminar 2 estratégias). Se
  P270.4 confirmar Type 6 reader compatibility, pode tornar-se
  obrigatório.
- **Continuation patches (flag 1/2/3)**: optimization optional;
  cristalino P270.3+P270.4 usa apenas flag=0 (new patches) — simplifica
  emit sem perda factual.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|---|---|---|
| α1 — Cenário B P270.2 (Conic CMYK scope-out preserved final) | -1 ADR | Utilizador escolheu fechar cluster |
| α2 — Type 4 Gouraud + CMYK directo (cluster 24/24 sem mudar estratégia) | -1 ADR | Pesquisa P270.3 confirma reader inconsistente |
| **α3 — Cenário A revisado (Type 4 RGB + Type 6 CMYK; escolhida)** | **Industry-aligned; reader compatibility universal CMYK; preserva ADR-0090 estratégia perceptível Type 4** | **2 estratégias coexistentes; complexidade arquitectural** |
| α4 — Type 7 Tensor patches | Cairo/Inkscape precedent absoluto | Magnitude L+ adicional sem ganho factual demonstrado vs Type 6 |
| α5 — Reverter Type 4 RGB para Type 6 imediatamente | Cluster unificado | Custo L sem ganho factual demonstrado; preserved candidato futuro |

**Decisão**: **α3 (Cenário A revisado) + Opção α (ADR-0092 nova)** per
paridade pattern ADR-0083/0086/0087/0088/0089/0091.

---

## Critério revisão

Esta ADR pode ser revisitada se:

1. **P270.4 revelar Type 6 reader compatibility problemático para
   CMYK**.
2. **P-Gradient-Coons-RGB-Final tornar-se prioridade** (Type 4
   banding RGB detectado empíricamente).
3. **Vanilla krilla actual revelar-se Coons** (cristalino convergiria
   totalmente).
4. **Type 7 Tensor tornar-se necessário** para conic complexo
   (improvável).

Cada activação é **passo dedicado pequeno** (P270.4 já planeado).

---

## Subpadrões aplicados

### "ADR PROPOSTO+IMPLEMENTADO mesmo passo" N=6 → N=7 cumulativo

- N=1 P257 (ADR-0083 Color).
- N=2 P261 (ADR-0086 Paint).
- N=3 P262 (ADR-0087 Gradient Linear).
- N=4 P264 (ADR-0088 Gradient Radial).
- N=5 P267 (ADR-0089 Gradient Conic).
- N=6 P270 (ADR-0091 ColorSpace runtime + CMYK strategy).
- **N=7 P270.3** (ADR-0092 Conic Coons Patches).

### "ADR scope-out revogado parcialmente" N=4 → N=5 cumulativo

- N=1 P267 (ADR-0088 §Conic scope-out).
- N=2 P269 (ADR-0088 §focal_* scope-out).
- N=3 P270 (ADR-0083 §ColorSpace runtime scope-out).
- N=4 P270.2 (ADR-0083 §DeviceCMYK scope-out parcial Linear+Radial).
- **N=5 P270.3** (ADR-0090 §Type 6 scope-out — esta ADR).

**N=5 limiar formalização clara muito ultrapassado** — candidato
meta-ADR URGENTE paridade P260 ADR-0084/0085.

### "Anotação cumulativa cross-ADR" N=3 → N=4 cumulativo

- N=1 P270 (6 ADRs anotadas).
- N=2 P270.1 (6 ADRs anotadas).
- N=3 P270.2 (6 ADRs anotadas).
- **N=4 P270.3** (esta ADR cria; 4 anotações cumulativas paralelas:
  ADR-0089/0090/0091/0054).

### "Reutilização literal helpers cross-passos" N=8 → N=9 cumulativo

P270.3 reutiliza literal:
- `multispace_sample_stops_conic` (P270.1) — usado nas tests P270.3
  para baseline RGB.
- `interpolate_in_space` (P270 dispatcher) arm RGB-family.
- `Color::to_rgba_f32` (P257) para corner colors.
- Helpers Oklab P262 reutilizáveis (não tocados P270.3 mas referenciados
  ADR-0090).

### "Diagnóstico imutável" N=15 → N=16 (décimo primeiro consumo)

P262/P264/P267/P268/P268.1/P268.2/P269/P270/P270.1/P270.2 + **P270.3**
vanilla Coons patches + Cairo precedente + ISO 32000-1 §7.5.7.4 +
Stanislaw Adaszewski.

### "Fase A com industry research proactiva" N=2 → N=3 cumulativo

- N=1 P270 (vanilla docs + blog 2023 + issue #4422 + W3C).
- N=2 P270.2 (CMYK pré-spec; bug #4422 causa raiz).
- **N=3 P270.3** (Cairo/Inkscape/Typst blog 2023/W3C/pdf.js/PDFBOX/
  matplotlib/Adaszewski/ISO 32000-1).

**N=3 atinge limiar formalização clara** — candidato meta-ADR.

### "Cap LOC hard vs soft explícito" N=2 → N=3 cumulativo (consolidação)

- N=1 P270.1 (inaugural).
- N=2 P270.2 (segunda aplicação).
- **N=3 P270.3** (terceira aplicação consolida pattern).

Pattern estabelecido.

---

## Referências cross-passos

- **P262/P264/P267** — Variant L1+stdlib (preserved).
- **P263/P265/P268** — L3 emit templates (Type 4 Gouraud preserved).
- **P268.2** — Adaptive N hybrid Conic (preserved; aplicado a Type 4
  Gouraud).
- **P269** — Radial focal_* (preserved; não afecta Conic).
- **P270** — ColorSpace runtime L1+stdlib (ADR-0091).
- **P270.1** — L3 emit 7 spaces (preserved).
- **P270.2** — L3 Linear+Radial CMYK directo (Cenário B; Conic CMYK
  scope-out preserved revogado por P270.3+P270.4).
- **P270.4** — Coons CMYK activação opt-in flag ON; fecha cluster
  24/24 absoluto.
- **P-Gradient-Coons-RGB-Final** — candidato futuro converger Conic
  RGB.

---

## ADRs anotadas cumulativamente P270.3

- **ADR-0089** (Gradient Conic-only): 2 emit paths agora coexistem.
- **ADR-0090** (Type 4 strategy): §Type 6 scope-out revogado
  parcialmente; decisão de fundo (Type 4 RGB) preservada.
- **ADR-0091** (ColorSpace runtime + CMYK): preparação P270.4.
- **ADR-0054** (Perfil graded): infra-estrutura Coons adicionada.

---

## Industry research fontes

- **Igalia blog "Renderization of Conic gradients"** (2020) — Cairo
  Type 6/7.
- **Typst blog "Color gradients and my gradual descent into madness"**
  (2023) — Type 6 1 patch per stop literal.
- **W3C CSS-Color-4 Workshop** (2021, Mike Bremford bfo) — Coons
  como única forma render conic em PDF.
- **pdf.js issue #6283** — Type 4 Gouraud not supported (cristalino
  diverge intencionalmente per ADR-0090).
- **Apache PDFBOX-2100** — Type 4 historical broken (Adobe Reader
  rejected "unknown imaging construct").
- **matplotlib issue #18034** — Type 4 + Adobe Illustrator "unknown
  imaging construct".
- **Stanislaw Adaszewski "Drawing a Circle with Bezier Curves"** —
  offset = r·(4/3)·tan(angle/4); 4 Bezier cúbicos cobrem 360°.
- **ISO 32000-1 §7.5.7.4** — Type 6 Coons Patch Mesh structure
  literal.

---

## Próximos passos

1. **P270.3.C** executa materialização imediata (helpers Bezier +
   `emit_conic_coons_stream` + dispatcher branching opt-in flag
   default OFF + 15 tests).
2. **P270.3.D** promove ADR-0092 → `IMPLEMENTADO`.
3. **P270.4** (futuro S) — activa opt-in flag ON para `space == Cmyk`;
   revoga ADR-0091 §"Conic CMYK scope-out preserved" definitivamente;
   cluster L3 24/24 absoluto.
4. **P-Gradient-Coons-RGB-Final** (candidato futuro M; converge Conic
   RGB para Type 6 Coons).
5. **Meta-ADR formalização sub-padrões** (passo administrativo XS
   candidato futuro paridade P260):
   - "ADR scope-out revogado parcialmente" N=5 (P267+P269+P270+P270.2+P270.3).
   - "Fase A com industry research proactiva" N=3 (P270+P270.2+P270.3).
   - "Cap LOC hard vs soft explícito" N=3 (P270.1+P270.2+P270.3).

---

## Anotação cumulativa P270.4 — Coons CMYK activação opt-in flag ON (cluster L3 24/24 absoluto)

**Data**: 2026-05-17.

**Motivo**: ADR-0092 §"Decisão Cenário A revisado" P270.4 — Conic CMYK
via Coons materializado. **Cluster Gradient L3 emit feature-complete
24/24 absoluto** — marco arquitectural máximo do cluster.

### Materialização opt-in flag ON

- **`emit_conic_coons_stream_cmyk` variant criado** — paridade
  estrutural `emit_conic_coons_stream` P270.3 RGB. Corner colors
  4-component CMYK (vs 3 RGB); total 41 bytes per patch (vs 37 RGB).
- **Dispatcher Conic em `emit_gradient_objects`**:
  - `space == Cmyk` → `/ShadingType 6 /ColorSpace /DeviceCMYK` +
    Coons stream 41 bytes/patch.
  - senão → `/ShadingType 4 /ColorSpace /DeviceRGB` + Gouraud
    P268+P268.2 preserved literal.
- **3 helpers Coons P270.3 perdem `#[allow(dead_code)]`** — agora
  em uso pelo dispatcher.

### Conic 2 emit paths AGORA AMBOS ACTIVOS

| Space | Strategy | Shading |
|---|---|---|
| Oklab/Oklch/sRGB/Luma/LinearRGB/HSL/HSV | Type 4 Gouraud (P268+P268.2 preserved) | `/ShadingType 4 /DeviceRGB` |
| **CMYK** | **Type 6 Coons (P270.4 activação)** | **`/ShadingType 6 /DeviceCMYK`** |

### Bug #4422 resolvido para Conic CMYK por construção

Cristalino emit `/ColorSpace /DeviceCMYK` correcto via Coons (paridade
P270.2 Linear+Radial). pdfkit #532 análogo confirma causa raiz
universal: dictionary errado por wrapper intermediário. Cristalino
implementação directa sem wrapper evita o bug em 3 variants × CMYK
absoluto.

### Reader compatibility Type 6 + DeviceCMYK

- Cairo precedent 20+ anos (Igalia blog 2020): universal suporte.
- Inkscape, Adobe Reader, Apple Preview: universal.
- pdf.js / PDFBOX: suporte adequado (vs Type 4 issues #6283 / #2100).
- PDF/A compliance: DeviceCMYK directo sem ICC; refino futuro
  candidato P-Gradient-CMYK-ICC.

### Adaptive N NÃO se aplica a Coons (clarificação)

Coons strategy "1 patch per stop" — N = `conic.stops.len()`. Não há
adaptive N a recalibrar (apenas em Gouraud P268.2). Sub-decisão prévia
"recalibrar factor_delta CMYK" preservada reserva para
P-Gradient-Adaptive-Multispace candidato futuro.

### Helpers reutilizados literal

- `bezier_control_points_for_arc` (P270.3) — coord patches preserved.
- `compute_coons_patches_n_stops` (P270.3) — preserved.
- `emit_conic_coons_stream` (P270.3 RGB) — template estrutural CMYK.
- `rgb_to_cmyk` (P270.2 fallback helper) — paridade extraction.
- `interpolate_cmyk` (L1 dispatcher P270 arm Cmyk) — via
  `Color::Cmyk` pattern-match.

Sub-padrão "Reutilização literal helpers cross-passos" **N=9 → N=10
cumulativo**.

### Cluster Gradient L1+stdlib+L3 emit feature-complete 24/24 absoluto

**Marco arquitectural máximo do cluster**:
- L1 sample: 3 variants × 8 spaces (P270).
- Stdlib named args: 3 variants × 8 spaces (P270).
- L3 PDF emit: 3 variants × 8 spaces (P270.1 + P270.2 + P270.4).

**24 combinações user-facing total** completamente materializadas.

### Sub-padrão "Anotação cumulativa em vez de ADR nova" N=9 → N=10 cumulativo

P258.B + P259.B + P263 + P265 + P268 + P268.2 + P270 + P270.1 + P270.2
+ **P270.4** anotada ADR-0092.

### Sub-padrão "ADR scope-out revogado parcialmente" N=5 → N=6 cumulativo

- N=1 P267 (ADR-0088 §Conic).
- N=2 P269 (ADR-0088 §focal_*).
- N=3 P270 (ADR-0083 §ColorSpace runtime).
- N=4 P270.2 (ADR-0083 §DeviceCMYK Linear+Radial).
- N=5 P270.3 (ADR-0090 §Type 6 Coons).
- **N=6 P270.4** (ADR-0091 §"Conic CMYK scope-out preserved"
  + ADR-0083 §DeviceCMYK definitivo).

**N=6 limiar formalização clara ainda mais ultrapassado** — meta-ADR
URGENTE FINAL.

### Marco final série P270

P270 + P270.1 + P270.2 + P270.3 + **P270.4** fecham cluster Gradient
feature-complete a nível user-facing. Cluster Color (ADR-0083) §"8/8
spaces" agora cobre L1 + L3 em 24/24 combinações.

Status `IMPLEMENTADO` preservado literal.

## Anotação cumulativa P271 — Sub-padrões formalizados

Esta ADR aplica todos os 5 sub-padrões formalizados via meta-ADRs
P271:

- **"Fase A com industry research proactiva"** (P270.3 pesquisa
  9 fontes pré-spec; N=4 cumulativo) → **ADR-0094 EM VIGOR**.
- **"Cap LOC hard vs soft explícito"** (P270.3 N=3; P270.4 N=4
  consolidação) → ADR-0094 §"Pattern 1".
- **"Reutilização literal helpers cross-passos"** (helpers P270.3 RGB
  reutilizados P270.4 CMYK; N=10 cumulativo) → ADR-0094 §"Pattern 2".
- **"Anotação cumulativa em vez de ADR nova"** (P270.4 anotação;
  N=10 cumulativo) → **ADR-0093 EM VIGOR**.
- **"ADR scope-out revogado parcialmente"** (cross-ref ADR-0090
  §Type 6 P270.3; cross-ref ADR-0091 §Conic CMYK P270.4; N=6
  cumulativo) → ADR-0093 §"Pattern 1".

Ver ADR-0093 + ADR-0094 EM VIGOR para meta-formalização. Status
`IMPLEMENTADO` preservado.

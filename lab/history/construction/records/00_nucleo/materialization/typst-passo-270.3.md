# typst-passo-270.3 — Conic Type 6 Coons Patch Mesh L3 emit infra-estrutura RGB (preparação CMYK P270.4)

**Magnitude**: M (cap composto: L3 hard ≤ 350 LOC + testes hard ≤ 25; cap soft L3 250 / testes 18).
**Cluster**: Visualize / Gradient / PDF export (refino L3 estratégia adicional).
**Tipo**: sub-passo .3 da série P270. Pattern análogo P270.1/P270.2.
**Origem**: ADR-0091 §"P-Gradient-Conic-CMYK candidato futuro"; pesquisa industry P270.2 + P270.3 preventiva revelou Cenário A revisado.
**Sequência**: P270.2 (Linear+Radial CMYK directo; Conic CMYK scope-out preserved) → **P270.3 (Coons RGB infra-estrutura; opt-in flag reserved)** → P270.4 (Coons CMYK activado; cluster 24/24 absoluto).
**Estratégia decidida**: Opção 2 decomposta (utilizador escolheu); Cenário A revisado — Type 6 Conic CMYK preparação + P-Gradient-Coons-Patch-RGB-Final futuro converge Conic RGB.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L3 nunca antes de prompt L0 + ADR. Ordem: diagnóstico → ADR-0092 criação PROPOSTO → prompt L0 → fix-hashes → testes-primeiro → código → ADR-0092 IMPLEMENTADO.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-conic-coons-passo-270-3.md` imutável. **Décimo primeiro consumo directo de fonte** (P262-P270.2 + **P270.3 vanilla Coons patches + Cairo precedente + ISO 32000-1 §7.5.7**).

3. **ADR-0092 criação PROPOSTO+IMPLEMENTADO mesmo passo** — formaliza:
    - Type 6 Coons como estratégia adicional Conic L3 emit.
    - Cenário A revisado: Type 6 activado apenas para `space == Cmyk` (P270.4 conecta; P270.3 reserva infra-estrutura).
    - Type 4 Gouraud preservado para 7 spaces RGB-family + perceptual (P268+P268.2+P270.1 não tocados; flag opt-in).
    - Sub-padrão "ADR PROPOSTO+IMPLEMENTADO mesmo passo" **N=6 → N=7 cumulativo**.

4. **ADR-0090 anotação cumulativa P270.3 (revogação parcial scope-out)** — §"Type 6/7 Coons/Tensor patches scope-out" revogado **parcialmente** (Type 6 sai; Type 7 preserved). Sub-padrão "ADR scope-out revogado parcialmente" **N=4 → N=5 cumulativo** (P267 + P269 + P270 + P270.2 + **P270.3**) — **limiar formalização clara** muito ultrapassado; candidato meta-ADR urgente.

5. **ADR-0089 anotação cumulativa P270.3** — Conic agora tem 2 emit paths (Type 4 Gouraud + Type 6 Coons coexistem).

6. **ADR-0091 anotação cumulativa P270.3** — preparação Cenário A revisado para P-Gradient-Conic-CMYK; P270.4 fecha.

7. **ADR-0087/ADR-0088 preservadas literal** — Linear/Radial intocados.

8. **ADR-0054 perfil graded DEBT-1**: anotação cumulativa P270.3 (infra-estrutura Coons adicionada; cluster Gradient ganha estratégia mesh-based industry-aligned).

9. **ADR-0018 preservado** — implementação autónoma; sem dependências externas.

10. **ADR-0039 preservado** — TextStyle intocado.

11. **Crystalline-lint zero violations** obrigatório.

12. **Reutilização literal**:
    - `multispace_sample_stops_conic` (P270.1) para amostragem stops.
    - `interpolate_in_space` (L1 P270 dispatcher arm RGB-family) para vertex colors.
    - Helpers Oklab P262/P263/P265/P268 reutilizáveis se necessário.
    - Sub-padrão "Reutilização literal helpers cross-passos" **N=8 → N=9 cumulativo**.

13. **Vanilla read-first autorizado** — `lab/typst-original/crates/typst-pdf/src/` Coons patches emit + `lab/typst-original/blog 2023 reference` (Coons 1 patch per stop) + Cairo `cairo_mesh_pattern_*` source via web research (não filesystem cristalino).

14. **Regressão tests P262-P270.2 proibida** — 2545 baseline preservado. Type 6 Coons opt-in flag por defeito desligado; Conic continua Type 4 Gouraud para todos os spaces P270.3. P270.4 activará para CMYK.

15. **Cap LOC hard vs soft explícito** (terceira aplicação consolida sub-padrão N=3):
    - **Cap hard L3**: 350 LOC. Estouro dispara §política condição 4.
    - **Cap soft L3**: 250 LOC. Estouro regista relatório.
    - **Cap hard testes**: 25.
    - **Cap soft testes**: 18.

16. **Type 6 Coons opt-in flag por defeito DESLIGADO P270.3** — 2545 baseline preservado bit-exact. P270.4 activa para `space == Cmyk`. Reader-compatibility refino futuro candidato P-Gradient-Coons-RGB-Final convergir Conic RGB.

17. **Industry research consolidada P270.3 §"Pesquisa empírica industry"** — pesquisa Type 4 Gouraud + CMYK reader inconsistente (pdf.js #6283 + PDFBOX-2100 + matplotlib #18034); Cairo Type 6/7 mesh-based industry-aligned (Igalia blog + W3C Workshop 2021).

---

## §1 — Sub-passo P270.3.A — Diagnóstico empírico Coons matemática + emit

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-conic-coons-passo-270-3.md`.

### Comandos exactos a executar

```bash
# 1. Cristalino L3 actual emit Conic Type 4 (P268+P268.2; preservado P270.3)
rg -n "emit_conic_gouraud_stream|compute_adaptive_n_conic|GradientObjectKind::Conic" 03_infra/src/export.rs | head -30

# 2. Cristalino L1 Conic struct + sample
rg -n "struct Conic|impl Conic|fn sample" 01_core/src/entities/gradient.rs | head -20

# 3. Cristalino helpers reutilizáveis P270.1
rg -n "multispace_sample_stops_conic|interpolate_in_space" 01_core/src/entities/gradient.rs 03_infra/src/export.rs | head -20

# 4. Vanilla typst Coons patches emit (pré-krilla blog 2023)
rg -n "Coons|coons|ShadingType.*6|patch|control.*point" lab/typst-original/crates/typst-pdf/src/ 2>/dev/null | head -30

# 5. Vanilla typst conic emit estrutura (krilla actual)
rg -n "Conic|conic|SweepGradient" lab/typst-original/crates/typst-pdf/src/paint.rs | head -20

# 6. Verificar se lab/krilla-reference/ disponível para Coons emit krilla
ls lab/krilla-reference/ 2>/dev/null && rg -n "Coons|coons|patch_mesh" lab/krilla-reference/src/ | head -20 || echo "krilla-reference indisponível; pular"

# 7. Tests P262-P270.2 baseline (regressão obrigatória)
cargo test -p typst-cristalino-infra gradient 2>&1 | tail -10
```

### Estrutura do diagnóstico (§A.1 a §A.16)

```
§A.1 Cristalino emit_conic_gouraud_stream actual (P268+P268.2 baseline):
     - Vertex bytes 18N (com adaptive N P268.2).
     - /ShadingType 4 /ColorSpace /DeviceRGB.
     - Preservado P270.3 (Type 6 é estratégia paralela).

§A.2 Cristalino sample/L1 helpers reutilizáveis:
     - multispace_sample_stops_conic (P270.1).
     - interpolate_in_space arm RGB-family (P270 dispatcher).

§A.3 Vanilla pré-krilla Coons emit (blog 2023):
     - Strategy literal: "1 patch per stop in the gradient".
     - Reasoning: Apple PDF reader não suporta shading function nos patches;
       força usar 1 patch por stop em vez de função.

§A.4 Vanilla krilla actual Coons emit:
     - Verificar via lab/krilla-reference/ ou web research.
     - Se opaco, blog 2023 strategy é referência canónica.

§A.5 PDF spec ISO 32000-1 §7.5.7 Type 6 estrutura:
     - 12 control points per patch (16 pontos com sharing extremidades).
     - 4 corner colors.
     - Edge flag: 0 (new patch) ou 1/2/3 (continuation).
     - BitsPerCoordinate, BitsPerComponent, BitsPerFlag, Decode array.

§A.6 PROPOSTA Cristalino Coons emit para Conic:
     - Strategy: 1 patch per stop (paridade Typst original blog 2023).
     - N stops → N patches angulares.
     - Cada patch: setor angular do disco; control points definem Bezier
       cúbicos aproximando arc círculo + radial linhas até centro.
     - Corner colors: stop_i (centro), stop_i (edge_start), stop_i+1 (edge_end), stop_i+1 (centro).
     - Wait: centro é singularidade; corners 1 e 4 são mesmo ponto. Convenção
       cor central = stop_i (paridade P268 P268.1-correção P270.2).

§A.7 Control points matemática Bezier cúbico para arc círculo:
     - Standard approximation: 4 Bezier cúbicos cobrem 360° com erro ~0.0003.
     - Para N patches angulares, cada cobre 360°/N graus.
     - Control point offset: r * (4/3) * tan(angle/4).
     - Reference: Stanislaw Adaszewski's "Drawing a Circle with Bezier Curves".

§A.8 PROPOSTA stream binary Type 6:
     - Per patch: 1 flag byte + 12 control points × 2 coord bytes + 4 corners × 3 bytes RGB.
     - Total per patch: 1 + 24 + 12 = 37 bytes.
     - N patches angulares (~stops count): 37N bytes total.
     - Comparação P268 Type 4 Gouraud: 18N bytes vertices (N adaptive ~32 default).
     - Type 6 mais bytes-per-stop mas N tipicamente menor (N stops vs N=32 vertices).

§A.9 PROPOSTA dispatcher cristalino:
     - emit_gradient_objects branch Conic ganha opt-in flag interno:
       - Opt-in flag DESLIGADO P270.3 (defeito) → Type 4 Gouraud preservado literal P268+P268.2.
       - Opt-in flag LIGADO P270.4 para space == Cmyk → Type 6 Coons.
     - Flag não exposto user-facing; decisão interna cristalino.

§A.10 PROPOSTA helpers L3 novos P270.3:
     - emit_conic_coons_stream (paridade emit_conic_gouraud_stream estrutural).
     - bezier_control_points_for_arc (matemática Bezier circle approximation).
     - compute_coons_patches_n_stops (1 patch per stop).
     - sample_stops_conic_for_coons_corners (corner colors per patch).

§A.11 Estimativa cap LOC P270.3:
     - emit_conic_coons_stream: ~100-130 LOC.
     - bezier_control_points_for_arc: ~30-40 LOC.
     - compute_coons_patches: ~30-40 LOC.
     - corner colors helper: ~20-30 LOC.
     - Dispatcher branching minimal: ~10-15 LOC (flag opt-in default OFF).
     - Tests: ~120-150 LOC.
     - Total L3 production: ~200-260 LOC.
     - Cap hard 350 com folga ~25-43%; cap soft 250 ligeiramente acima
       provável.

§A.12 Defaults preservam P270.2:
     - Flag opt-in OFF: emit_conic_gouraud_stream literal preserved.
     - 2545 baseline bit-exact preserved.
     - Verificar empíricamente.

§A.13 Industry research consolidada:
     - Cairo Type 6/7 mesh patches (Igalia blog 2020).
     - Inkscape Type 7 (Cairo follower).
     - Typst original blog 2023 Type 6 (1 patch per stop).
     - W3C Workshop 2021: "the only way we can render conic gradients" em PDF é Coons.
     - Reader compatibility Type 4 problemático (pdf.js #6283, PDFBOX-2100, matplotlib #18034).
     - Type 6 industry-standard para conic; cristalino converge.

§A.14 Cenário detectado:
     - **B1 fecho conceptual** (infra-estrutura Coons RGB OFF; preparação P270.4).
     - **B2 sub-passos** (se Coons matemática revelar complexidade não prevista).

§A.15 Decisão arquitectural — Cenário A revisado P270.3.

§A.16 Pendência P270.4: Coons CMYK conecta opt-in flag para
     `space == Cmyk`; revoga ADR-0091 §"Conic CMYK scope-out preserved";
     cluster L3 24/24 absoluto.
```

### Critério de aceitação Fase A

- §A.6 confirma "1 patch per stop" estratégia paridade Typst original.
- §A.7 confirma matemática Bezier cúbico para arc círculo (offset = r·(4/3)·tan(angle/4)).
- §A.9 confirma flag opt-in default OFF preserva P268+P268.2 bit-exact.
- §A.12 confirma 2545 baseline preserved.
- §A.13 industry research factualmente verificada (Cairo/Inkscape/blog Typst 2023/W3C).
- §A.14 cenário B1 esperado; B2 dispara §política condição 1.

---

## §2 — Sub-passo P270.3.B — ADR-0092 criação + anotações cumulativas

### B.1 — ADR-0092 estrutura

Ficheiro novo `00_nucleo/adr/typst-adr-0092-conic-coons-patches-rgb-cmyk.md`.

```
# ADR-0092 — Conic Type 6 Coons Patch Mesh L3 emit (preparação CMYK + RGB futuro)

**Status**: PROPOSTO → IMPLEMENTADO (sub-padrão N=7 cumulativo)
**Data**: 2026-05-17
**Passo origem**: P270.3 (preparação infra-estrutura RGB; P270.4 activa CMYK).
**Cluster**: Visualize / Gradient / PDF export
**Tipo**: estratégia adicional Conic L3 emit (Cenário A revisado)

## Contexto

ADR-0089 + ADR-0090 estabeleceram Type 4 Gouraud como estratégia
cristalina Conic L3 emit (P268 + P268.2 adaptive N hybrid). ADR-0091
materializou ColorSpace runtime cross-variant (P270 + P270.1 + P270.2);
Conic CMYK ficou scope-out preserved P270.2 (Cenário B) por
reader-compatibility incerto Type 4 Gouraud + DeviceCMYK.

Pesquisa industry preventiva P270.3 (pdf.js #6283 + PDFBOX-2100 +
matplotlib #18034 + Cairo Igalia 2020 + Typst blog 2023 + W3C Workshop
2021) revelou:

- Type 4 Gouraud + CMYK reader compatibility problemático (Adobe
  Illustrator "unknown imaging construct"; pdf.js Type 4 inconsistente).
- Industry mesh-based para conic = Type 6 Coons Patches (Cairo,
  Inkscape, Typst original pré-krilla).
- Typst original blog 2023 strategy literal: "1 patch per stop".
- W3C 2021: "the only way we can render conic gradients" em PDF é
  Coons.

Cristalino diverge intra-família mesh (Type 4 vs Type 6 industry).
ADR-0090 §"Type 6/7 Coons/Tensor patches scope-out" listou Type 6
como candidato futuro; este passo revoga parcialmente para
materializar via "P-Gradient-Conic-CMYK" Cenário A revisado.

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

3. **Matemática Bezier cúbico**: control points offset = r · (4/3) ·
   tan(angle/4) — standard approximation arc círculo (Stanislaw
   Adaszewski).

4. **Dispatcher opt-in flag interno** (não user-facing):
   - P270.3: flag OFF default → emit_conic_gouraud_stream literal
     preserved.
   - P270.4: flag ON para `space == Cmyk` → emit_conic_coons_stream
     activado.
   - Flag interno cristalino; decisão arquitectural não exposta API.

5. **Type 7 Tensor patches preserved scope-out** — apenas Type 6 sai
   do scope-out ADR-0090. Type 7 candidato futuro adicional se
   necessário.

## Pesquisa empírica industry consolidada

| Projecto | Estratégia Conic PDF | Tipo PDF | Reader compatibility |
|---|---|---|---|
| Cairo | Coons/Tensor mesh | /ShadingType 6 ou 7 | universal (20+ anos) |
| Inkscape | Tensor patches | /ShadingType 7 | universal |
| Typst original pré-krilla | Coons 1 patch per stop | /ShadingType 6 | universal (Apple Preview confirmed) |
| Typst actual via krilla | desconhecido publicamente | abstracção | ? |
| **Cristalino actual P270.2** | **Type 4 Gouraud + CMYK scope-out** | **/ShadingType 4 + RGB only** | Type 4 problemático em readers minoritários |
| **Cristalino P270.3+P270.4** | **Type 4 RGB + Type 6 CMYK** | **dispatcher dual** | universal para CMYK; preservado P268 para RGB |

## Convenção cor central preservada

Cor central = primeiro stop (paridade P268+P268.1-correção+P270.2).
Em Coons Patch Mesh, corner 1 e corner 4 de cada patch são mesmo
ponto (centro do disco); ambos = stop_i (primeiro corner = stop início
patch, último corner = stop fim patch... wait, em Coons isto é mais
nuanceado; §A.6 diagnóstico clarifica).

## Consequências

+ Cluster Gradient L3 emit Cluster A revisado preparado para 24/24
  absoluto (P270.4 fecha).
+ Industry alignment forte (Cairo/Inkscape/Typst original Type 6
  precedent).
+ Reader compatibility Conic CMYK fica universal via Type 6.
+ Cristalino preserva Type 4 RGB (2545 baseline; ADR-0090 estratégia
  perceptível original).
- Cristalino tem 2 estratégias Conic emit coexistentes — complexidade
  arquitectural adicional.
- ADR-0090 §"Type 6/7 Coons/Tensor patches scope-out" revogado
  parcialmente (Type 6; Type 7 preserved).

## Scope-outs preserved

- **Type 7 Tensor patches**: scope-out preserved. Cairo/Inkscape usam
  para conic complexo; cristalino Type 6 1-patch-per-stop suficiente
  para Conic CMYK.
- **P-Gradient-Coons-RGB-Final**: candidato futuro converger Conic RGB
  de Type 4 Gouraud para Type 6 Coons (eliminar 2 estratégias). Se
  P270.4 confirmar Type 6 reader compatibility, pode tornar-se
  obrigatório.

## Alternativas consideradas

- **Cenário B P270.2** (Conic CMYK scope-out preserved final):
  rejeitada — utilizador escolheu fechar cluster.
- **Type 4 Gouraud + CMYK directo** (cluster 24/24 sem mudar estratégia):
  rejeitada — pesquisa P270.3 confirma reader inconsistente.
- **Type 7 Tensor patches**: rejeitada — magnitude L+ adicional sem
  ganho factual demonstrado vs Type 6.
- **Reverter Type 4 RGB para Type 6 imediatamente**: rejeitada —
  custo L sem ganho factual demonstrado; preserved candidato futuro.

## Critério revisão

Esta ADR pode ser revisitada se:
- P270.4 revelar Type 6 reader compatibility problemático para CMYK.
- P-Gradient-Coons-RGB-Final tornar-se prioridade (Type 4 banding RGB
  detectado empíricamente).
- Vanilla krilla actual revelar-se Coons (cristalino convergiria
  totalmente).
- Type 7 Tensor tornar-se necessário para conic complexo (improvável).

## Subpadrões aplicados

- ADR PROPOSTO+IMPLEMENTADO mesmo passo: **N=7 cumulativo**.
- ADR scope-out revogado parcialmente: **N=5 cumulativo (limiar
  formalização clara muito ultrapassado)** — candidato meta-ADR
  urgente (P267 + P269 + P270 + P270.2 + **P270.3**).
- Anotação cumulativa cross-ADR: **N=4 cumulativo** (P270 + P270.1 +
  P270.2 + **P270.3**).
- Fase A com industry research proactiva: **N=3 cumulativo** (P270 +
  P270.2 + **P270.3**).
- Reutilização literal helpers cross-passos: **N=9 cumulativo**.
- Diagnóstico imutável (décimo primeiro consumo de fonte): **N=16**.

## Referências cross-passos

- P262/P264/P267 — Variant L1+stdlib.
- P263/P265/P268 — L3 emit templates (Type 4 Gouraud preserved).
- P268.2 — Adaptive N hybrid Conic (preserved).
- P269 — Radial focal_* (preserved).
- P270 — ColorSpace runtime L1+stdlib (ADR-0091).
- P270.1 — L3 emit 7 spaces (preserved).
- P270.2 — L3 Linear+Radial CMYK directo (Cenário B; Conic CMYK
  scope-out preserved revogado por P270.3+P270.4).
- **P270.4** — Coons CMYK activação opt-in flag ON; fecha cluster
  24/24 absoluto.
- **P-Gradient-Coons-RGB-Final** — candidato futuro converger Conic
  RGB.

## ADRs anotadas cumulativamente P270.3

- ADR-0089 (Gradient Conic-only): 2 emit paths agora coexistem.
- ADR-0090 (Type 4 strategy): §Type 6 scope-out revogado parcialmente.
- ADR-0091 (ColorSpace runtime + CMYK): preparação P270.4.
- ADR-0054 (Perfil graded): infra-estrutura Coons adicionada.

## Industry research fontes

- Igalia blog 2020 conic gradients (Cairo Type 6/7).
- Typst blog 2023 "Color gradients and my gradual descent into madness"
  (Type 6 1 patch per stop).
- W3C CSS-Color-4 Workshop 2021 Mike Bremford (Coons como única forma
  conic em PDF).
- pdf.js issue #6283 (Type 4 Gouraud not supported).
- Apache PDFBOX-2100 (Type 4 historical broken).
- matplotlib issue #18034 (Type 4 + Illustrator "unknown imaging
  construct").
- Stanislaw Adaszewski "Drawing a Circle with Bezier Curves".
- ISO 32000-1 §7.5.7 (Type 6 Coons spec literal).
```

### B.2 — ADR-0090 anotação cumulativa P270.3 (revogação parcial scope-out)

Adicionar após anotação P268.2 + P268.1-correção:

```
## Anotação cumulativa P270.3 — Type 6 Coons scope-out revogado parcialmente

§"Scope-outs preserved" §"Type 6/7 Coons/Tensor patches" revogado
parcialmente:
- **Type 6 Coons**: revogado P270.3 — materialização cristalina como
  estratégia adicional Conic (paralela Type 4 Gouraud; opt-in flag
  para CMYK P270.4). Ver ADR-0092 EM VIGOR.
- **Type 7 Tensor**: preserved scope-out — refino futuro candidato.

ADR-0090 decisão de fundo (Type 4 Gouraud RGB cristalino) preservada
literal — Type 6 é estratégia adicional, não substituição.

Sub-padrão "ADR scope-out revogado parcialmente" **N=5 cumulativo
limiar formalização clara ultrapassado**; candidato meta-ADR urgente.
```

### B.3 — ADR-0089 anotação cumulativa P270.3

```
## Anotação cumulativa P270.3 — Conic 2 emit paths coexistem

Conic L3 emit ganha estratégia adicional:
- **Type 4 Gouraud** (P268+P268.2 preserved): 7 spaces RGB-family
  + perceptual.
- **Type 6 Coons** (P270.3 infra + P270.4 activação): CMYK; futuro
  RGB se P-Gradient-Coons-RGB-Final.

Dispatcher opt-in flag interno (não user-facing). P270.3 default OFF;
P270.4 ON para `space == Cmyk`. Ver ADR-0092 EM VIGOR.
```

### B.4 — ADR-0091 anotação cumulativa P270.3 (preparação P270.4)

```
## Anotação cumulativa P270.3 — Preparação Conic CMYK via Type 6 Coons

ADR-0091 §"Conic CMYK scope-out preserved" P270.2 fica preparado para
revogação P270.4 via infra-estrutura Type 6 Coons materializada P270.3.

Cluster Gradient L3 emit Cenário A revisado:
- Linear+Radial CMYK (P270.2 Cenário B preserved).
- Conic CMYK via Type 6 Coons (P270.3 infra + P270.4 activação).

P270.4 fecha cluster L3 24/24 absoluto. Ver ADR-0092 EM VIGOR.
```

### B.5 — ADR-0054 anotação cumulativa P270.3

```
P270.3 — infra-estrutura Type 6 Coons Patch Mesh materializada como
estratégia adicional Conic L3 emit. Cluster Gradient ganha
industry-aligned mesh-based para conic CMYK (Cairo/Inkscape/Typst
original Type 6 precedent). Perfil graded DEBT-1 preservado.
```

### B.6 — L0 `entities/gradient.md` anotação P270.3

Adicionar após anotação P270.2:

```
**Anotação P270.3**: infra-estrutura Type 6 Coons Patch Mesh
materializada como estratégia adicional Conic L3 emit. Dispatcher
opt-in flag interno (default OFF P270.3; ON P270.4 para CMYK).
Strategy "1 patch per stop" (paridade Typst original blog 2023).
Matemática Bezier cúbico arc círculo (offset = r·(4/3)·tan(angle/4)).
Type 4 Gouraud preservado P268+P268.2 + P270.1. Ver ADR-0092 EM VIGOR.
```

### B.7 — Hashes propagados

`crystalline-lint --fix-hashes` propaga hashes em L0. Zero violations.

---

## §3 — Sub-passo P270.3.C — Materialização L3 (testes primeiro)

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0092 criada PROPOSTO §2.B.1.
3. ADR-0090/0089/0091/0054 anotações §2.B.2-§2.B.5.
4. L0 anotação §2.B.6.
5. `crystalline-lint --fix-hashes`.
6. **Testes-primeiro** — adicionar ~15-20 testes ANTES de qualquer LOC L3.
7. L3 código — helpers Coons + emit_conic_coons_stream + dispatcher branching.
8. Verificação final.
9. ADR-0092 promoção PROPOSTO → IMPLEMENTADO.

### Cap LOC

- **L3 hard**: 350 LOC. Estouro dispara §política condição 4.
- **L3 soft**: 250 LOC. Estouro regista relatório.
- **Testes hard**: 25.
- **Testes soft**: 18.

### Helpers L3 esperados

```rust
// 03_infra/src/export.rs P270.3

/// Matemática Bezier cúbico para arc círculo (Stanislaw Adaszewski).
/// Returns 2 control points entre start_angle e end_angle.
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
    
    // Control point 1: rotated 90° from start tangent.
    let cp1 = (
        center.0 + radius * cos_s - offset * sin_s,
        center.1 + radius * sin_s + offset * cos_s,
    );
    // Control point 2: rotated -90° from end tangent.
    let cp2 = (
        center.0 + radius * cos_e + offset * sin_e,
        center.1 + radius * sin_e - offset * cos_e,
    );
    
    [cp1, cp2]
}

/// Cada patch cobre 360°/N graus angulares onde N = stops count.
/// Strategy: 1 patch per stop (paridade Typst original blog 2023).
fn compute_coons_patches_n_stops(conic: &Conic) -> usize {
    conic.stops.len()  // N stops = N patches angulares
}

/// Emit Coons Patch Mesh stream binary.
/// 1 flag byte + 12 control points × 2 coord bytes + 4 corner colors × 3 bytes RGB
/// = 37 bytes per patch.
fn emit_conic_coons_stream(conic: &Conic) -> Vec<u8> {
    let n = compute_coons_patches_n_stops(conic);
    let mut stream = Vec::with_capacity(37 * n);
    
    let center = (0.5, 0.5);  // disco unidade per P268 convenção
    let radius = 0.5;
    
    for i in 0..n {
        let stop_curr = &conic.stops[i];
        let stop_next = &conic.stops[(i + 1) % n];
        
        let angle_start = (i as f32) / (n as f32) * std::f32::consts::TAU;
        let angle_end = ((i + 1) as f32) / (n as f32) * std::f32::consts::TAU;
        
        // Patch flag: 0 (new patch); 1/2/3 (continuation) podem optimizar.
        // P270.3 default: flag 0 (todos novos).
        let flag: u8 = 0;
        
        // Edge 1: arc círculo de angle_start → angle_end.
        let cp_arc = bezier_control_points_for_arc(center, radius, angle_start, angle_end);
        
        // Edge 2: radial linha do edge final até centro.
        // Edge 3: arc (mesmo ponto central; degenerado).
        // Edge 4: radial linha do centro até edge inicial.
        // (Detalhes Coons matemática nos §A.6 diagnóstico).
        
        let corner_colors_rgb = [
            stop_curr.color.to_rgba_f32(),  // corner 1: centro
            stop_curr.color.to_rgba_f32(),  // corner 2: edge_start (mesmo stop)
            stop_next.color.to_rgba_f32(),  // corner 3: edge_end
            stop_next.color.to_rgba_f32(),  // corner 4: centro (singularidade)
        ];
        
        stream.push(flag);
        // 12 control points × 2 coord bytes per patch.
        // 4 corner colors × 3 bytes RGB per patch.
        // ... (detalhes binary emit §A.6 diagnóstico).
    }
    
    stream
}

/// Dispatcher branching em emit_gradient_objects branch Conic.
GradientObjectKind::Conic(conic) => {
    // P270.3: flag opt-in interno default OFF.
    let use_coons_emit = false;  // P270.3 reservado; P270.4 ON para Cmyk.
    
    if use_coons_emit {
        let stream = emit_conic_coons_stream(conic);
        let shading = format!(
            "<< /ShadingType 6 /ColorSpace /DeviceRGB \
               /BitsPerCoordinate 8 /BitsPerComponent 8 \
               /BitsPerFlag 8 \
               /Decode [0 1 0 1 0 1 0 1 0 1] >>"
        );
        // ... emit Type 6 patches
    } else {
        // P268+P268.2 literal preserved (default).
        let n = compute_adaptive_n_conic(conic);
        let stream = emit_conic_gouraud_stream(conic, n);
        // ...
    }
}
```

### Estrutura testes esperada

**Unit helpers Coons** (8 tests):
- `p270_3_bezier_control_points_for_arc_quarter_circle`: 90° → control points correctos.
- `p270_3_bezier_control_points_offset_formula`: offset = r·(4/3)·tan(angle/4) verificado.
- `p270_3_compute_coons_patches_n_stops_2_stops`: 2 patches.
- `p270_3_compute_coons_patches_n_stops_8_stops`: 8 patches.
- `p270_3_emit_conic_coons_stream_size_n_stops`: stream size = 37·N bytes.
- `p270_3_coons_corner_colors_paridade_first_stop`: convenção cor central preservada.
- `p270_3_coons_flag_byte_per_patch`: flag 0 per patch (P270.3 default).
- `p270_3_coons_stream_4_corner_rgb_bytes`: 4 corners × 3 RGB bytes.

**E2E PDF dispatcher opt-in flag** (4 tests):
- `p270_3_export_pdf_conic_opt_in_default_off_preserva_p268`: 2545 baseline bit-exact.
- `p270_3_export_pdf_conic_opt_in_on_emit_shading_type_6`: confirma `/ShadingType 6`.
- `p270_3_export_pdf_conic_opt_in_devicergb_p270_3`: RGB ColorSpace P270.3.
- `p270_3_export_pdf_cluster_3_variants_opt_in_off`: cluster preserved.

**Snapshot determinístico** (3 tests):
- `p270_3_pdf_bytes_opt_in_off_reproduziveis`: 2545 baseline preserved.
- `p270_3_pdf_bytes_opt_in_on_coons_reproduziveis`.
- `p270_3_pdf_bytes_bezier_control_points_reproduziveis`.

**Regressão P262-P270.2** (não novos; verificar verdes):
- 2545 baseline preserved literal (flag opt-in OFF default).

Total esperado: 8 + 4 + 3 = **15 testes**. Cap soft 18 / hard 25; folga grande.

---

## §4 — Sub-passo P270.3.D — Promoção + README + relatório

1. **ADR-0092** PROPOSTO → IMPLEMENTADO (sub-padrão N=7 cumulativo).
2. **ADR-0090** anotação cumulativa P270.3 (§Type 6 scope-out revogado parcialmente).
3. **ADR-0089** anotação cumulativa P270.3 (2 emit paths Conic).
4. **ADR-0091** anotação cumulativa P270.3 (preparação P270.4).
5. **ADR-0054** anotação cumulativa P270.3.
6. **README.md** actualizar:
   - Tabela cobertura Visualize PDF (preparação P270.4; cobertura efectiva preserved P270.2 ~83-85%).
   - Entrada P270.3 ~70-90 linhas (infra-estrutura + ADR-0092 dedicada).
   - Cross-reference ADR-0092 EM VIGOR.
7. **Distribuição ADRs**: total 78 → **79** (ADR-0092 IMPLEMENTADO; EM VIGOR 33 preserved; IMPLEMENTADO 30 → 31).
8. **Relatório** `00_nucleo/materialization/typst-passo-270-3-relatorio.md`:
   - Métricas finais (esperado 2545 + 15 = ~2560).
   - Fase A §A.6/§A.7/§A.14 decisões documentadas.
   - Diff helpers L3 antes/depois.
   - Sub-padrões + N cumulativo (5 atingem limiar formalização clara).
   - Regressão zero 2545 baseline preserved.
   - **Infra-estrutura Coons RGB materializada; P270.4 conecta CMYK**.

---

## §política de paragem

1. **§A.6/§A.7 matemática Coons não trivial** — se diagnóstico revelar
   complexidade Bezier control points para conic singularidade
   centro maior que estimativa §A.11, magnitude estoura.

2. **Cap LOC L3 hard (350) ameaça ser ultrapassado** — refactor maior
   que estimativa §A.11 (~200-260 LOC). Confirmar antes de continuar.

3. **Cap testes hard (25) ameaça ser ultrapassado**.

4. **§A.12 verificação falha**: flag opt-in OFF default não preserva
   2545 baseline bit-exact. §política absoluta.

5. **Snapshot bytes PDF não reproduzíveis** — float non-determinism em
   Bezier control points (`tan`, `sin_cos` precision).

6. **Crystalline-lint reporta violations** após anotações.

7. **Regressão tests P262-P270.2** — qualquer test anterior falha.
   §política absoluta.

8. **Coons corner colors convenção ambígua** — singularidade centro
   patch (corner 1 e corner 4 mesmo ponto físico mas cores
   diferentes); §A.6 diagnóstico decide.

9. **PDF spec ISO 32000-1 §7.5.7 stream binary detalhes ambíguos** —
   alguns detalhes Type 6 (Bounds array; Function reference)
   requerem leitura cuidadosa.

10. **Cluster Gradient marco quebra** — `p268_export_pdf_cluster_3_variants_coexistem` falha.

11. **Dispatcher branching expande call sites externos** — outros
    sítios export.rs chamam Conic emit directamente.

12. **Industry research §A.13 factualmente incorrecta** — alguma das
    citações Cairo/Inkscape/Typst blog não verificável.

13. **ADR-0090 revogação total ameaçada** — anotação cumulativa toca
    Type 7 ou outros pontos scope-out além de Type 6.

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N pós-P270.3 | Nota |
|---|---|---|
| ADR PROPOSTO+IMPLEMENTADO mesmo passo | **N=6 → N=7 cumulativo** | + P270.3 ADR-0092 |
| **ADR scope-out revogado parcialmente** | **N=4 → N=5 cumulativo (limiar muito ultrapassado)** | + P270.3 (Type 6) — **candidato meta-ADR URGENTE** |
| Anotação cumulativa em vez de ADR nova | N=9 preserved | P270.3 cria ADR-0092 (não anota); 4 anotações cumulativas paralelas |
| Reutilização literal helpers cross-passos | **N=8 → N=9 cumulativo** | + P270.3 (P270 dispatcher + P270.1 helpers L3 templates) |
| Diagnóstico imutável (décimo primeiro consumo) | **N=15 → N=16 cumulativo** | + P270.3 |
| Auditoria condicional (ADR-0084) | **N=14 → N=15 cumulativo** | + P270.3 |
| Auto-aplicação ADR-0065 inline | **N=13 → N=14 cumulativo** | + P270.3 |
| Cap LOC hard vs soft explícito | **N=2 → N=3 cumulativo** | P270.1 + P270.2 + **P270.3** — consolidação clara |
| Anotação cumulativa cross-ADR | **N=3 → N=4 cumulativo** | + P270.3 (4 ADRs anotadas + ADR-0054) |
| Fase A com industry research proactiva | **N=2 → N=3 cumulativo (limiar formalização clara)** | + P270.3 — candidato meta-ADR formalização |

### Marcos arquitecturais P270.3

**Primeiro caso "2 estratégias L3 emit coexistem para mesmo variant"** — Conic ganha Type 4 Gouraud (RGB) + Type 6 Coons (CMYK preparation). Estabelece precedente para futuras divergências intra-emit fundamentadas em reader-compatibility.

**Sub-padrão "ADR scope-out revogado parcialmente" N=5 limiar formalização clara muito ultrapassado** — candidato meta-ADR URGENTE. Pattern: P267 Conic + P269 focal_* + P270 ColorSpace + P270.2 DeviceCMYK + P270.3 Type 6 Coons.

**Sub-padrão "Fase A com industry research proactiva" N=3 atinge limiar formalização clara** — P270 + P270.2 + P270.3. Candidato meta-ADR junto com "scope-out revogado".

**Sub-padrão "Cap LOC hard vs soft explícito" N=3 consolidação clara** — P270.1 + P270.2 + P270.3. Padrão estabelecido.

### Sequência pós-P270.3

- **P270.4** — Coons CMYK activação opt-in flag ON para `space == Cmyk`. Revoga ADR-0091 §"Conic CMYK scope-out preserved" definitivamente. Cluster L3 24/24 absoluto. Magnitude S esperada.
- **P-Gradient-Coons-RGB-Final** — candidato futuro converge Conic RGB de Type 4 Gouraud para Type 6 Coons. Magnitude L. Não bloqueante.
- **Meta-ADR formalização sub-padrões N=5 (scope-out parcial), N=3 (industry research), N=3 (cap hard/soft)** — passo administrativo XS candidato futuro paridade P260 ADR-0084/0085.
- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).

---

## §referências cross-passos

- **P268** — PDF Conic Type 4 Gouraud (preserved; opt-in flag default OFF P270.3 + P270.4 ON CMYK).
- **P268.1-correção** — ADR-0090 correcção factual (preserved).
- **P268.2** — Adaptive N hybrid (preserved; aplicado a Type 4 Gouraud).
- **P270/P270.1/P270.2** — ColorSpace runtime + L3 emit RGB + L3 emit CMYK Linear+Radial (preservados; Conic CMYK scope-out preserved P270.2 revogado por P270.3+P270.4).
- **P270.4** — Coons CMYK activação (próximo passo).
- ADR-0089 — Gradient Conic-only (anotada cumulativa P270.3).
- ADR-0090 — Type 4 strategy (anotada cumulativa P270.3 §Type 6 revogado parcialmente).
- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa P270.3 preparação P270.4).
- ADR-0092 — Conic Coons Patches (criada PROPOSTO+IMPLEMENTADO P270.3).
- ADR-0054 — Perfil graded (anotada cumulativa P270.3).
- ADR-0018 — Whitelist crates (preservada).
- ADR-0085 — Diagnóstico imutável (décimo primeiro consumo).

---

## §0.1 — Notas de execução para Claude Code

- **Fase A §A.6 + §A.7 críticas** — matemática Coons + Bezier control points decide complexidade real LOC. Se estimativa §A.11 exceder, §política condição 1 dispara.
- **Flag opt-in OFF default P270.3** — verificar empíricamente que 2545 baseline preserved bit-exact via `cargo test p262_ p264_ p265_ p267_ p268_ p269_ p270_`. §política condição 4 absoluta.
- **Snapshot bytes determinísticos** — Bezier `tan`, `sin_cos` podem ter precision issues; verificar com snapshot tests.
- **Regressão tests P262-P270.2 zero** (2545 baseline) — §política condição 7 absoluta.
- **ADR-0092 PROPOSTO+IMPLEMENTADO mesmo passo** — paridade pattern P257/P261/P262/P264/P267/P270 (N=7 cumulativo).
- **Anotações cumulativas 4 ADRs paralelo** (ADR-0090 + ADR-0089 + ADR-0091 + ADR-0054) — verificar coerência cada anotação refere ADR-0092 EM VIGOR.
- **Cap hard L3 350 + testes hard 25** — gate absoluto; estouro dispara §política.
- **Cap soft L3 250 + testes soft 18** — informativo; estouro regista mas continua.
- **Industry research §A.13** — verificar Igalia blog 2020 + Typst blog 2023 + W3C 2021 + pdf.js #6283 + PDFBOX-2100 + matplotlib #18034 + Stanislaw Adaszewski + ISO 32000-1 §7.5.7.
- **Coons matemática Bezier**: `offset = radius * (4.0 / 3.0) * (angle_delta / 4.0).tan()` — verificar literal Stanislaw Adaszewski source.
- **Relatório final esperado**: 2545 + 15 = ~2560 testes verdes; hash drift L0; lint zero; ADRs 78 → 79 (ADR-0092 IMPLEMENTADO).
- **Marco "2 estratégias L3 emit Conic coexistem"** documentado em relatório §1 + ADR-0092 §"Decisão".

# ⚖️ ADR-0091: Gradient ColorSpace runtime cross-variant + CMYK strategy

**Status**: **`IMPLEMENTADO`** (PROPOSTO P270.B → **IMPLEMENTADO
P270.D** — materialização cumpre L1+stdlib 3 variants × 8 spaces;
defaults Oklab preservam P262/P264/P267 bit-exact)
**Data**: 2026-05-17
**Autor**: Humano + IA
**Passo origem**: P270
**Cluster**: Visualize / Gradient / Color
**Tipo**: activação de feature cross-variant + estratégia L3 documentada
**Validado**: Passo 270 (criação PROPOSTO → promoção IMPLEMENTADO mesmo
passo via Cenário B1; pattern análogo ADR-0083/0086/0087/0088/0089 N=5 →
N=6 cumulativo).
**Diagnóstico prévio**:
`00_nucleo/diagnosticos/diagnostico-gradient-space-passo-270.md`
(imutável per ADR-0085 — **oitavo consumo directo de fonte**
pós-P262/P264/P267/P268/P268.1/P268.2/P269).

---

## Contexto

ADR-0083 §"Scope-outs" listou:
- **ColorSpace runtime**: cross-variant em gradient — scope-out P257.
- **DeviceCMYK PDF** — scope-out P257.

P270 revoga parcialmente o primeiro (L1+stdlib materializado).
DeviceCMYK PDF preservado scope-out P270 — revogação adiada P270.2.

Cluster Gradient cristalino actual usa **Oklab hardcoded** em sample
matemática (P262/P264/P267) e pré-amostragem N=16 Oklab em L3 emit
(P263/P265/P268). Activar `space: ColorSpace` cross-variant permite
`gradient.linear(red, blue, space: "hsl")` interpolando em HSL conforme
vanilla user-facing semantics.

Sub-padrão **"ADR PROPOSTO+IMPLEMENTADO mesmo passo"** N=5 → **N=6
cumulativo** (P257/P261/P262/P264/P267/**P270**).

Sub-padrão **"ADR scope-out revogado parcialmente"** N=2 → **N=3
cumulativo** (P267 Conic + P269 focal_* + **P270 ColorSpace**). **Atinge
limiar formalização clara N=3-5**; candidato meta-ADR formalização
futura.

---

## Pesquisa industry consolidada (P270 proactiva)

**Sub-padrão "Fase A com industry research proactiva"** N=1 inaugural —
P270 é primeira aplicação preventiva pré-spec vs P268.1/P268.1-correção
reactivas.

Vanilla typst implementa via **estratégia dual**:

### Family A — perceptual spaces (Oklab, Oklch, HSL, HSV)

- L1 sample: interpola no space escolhido com hue-wrap shorter para
  HSL/Oklch (vanilla `mix_iter` linha 1126-1136).
- L3 PDF emit: pré-amostragem N=16 → DeviceRGB. Documentação vanilla:
  > "PDF gradients in the color.oklab, color.hsv, color.hsl, and
  > color.oklch color spaces are stored as a list of color.rgb colors
  > with extra stops in between."

### Family B — PDF-native spaces (sRGB, LinearRGB, Luma, CMYK)

- L1 sample: interpola no space directamente.
- L3 PDF emit: directo `/DeviceRGB`, `/DeviceGray`, `/DeviceCMYK`.

### Bug conhecido vanilla #4422

CMYK gradient PDF emit tem problemas reportados (typst/typst #4422;
comportamento depende viewer/printer). Cristalino terá oportunidade de
melhorar isto em P270.2.

---

## Decisão P270 (este passo)

1. **L1 sample multi-space**: cada variant (Linear/Radial/Conic) ganha
   campo `space: ColorSpace` (default Oklab; preserva P262/P264/P267
   behavior).

2. **`ColorSpace` enum criado** (`01_core/src/entities/color.rs`):

   ```rust
   pub enum ColorSpace {
       Oklab, Oklch, Srgb, Luma, LinearRgb, Hsl, Hsv, Cmyk,
   }
   ```

   8 variants paridade vanilla (Luma ≡ D65Gray nome cristalino histórico
   P257).

3. **Stdlib named arg `space: Str`** cross-variant. Validação: deve
   ser um dos 8 ColorSpace válidos via parser
   `parse_color_space(&str) -> SourceResult<ColorSpace>`.

4. **Hue-wrap shorter default** para HSL/Oklch/HSV (CSS standard;
   paridade vanilla literal portado).

5. **L3 emit estratégia documentada (não materializada P270)**: Op B
   estratégia uniforme — Oklab pipeline pré-amostragem N=16 para 7
   spaces RGB-family; CMYK directo `/DeviceCMYK` único caso especial.

---

## Decisão L3 futura (P270.1 + P270.2)

### P270.1 — RGB-family + perceptual (M+; ~300-400 LOC)

7 spaces (sRGB, LinearRGB, Luma, Oklab, Oklch, HSL, HSV) emit via Oklab
pipeline N=16 → DeviceRGB.

**Convergência parcial** com vanilla Family A (oklab/oklch/hsl/hsv).
**Divergência intencional** com vanilla Family B — cristalino usa Oklab
pipeline também para sRGB/LinearRGB/Luma em vez de DeviceRGB directo.

**Justificação Op B**:
- Simplifica L3 (1 pipeline para 7 spaces vs 4+ em vanilla).
- Banding visual imperceptível em sRGB/LinearRGB/Luma com N=16.
- Reutiliza helpers P263/P265/P268 (sub-padrão "Reutilização literal").

### P270.2 — CMYK directo (S+; ~150-200 LOC)

CMYK emit directo `/DeviceCMYK` único caso especial. Revoga ADR-0083
§"DeviceCMYK PDF" scope-out. Pode resolver bug vanilla #4422 com
implementação cristalina autónoma.

---

## Defaults preservam P262/P264/P267 behavior — zero regressão

- `Gradient::linear(stops, angle)` mantém assinatura; internamente
  `space: ColorSpace::Oklab`.
- `Gradient::radial(stops, center, radius)` idem; preserva P269 focal_*
  defaults.
- `Gradient::conic(stops, center, angle)` idem.
- Stdlib `gradient.linear/radial/conic(...)` sem named `space:` arg →
  P262/P264/P267 behavior bit-exact idêntico.
- Tests P262/P264/P265/P267/P268/P268.2/P269 zero regressão.

§política condições 6 + 9 satisfeitas absolutas.

---

## Hue-wrap handling cristalino

Para HSL/Oklch/HSV (polar color spaces):
- Default "shorter hue" (CSS standard; vanilla paridade literal).
- Implementação: se `|h1 - h0| > 180°`, ajusta para wrap de modo que
  caminho seja mais curto.
- Edge case: `|h1 - h0| == 180°` → wrap em sentido positivo (paridade
  CSS).

```rust
fn interpolate_hue_shorter(h0: f32, h1: f32, t: f32) -> f32 {
    let diff = h1 - h0;
    let wrapped_h1 = if diff.abs() > 180.0 {
        if diff > 0.0 { h1 - 360.0 } else { h1 + 360.0 }
    } else {
        h1
    };
    (h0 + (wrapped_h1 - h0) * t).rem_euclid(360.0)
}
```

**Scope-outs P270**:
- "longer hue" / "increasing hue" / "decreasing hue" CSS modes —
  preservados scope-out; candidato refino futuro.

---

## Consequências

### Positivas

- User-facing `gradient.linear(red, blue, space: "hsl")` funcional.
- L1 sample paridade vanilla em todos os 8 spaces.
- ADR-0083 §ColorSpace runtime revogado parcialmente (L1+stdlib;
  L3 adiado P270.1/P270.2).
- Cluster Gradient L1+stdlib **feature-complete em 3 variants × 8
  spaces**.
- Sub-padrão "ADR scope-out revogado parcialmente" atinge N=3 cumulativo
  (limiar formalização clara).

### Negativas

- L3 emit ainda Oklab hardcoded (utilizadores não verão diferença
  visual até P270.1 + P270.2 fecharem).
- CMYK ainda usa Oklab pipeline em L3 (vai mudar em P270.2).
- Tests E2E PDF que verifiquem bytes específicos podem precisar
  actualização quando P270.1 mudar L3 emit; preservação P270 garante
  só L1+stdlib mudou.

### Neutras

- ADR-0087/0088/0089/0090 anotadas cumulativa (preservadas literal em
  estratégia; só notam space field cross-variant).
- Tests existentes P262-P269 zero regressão (defaults Oklab
  bit-exact).

---

## Scope-outs preserved

- **DeviceCMYK PDF emit** (revogação P270.2 futura).
- **"longer hue" / "increasing hue" / "decreasing hue" CSS modes**.
- **ICC profiles para CMYK PDF/A compliance**.
- **PostScript functions emit para spaces não-PDF-native** (vanilla
  história legacy; cristalino preserva Oklab pipeline N=16 alternativa).
- **`space: Smart<ColorSpace>` Auto resolved** — cristalino usa default
  via construtor (não-opcional); paridade observable preservada.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|---|---|---|
| α1 — Convergir vanilla literal (Family A pré-amostragem; Family B PDF-native directo) | Bit-exact com vanilla | +LOC L3 ~200; sem ganho real vs Op B (N=16 Oklab é qualidade suficiente sRGB/LinearRGB/Luma) |
| **α2 — Op B estratégia uniforme L3 (escolhida)** | **Simplifica L3 (1 pipeline para 7 spaces); reutiliza helpers P263/P265/P268** | **Divergência intencional vs vanilla Family B; bit-exact apenas Oklab** |
| α3 — Decisão L3 adiada (P270.1 implementa Op A; P270.2 verifica B) | Reversibilidade | Utilizador escolheu Op B explicitamente; já decidida |
| β — Anotação cumulativa ADR-0083 vs ADR nova | -1 ADR | Escala (ColorSpace runtime cross-variant + futuro DeviceCMYK) justifica ADR dedicada per princípio P0 |

**Decisão**: **α2 (Op B) + Opção α (ADR-0091 nova)** per paridade
pattern ADR-0083/0086/0087/0088/0089.

---

## Critério revisão

ADR-0091 pode ser revisitada se:

1. **P270.1 revelar banding visível** em sRGB/LinearRGB com N=16
   (forçaria Op A para Family B).
2. **CMYK PDF emit P270.2 revelar incompatibilidade reader-level**
   (forçaria Op A para CMYK).
3. **Vanilla bug #4422 revelar-se específico de krilla**; cristalino
   solução pode ser literal Op B preservado.
4. **CSS Color 4 standards mudarem** hue-wrap defaults (improvável).

Cada activação é **passo dedicado pequeno** (P270.1/P270.2 já
planeados).

---

## Subpadrões aplicados

### "ADR PROPOSTO+IMPLEMENTADO mesmo passo" N=5 → N=6 cumulativo

- N=1 P257 (ADR-0083 Color).
- N=2 P261 (ADR-0086 Paint).
- N=3 P262 (ADR-0087 Gradient Linear).
- N=4 P264 (ADR-0088 Gradient Radial).
- N=5 P267 (ADR-0089 Gradient Conic).
- **N=6 P270** (ADR-0091 ColorSpace runtime + CMYK strategy).

**Patamar N=6 extends limiar formalização clara excedido** (N=5 P267).
Pattern auto-documentado em cada ADR individual; meta-ADR desnecessário.

### "ADR scope-out revogado parcialmente" N=2 → N=3 cumulativo

- N=1 P267 — ADR-0088 §"variants não materializados" §Conic revogado
  parcialmente.
- N=2 P269 — ADR-0088 §"Scope-outs documentados" §focal_* revogado
  parcialmente.
- **N=3 P270** — ADR-0083 §"Scope-outs" §ColorSpace runtime revogado
  parcialmente.

**N=3 atinge limiar formalização clara** — candidato meta-ADR
formalização futura paridade P260 (ADR-0084/0085 formalizadas em N=5-6
cumulativo).

### "Reutilização literal helpers cross-passos" N=5 → N=6 cumulativo

- N=1 P265 (PDF Radial reutiliza helpers P263).
- N=2 P267 (Conic L1 reutiliza helpers Oklab P262).
- N=3 P268 (PDF Conic reutiliza P262/P265).
- N=4 P268.2 (compute_adaptive_n_conic reutiliza color_to_oklab_with_alpha).
- N=5 P269 (focal Radial reutiliza pipeline inteiro).
- **N=6 P270** (interpolate_in_space reutiliza `interpolate_oklab`
  P262 para arm Oklab; `Color::to_rgba_f32` P257 cross-space conversion
  bridge).

### "Diagnóstico imutável" N=12 → N=13 (oitavo consumo directo de fonte)

P262/P264/P267/P268/P268.1/P268.2/P269 + **P270** vanilla space-aware
sample + ColorSpace enum.

### "Fase A com industry research proactiva" — N=1 inaugural

P270 é primeira aplicação preventiva pré-spec (research vanilla docs +
issue tracker + blog ANTES de escrever spec). Distinto de P268.1/correcção
reactivas (research pós-divergência).

### "Anotação cumulativa cross-ADR" — N=1 inaugural

P270 anota 6 ADRs simultâneo (ADR-0083 + ADR-0054 + ADR-0087 + ADR-0088
+ ADR-0089 + ADR-0090). Distinto de anotação single-ADR cumulativa
padrão.

### "Decomposição L+ em sub-passos" — N=1 inaugural

P270 + P270.1 + P270.2 decompõe magnitude L+ original em 3 passos
M+M+S+ proporcional ao cap ADR-0061 (granularidade 1-2 features/passo).
Reversibilidade preservada via ADR-0091 documentação L3 estratégia.

---

## Referências cross-passos

- **P262** — Gradient Linear L1+stdlib (Oklab hardcoded; extendido P270).
- **P264** — Gradient Radial L1+stdlib (Oklab hardcoded; extendido).
- **P267** — Gradient Conic L1+stdlib (Oklab hardcoded; extendido).
- **P263/P265/P268** — L3 emit Oklab pipeline (preservado P270;
  refactor P270.1).
- **P268.2** — Adaptive N hybrid (preservado).
- **P269** — Radial focal_* activated (preservado; campo space adicional
  cross-variant).
- **P257** — Color 8/8 spaces (ADR-0083; §ColorSpace runtime revogado
  parcialmente P270).
- **ADR-0083** — Color paridade (anotada cumulativa P270).
- **ADR-0054** — Perfil graded (anotada cumulativa P270).
- **ADR-0087/0088/0089/0090** — Variant strategies (anotadas cumulativa
  P270).
- **ADR-0085** — Diagnóstico imutável (oitavo consumo).
- **typst/typst issue #4422** — CMYK gradient bug vanilla.
- **typst.app blog 2023** — PostScript functions legacy history.
- **typst.app/docs/reference/visualize/gradient** — vanilla dual
  strategy documentation.

---

## Próximos passos

1. P270.C executa materialização L1+stdlib (testes-primeiro; ColorSpace
   enum; 3 struct fields; sample multi-space; helpers).
2. P270.D promove ADR-0091 → `IMPLEMENTADO`.
3. **P270.1** (futuro M+) — L3 RGB-family + perceptual via Oklab
   pipeline N=16.
4. **P270.2** (futuro S+) — L3 CMYK directo `/DeviceCMYK` (revoga
   ADR-0083 §CMYK).
5. **P-Gradient-Relative-Custom** (futuro M) — activa
   `relative: RelativeTo`.

---

## Anotação cumulativa P270.1 — L3 emit multi-space materializado (7 spaces)

**Data**: 2026-05-17.

**Motivo**: ADR-0091 §"Decisão L3 futura" P270.1 — **Op B uniforme
materializada**. Pipeline cristalino L3 emit ganha consciência de
`<variant>.space` em pré-amostragem N=16.

### Descoberta arquitectural P270.1.A

**P270 já passou L3 multi-space implicitamente** — `oklab_sample_stops_*`
helpers chamam `<variant>.sample(t)` que despacha via P270
`interpolate_in_space` per `self.space`. **P270.1 é maioritariamente
cosmético** (rename + docs + tests).

### Mudança literal

**L3** (`03_infra/src/export.rs`):
- Helpers renomeados:
  - `oklab_sample_stops` → `multispace_sample_stops`
  - `oklab_sample_stops_radial` → `multispace_sample_stops_radial`
  - `oklab_sample_stops_conic` → `multispace_sample_stops_conic`
- **Body literal preservado** (zero mudança operacional); apenas rename
  + docs ampliadas documentando comportamento multi-space via
  `<variant>.sample(t)` dispatcher P270.
- 3 callsites production em `emit_gradient_objects` actualizados
  (rename mecânico).

### 7 spaces materializados L3 emit

- Oklab (preservado bit-exact via arm Oklab dispatcher P262 literal).
- Oklch, sRGB, LinearRGB, Luma, HSL, HSV (novos via pipeline
  uniforme — interpolate_in_space dispatch invocado por
  `<variant>.sample(t)` P270).

### CMYK preservado scope-out P270.1

- Pipeline P270.1 NÃO trata CMYK especificamente.
- Comportamento natural: `interpolate_cmyk` (P270 helper L1) +
  `Color::to_rgba_f32()` (CMYK → sRGB; P257) → pipeline normal
  `/ColorSpace /DeviceRGB`.
- Sub-óptimo até P270.2 (gama CMYK perdida na conversão para sRGB).
- **P270.2** materializa `/DeviceCMYK` directo; revoga ADR-0083
  §"DeviceCMYK PDF" final.

### Defaults Oklab preservam bytes P263/P265/P268.2 bit-exact

Arm Oklab em `interpolate_in_space` chama `interpolate_oklab` P262
literal → sample stops idênticos a P263/P265/P268.2 actuais.
2500 tests baseline P262-P270 zero regressão verificado §política
condição 7.

### Sub-padrão "Anotação cumulativa em vez de ADR nova" N=7 → N=8 cumulativo

P258.B + P259.B + P263 + P265 + P268 + P268.2 + P269 + P270.1 anotada
ADR-0091.

### Sub-padrão "Reutilização literal helpers cross-passos" N=6 → N=7 cumulativo

P270.1 reutiliza literal:
- Dispatcher P270 `interpolate_in_space` (via `<variant>.sample(t)`).
- 3 helpers L3 P263/P265/P268.2 (templates; body preservado literal,
  só renomeados).
- Pipeline DeviceRGB P263 (preservado).

### Sub-padrão "Cap LOC hard vs soft explícito" — N=1 inaugural

P270.1 inaugura distinção formal entre **cap hard** (gate; estouro
dispara §política condição absoluta) vs **cap soft** (informativo;
estouro registado no relatório). Lição operacional P270 (cap L1
ligeiramente acima sem disparo) aplicada — P270.1 explicita
classification.

### Sub-padrão "Anotação cumulativa cross-ADR" N=1 → N=2 cumulativo

P270 inaugurou (6 ADRs anotadas simultâneo); **P270.1 estende** (5
ADRs: ADR-0091/0087/0088/0089/0090 + ADR-0054). Limiar formalização
clara N=3 não atingido ainda; candidato refinamento futuro.

### Cluster Gradient L3 emit pós-P270.1

| Variant | L1 | Stdlib | L3 (7 spaces sem CMYK) | L3 CMYK |
|---------|----|----|----|----|
| Linear | P262 ✓ | P262 ✓ | **P270.1 ✓** (/ShadingType 2 + DeviceRGB) | Pipeline natural CMYK→sRGB (sub-óptimo; P270.2 fecha) |
| Radial (subset 5 campos P269) | P264/P269 ✓ | P264/P269 ✓ | **P270.1 ✓** (/ShadingType 3 + DeviceRGB) | idem |
| Conic | P267 ✓ | P267 ✓ | **P270.1 ✓** (/ShadingType 4 Gouraud + P268.2 adaptive N + DeviceRGB) | idem |

**Cluster Gradient L3 emit 7/8 spaces materializado** (3 variants × 7
spaces RGB-family + perceptual). CMYK último P270.2.

Status `IMPLEMENTADO` preservado literal (anotação cumulativa não muda
status; refina aplicação Op B materializada).

---

## Anotação cumulativa P270.2 — L3 emit CMYK directo (fecha L3 Linear+Radial 8/8; Conic CMYK scope-out preserved)

**Data**: 2026-05-17.

**Motivo**: ADR-0091 §"Decisão L3 futura" P270.2 — CMYK directo
materializado via `/ColorSpace /DeviceCMYK` + Function 4-component
output. **Cenário B confirmado** (§A.8/§A.11 diagnóstico) —
Linear+Radial CMYK materializados; Conic CMYK scope-out preserved
P-Gradient-Conic-CMYK futuro.

### Estratégia materializada

**L3** (`03_infra/src/export.rs`):
- 2 helpers samplers CMYK 4-component:
  - `multispace_sample_stops_linear_cmyk(linear, n)` → `Vec<(f32,
    f32, f32, f32)>` (c, m, y, k).
  - `multispace_sample_stops_radial_cmyk(radial, n)` análogo.
  - **Conic CMYK preservado P270.1 fallback** (sample CMYK convert
    para sRGB sub-óptimo).
- 1 helper `emit_function_dict_cmyk` 4-component
  (`/Range [0 1 0 1 0 1 0 1]` 8 values; `/C0 [c m y k]`
  `/C1 [c m y k]`).
- 1 helper `rgb_to_cmyk` inline (fallback precaução).
- **Dispatcher dual em `emit_gradient_objects`**:
  - Linear `space == Cmyk`: CMYK branch novo (emit
    `/ColorSpace /DeviceCMYK` + 4-component Function).
  - Linear `space != Cmyk`: pipeline P270.1 literal preserved.
  - Análogo Radial.
  - Conic preserved P270.1 literal (sub-óptimo CMYK fallback).

**L1**: `to_cmyk_components` (helper P270 em gradient.rs)
promovido a `pub` (paridade pattern P268.2 `color_to_oklab_with_alpha`;
gap minimal 4 caracteres).

### Bug #4422 resolvido por construção

Cristalino emit `/ColorSpace /DeviceCMYK` correcto (vs vanilla
bug `/DeviceRGB` errado para CMYK gradients). pdfkit #532 análogo
confirma causa raiz universal: dictionary errado por wrapper
intermediário. Cristalino implementação directa sem wrapper
intermediário evita o bug.

### ICC profiles scope-out preserved

ADR-0091 §"Scope-outs preserved" preservado:
- Cristalino emit `/DeviceCMYK` directo sem ICC profile.
- Refino futuro candidato P-Gradient-CMYK-ICC (paridade krilla
  custom ICC profiles).
- PDF/A compliance: scope-out (cristalino não declara PDF/A).

### Conic CMYK Cenário B — scope-out preserved

§A.8 diagnóstico decide preservar Conic CMYK scope-out:
- Vanilla Conic CMYK suporte incerto (krilla opaco).
- PDF reader compatibility Type 4 Gouraud + CMYK incerto.
- Complexidade extra (stream binary 4 bytes/vertex; `/Decode`
  array 5 pares vs 4) adiciona ~50 LOC L3.
- Linear + Radial cobrem maioria dos use cases user-facing.

**Conic com `space: Cmyk` em P270.2**: pipeline P270.1 fallback
sub-óptimo (sample CMYK convert para sRGB via
`to_rgba_f32()`). Funcional mas gama CMYK perdida no emit.

Candidato futuro P-Gradient-Conic-CMYK ao revogar definitivamente.

### Defaults preservam P270.1 — zero regressão

`space != Cmyk` → arm "else" dispatcher dual → pipeline P270.1
literal preserved. 2533 baseline tests preservados bit-exact.

§política condições 4 + 7 + 9 satisfeitas absolutas.

### Sub-padrão "Anotação cumulativa em vez de ADR nova" N=8 → N=9 cumulativo

P258.B + P259.B + P263 + P265 + P268 + P268.2 + P270 + P270.1 +
**P270.2** anotada ADR-0091.

### Sub-padrão "Reutilização literal helpers cross-passos" N=7 → N=8 cumulativo

P270.2 reutiliza literal:
- Dispatcher P270 arm Cmyk (`interpolate_cmyk`).
- 3 helpers L3 `multispace_sample_stops_*` P270.1 templates
  estendidos (não substituídos).
- `to_cmyk_components` L1 P270 (promovido a `pub`).

### Sub-padrão "Cap LOC hard vs soft explícito" N=1 → N=2 cumulativo

P270.1 inaugurou; P270.2 segunda aplicação consolida pattern.
Cap soft L3 150 ligeiramente acima (~8 LOC sobre — total ~138);
cap hard L3 250 respeitado (folga 45%). Cap testes hard 35
respeitado (12 reais; folga 66%).

### Sub-padrão "Anotação cumulativa cross-ADR" N=2 → N=3 cumulativo

P270 + P270.1 + **P270.2** terceira aplicação consolida pattern.
P270.2 anota 6 ADRs simultâneo: ADR-0091 (esta) + ADR-0083 +
ADR-0087/0088/0089/0090 + ADR-0054.

### Cluster Gradient L3 emit pós-P270.2

| Variant | Oklab/Oklch/sRGB/Luma/LinearRGB/HSL/HSV | CMYK |
|---------|-----|-----|
| Linear | **P270.1 ✓** /DeviceRGB | **P270.2 ✓** /DeviceCMYK |
| Radial | **P270.1 ✓** /DeviceRGB | **P270.2 ✓** /DeviceCMYK |
| Conic | **P270.1 ✓** /DeviceRGB | **Fallback sub-óptimo** (P-Gradient-Conic-CMYK futuro; ADR-0083 §DeviceCMYK revogação parcial) |

**Cluster L3 emit 7+2/8 spaces materializado** (Linear+Radial em 8/8;
Conic em 7/8 + fallback CMYK). ADR-0083 §"DeviceCMYK PDF"
**revogação parcial** P270.2 (não final 100%; Conic preserved
scope-out).

Status `IMPLEMENTADO` preservado literal.

---

## Anotação cumulativa P270.3 — Preparação Conic CMYK via Type 6 Coons (Cenário A revisado)

**Data**: 2026-05-17.

**ADR-0091 §"Conic CMYK scope-out preserved" P270.2 fica preparado
para revogação P270.4** via infra-estrutura **Type 6 Coons Patch Mesh**
materializada P270.3 (ADR-0092 EM VIGOR).

### Cluster Gradient L3 emit Cenário A revisado pós-P270.3+P270.4

| Variant | 7 spaces RGB-family + perceptual | CMYK |
|---------|-----|-----|
| Linear | P270.1 ✓ `/DeviceRGB` Function 3-comp | **P270.2 ✓** `/DeviceCMYK` Function 4-comp |
| Radial | P270.1 ✓ `/DeviceRGB` | **P270.2 ✓** `/DeviceCMYK` |
| Conic Type 4 Gouraud | **P268+P268.2 preserved** ✓ `/ShadingType 4` `/DeviceRGB` | (Type 4 + CMYK reader inconsistente) |
| Conic Type 6 Coons | **P270.3 infra-estrutura** (default OFF) | **P270.4 ON** ✓ `/ShadingType 6` `/DeviceCMYK` |

**Cluster L3 emit 24/24 absoluto pós-P270.4** — 3 variants × 8 spaces
totalmente materializados via 2 estratégias Conic emit coexistentes.

### P270.4 fecha cluster definitivamente

P270.4 activa opt-in flag ON para `conic.space == ColorSpace::Cmyk`:
- Emit `/ShadingType 6 /ColorSpace /DeviceCMYK`.
- Coons stream binary com 4 corner colors CMYK (4 bytes per color;
  41 bytes per patch CMYK vs 37 bytes RGB).
- Revoga ADR-0091 §"Conic CMYK scope-out preserved" definitivamente.

Magnitude P270.4 esperada: **S** (~80-100 LOC L3 extensão CMYK Coons
+ ~5-8 testes).

Sub-padrão "Anotação cumulativa cross-ADR" N=3 → N=4 cumulativo.

Status `IMPLEMENTADO` preservado literal. Ver ADR-0092 EM VIGOR.

---

## Anotação cumulativa P270.4 — Conic CMYK scope-out revogação final (cluster L3 24/24 absoluto)

**Data**: 2026-05-17.

**Motivo**: §"Conic CMYK scope-out preserved" P270.2 Cenário B revogado
**definitivamente** por activação Coons CMYK P270.4 via ADR-0092
§"Anotação cumulativa P270.4". Infra-estrutura P270.3 conectada via
opt-in flag ON para `space == Cmyk`.

### Cluster Gradient L3 emit pós-P270.4

| Variant | 7 RGB-family + perceptual | CMYK |
|---------|-----|-----|
| Linear | P270.1 ✓ `/DeviceRGB` Function 3-comp | P270.2 ✓ `/DeviceCMYK` Function 4-comp |
| Radial | P270.1 ✓ `/DeviceRGB` (focal_* P269 preserved) | P270.2 ✓ `/DeviceCMYK` |
| **Conic** | **P268+P268.2 ✓** `/ShadingType 4` Gouraud RGB | **P270.4 ✓** `/ShadingType 6` Coons CMYK |

**Cluster L3 emit 24/24 absoluto** — paridade vanilla user-facing
total. **3 variants × 8 spaces** materializados completamente.

### 2 estratégias L3 emit Conic AMBAS ACTIVAS

Estrutura emit Conic divide entre 2 estratégias coexistentes
fundamentadas em reader compatibility:
- **Type 4 Gouraud (P268+P268.2 preserved)**: 7 spaces RGB-family +
  perceptual. Strategy historical com adaptive N hybrid.
- **Type 6 Coons (P270.3 infra + P270.4 activação)**: CMYK. Strategy
  industry-aligned (Cairo/Inkscape/Typst original precedent) com
  reader compatibility universal.

Ver ADR-0092 §"Anotação cumulativa P270.4" para detalhes técnicos
materialização opt-in flag ON.

### Cluster ColorSpace runtime completo L1+stdlib+L3

Paridade vanilla total a nível user-facing. ADR-0091 §"Decisão L3
(materializada P270.1+P270.2+P270.4)" — sequência completa
materializada:

- **P270.1**: L3 7 spaces RGB-family + perceptual (Linear/Radial/Conic).
- **P270.2**: L3 CMYK Linear+Radial.
- **P270.3**: Coons RGB infra-estrutura preparação CMYK.
- **P270.4**: Coons CMYK activação opt-in flag ON.

### Sub-padrão "Anotação cumulativa cross-ADR" N=4 → N=5 cumulativo

P270 + P270.1 + P270.2 + P270.3 + **P270.4** (5 ADRs anotadas
paralelas; pattern consolidado).

Status `IMPLEMENTADO` preservado literal. Ver ADR-0092 §"Anotação
cumulativa P270.4".

## Anotação cumulativa P271 — Sub-padrões formalizados

Esta ADR é **centro de aplicação** dos sub-padrões formalizados P271
— 4 anotações cumulativas paralelas (P270.1+P270.2+P270.3+P270.4) +
cross-ADR cluster Gradient pattern:

- **"Anotação cumulativa em vez de ADR nova"** (4 anotações desta
  ADR; N=10 cumulativo agregado) → **ADR-0093 EM VIGOR**.
- **"Anotação cumulativa cross-ADR"** (cada anotação P270.x afecta
  múltiplas ADRs simultâneo; N=5 cumulativo) → ADR-0093 §"Anotação
  cumulativa cross-ADR".
- **"Reutilização literal helpers cross-passos"** (helpers P262/P265/
  P268.2/P270/P270.1-P270.4; N=10 cumulativo) → **ADR-0094 EM VIGOR**.
- **"Fase A com industry research proactiva"** (P270 inaugural; N=4
  cumulativo via série P270) → ADR-0094 §"Pattern 3".

Ver ADR-0093 + ADR-0094 EM VIGOR para meta-formalização. Status
`IMPLEMENTADO` preservado.

## Anotação cumulativa P272 — Estratégia Conic unificada Coons

§"Decisão L3 (materializada P270.1+P270.2+P270.4)" estendida P272 —
Conic agora estratégia única Coons para 8/8 spaces (**ADR-0090
REVOGADO** P272; **ADR-0092 expandida cumulativamente**).

### Estratégia L3 emit pós-P272

| Variant | Pipeline L3 emit |
|---|---|
| Linear (8 spaces) | `/ShadingType 2` axial (preserved) |
| Radial (8 spaces) | `/ShadingType 3` radial (preserved) |
| **Conic (7 RGB-family/perceptual + CMYK)** | **`/ShadingType 6` Coons unified** |

Cluster Gradient L3 emit estratégia única materializada feature-complete
**24/24 simplificado** — eliminação 2 estratégias coexistentes Conic
(Type 4 Gouraud P268+P268.2 + Type 6 Coons CMYK P270.4) converge em
Type 6 Coons único.

Strategy Conic RGB: **N = stops * 4 patches** (divergência intencional
Typst original blog 2023; corner colors via `Conic::sample(t)`
dispatcher P270).

Strategy Conic CMYK: **N = stops patches** (preserved P270.4).

Sub-padrão "Anotação cumulativa cross-ADR" N=5 → **N=6 cumulativo**
(P272 anota 5 ADRs paralelas).

Status `IMPLEMENTADO` preservado literal. Ver ADR-0092 §"Anotação
cumulativa P272 — Decisão Cenário A revisado FINAL" para detalhes
técnicos.

## Anotação cumulativa P273 — Cross-variant runtime fields (`relative: RelativeTo`)

**Data**: 2026-05-17.
**Motivo**: cluster Gradient ganha campo `relative: Option<RelativeTo>`
cross-variant (Linear/Radial/Conic). Activação user-facing paridade
vanilla.

### Estratégia materializada P273

- **L1 enum `RelativeTo { Self_, Parent }`** com `Default = Self_`.
- **L1 cada variant** ganha `relative: Option<RelativeTo>` (default
  `None` = Auto = `Self_`). Pattern ADR-0064 §Caso A (`Smart<T>` →
  `Option<T>` cristalino).
- **Stdlib named arg `"relative"`** cross-variant (parse "self"/"parent"/
  "auto"; whitelist estendida em 3 fns).
- **L3 dispatcher dual**:
  - `Self_` (default) → pipeline P272 preserved literal (page-relative
    coords).
  - `Parent` → coords via `apply_parent_transform(local, parent_bbox)`.
    Estrutural: `parent_bbox = None` em call sites P273 → identity
    (defaults preserved); futuro callsite poderá passar bbox real.

### Decisão arquitectural transform Rust (paridade vanilla)

Cristalino calcula coordinates transformadas em Rust nativo (não
PDF `/Matrix` shading dictionary). **Paridade vanilla** confirmada
em Fase A §A.3 — vanilla `lab/typst-original/crates/typst-pdf/src/paint.rs:175-181`
usa `match unwrap_relative(...)` em Rust + `correct_transform(state, ...)`
Rust nativo; PDF `/Matrix` permanece identity.

Industry research P273 consolidada §"Pesquisa empírica industry":

- PDF `/Matrix` em shading dictionary existe literal (iText/PDFTron
  APIs `setMatrix`/`getMatrix`); Type 2/3/6 aceitam `/Matrix` entry.
- Cairo/Inkscape e vanilla usam transform Rust nativo.
- **Cristalino paridade**: simplicidade auditoria pipeline; PDF
  `/Matrix` rejeitado por todos os 3 (Cairo/Inkscape/vanilla).
- Sub-padrão "Fase A com industry research proactiva" N=4 → **N=5
  cumulativo** (limiar formalização clara muito ultrapassado).

### Cluster Gradient cross-variant runtime fields canónica

| N | Passo | Campo | Variants |
|---|---|---|---|
| 1 | P269 | `focal_center` + `focal_radius` | Radial only |
| 2 | P270 | `space: ColorSpace` | Linear + Radial + Conic |
| 3 | **P273** | **`relative: Option<RelativeTo>`** | **Linear + Radial + Conic** |

**Lista canónica 3 elementos**; pattern N=3 cross-variant runtime
fields cumulativos.

### Defaults preservam P272 literal

- `relative: None` (Auto) → `resolve_relative(None) = Self_` → branch
  literal P272 (2557 baseline preserved bit-exact).
- Stdlib parsing omitir arg → `None`.
- L3 dispatcher `parent_bbox = None` → identity em ambos branches.

### Helpers reutilizados literal P273

- Pattern `Option<T>` (ADR-0064 §Caso A; precedentes Parity/Dir).
- L1 enum pattern (P270 ColorSpace).
- Stdlib named arg parsing (P270 `parse_space_named` template).
- L3 dispatcher dual (P272 RGB/CMYK).
- Sub-padrão "Reutilização literal helpers cross-passos" N=11 → **N=12
  cumulativo consolidação clara persistente**.

### Sub-padrões aplicados P273

- "Anotação cumulativa em vez de ADR nova" N=11 → **N=12 cumulativo**.
- "Reutilização literal helpers cross-passos" N=11 → **N=12 cumulativo
  consolidação clara persistente**.
- "Cap LOC hard vs soft explícito" N=5 → **N=6 cumulativo consolidação
  total**.
- "Fase A com industry research proactiva" N=4 → **N=5 cumulativo
  (limiar formalização clara muito ultrapassado)**.
- "Aplicação meta-ADR (ADR-0094)" N=1 → **N=2 cumulativo** (Cap LOC +
  industry research aplicação prática).
- "Anotação cumulativa cross-ADR" N=6 → **N=7 cumulativo** (P273
  anota 5 ADRs + ADR-0054 = 6 ADRs paralelas).

Status `IMPLEMENTADO` preservado literal. Ver
`00_nucleo/diagnosticos/diagnostico-relative-custom-passo-273.md`
para Fase A empírica.

## Anotação cumulativa P274 — Adaptive N multispace refino qualitativo

**Data**: 2026-05-17.
**Motivo**: P270.1 estabeleceu pré-amostragem N=16 fixo para 7 spaces
RGB-family + perceptual em Linear+Radial. Casos extremos (muitos stops
ou contraste cromático alto adjacente) podem apresentar banding visível.
P274 refina N adaptive por gradient sem alterar estratégia
(`/ShadingType 2/3` + Function Type 3 stitching).

### Fórmula adaptive N escolhida (Opção 1B threshold-based; §A.3 Decisão 1)

```rust
// §A.6 helper assinatura fixada:
fn adaptive_n_for_stops(stops, space) -> usize {
    if stops.len() < 2 { return 16; }  // degenerado
    let max = stops.windows(2)
        .map(|p| perceptual_distance_in_space(p[0].color, p[1].color, space))
        .fold(0.0, f32::max);
    match max {
        x if x < 0.05 => 16,   // pastel (paridade P270.1)
        x if x < 0.3  => 32,   // moderate
        _             => 64,   // high contrast (cap N_max)
    }
}
```

- **N_max = 64** (§A.3 Decisão 3; 4× N_base preservado).
- **ΔE Oklab nativo** (sem multiplier; thresholds 0.05/0.3 em unidades
  Oklab per Björn Ottosson + W3C CSS Color 4).
- **Param `space` preservado futuro-proofing** (ADR-0094 Pattern 2;
  permite refino futuro com métrica nativa per space).

### Variants afectados (§A.3 Decisão 2 Opção 2A)

- **Linear + Radial RGB-family + perceptual** (7 spaces): adaptive N
  por gradient via `adaptive_n_for_stops`.
- **CMYK preserved P270.2** directo (sem pré-amostragem em primeiro
  lugar; inalterado).
- **Conic preserved P272** literal — estratégia Coons N=stops*4 já é
  função do número de stops; sem evidência empírica banding baseline.

### Helper genérico `perceptual_distance_in_space`

Introduzido em `03_infra/src/export.rs` (privado). Por construção
desacoplado do space; aceita `ColorSpace` como parâmetro. Reutiliza
literal `color_to_oklab_with_alpha` (L1; promovido `pub` em P268.2;
quarto reuse no cluster Gradient).

Sub-padrão "Reutilização literal helpers cross-passos" **N=12 → N=13
cumulativo consolidação clara persistente**.

### Distinção vs precedente P268.2 removido P272

| Aspecto | P268.2 `oklab_delta_e` (removed P272) | P274 `perceptual_distance_in_space` |
|---|---|---|
| Assinatura | `(c1, c2) -> f32` | `(c0, c1, space) -> f32` |
| Acoplamento | Hardcoded Oklab; nome reflecte métrica | Genérico cross-space; nome reflecte intent |
| Uso | Type 4 Gouraud adaptive N (Conic-only) | Linear+Radial RGB-family (cross-variant) |
| Futuro-proofing | Recriar para outros spaces | Mudar implementation; assinatura estável |

Desacoplamento por construção corrige sintoma da remoção P272
(acoplamento excessivo).

### Regressão tests P270.1 originais zero (§A.7)

- Tests byte snapshot (`pdf1==pdf2`) são reproducibility — qualquer
  fórmula determinística preserva.
- Tests unit `multispace_sample_stops(l, 16)` passam N=16 literal —
  não usam `adaptive_n_for_stops`.
- Tests E2E PDF emit verificam strings (`/ShadingType 2/3/6`) —
  inalterados.

**Zero regressão** P270.1 + P270.2 + P271 + P272 + P273.

### Helpers Oklab P262 reutilizados literal

`color_to_oklab_with_alpha` é o único helper L1 chamado pelo L3 helper
P274 (cross-crate; pub). Sub-padrão "Reutilização literal helpers
cross-passos" N=13 (quarta reuse do helper Oklab no cluster Gradient).

### ADR-0091 preservada literal

Centro de aplicação ColorSpace intocado; P274 só refina N por path
RGB-family + perceptual sem cross-space dispatch logic.

### Sub-padrões aplicados P274

- **"Anotação cumulativa em vez de ADR nova"** N=12 → **N=13 cumulativo
  consolidação clara persistente** (P274 anota esta ADR; sem ADR nova).
- **"Reutilização literal helpers cross-passos"** N=12 → **N=13
  cumulativo consolidação clara persistente** (color_to_oklab_with_alpha
  P262 reuse 4ª vez).
- **"Cap LOC hard vs soft explícito"** N=6 → **N=7 cumulativo
  consolidação total** (L3 hard 250 / soft 180; testes hard 18 /
  soft 12; reais ~80 LOC + 12 testes; folga grande).
- **"Fase A com industry research proactiva"** N=5 → **N=6 cumulativo**
  (4 fontes canónicas W3C/Ottosson/Skia/Cairo consolidadas pré-sessão).
- **"Aplicação meta-ADR (ADR-0094)"** N=2 → **N=3 cumulativo**
  (Pattern 3 industry research + Pattern 1 cap LOC hard/soft —
  terceira aplicação prática pós-formalização P271).
- **"Aplicação meta-ADR (ADR-0093)"** N=1 → **N=2 cumulativo**
  (Pattern 2 anotação cumulativa em vez de ADR nova — segunda
  aplicação prática meta-ADR ADR-0093).
- **"Diagnóstico imutável"** N=19 → **N=20 cumulativo** (décimo quinto
  consumo directo de fonte).

Status `IMPLEMENTADO` preservado literal. Ver
`00_nucleo/diagnosticos/typst-passo-274A-diagnostico.md` para Fase A
empírica + matriz §A.5 + decisões fixadas.

## Anotação cumulativa P273.5 — Parent bbox callsite (fecha #[allow(dead_code)] P273)

**Data**: 2026-05-17.
**Motivo**: P273 deixou `apply_parent_transform` em `#[allow(dead_code)]`
— função existe + tests unit transformação, mas sem callsite real
fornecendo `parent_bbox`. P273.5 fecha pendência: callsites L3
emit_gradient_objects passam `Some(page_bbox)` para dispatcher
quando `relative == Parent`.

### Decisão 3 — Semântica Parent escolhida: 3γ híbrida

- **3γ.1 (materializado P273.5)**: callsite L3 passa **page_bbox**
  como fallback parent_bbox. Cobre o caso vanilla "shape top-level com
  relative=Parent ancora à página" — usecase mais comum.
- **3γ.2 (pendência preservada)**: Layouter populando `parent_bbox`
  real do Block/Boxed/Group contentor imediato via save/restore
  (paridade vanilla full). **Estruturalmente preparado** via campo
  `Option<Rect>` em Layouter; população real fica para refino futuro
  incremental per ADR-0054 graded.

### Mecanismo de propagação P273.5 materializado

1. **L1 novo `Rect { x: Pt, y: Pt, w: Pt, h: Pt }`** em
   `entities/layout_types.rs` (paridade `Point` + `Size`).
2. **L1 novo campo `parent_bbox: Option<Rect>`** no Layouter
   (`rules/layout/mod.rs`; padrão DEBT-37 P84.6 `cell_origin_*` reused
   estructuralmente).
3. **L1 Constructor init `parent_bbox: None`** (future Block
   save/restore populará).
4. **L3 callsites L3** lê `page_dimensions` e constrói `page_bbox`
   como fallback parent_bbox (3γ.1).
5. **L3 dispatcher** Linear/Radial RGB-family quando
   `relative == Some(Parent)` chama `apply_parent_transform`.
6. **L3 `#[allow(dead_code)]` removido** de `apply_parent_transform`
   — função tem callsite real.

### Defaults preservam P273 bit-exact

- Inputs sem `relative=Parent` continuam a dispatchar para Self_ branch
  literal (pipeline P272/P273 preserved).
- 2597 baseline (P274 fim) preserved (regressão zero).
- Path Self_ é pipeline P272+P274 literal preserved.

### Sub-padrão "Reutilização literal helpers cross-passos" N=13 → N=14 cumulativo

- `apply_parent_transform` P273 **reused literal** (sem alteração de
  fórmula).
- Padrão DEBT-37 P84.6 `cell_origin_x/y: Option<f64>` **reused
  estructuralmente** (campo opcional Layouter + future save/restore).
- `Rect` adicionado como tipo L1 paridade `Point`/`Size`.

### Sub-padrão "Pattern DEBT-37 cell_origin_* replicado" N=1 → N=2 cumulativo emergente

P84.6 P273.5 — padrão "campo opcional no Layouter para contexto pai"
replicado em P273.5 (`parent_bbox: Option<Rect>`). N=2 não atinge
limiar formalização N=3-4; promoção a ADR meta NÃO reservada.

### `#[allow(dead_code)]` fechado

P273 §7 pendência **resolvida P273.5**. Cluster Gradient refino
estrutural encerrado.

### Sub-padrões aplicados P273.5

- **"Anotação cumulativa em vez de ADR nova"** N=13 → **N=14
  cumulativo consolidação clara persistente** (sétima anotação
  consecutiva ADR-0091).
- **"Reutilização literal helpers cross-passos"** N=13 → **N=14
  cumulativo consolidação clara persistente** (apply_parent_transform
  reused; padrão DEBT-37 reused).
- **"Cap LOC hard vs soft explícito"** N=7 → **N=8 cumulativo
  consolidação total** (L1 hard 60 / soft 40; L3 hard 80 / soft 50;
  testes hard 12 / soft 8 — caps respeitados).
- **"Aplicação meta-ADR (ADR-0093)"** N=2 → **N=3 cumulativo**
  (Pattern 2 anotação cumulativa em vez de ADR nova — terceira
  aplicação prática).
- **"Aplicação meta-ADR (ADR-0094)"** N=3 → **N=4 cumulativo**
  (Pattern 1 cap LOC hard/soft — quarta aplicação prática).
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=1 → **N=2
  cumulativo emergente**.
- **"Diagnóstico imutável"** N=20 → **N=21 cumulativo** (décimo sexto
  consumo directo de fonte).

Status `IMPLEMENTADO` preservado literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-5A-diagnostico.md` para Fase A
empírica + decisão 3γ híbrida + decisões fixadas.

## Anotação cumulativa P273.6 — Parent bbox real save/restore (fecho 3γ.2)

**Data**: 2026-05-17.
**Motivo**: P273.5 materializou 3γ.1 (callsite L3 com page_bbox
fallback; identity transform). 3γ.2 ficou pendente — Layouter
`parent_bbox` em `#[allow(dead_code)]` sem consumer real. P273.6
fecha: arm Block save/restore real; FrameItem::Shape ganha
`parent_bbox_at_emit`; callsite L3 consulta bbox via GradientObject.

### Decisão 1 — Semântica bbox: 3γ.2.γ (pré-layout com fields literais)

`parent_bbox` populated **apenas quando** `width.is_some() &&
height.is_some()` no arm `Content::Block`. Caso ambíguo (dimensions
auto/fr) cai no fallback page_bbox L3 P273.5 (LIFO restore via
parent_bbox outer).

### Decisão 2 — Propagação L1→L3: Prop-A revisitada

- L1 `FrameItem::Shape` ganha `parent_bbox_at_emit: Option<Rect>`.
- L1 cascade pattern-match completo (~86 sites sem `..`; bulk-patch
  via script).
- L1 emit shape sites do Block arm populam
  `parent_bbox_at_emit: self.parent_bbox` (3 sítios: clip=true,
  clip=false, end-of-arm).
- L3 `GradientObject` struct ganha campo `parent_bbox_at_emit:
  Option<Rect>` propagado quando construído a partir do
  FrameItem::Shape.
- L3 callsite dispatcher consulta `gradient_obj.parent_bbox_at_emit`
  via `Some(rect)`; fallback `page_bbox` directo quando `None`
  (P273.5 preserved literal).

### Decisão 3 — Lista de contentores: {Block} apenas

- **Materializado P273.6**: `Content::Block`.
- **Diferido P273.7** (se necessário): `Content::Boxed`.
- **Scope-out per ADR-0054 graded**: Stack/Pad/Group/Grid cell.

### Pattern DEBT-37 `cell_origin_*` replicado N=3 cumulativo

**Atinge limiar formalização ADR meta N=3-4**:
- N=1: P84.6 (DEBT-37 `cell_origin_x/y/w`).
- N=2: P273.5 (`parent_bbox: Option<Rect>` estrutural; consumer
  pending).
- **N=3: P273.6** (`parent_bbox` save/restore real + consumer real).

Candidato a sub-passo administrativo XS futuro NÃO reservado;
documentado para meta-formalização ADR potencial.

### Defaults preservam P273.5 + P262-P273.5 bit-exact

- Shape sem `parent_bbox_at_emit` (None) → fallback page_bbox P273.5
  preservado literal.
- Block sem dimensions literais (Decisão 3γ.2.γ) → `parent_bbox` outer
  preservado (cai eventualmente no page fallback).
- Default field `parent_bbox_at_emit: None` em todos os ~86 sites
  bulk-patched.
- 2605 baseline P273.5 preserved.

### `#[allow(dead_code)]` no Layouter fechado

- Campo `parent_bbox` **consumed** por:
  - **Write** em arm `Content::Block` (save/restore quando dimensions
    literais).
  - **Read** em emit shape Block (popular `parent_bbox_at_emit` do
    FrameItem::Shape).
- `cargo build` zero warning de dead code no campo.

### Sub-padrões aplicados P273.6

- **"Anotação cumulativa em vez de ADR nova"** N=14 → **N=15
  cumulativo consolidação clara persistente** (oitava anotação
  consecutiva ADR-0091).
- **"Reutilização literal helpers cross-passos"** N=14 → **N=15
  cumulativo consolidação clara persistente** (apply_parent_transform
  P273 + Rect P273.5 reused; padrão DEBT-37 reused com consumer real).
- **"Cap LOC hard vs soft explícito"** N=8 → **N=9 cumulativo
  consolidação total** (L1 hard 100 / soft 70 — estouro soft esperado
  per spec via cascade; L3 hard 40 / soft 25; testes hard 15 / soft 10).
- **"Aplicação meta-ADR (ADR-0093)"** N=3 → **N=4 cumulativo** —
  quarta aplicação prática Pattern 2.
- **"Aplicação meta-ADR (ADR-0094)"** N=4 → **N=5 cumulativo** —
  quinta aplicação prática Pattern 1 + Pattern 3.
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=2 → **N=3
  cumulativo (atinge limiar formalização N=3-4)** — candidato
  meta-ADR futura.
- **"Sub-passos decimais consecutivos do mesmo cluster"** N=1 →
  **N=2 cumulativo emergente** (P273.5 + P273.6).
- **"Cascade pattern-match cross-FrameItem"** N=1 → **N=2 cumulativo**
  (P156C 12 sites + P273.6 ~86 sites — maior cascade do cluster
  Visualize).
- **"Diagnóstico imutável"** N=21 → **N=22 cumulativo** (décimo
  sétimo consumo directo de fonte).

Status `IMPLEMENTADO` preservado literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-6A-diagnostico.md` para Fase A
empírica + decisões 1/2/3 fixadas + critério §A.6 fecho
`#[allow(dead_code)]`.

---

## Anotação cumulativa P273.7 — Boxed save/restore (completa Decisão 3 P273.6)

**Data**: 2026-05-17.
**Motivo**: P273.6 fechou 3γ.2 para `Content::Block` apenas; Decisão 3
P273.6 enumerou Boxed como pendência específica "P273.7 — Boxed
save/restore (se necessário; análogo Block)". P273.7 fecha essa
pendência estendendo o template P273.6 literal ao arm `Content::Boxed`.

### Decisão 1 — bbox.y semantic em contexto inline: 3γ.2.γ-inline-baseline-y

Box é inline: `cursor.y` no momento do arm corresponde à **baseline
da linha em curso**, não ao topo do box. Vanilla `RelativeTo::Parent`
para gradient dentro de box vê o box como contentor com origin no
canto top-left.

**Opções**:
- **3γ.2.γ-inline-baseline-y** (fixada): `bbox.y = cursor.y` literal.
  Aproximação aceitável — alinha com limitação consciente P156H
  ("height em contexto inline alteraria line_height — refino futuro").
- **3γ.2.γ-inline-topo-estimado**: `bbox.y = cursor.y - ascender`.
  Mais correcto semanticamente; introduz dependência adicional na
  font_metrics no arm save/restore.
- **3γ.2.γ-inline-defer**: não popular `parent_bbox` no Boxed;
  fallback page_bbox preservado.

**Fixada `3γ.2.γ-inline-baseline-y`**. Razões:
1. Não introduz dependência na font_metrics no arm save/restore;
   mantém template Block literal.
2. Coerente com limitação P156H pré-existente.
3. Test E2E confirma output PDF observable **distinto** do fallback
   page_bbox mesmo com bbox.y aproximada (box 200×100pt vs page
   595×842pt produz transform PDF visualmente diferenciada).
4. Refino topo-exacto fica registado como `P273.X-bis2` per ADR-0054
   graded — promoção apenas se houver demanda empírica.

### Decisões 2/3/4 — herdadas P273.6 literal

- **Decisão 2 (semântica bbox W/H)**: 3γ.2.γ — popular apenas quando
  `width.is_some() && height.is_some()`. Idêntica Block.
- **Decisão 3 (propagação L1→L3)**: Prop-A revisitada inalterada —
  emit shape sites no Boxed já populam `parent_bbox_at_emit:
  self.parent_bbox` desde P273.6 (linha 1495 `mod.rs`). P273.7 não
  altera L3 nem cascade.
- **Decisão 4 (escopo contentores)**: P273.7 estende Decisão 3
  P273.6 de `{Block}` para **`{Block, Boxed}`**. Stack/Pad/Group/
  Grid cell continuam scope-out per ADR-0054 graded.

### Mudanças P273.7

- **L1 arm `Content::Boxed`**: save/restore `parent_bbox` análogo
  Block (template P273.6 literal; bbox.y = cursor.y).
- **L1 emit shape site interno Boxed (linha 1495)**: **inalterado** —
  já populated desde P273.6.
- **L3 dispatcher**: **inalterado** — consome `effective_parent_bbox`
  desde P273.6.
- **L3 GradientObject**: **inalterado** — campo
  `parent_bbox_at_emit` já existe desde P273.6.
- **Sem cascade novo**: `FrameItem::Shape.parent_bbox_at_emit`
  cascade ~86 sites já feito P273.6.

### Defaults preservam P262-P273.6 bit-exact

- Boxed sem dimensions literais (Decisão 2 herdada) → `parent_bbox`
  outer preservado (LIFO restore; cai eventualmente no page fallback).
- Self_/None relative ignora `parent_bbox_at_emit` (paridade Block).
- 2612 baseline P273.6 preserved.

### Pattern DEBT-37 `cell_origin_*` replicado N=3 (mantém limiar)

P273.7 é **extensão da terceira aplicação** (mesmo cluster Gradient
+ mesmo campo `parent_bbox` + mesmo consumer Block), não quarta
aplicação independente. N=3 cumulativo preserved — limiar
formalização ADR meta N=3-4 mantido. Candidato meta-ADR futura
documentado P273.6.

### Limitação consciente bbox.y aproximada

`cursor.y` é baseline em contexto inline; refino topo-exacto requer
refactor inline line_height (diferido permanente per ADR-0054
graded; coerente com P156H). Pendência específica `P273.X-bis2`
registada §7 spec P273.7.

### Sub-padrões aplicados P273.7

- **"Anotação cumulativa em vez de ADR nova"** N=15 → **N=16
  cumulativo consolidação clara persistente** (nona anotação
  consecutiva ADR-0091: P270/P270.1/P270.2/P270.3/P273/P274/P273.5/
  P273.6/**P273.7**).
- **"Reutilização literal helpers cross-passos"** N=15 → **N=16
  cumulativo consolidação clara persistente** (apply_parent_transform
  + Rect + DEBT-37 padrão reused; template Block save/restore reused
  literal — nova aplicação cross-passo).
- **"Cap LOC hard vs soft explícito"** N=9 → **N=10 cumulativo
  consolidação total** (L1 hard 30 / soft 20; tests hard 8 / soft 5;
  L3 hard 0).
- **"Aplicação meta-ADR (ADR-0093)"** N=4 → **N=5 cumulativo** —
  quinta aplicação prática Pattern 2.
- **"Aplicação meta-ADR (ADR-0094)"** N=5 → **N=6 cumulativo** —
  sexta aplicação prática Pattern 1.
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=3 → **N=3
  cumulativo preserved** — P273.7 é extensão da terceira aplicação;
  limiar formalização ADR meta mantido.
- **"Sub-passos decimais consecutivos do mesmo cluster"** N=2 →
  **N=3 cumulativo emergente** (P273.5 + P273.6 + P273.7).
- **"Cascade pattern-match cross-FrameItem"** N=2 → **N=2 cumulativo
  preserved** — P273.7 sem cascade novo (reutiliza ~86 sites P273.6).
- **"Template-passo replicado literal"** N=0 → **N=1 emergente** —
  P273.7 inaugura sub-padrão: replicação literal do save/restore
  P273.6 a outro arm (Boxed) com diferença mínima (bbox.y
  baseline-relative vs topo). Análogo histórico P156H replicando
  P156G. Promoção a sub-padrão consolidado candidato N=3-4.
- **"Diagnóstico imutável"** N=22 → **N=23 cumulativo** (décimo
  oitavo consumo directo de fonte).

Status `IMPLEMENTADO` preservado literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-7A-diagnostico.md` para Fase A
empírica + Decisão 1 fixada + critério §A.6.

---

## Anotação cumulativa P273.9 — Containers estendidos (Grid + Stack + Pad — escopo 1γ)

**Data**: 2026-05-18.
**Motivo**: P273.7 fechou Decisão 3 para `{Block, Boxed}`; P273.9
estende para Grid cell + Stack + Pad — escopo 1γ (M magnitude
reconhecida) per Decisão 1 utilizador. Stack/Pad introduzem **layout
duplo via `measure_content_constrained`** — adaptação ao template
P273.6 que originalmente rejeitou 3γ.2.β (medição pós-layout) por
custo. Para containers sem dimensions literais (Stack/Pad) o layout
duplo é arquitecturalmente necessário e aceito.

### Decisão 1 fixada (escopo) — 1γ

**1γ — Grid cell + Stack + Pad** (Decisão utilizador). Trade-offs
aceites:
- Magnitude M (~50 LOC L1).
- Risco regressão alto — mitigado via defaults rigorosos (popular
  bbox apenas se measured w/h > 0).
- Layout duplo Stack/Pad — custo perf ~1.5-2× **apenas em pipelines
  com gradient `relative=parent`** (defaults Self_/None preservam
  zero overhead).

### Decisão 2 fixada (Grid bbox) — 2α

`Rect { x: body_x, y: body_y, w: body_w, h: body_h }` — todos 4
disponíveis pré-body via track resolution + insets aplicados.
Defaults: não popular se w<=0 || h<=0.

### Decisão 3 fixada (Stack + Pad bbox) — medição inline / INNER

- **Stack**: bbox medido via inline replicação do handler Stack
  em `measure_content_constrained` (vertical: max_w × sum_h;
  horizontal: sum_w × max_h). Defaults: não popular se medição
  retorna 0×0 (n=0).
- **Pad**: bbox INNER (body region, sem insets) — paralela a Block
  semantic. `(body_w, body_h) = measure_content_constrained(body,
  available_inner)`. Defaults: não popular se w<=0 || h<=0.

### Mudanças P273.9

- **L1 arm `Content::Grid`** (grid.rs): save/restore `parent_bbox`
  paralelo ao `cell_origin_*` save/restore existente.
- **L1 arm `Content::Stack`** (mod.rs): inline measurement do bbox
  Stack + save/restore.
- **L1 arm `Content::Pad`** (mod.rs): `measure_content_constrained`
  call + save/restore inner bbox.
- **L3**: 0 LOC — dispatcher já consome `effective_parent_bbox` desde
  P273.6.
- **Sem cascade novo** — `FrameItem::Shape.parent_bbox_at_emit`
  cascade ~86 sites preserved P273.6.

### Defaults preservam P262-P273.8 bit-exact

- Grid cell sem `body_w/h > 0` (degenerate) → `parent_bbox` outer
  preservado.
- Stack vazio (n=0) ou measured 0×0 → preservado.
- Pad com body vazio → preservado.
- Self_/None relative ignora `parent_bbox_at_emit` (paridade).
- 2620 baseline P273.8 preserved.

### Pattern DEBT-37 `cell_origin_*` replicado N=4 cumulativo

**Crosses N=3-4 limiar formalização ADR meta com folga**:
- N=1: P84.6 (DEBT-37 `cell_origin_x/y/w`).
- N=2: P273.5 (`parent_bbox` estrutural; consumer pending).
- N=3: P273.6 (`parent_bbox` save/restore Block + consumer real).
- **N=4: P273.9** (`parent_bbox` save/restore Grid cell paralelo a
  `cell_origin_*` existente).

### Sub-padrões aplicados P273.9

- **"Anotação cumulativa em vez de ADR nova"** N=16 → **N=17
  cumulativo consolidação clara persistente** (décima anotação
  consecutiva ADR-0091).
- **"Reutilização literal helpers cross-passos"** N=16 → **N=17
  cumulativo consolidação clara persistente** (template
  save/restore + `measure_content_constrained` handlers reused).
- **"Cap LOC hard vs soft explícito"** N=11 → **N=12 cumulativo**
  (L1 hard 80 / soft 60 — 1γ M magnitude recalibrada).
- **"Aplicação meta-ADR (ADR-0093)"** N=5 → **N=6 cumulativo**.
- **"Aplicação meta-ADR (ADR-0094)"** N=7 → **N=8 cumulativo**.
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=3 → **N=4
  cumulativo crossing limiar formalização N=3-4** — candidato
  meta-ADR formalização **com folga consolidada**.
- **"Template-passo replicado literal"** N=1 → **N=2 cumulativo**
  (Grid replica template Block/Boxed literal; Stack/Pad replicam
  template com adaptação layout duplo).
- **"Sub-passos consecutivos do mesmo cluster"** N=4 → **N=5
  cumulativo emergente** (P273.5/6/7/8/9).
- **"Layout duplo arquitectural aceite"** N=0 → **N=1 inaugural
  emergente** — P273.9 estabelece precedente para containers sem
  dimensions literais usarem `measure_content_constrained`
  pre-layout para construir `parent_bbox`.
- **"Diagnóstico imutável"** N=24 → **N=25 cumulativo** (vigésimo
  consumo).

Status `IMPLEMENTADO` preservado literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-9-diagnostico.md` para Fase A
empírica + Decisões 1γ/2α/3 fixadas + critério §A.8.

---

## Anotação cumulativa P273.10 — Group L3-only parent_bbox (sub-padrão "L3-only" inaugural)

**Data**: 2026-05-18.
**Motivo**: P273.9 escopou save/restore L1 a 5 containers Layouter
(Block + Boxed + Grid cell + Stack + Pad). `FrameItem::Group` é
categorialmente diferente — pós-layout; bbox conhecida apenas em L3
emit-time. P273.10 fecha pendência P273.9 §8 via **mecanismo L3 puro**
(zero touch Layouter L1).

### Decisão 1 fixada (mecanismo override) — 1α parameter threading

`scan_all_gradients` ganha helper recursivo interno com parâmetro
`parent_bbox_override: Option<Rect>`. Group arm constrói bbox e
recurse com override; Shape arm aplica Inner-wins via
`parent_bbox_at_emit.or(override)`. Sub-padrão "L3-only parent_bbox"
via signature explícita — auto-documentado.

### Decisão 2 fixada (Group bbox) — 2α exacto frame coords cristalino

`Rect { x: pos.x, y: pos.y, w: inner_width, h: inner_height }`.
Sem Y-inversion (paridade com `parent_bbox_at_emit` Layouter
P273.6/7/9). Y-inversion é responsabilidade exclusiva do PDF emit
final via `apply_parent_transform`.

### Decisão 3 fixada (override precedence) — 3α Inner wins

`effective_bbox = parent_bbox_at_emit.or(parent_bbox_override)`.
Shapes com `parent_bbox_at_emit` populated (P273.9 5 containers
Layouter) → mantêm o próprio campo; override Group ignorado.
Paridade vanilla: "relative=parent" resolve ao contentor mais
próximo — Layouter L1 conhece os contentores canónicos; Group é
wrapper estrutural pós-layout que NÃO redefine "parent"
semanticamente, apenas oferece bbox de fallback quando Layouter
não populou.

### Scope creep arquitectural — `pattern_resources_for_page` recursão

Bug latent pré-existente: `scan_all_gradients` + `pattern_resources_for_page`
não recurse em `FrameItem::Group`. Gradients dentro de Groups
actualmente NÃO são registados nem listados em page resources
`/Pattern << >>` — PDF emit quebrado para esse caso. P273.10 corrige
**ambos** os sítios em paralelo (symmetric helper recursivo) per
§A.7 do diagnóstico. Sem scope creep, a feature P273.10 não produz
observable behavior.

### Mudanças P273.10

- **L3 `scan_all_gradients`**: refactor com helper recursivo interno
  `walk(items, parent_bbox_override, ...)`; Group arm constrói
  `group_bbox` e recurse com override; Shape arm aplica Inner-wins.
- **L3 `pattern_resources_for_page`**: refactor symmetric recursão
  (scope creep §A.7).
- **L1**: 0 LOC — sem touch Layouter; ADR-0029 pureza física L1
  preserved.

### Defaults preservam P262-P273.9 bit-exact

- Shapes com `parent_bbox_at_emit` populated (P273.9 5 containers)
  unaffected — Inner wins.
- Shapes top-level (não dentro de Group) → `override = None`
  propagado; preserved literal P273.9.
- Self_/None relative ignora override.
- Dedup gradient por Arc preserved (limitação P273.6 §9).
- 2625 baseline P273.9 preserved.

### Sub-padrão emergente "L3-only parent_bbox" N=1 inaugural

P273.10 inaugura sub-padrão: contentores post-layout cuja bbox é
conhecida apenas em L3 emit-time usam **L3 dispatcher override**
em vez de L1 Layouter save/restore. Distingue de:
- **Pattern DEBT-37** (N=4 P273.9; L1 Layouter save/restore).
- **Layout duplo arquitectural aceite** (N=1 P273.9; L1
  `measure_content_constrained`).
- **L3-only parent_bbox** (N=1 P273.10; L3 emit-time override).

### Sub-padrões aplicados P273.10

- **"Anotação cumulativa em vez de ADR nova"** N=17 → **N=18
  cumulativo consolidação clara persistente** (décima primeira
  anotação consecutiva ADR-0091).
- **"Reutilização literal helpers cross-passos"** N=17 → **N=17
  preserved** (primeiro mecanismo L3-only — não reusa helper L1).
- **"Cap LOC hard vs soft explícito"** N=12 → **N=13 cumulativo**.
- **"Aplicação meta-ADR (ADR-0093)"** N=6 → **N=7 cumulativo**.
- **"Aplicação meta-ADR (ADR-0094)"** N=8 → **N=9 cumulativo**.
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=4 → **N=4
  preserved** (P273.10 sem touch Layouter).
- **"Template-passo replicado literal"** N=2 → **N=2 preserved**
  (mecanismo diferente; não template).
- **"Sub-passos consecutivos do mesmo cluster"** N=5 → **N=6
  cumulativo emergente** (P273.5/6/7/8/9/10).
- **"Layout duplo arquitectural aceite"** N=1 → **N=1 preserved**.
- **"L3-only parent_bbox"** N=0 → **N=1 inaugural emergente**.
- **"Diagnóstico imutável"** N=25 → **N=26 cumulativo** (vigésimo
  primeiro consumo).

Status `IMPLEMENTADO` preservado literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-10-diagnostico.md` para Fase
A empírica + Decisões 1α/2α/3α fixadas + critério §A.9.

---

## Anotação cumulativa P273.12 — Dedup bbox-aware (refino arquitectural pós-P273.10)

**Data**: 2026-05-18.
**Motivo**: P273.6 §9 quarto bullet documentou limitação preservada
em todos os relatórios subsequentes até P273.11: "gradient com mesmo
Arc usado em contextos distintos: actualmente primeiro wins". P273.12
fecha o pendente.

### Origem arquitectural

Export L3 dedupa gradients via `Arc::as_ptr(g) as usize`
(sub-padrão "Dedup Arc::as_ptr resources" N=2 — P73 image + P263
pattern). `HashMap<usize, usize>` mapeia `Arc::as_ptr` → idx no
`Vec<GradientObject>`. Pós-P273.6, cada `GradientObject` carrega
`parent_bbox_at_emit`. Quando mesmo `Arc` aparece em N callsites com
bboxes effective distintos → singleton captura apenas o primeiro
bbox (semântica errada para callsites 2..N).

### Decisão 1 fixada (chave dedup) — 1β + 1γ combinados

```rust
struct RectKey(i32, i32, i32, i32);  // milipontos quantizados
struct DedupKey { arc_ptr: usize, bbox: Option<RectKey> }
```

Quantização `(r.0 * 1000.0).round() as i32` em milipontos resolve
problemas de `f64` em HashMap key (NaN, precision creep) e preserva
precisão sub-typográfica (1 mpt = 0.001 pt; typografia opera em pt).

### Decisão 2 fixada (callsite vs scan-side) — 2β scan-side

`scan_all_gradients.walk` + `pattern_resources_for_page.walk`
computam `effective_bbox = parent_bbox_at_emit.or(parent_bbox_override)`
e usam-no como dedup key. `emit_stroke_paint` ganha
`effective_bbox: Option<Rect>` param adicional para construir
DedupKey lookup.

### Decisão 3 fixada (cross-page) — 3α global ao documento

`pat_ptr_to_idx: HashMap<DedupKey, usize>` permanece global ao
documento. Pattern PDF reusado entre pages via `/Pattern << >>`
page resource dict. P273.12 não altera escopo.

### Cascade emit_stroke_paint (mínimo per §A.6)

3 sítios calleros (linhas 2078, 2588, 2772) — todos page-level
top-level Shape destructure. Modificação:
- Destructure `parent_bbox_at_emit` (P273.7.1 cleanup mudou para
  `_`; agora reverter para uso real).
- Passar como `effective_bbox` ao `emit_stroke_paint`.

`draw_item_local` Group recursion usa solid fallback
(`s.paint.to_color()`) — NÃO chama emit_stroke_paint; sem cascade
para esse path. Limitação pré-existente preserved.

### Defaults preservam P262-P273.11 bit-exact

- Gradient `relative=self/None` (`bbox=None`) → preserved literal
  (dedup por Arc apenas; DedupKey = (arc_ptr, None) factorizes a
  singleton key per Arc).
- Arc usado em context único → idem; uma entry no map.
- Apenas Arc com bboxes effective distintos em N contexts produz N
  PDF patterns (semântica correcta vs primeira-wins).

### Trade-off PDF size

Pior caso: N callsites mesmo Arc + N bboxes distintos → N PDF
patterns onde antes era 1. Custo aceitável — semântica correcta
sobrepõe-se ao custo bytes em casos genuínos. Caso comum (Arc usado
em contexto único ou contextos idênticos) preserved literal — sem
inflação.

### Sub-padrões aplicados P273.12

- **"Anotação cumulativa em vez de ADR nova"** N=18 → **N=19
  cumulativo consolidação clara persistente** (décima segunda
  anotação consecutiva ADR-0091).
- **"Reutilização literal helpers cross-passos"** N=17 → **N=17
  preserved**.
- **"Cap LOC hard vs soft explícito"** N=14 → **N=15 cumulativo**
  (L3 hard 100 / soft 70 — real ~73 (5% estouro soft) per ADR-0094
  Pattern 1).
- **"Aplicação meta-ADR (ADR-0093)"** N=7 → **N=8 cumulativo**.
- **"Aplicação meta-ADR (ADR-0094)"** N=10 → **N=11 cumulativo**.
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=4 → **N=4
  preserved**.
- **"Template-passo replicado literal"** N=2 → **N=2 preserved**.
- **"Sub-passos consecutivos do mesmo cluster"** N=7 → **N=8
  cumulativo emergente** (P273.5/6/7/8/9/10/11/12).
- **"Layout duplo arquitectural aceite"** N=1 → **N=1 preserved**.
- **"L3-only parent_bbox"** N=1 → **N=1 reused** — P273.12 também é
  L3-only.
- **"Extract helper de replicação inline"** N=1 → **N=1 preserved**.
- **"Dedup Arc::as_ptr resources"** N=2 → **N=3 cumulativo crossing
  limiar formalização N=3-4** (P73 image + P263 pattern + **P273.12
  pattern bbox-aware**). Candidato meta-ADR formalização NÃO
  reservado.
- **"Bug arquitectural intencional corrigido"** N=0 → **N=1 inaugural
  emergente** — limitação documentada P273.6 §9 corrigida 6 sub-passos
  depois com refino arquitectural deliberado. Distingue de "Bug
  latent corrigido em scope creep" (que é defeito não-detectado).
- **"Diagnóstico imutável"** N=27 → **N=28 cumulativo** (vigésimo
  terceiro consumo).

Status `IMPLEMENTADO` preservado literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-12-diagnostico.md` para Fase
A empírica + Decisões 1β+1γ/2β/3α fixadas + critério §A.8.

---

## Anotação cumulativa P273.13 — Fix draw_item_local Group gradient (caminho emit real)

**Data**: 2026-05-18.
**Motivo**: P273.10 corrigiu caminho de *registo* (scan_all_gradients
+ pattern_resources_for_page) para Groups. P273.12 expandiu chave
dedup para DedupKey. **Caminho de emit real** (`draw_item_local`
recursive para Group children) continuou a usar fallback solid
color via `s.paint.to_color()` — patterns registados mas não
consumidos. Pendência P263 §8 #3 (original 2026-05-16) +
P273.12 §9 quarto bullet fechadas.

### Decisão 1 fixada (propagação) — 1α parameter threading

`draw_item_local` ganha 3 params adicionais:
- `parent_bbox_override: Option<Rect>`.
- `pat_ptr_to_idx: &HashMap<DedupKey, usize>`.
- `pat_refs: &[PatternRef]`.

3 callsites em build_page_stream_*  (linhas 2234/2720/2906) passam
o contexto recebido (Group bbox próprio + pat_ptr_to_idx + pat_refs
já no escopo).

### Decisão 2 fixada (Group bbox source) — 2α paridade literal

`draw_item_local` arm `FrameItem::Group` constrói `group_bbox`
literal-equivalente a `scan_all_gradients.walk` + a
`pattern_resources_for_page.walk`. Construção idêntica nos 3 sítios
para garantir `dedup_key_for(g, effective_bbox)` produzir chave
idêntica → lookup encontra pattern registado.

**Scope creep aceito**: arm Group novo em `draw_item_local`. Pre-P273.13
nested Groups eram silenciosamente descartados via `_ => {}`
catch-all. P273.13 corrige bug pre-existente + suporta nested Group
recursion (paridade scan).

### Decisão 3 fixada (coords) — 3α coords cristalino

`group_bbox` em coords cristalino (Y-down). Paridade exacta com
scan + pattern_resources walks. Y-inversion exclusiva do PDF emit
final via `apply_parent_transform`.

### Mudanças P273.13

- **L3 `draw_item_local` signature**: 3 params adicionais.
- **L3 `draw_item_local` arm Shape**: destructure `parent_bbox_at_emit`
  (revertendo P273.7.1 `_`); computa `effective_bbox = parent_bbox_at_emit
  .or(parent_bbox_override)` (Inner wins paridade P273.10); substitui
  fallback solid por `emit_stroke_paint(...)` com DedupKey lookup.
- **L3 `draw_item_local` arm Group novo**: constrói `group_bbox`
  literal-equivalente; for-loop recurse com `Some(group_bbox)`.
- **L3 3 callsites em build_page_stream_*** Group arms: passam
  `Some(group_bbox)`, `pat_ptr_to_idx`, `pat_refs`.
- **L1**: 0 LOC.

### Defaults preservam P262-P273.12 bit-exact

- Shapes top-level (não dentro de Group) → caminho directo
  `emit_stroke_paint` em `build_page_stream_*` (sem mudança).
- Shapes dentro de Group sem gradient → solid color preserved
  literal.
- Shapes dentro de Group com Self_/None relative → DedupKey
  `{arc_ptr, None}` lookup encontra pattern registado pelo scan
  (P273.10 already registered them).
- Shapes dentro de Group com gradient relative=parent → DedupKey
  `{arc_ptr, Some(rect_to_key(group_bbox))}` lookup encontra pattern
  (P273.12 already dedups them).

### Patterns registados pós-P273.12 agora consumidos pós-P273.13

P273.12 explicitamente registou patterns para gradients dentro de
Groups mas `draw_item_local` continuou a usar solid fallback —
patterns ficavam "unused declarations" em PDF resources. P273.13
fecha esse ciclo: render real chama emit_stroke_paint → lookup
pattern via DedupKey → renderiza `/Pattern CS /Pi SCN`.

### Sub-padrões aplicados P273.13

- **"Anotação cumulativa em vez de ADR nova"** N=19 → **N=20
  cumulativo consolidação clara persistente** (décima terceira
  anotação consecutiva ADR-0091).
- **"Reutilização literal helpers cross-passos"** N=17 preserved.
- **"Cap LOC hard vs soft explícito"** N=15 → **N=16 cumulativo**
  (L3 hard 70 / soft 50 — real ~26; folga 48%).
- **"Aplicação meta-ADR (ADR-0093)"** N=8 → **N=9 cumulativo**.
- **"Aplicação meta-ADR (ADR-0094)"** N=11 → **N=12 cumulativo**.
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=4 preserved.
- **"Template-passo replicado literal"** N=2 preserved.
- **"Sub-passos consecutivos do mesmo cluster"** N=8 → **N=9
  cumulativo emergente** (P273.5/6/7/8/9/10/11/12/13).
- **"Layout duplo arquitectural aceite"** N=1 preserved.
- **"L3-only parent_bbox"** N=1 → **N=2 cumulativo emergente**
  (P273.10 inaugural + **P273.13 reaplicação**). Padrão consolidado.
- **"Dedup Arc::as_ptr resources"** N=3 preserved (reused via
  DedupKey lookup).
- **"Bug arquitectural intencional corrigido"** N=1 preserved
  (P273.13 não é arquitectural — é fix tactical de pendência
  específica documentada).
- **"Triplicação Group bbox"** N=0 → **N=1 emergente** —
  `scan_all_gradients.walk` + `pattern_resources_for_page.walk` +
  `draw_item_local` constroem mesmo `group_bbox` triplicadamente.
  Candidato extract helper P273.X-bis-helper-group-bbox.
- **"Diagnóstico imutável"** N=28 → **N=29 cumulativo** (vigésimo
  quarto consumo).

Status `IMPLEMENTADO` preservado literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-13-diagnostico.md` para Fase
A empírica + Decisões 1α/2α/3α + scope creep arm Group + critério §A.7.

---

## Anotação cumulativa P273.14 — CMYK-ICC scope-out reconfirmado (NO-GO Fase A)

**Data**: 2026-05-18.
**Decisão**: **NO-GO** via §A.4 critério #1 + #2 combinados Fase A
P273.14.
**Status do passo**: **SCOPE-OUT-RECONFIRMED** (não IMPLEMENTADO; não
falha — outcome legítimo per spec §A.4).

### Razão concreta (triplicada)

1. **Caminho 1 (crate)**: requer ADR nova revogando/clarificando
   invariante L0 `export.md` linha 18 "sem crates externas de PDF".
   Decisão arquitectural maior **fora do escopo P273.14** per spec
   §A.3.
2. **Caminho 2 (profile bytes hardcoded)**: zero profiles CMYK
   royalty-free industry-recognized para redistribuição em produto
   existem. Todos proprietários (Adobe SWOP / ECI FOGRA / IDEAlliance
   GRACoL). "Generic CMYK no-profile" royalty-free não existe
   (ICC.org Tech Note 7 explícito).
3. **Caminho 3 (scope-out)**: ADR-0091 §"ICC profile scope-out
   preserved" decisão original P270.2 reconfirmada por evidência
   empírica P273.14.

### Trabalho prévio externo identificado

Ver `00_nucleo/diagnosticos/typst-passo-273-14-trabalho-previo-externo.md`
para 3 pré-requisitos para futuro hipotético GO:
1. Decisão arquitectural sobre invariante L0 (ADR nova).
2. Profile concreto + licença redistribuível.
3. Decisão sobre PDF size impact (dedup/condicional/aceitação).

### ADR-0091 §"ICC profile scope-out" preserved literal

Decisão P270.2 original mantida:
> "ICC profiles scope-out preserved (cristalino DeviceCMYK directo;
> refino futuro)."

P273.14 confirma empíricamente que "refino futuro" não é imediato —
requer trabalho prévio externo §1 do documento dedicado.

### Sub-padrão emergente — "Scope-out reconfirmado por Fase A" N=1 inaugural

P273.14 inaugura sub-padrão: passo executado até critério go/no-go
binário; quando empírica revela inviabilidade (custo, licensing,
pré-requisitos arquitecturais), output legítimo é **documento de
pendência preserved + trabalho prévio externo identificado**.
Trabalho de diagnóstico legítimo per ADR-0054 graded.

**Distingue de**:
- **"Bug arquitectural intencional corrigido"** P273.12 (limitação
  fechada por refino deliberado quando contexto madura). P273.12 era
  uma limitação **fechável** com refino L3 puro; P273.14 é uma
  pendência **dependente de trabalho externo**.
- **"Refino qualitativo opcional materializado"** N=0 — sub-padrão
  GO-only que **não foi inaugurado** por P273.14 (NO-GO outcome).

### Defaults preservados P262-P273.13 bit-exact

- Gradient não-CMYK preserved literal (Linear/Radial/Conic; RGB-family
  + Oklab + outros spaces).
- Gradient CMYK pré-P273.14 continua via `/DeviceCMYK` directo
  (P270.2 caminho actual).
- 2644 baseline P273.13 preserved (zero alterações código L1/L3).

### Sub-padrões aplicados P273.14

- **"Anotação cumulativa em vez de ADR nova"** N=20 → **N=21
  cumulativo consolidação clara persistente** (décima quarta
  anotação consecutiva ADR-0091).
- **"Reutilização literal helpers cross-passos"** N=17 preserved.
- **"Cap LOC hard vs soft explícito"** N=16 preserved (NO-GO — sem
  cap LOC aplicado; spec §3 caps só GO).
- **"Aplicação meta-ADR (ADR-0093)"** N=9 → **N=10 cumulativo**
  (Pattern 2 anotação cumulativa).
- **"Aplicação meta-ADR (ADR-0094)"** N=12 preserved (NO-GO — sem
  Pattern 1 cap LOC aplicado).
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=4 preserved.
- **"Template-passo replicado literal"** N=2 preserved.
- **"Sub-passos consecutivos do mesmo cluster"** N=9 → **N=10
  cumulativo emergente** (P273.5/6/7/8/9/10/11/12/13/14). Limiar
  N=3-4 atravessado com folga máxima do cluster.
- **"Layout duplo arquitectural aceite"** N=1 preserved.
- **"L3-only parent_bbox"** N=2 preserved.
- **"Dedup Arc::as_ptr resources"** N=3 preserved.
- **"Bug arquitectural intencional corrigido"** N=1 preserved.
- **"Triplicação Group bbox"** N=1 preserved.
- **"Scope-out reconfirmado por Fase A"** N=0 → **N=1 inaugural
  emergente**.
- **"Diagnóstico imutável"** N=29 → **N=30 cumulativo** (vigésimo
  quinto consumo).

Status `IMPLEMENTADO` ADR-0091 preserved literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-14-diagnostico.md` para Fase
A empírica + decisão NO-GO + critério §A.7 cumprido.
Ver `00_nucleo/diagnosticos/typst-passo-273-14-trabalho-previo-externo.md`
para 3 pré-requisitos identificados.

---

## Anotação cumulativa P273.15 — Bbox pós-layout scope-out reconfirmado (NO-GO Fase A; reaplicação)

**Data**: 2026-05-18.
**Decisão**: **NO-GO** via §A.5 critério #1 + #2 + #4 combinados Fase
A P273.15.
**Status do passo**: **SCOPE-OUT-RECONFIRMED** (não IMPLEMENTADO;
não falha — outcome legítimo per spec §A.5).

### Razão concreta (quádrupla)

1. **§A.1 confirma zero demanda empírica** — grep verificação em 20
   documentos `00_nucleo/` retorna 0 casos onde 3γ.2.γ produziu
   output observable incorrecto em 8 sub-passos consecutivos
   (P273.6-P273.13).
2. **Caminho 1 (eager) tem custo perf inaceitável** — `measure_content_constrained`
   executado em todos os Blocks sem dimensions, mesmo quando não há
   gradient `relative=parent` interno. Pior caso O(N²) onde antes
   era O(N).
3. **Caminho 2 (lazy) tem custo de implementação desproporcional** —
   walker novo (~60-100 LOC L1) + manutenção para resolver problema
   sem demanda registada.
4. **3γ.2.γ é aceito por ADR-0054 graded** — "menor mudança
   suficiente" preserved; refino sem demanda é over-engineering.

### Trabalho prévio externo identificado

Ver `00_nucleo/diagnosticos/typst-passo-273-15-trabalho-previo-externo.md`
para 2 pré-requisitos para futuro hipotético GO:
1. Caso empírico concreto identificado (test ou reporte utilizador).
2. Decisão executiva sobre custo perf (aceitar Caminho 1 / Caminho 2
   / optimização específica).

### Decisão P273.6 §A.3 (3γ.2.γ) preserved literal

8 sub-passos consecutivos (P273.6-P273.13) sem contraproba.
Decisão Fase A P273.6 reconfirmada por evidência empírica P273.15.

### Sub-padrão "Scope-out reconfirmado por Fase A" N=1 → N=2 cumulativo

**Reaplicação do padrão inaugurado P273.14**:
- **N=1 (P273.14)**: CMYK-ICC via NO-GO; razão constraints externas
  (profile licensing + crate externa).
- **N=2 (P273.15)**: Bbox medido pós-layout via NO-GO; razão
  ausência de demanda empírica + custo perf inaceitável.

**Padrão consolidado por primeira reaplicação**. Mecânica idêntica:
1. Fase A factual com inventário.
2. Decisão go/no-go binária.
3. NO-GO → output Fase A + trabalho prévio externo.
4. Zero alterações código.
5. Sub-padrão cresce.

**Distingue de**:
- **"Bug arquitectural intencional corrigido"** P273.12 (limitação
  fechável por refino deliberado).
- **"Refino qualitativo opcional materializado"** N=0 — GO-only
  sub-padrão NÃO inaugurado por P273.14 nem P273.15.

**Limiar formalização N=3-4 ainda longe** — candidato meta-ADR
futuro NÃO reservado.

### Defaults preservados P262-P273.14 bit-exact

- Zero alterações código L1/L3 (ADR-0029 preserved absoluto).
- Block com dimensions literais → 3γ.2.γ literal preserved.
- Block sem dimensions → cai no fallback page_bbox L3 P273.5
  (identity transform); comportamento aceito por defaults P273.5-
  P273.13.
- 2644 baseline P273.14 preserved bit-exact.

### Sub-padrões aplicados P273.15

- **"Anotação cumulativa em vez de ADR nova"** N=21 → **N=22
  cumulativo consolidação clara persistente** (décima quinta
  anotação consecutiva ADR-0091).
- **"Reutilização literal helpers cross-passos"** N=17 preserved.
- **"Cap LOC hard vs soft explícito"** N=16 preserved (NO-GO — sem
  cap aplicado).
- **"Aplicação meta-ADR (ADR-0093)"** N=10 → **N=11 cumulativo**
  (Pattern 2 anotação).
- **"Aplicação meta-ADR (ADR-0094)"** N=12 preserved (NO-GO — sem
  Pattern 1 cap aplicado).
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=4 preserved.
- **"Template-passo replicado literal"** N=2 preserved.
- **"Sub-passos consecutivos do mesmo cluster"** N=10 → **N=11
  cumulativo emergente** (P273.5/6/7/8/9/10/11/12/13/14/15). Folga
  máxima sobre limiar formalização N=3-4 preservada.
- **"Layout duplo arquitectural aceite"** N=1 preserved (P273.15
  NO-GO; não cresce — só GO crescia).
- **"L3-only parent_bbox"** N=2 preserved.
- **"Dedup Arc::as_ptr resources"** N=3 preserved.
- **"Bug arquitectural intencional corrigido"** N=1 preserved.
- **"Triplicação Group bbox"** N=1 preserved.
- **"Scope-out reconfirmado por Fase A"** N=1 → **N=2 cumulativo
  emergente** — primeira reaplicação consolida o padrão.
- **"Diagnóstico imutável"** N=30 → **N=31 cumulativo** (vigésimo
  sexto consumo).

Status `IMPLEMENTADO` ADR-0091 preserved literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-15-diagnostico.md` para
Fase A empírica + decisão NO-GO + critério §A.8 cumprido.
Ver `00_nucleo/diagnosticos/typst-passo-273-15-trabalho-previo-externo.md`
para 2 pré-requisitos identificados.

---

## Anotação cumulativa P273.16 — Bbox.y topo-exacto scope-out reconfirmado (NO-GO Fase A; terceira aplicação consolidando padrão; sub-padrão crossing limiar formalização N=3-4)

**Data**: 2026-05-18.
**Decisão**: **NO-GO** via §A.5 critério quadruplo Fase A P273.16.
**Status do passo**: **SCOPE-OUT-RECONFIRMED** (não IMPLEMENTADO;
não falha — outcome legítimo per spec §A.5).

### Descoberta empírica que actualiza premissa da spec

A spec P273.16 §0 declarava "DEBT-56 EM ABERTO desde 2026-04-25"
como bloqueador. **Verificação literal em
`00_nucleo/DEBT.md:535`** confirma:

> "## DEBT-56 — Column flow Fase 3 Layout (L+; refactor multi-region
> do Layouter) — **ENCERRADO (Passo 221) ✓**"

DEBT-56 **encerrado em 2026-05-12** (6 dias antes da spec). Premissa
factualmente desactualizada. **Fase A empírica prevalece sobre
premissa da spec** — cumprimento do critério "verificar
empíricamente" registado P273.7-P273.15.

### Razão concreta NO-GO (quádrupla, com fundamentos actualizados)

1. **§A.1 confirma zero demanda empírica** — 9 sub-passos
   consecutivos (P273.7-P273.15) sem caso registado onde
   3γ.2.γ-inline-baseline-y produziu output visualmente
   insuficiente.
2. **Caminho 1 (refactor inline line_height) fora do escopo P273.16** —
   magnitude L+ vs S-M cluster Gradient. Fase 4 multi-region per
   ADR-0078 §"Decisão" sub-fase (b) scope-out documentado.
3. **Caminho 2 (font_metrics.ascender ad-hoc) cria dívida
   invisível** sem demanda registada — over-engineering per
   ADR-0054 graded.
4. **3γ.2.γ-inline-baseline-y P273.7 aceito por ADR-0054 graded** +
   coerente com **P156H limitação consciente** preserved literal —
   "menor mudança suficiente" preserved.

### Bloqueador real identificado (substituindo DEBT-56 fechado)

- **P156H limitação consciente** em
  `00_nucleo/prompts/entities/content.md:817-829`:
  "`inset.top`/`inset.bottom` armazenados mas não aplicados em
  layout inline (alterariam line_height)".
- **ADR-0078 §"Decisão" sub-fase (b)** — Refino multi-region flow
  real → Fase 4 candidata NÃO-reservada per política P158.
- **NÃO DEBT-56** — fechado P221.

### Trabalho prévio externo identificado

Ver `00_nucleo/diagnosticos/typst-passo-273-16-trabalho-previo-externo.md`
para 3 pré-requisitos para futuro hipotético GO:
1. Caso empírico concreto identificado.
2. Refactor inline line_height (Fase 4 multi-region ou dedicated).
3. Decisão arquitectural sobre dívida invisível Caminho 2.

### Decisão P273.7 §A.3 (3γ.2.γ-inline-baseline-y) preserved literal

9 sub-passos consecutivos (P273.7-P273.15) sem contraproba.
Decisão Fase A P273.7 reconfirmada por evidência empírica P273.16.

### Sub-padrão "Scope-out reconfirmado por Fase A" N=2 → N=3 cumulativo crossing limiar formalização N=3-4

**Terceira aplicação consolidando padrão com 3 razões NO-GO distintas
e legítimas**:

- **N=1 (P273.14)**: Constraints **externas** — CMYK-ICC profile
  licensing + crate externa + invariante L0 export.md.
- **N=2 (P273.15)**: Constraints **internas** — custo perf O(N²)
  + ausência de demanda empírica.
- **N=3 (P273.16)**: Bloqueador **estrutural aceito** — P156H
  limitação consciente per ADR-0054 graded + ausência demanda +
  actualização empírica da premissa da spec (DEBT-56 fechado).

**Limiar formalização ADR meta N=3-4 atingido com folga
consolidada**. Candidato **meta-ADR formalização NÃO reservado**
— sub-padrão consolidado a nível meta-metodológico per ADR-0093
Pattern 2.

### Cluster Gradient declarável feature-complete pós-P273.16

A sequência "terminar cluster Gradient" iniciada em P273.10
considera-se esgotada com 7 sub-passos materializados (P273.10-P273.13)
+ 3 scope-outs reconfirmados (P273.14-P273.16) + sub-passos
precedentes P273.5-P273.9.

**12 sub-passos consecutivos** total (P273.5-P273.16) — caminho mais
longo documentado no projecto cristalino.

### Pendência candidata XS nova (descoberta empírica P273.16)

- **P273.X-bis-content-md-debt56-update** — L0 `content.md:824`
  referência DEBT-56 desactualizada após fechamento P221.
  Candidato cleanup XS (~1 LOC L0) sub-padrão "Cleanup XS derivado"
  análogo P273.8. NÃO reservado.

### Defaults preservados P262-P273.15 bit-exact

- Zero alterações código L1/L3 (ADR-0029 preserved absoluto).
- 3γ.2.γ-inline-baseline-y Boxed inline preserved literal.
- P156H limitação consciente preserved literal.
- 2644 baseline P273.15 preserved bit-exact.

### Sub-padrões aplicados P273.16

- **"Anotação cumulativa em vez de ADR nova"** N=22 → **N=23
  cumulativo consolidação clara persistente** (décima sexta
  anotação consecutiva ADR-0091).
- **"Reutilização literal helpers cross-passos"** N=17 preserved.
- **"Cap LOC hard vs soft explícito"** N=16 preserved (NO-GO — sem
  cap aplicado).
- **"Aplicação meta-ADR (ADR-0093)"** N=11 → **N=12 cumulativo**
  (Pattern 2 anotação).
- **"Aplicação meta-ADR (ADR-0094)"** N=12 preserved (NO-GO — sem
  Pattern 1 cap aplicado).
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=4 preserved.
- **"Template-passo replicado literal"** N=2 preserved.
- **"Sub-passos consecutivos do mesmo cluster"** N=11 → **N=12
  cumulativo emergente** (P273.5-P273.16). **Caminho mais longo
  documentado no projecto cristalino**.
- **"Layout duplo arquitectural aceite"** N=1 preserved.
- **"L3-only parent_bbox"** N=2 preserved.
- **"Dedup Arc::as_ptr resources"** N=3 preserved.
- **"Bug arquitectural intencional corrigido"** N=1 preserved.
- **"Triplicação Group bbox"** N=1 preserved.
- **"Scope-out reconfirmado por Fase A"** N=2 → **N=3 cumulativo
  crossing limiar formalização N=3-4**. Terceira aplicação
  consolida sub-padrão com 3 razões NO-GO distintas. Candidato
  meta-ADR formalização NÃO reservado.
- **"Diagnóstico imutável"** N=31 → **N=32 cumulativo** (vigésimo
  sétimo consumo).

Status `IMPLEMENTADO` ADR-0091 preserved literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-16-diagnostico.md` para
Fase A empírica + decisão NO-GO + actualização premissa da spec
+ critério §A.8 cumprido.
Ver `00_nucleo/diagnosticos/typst-passo-273-16-trabalho-previo-externo.md`
para 3 pré-requisitos identificados + descoberta candidata cleanup
XS futuro P273.X-bis-content-md-debt56-update.

---

## Anotação cumulativa P273.17 — Reflexão metodológica formal cluster Gradient + 3 ADRs meta novas

**Data**: 2026-05-18.
**Decisão**: passo administrativo S+ — formaliza 3 sub-padrões empíricos
N=3-4 cumulativos atingidos durante cluster Gradient via 3 ADRs meta
novas EM VIGOR + documento reflexão.
**Status do passo**: **IMPLEMENTADO** (admin).

### Marco final cluster Gradient

Cluster Gradient encerrado definitivamente pós-P273.17 com:

- **13 sub-passos consecutivos** P273.5-P273.17 — **caminho mais
  longo de sub-passos consecutivos do mesmo cluster documentado no
  projecto cristalino**.
- **9 sub-passos materializados** (P273.5-P273.13) + **3 sub-passos
  scope-out reconfirmados** (P273.14-P273.16) + **1 sub-passo
  administrativo** (P273.17).
- **17 anotações cumulativas ADR-0091** consecutivas (P273.5-P273.17).
- **3 ADRs meta novas** criadas EM VIGOR directo.
- **Documento reflexão** standalone como output legível.

### ADRs criadas EM VIGOR (paridade pattern P271 ADR-0093/0094)

- **ADR-0095** — "Dedup `Arc::as_ptr` resources" (N=3 cumulativo:
  P73 image_resources + P263 pattern_resources + P273.12
  pattern_resources bbox-aware via DedupKey). Paradigma L3 export
  consolidado cross-cluster.

- **ADR-0096** — "Pattern DEBT-37 campo Layouter consumer-pending"
  (N=4 cumulativo com folga: P84.6 Grid cell + P273.5 parent_bbox
  consumer-pending + P273.6 consumer real + P273.9 Grid cell
  paralelo). Paradigma refino estrutural Layouter incremental
  consolidado cross-cluster.

- **ADR-0097** — "Scope-out reconfirmado por Fase A" (N=3 cumulativo
  com 3 razões NO-GO distintas: P273.14 constraints externas +
  P273.15 constraints internas + P273.16 bloqueador estrutural
  aceito). Paradigma decisional Fase A consolidado.

### Documento reflexão como output independente

`00_nucleo/diagnosticos/typst-cluster-gradient-reflexao.md` —
documento legível standalone com 10 secções:

1. Trajectória factual P273.5-P273.16.
2. Sub-padrões emergentes (17 cumulativos registados).
3. Limiares formalização atingidos (3 → ADRs novas).
4. Descobertas metodológicas (caps soft sub-estimados; Fase A
   factual prevalece; cleanups XS; bugs latentes).
5. Pendências residuais (3 scope-outs + 2 candidatos XS NÃO
   reservados + 1 pendência fora cluster).
6. Trade-offs aceitos (`/DeviceCMYK` + 3γ.2.γ + baseline-y; todos
   ADR-0054 graded).
7. Anti-padrões evitados (over-formalização + scope creep cego +
   NO-GO cobertura + inserção não-documentada).
8. Reflexão final — cluster Gradient como caso de estudo
   metodológico.
9. Conclusão.
10. Referências.

### Sub-padrões NÃO formalizados (preserved emergentes)

Anti-padrão over-formalização explícito P273.17 §0 — apenas
sub-padrões com N≥3 e valor metodológico claro foram formalizados.
**Sub-padrões preserved aguardando reaplicação cross-cluster**:

- L3-only parent_bbox (N=2 cumulativo).
- Template-passo replicado literal (N=2 cumulativo).
- Layout duplo arquitectural aceite (N=1 inaugural).
- Extract helper de replicação inline (N=1 inaugural).
- Triplicação Group bbox (N=1 inaugural).
- Bug arquitectural intencional corrigido (N=1 inaugural).
- Bug latent corrigido em scope creep (N=1).
- Cleanup XS derivado (N=1 inaugural).

### Sub-padrão meta-meta NÃO formalizado

"Passo administrativo XS/S criar ADRs meta" N=3 cumulativo:
- N=1 P156K (ADR-0064 + 0065).
- N=2 P271 (ADR-0093 + 0094).
- **N=3 P273.17 (este passo)** (ADR-0095 + 0096 + 0097).

Limiar atingido mas **NÃO formalizado** (anti-padrão explícito).
Documentado em `typst-cluster-gradient-reflexao.md` §7 para futuro
hipotético se N=4 surgir noutro cluster.

### Defaults preservados P262-P273.16 bit-exact

- Zero alterações código L1/L3 (ADR-0029 preserved absoluto).
- 2644 baseline P273.16 preserved bit-exact.
- ADRs vigentes 81 → **84** (3 EM VIGOR novas).

### Sub-padrões aplicados P273.17

- **"Anotação cumulativa em vez de ADR nova"** N=22 → **N=23
  cumulativo consolidação clara persistente** (décima sétima
  anotação consecutiva ADR-0091).
- **"Reutilização literal helpers cross-passos"** N=17 preserved.
- **"Cap LOC hard vs soft explícito"** N=16 preserved (passo
  administrativo sem cap LOC aplicado).
- **"Aplicação meta-ADR (ADR-0093)"** N=12 → **N=13 cumulativo**
  (Pattern 2 anotação cumulativa).
- **"Aplicação meta-ADR (ADR-0094)"** N=12 preserved (sem Pattern
  1 cap aplicado; documental).
- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=4 preserved
  (formalizada em ADR-0096).
- **"Template-passo replicado literal"** N=2 preserved.
- **"Sub-passos consecutivos do mesmo cluster"** N=12 → **N=13
  cumulativo emergente** (P273.5-P273.17).
- **"Layout duplo arquitectural aceite"** N=1 preserved.
- **"L3-only parent_bbox"** N=2 preserved.
- **"Dedup Arc::as_ptr resources"** N=3 preserved (formalizada
  em ADR-0095).
- **"Bug arquitectural intencional corrigido"** N=1 preserved.
- **"Triplicação Group bbox"** N=1 preserved.
- **"Scope-out reconfirmado por Fase A"** N=3 preserved
  (formalizada em ADR-0097).
- **"Diagnóstico imutável"** N=32 → **N=33 cumulativo** (vigésimo
  oitavo consumo).

### Cluster Gradient encerrado definitivamente

Cluster Gradient feature-complete + qualitativo + refino estrutural
extensivo + cleanup intra-cluster + dedup bbox-aware + render real
Groups + 3 scope-outs reconfirmados + sub-padrões formalizados +
reflexão metodológica documentada. **Próximo passo natural: sair
do cluster Gradient definitivamente**.

Status `IMPLEMENTADO` ADR-0091 preserved literal. Ver
`00_nucleo/diagnosticos/typst-passo-273-17-diagnostico.md` para Fase
A empírica + critério §A.8 cumprido. Ver
`00_nucleo/diagnosticos/typst-cluster-gradient-reflexao.md` para
documento reflexão completo. Ver ADR-0095/0096/0097 para mecânica
formal de cada sub-padrão formalizado.

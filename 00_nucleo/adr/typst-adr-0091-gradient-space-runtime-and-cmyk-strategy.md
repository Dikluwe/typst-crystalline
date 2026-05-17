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

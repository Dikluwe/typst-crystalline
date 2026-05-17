# typst-passo-270 — P-Gradient-Space-Custom L1+stdlib (activa `space: ColorSpace` cross-variant)

**Magnitude**: M (cap composto: L1 ≤ 350 LOC + stdlib ≤ 50 LOC + ≤ 50 testes).
**Cluster**: Visualize / Gradient (activação de feature cross-variant).
**Origem**: §8 relatório P269 candidato #1; ADR-0083 §"ColorSpace runtime" scope-out.
**Tipo**: passo principal P270 (não sub-passo). Activação de feature nova: campo `space: ColorSpace` em Linear/Radial/Conic passa de Oklab hardcoded para configurável.
**Sequência**: P262/P264/P267 (3 variants L1+stdlib Oklab hardcoded) → **P270 (space activado L1+stdlib)** → P270.1 + P270.2 (L3 emit; decomposição decidida pós-P270).
**Estratégia decidida**:
- Utilizador escolheu **Op B (estratégia uniforme L3)** mas L3 é fora deste passo.
- **Decomposição L3 adiada** — P270 só toca L1+stdlib.
- **ADR-0091 dedicada** criada PROPOSTO+IMPLEMENTADO mesmo passo.
- Magnitude L+ total decomposta em P270 (M) + P270.1 (futuro) + P270.2 (futuro).
- **Industry research proactiva consolidada** (web_search P-Gradient-Space): vanilla Family A (Oklab/Oklch/HSL/HSV) via pré-amostragem N=16; Family B (sRGB/LinearRGB/Luma/CMYK) directo PDF-native. Cristalino L1 sample-no-space convergente com vanilla; L3 emit decidido P270.1/P270.2.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L1/stdlib nunca antes de prompt L0 + ADR. Ordem: diagnóstico → ADR-0091 criação → prompt L0 → fix-hashes → testes-primeiro → código.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-gradient-space-passo-270.md` imutável. **Oitavo consumo directo de fonte** (P262/P264/P267/P268/P268.1/P268.2/P269 vanilla + **P270 vanilla space-aware sample**).

3. **ADR-0091 dedicada criada PROPOSTO → IMPLEMENTADO mesmo passo** (utilizador escolheu vs anotação ADR-0083 cumulativa). Sub-padrão "ADR PROPOSTO+IMPLEMENTADO mesmo passo" **N=5 → N=6 cumulativo**.

4. **ADR-0083 revogação parcial registada via ADR-0091**:
   - **§"ColorSpace runtime" scope-out revogado** parcialmente por P270 (L1+stdlib activado; L3 fica para P270.1).
   - **§"DeviceCMYK PDF" scope-out preserved** — revogação adiada para P270.2.
   - **§"Operadores cor"** preservado scope-out P270.
   - **§"Constantes nomeadas extras"** preservado scope-out P270.

5. **Sub-padrão "ADR scope-out revogado parcialmente"** **N=2 → N=3 cumulativo** (P267 Conic + P269 focal_* + **P270 ColorSpace**). Atinge limiar formalização clara N=3-5; candidato meta-formalização futura.

6. **ADR-0087/ADR-0088/ADR-0089/ADR-0090 preservadas** — Linear/Radial/Conic/Type-4-strategy intocados em estratégia. Anotação cumulativa em cada uma (P270 ColorSpace runtime activado L1+stdlib).

7. **ADR-0039 preservado** — TextStyle intocado.

8. **ADR-0054 perfil graded DEBT-1**: anotação cumulativa P270 (cluster Gradient extensão space cross-variant materializada L1+stdlib; L3 adiado).

9. **ADR-0018 preservado** — implementação autónoma; sem dependências externas.

10. **Crystalline-lint zero violations** obrigatório.

11. **Reutilização literal helpers Color P257** — `Color::to_rgba_f32`, conversões cross-space; **sub-padrão "Reutilização literal helpers cross-passos" N=5 → N=6 cumulativo**.

12. **Vanilla read-first autorizado** — `lab/typst-original/crates/typst-library/src/visualize/gradient.rs` sample multi-space.

13. **Regressão tests P262/P264/P265/P267/P268/P268.2/P269 proibida** — todos os tests Gradient existentes devem continuar verdes. Defaults `space: Oklab` preservam behavior P262/P264/P267 bit-exact.

14. **Op B documentada na ADR-0091** — L3 emit decidirá estratégia uniforme Oklab pipeline para 7 spaces + CMYK directo. **Documentar mas não materializar L3** neste passo.

---

## §1 — Sub-passo P270.A — Diagnóstico empírico vanilla space-aware sample

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-gradient-space-passo-270.md`.

### Comandos exactos a executar

```bash
# 1. Vanilla: campo space em Linear/Radial/Conic
rg -n "space:\s*ColorSpace|pub space" lab/typst-original/crates/typst-library/src/visualize/gradient.rs

# 2. Vanilla: sample matemática multi-space
rg -n "fn sample|interpolate.*space|sample_at|sample_in" lab/typst-original/crates/typst-library/src/visualize/gradient.rs | head -40

# 3. Vanilla: hue wrap handling (HSL/Oklch/HSV têm singularidades)
rg -n "hue|wrap|polar|short.*long" lab/typst-original/crates/typst-library/src/visualize/gradient.rs | head -20

# 4. Vanilla: stdlib named arg `space:` parsing
rg -n "space.*named|named.*space" lab/typst-original/crates/typst-library/src/visualize/ | head -20

# 5. Cristalino L1 actual sample (3 variants; verificar onde Oklab está hardcoded)
rg -n "fn sample|interpolate_oklab|color_to_oklab" 01_core/src/entities/gradient.rs

# 6. Cristalino stdlib actual (verificar que `space:` ainda não existe)
rg -n "native_gradient_linear|native_gradient_radial|native_gradient_conic|space" 01_core/src/rules/stdlib/gradients.rs | head -20

# 7. Cristalino Color enum (P257) — confirmar 8 spaces materializados + conversões
rg -n "enum Color|impl Color|to_rgba|to_oklab|to_hsl|to_hsv|to_cmyk" 01_core/src/entities/color.rs

# 8. Cristalino tests Gradient existentes (regressão obrigatória)
cargo test -p typst-cristalino-core gradient 2>&1 | tail -10
```

### Estrutura do diagnóstico (§A.1 a §A.13)

```
§A.1 Vanilla Linear/Radial/Conic campos space — tipo + default Oklab.
§A.2 Vanilla sample multi-space — assinatura + matemática.
§A.3 Vanilla hue-wrap handling — HSL/Oklch/HSV singularidades em 0°/360°.
§A.4 Vanilla stdlib named arg space — parsing + validação.
§A.5 Cristalino L1 sample actual — Oklab hardcoded:
     - `Linear::sample(t)`: interpolate_oklab P262.
     - `Radial::sample(t)`: idem (preservado P264 e P269).
     - `Conic::sample(t)`: idem (preservado P267).
§A.6 Cristalino stdlib actual — sem named arg `space`.
§A.7 Cristalino Color P257 — 8 spaces materializados:
     - Verificar assinatura `Color::to_<space>()` para cada um dos 8.
     - Confirmar conversões cross-space disponíveis.
§A.8 PROPOSTA L1 — campo `space: ColorSpace` em cada variant struct:
     - Default Oklab (preserva P262/P264/P267 behavior).
     - Sample interpola no space escolhido via Color conversions.
§A.9 PROPOSTA hue-wrap default cristalino:
     - HSL/Oklch/HSV: "shorter hue" default (CSS standard) — paridade vanilla.
     - Implementação: interpolar hue com wrap em 360°; escolher caminho mais curto.
§A.10 PROPOSTA stdlib — named arg `space: ColorSpace` cross-variant:
     - 3 variants ganham named arg literal.
     - Default Oklab.
     - Validação: space deve ser um dos 8 ColorSpace válidos.
§A.11 Tests P262/P264/P265/P267/P268/P268.2/P269 — paridade comportamental:
     - Defaults space=Oklab preservam bytes idênticos.
     - Construtores existentes Linear::linear/radial/conic preservam assinatura.
§A.12 Cenário detectado:
     - **B1 fecho conceptual L1+stdlib** (esperado).
     - **B2 sub-passos** se vanilla revelar matemática hue-wrap muito complexa.
§A.13 Decisão arquitectural — Op B documentada (L3 estratégia uniforme; materialização adiada para P270.1).
```

### Critério de aceitação Fase A

- §A.5 confirma 3 sítios cristalinos onde Oklab é hardcoded literal.
- §A.7 confirma Color P257 tem todas as conversões cross-space necessárias.
- §A.9 propõe hue-wrap shorter como default; §A.10 confirma stdlib API.
- §A.11 confirma defaults preservam behavior P262/P264/P267 bit-exact.

---

## §2 — Sub-passo P270.B — ADR-0091 criação + anotações cumulativas

### B.1 — ADR-0091 estrutura

Ficheiro novo `00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md`.

```
# ADR-0091 — Gradient ColorSpace runtime cross-variant + CMYK strategy

**Status**: PROPOSTO → IMPLEMENTADO (sub-padrão N=6 mesma transição P270)
**Data**: 2026-05-15
**Passo origem**: P270
**Cluster**: Visualize / Gradient / Color
**Tipo**: activação de feature cross-variant + estratégia L3 documentada

## Contexto

ADR-0083 §"Scope-outs" listou:
- ColorSpace runtime: cross-variant em gradient — scope-out P257.
- DeviceCMYK PDF — scope-out P257.

P270 revoga parcialmente ColorSpace runtime scope-out (L1+stdlib materializado).
DeviceCMYK PDF preservado scope-out P270 — revogação adiada P270.2.

Cluster Gradient cristalino actual usa Oklab hardcoded em sample matemática
(P262/P264/P267) e pré-amostragem N=16 Oklab em L3 emit (P263/P265/P268).
Activar `space: ColorSpace` permite gradient.linear(red, blue, space: hsl)
interpolando em HSL conforme vanilla user-facing semantics.

## Pesquisa industry consolidada (P270 proactiva)

Vanilla typst implementa via **estratégia dual**:

**Family A — perceptual spaces (Oklab, Oklch, HSL, HSV)**:
- L1 sample: interpola no space escolhido com hue-wrap shorter para HSL/Oklch.
- L3 PDF emit: pré-amostragem N=16 → DeviceRGB. Documentação vanilla literal:
  "PDF gradients in the color.oklab, color.hsv, color.hsl, and color.oklch
  color spaces are stored as a list of color.rgb colors with extra stops in
  between."

**Family B — PDF-native spaces (sRGB, LinearRGB, Luma, CMYK)**:
- L1 sample: interpola no space directamente.
- L3 PDF emit: directo `/DeviceRGB`, `/DeviceGray`, `/DeviceCMYK`.

**Bug conhecido vanilla**: CMYK gradient PDF emit tem problemas reportados
(typst/typst #4422; comportamento depende viewer/printer). Cristalino terá
oportunidade de melhorar isto em P270.2.

## Decisão P270 (este passo)

1. **L1 sample multi-space**: cada variant (Linear/Radial/Conic) ganha campo
   `space: ColorSpace` (default Oklab; preserva P262/P264/P267 behavior).
2. **Stdlib named arg `space: ColorSpace`** cross-variant. Validação: deve
   ser um dos 8 ColorSpace válidos (P257 §8/8).
3. **Hue-wrap shorter default** para HSL/Oklch/HSV (CSS standard; paridade
   vanilla).
4. **L3 emit estratégia documentada (não materializada P270)**: Op B
   estratégia uniforme — Oklab pipeline pré-amostragem N=16 para 7 spaces
   RGB-family; CMYK directo `/DeviceCMYK` único caso especial.

## Decisão L3 futura (P270.1 + P270.2)

- **P270.1**: 7 spaces (sRGB, LinearRGB, Luma, Oklab, Oklch, HSL, HSV)
  emit via Oklab pipeline N=16 → DeviceRGB. Convergência parcial com
  vanilla Family A; divergência intencional com vanilla Family B
  (cristalino usa Oklab pipeline também para sRGB/LinearRGB/Luma em vez
  de DeviceRGB directo).
  - **Justificação Op B**: simplifica L3 (1 pipeline para 7 spaces); banding
    visual imperceptível em sRGB/LinearRGB/Luma com N=16; reutiliza helpers
    P263/P265/P268.
- **P270.2**: CMYK emit directo `/DeviceCMYK` único caso especial.
  Revoga ADR-0083 §"DeviceCMYK PDF" scope-out. Pode resolver bug vanilla
  #4422 com implementação cristalina autónoma.

## Defaults preservam P262/P264/P267 behavior

- `Gradient::linear(stops, angle)` mantém assinatura; internamente
  `space: ColorSpace::Oklab`.
- `Gradient::radial(stops, center, radius)` idem; preserva P269 focal_*
  defaults.
- `Gradient::conic(stops, center, angle)` idem.
- Stdlib `gradient.linear/radial/conic(...)` sem named `space:` arg →
  P262/P264/P267 behavior idêntico.
- Tests P262/P264/P265/P267/P268/P268.2/P269 zero regressão.

## Hue-wrap handling cristalino

Para HSL/Oklch/HSV (polar color spaces):
- Default "shorter hue" (CSS standard; vanilla paridade).
- Interpolação: se `hue_diff > 180°`, wrap (subtrai 360°) para escolher
  caminho mais curto.
- Edge case: hue_diff exactamente 180° → interpolação ambígua; cristalino
  default sentido positivo (paridade CSS).

Scope-outs P270:
- "longer hue" / "increasing hue" / "decreasing hue" CSS modes — scope-out
  P270; pode ser candidato refino futuro.

## Consequências

+ User-facing `gradient.linear(red, blue, space: hsl)` funcional.
+ L1 sample paridade vanilla em todos os 8 spaces.
+ ADR-0083 §ColorSpace runtime revogado parcialmente (L1+stdlib;
  L3 adiado P270.1/P270.2).
+ Cluster Gradient L1+stdlib feature-complete em 3 variants × 8 spaces.
- L3 emit ainda Oklab hardcoded (utilizadores não verão diferença visual
  até P270.1 + P270.2 fecharem).
- CMYK ainda usa Oklab pipeline em L3 (vai mudar em P270.2).
- Tests E2E PDF que verifiquem bytes específicos podem falhar quando
  P270.1 mudar L3 emit; preservação P270 garante só L1+stdlib mudou.

## Scope-outs preserved

- DeviceCMYK PDF emit (revogação P270.2 futura).
- "longer hue" / "increasing hue" / "decreasing hue" CSS modes.
- ICC profiles para CMYK PDF/A compliance.
- PostScript functions emit para spaces não-PDF-native (vanilla história
  legacy; cristalino preserva Oklab pipeline N=16 alternativa).

## Alternativas consideradas

- **Op A — Convergir vanilla literal** (Family A pré-amostragem; Family B
  PDF-native directo): rejeitada — mais LOC L3; cristalino sem ganho real
  vs Op B porque N=16 Oklab é qualidade suficiente para sRGB/LinearRGB/Luma.
- **Op C — Decisão adiada** (P270.1 implementa Op A; P270.2 verifica B): 
  rejeitada — utilizador escolheu Op B explicitamente.
- **Anotação cumulativa ADR-0083** vs ADR nova: rejeitada — escala
  (ColorSpace runtime cross-variant + futuro DeviceCMYK) justifica ADR
  dedicada per princípio P0.

## Critério revisão

ADR-0091 pode ser revisitada se:
- P270.1 revelar banding visível em sRGB/LinearRGB com N=16 (forçaria Op A).
- CMYK PDF emit P270.2 revelar incompatibilidade reader-level
  (forçaria Op A para CMYK).
- Vanilla bug #4422 revelar-se específico de krilla; cristalino solução
  pode ser literal Op B preservado.

## Subpadrões aplicados

- ADR PROPOSTO+IMPLEMENTADO mesmo passo: N=5 → N=6 cumulativo.
- ADR scope-out revogado parcialmente: N=2 → N=3 cumulativo (P267 + P269
  + P270).
- Reutilização literal helpers cross-passos: N=5 → N=6 (P257 Color
  conversões).
- Diagnóstico imutável (oitavo consumo directo): N=12 → N=13.
- Fase A com industry research proactiva: **N=1 inaugural** (P270 é
  primeira aplicação preventiva vs P268.1/P268.1-correção reactivas).

## Referências cross-passos

- P262/P264/P267 — Linear/Radial/Conic L1+stdlib Oklab hardcoded.
- P263/P265/P268 — L3 emit Oklab pipeline.
- P268.2 — adaptive N hybrid (preservado).
- P269 — focal_* activated.
- ADR-0083 — Color 8/8 spaces (§ColorSpace runtime revogado parcialmente).
- ADR-0087/ADR-0088/ADR-0089/ADR-0090 — Linear/Radial/Conic/Type-4 estratégias.
- ADR-0054 — Perfil graded.
- ADR-0085 — Diagnóstico imutável (oitavo consumo).
- typst/typst issue #4422 — CMYK gradient bug vanilla.
- typst/typst blog 2023 — PostScript functions legacy history.
- typst.app/docs/reference/visualize/gradient — vanilla dual strategy.

## Próximos passos

- P270.1 — L3 RGB-family + perceptual via Oklab pipeline N=16 (S+).
- P270.2 — L3 CMYK directo `/DeviceCMYK` (S+; revoga ADR-0083 §CMYK).
- P-Gradient-Relative-Custom (M; activa relative: RelativeTo).
```

### B.2 — Anotação cumulativa ADR-0083 P270

Adicionar após §"Scope-outs":

```
## Anotação cumulativa P270 — ColorSpace runtime revogado parcialmente

§"ColorSpace runtime" scope-out revogado parcialmente por P270 — L1+stdlib
gradient activa `space: ColorSpace` cross-variant (3 variants × 8 spaces).
L3 emit revogação adiada P270.1.

§"DeviceCMYK PDF" scope-out preserved P270 — revogação adiada P270.2.

§"Operadores cor" + §"Constantes nomeadas extras" preservados scope-out.

Ver ADR-0091 EM VIGOR para decisão completa.
```

### B.3 — Anotações cumulativas ADR-0087/0088/0089/0090

Cada uma recebe anotação curta:

```
## Anotação cumulativa P270 — ColorSpace runtime

Variant ganha campo `space: ColorSpace` (default Oklab; preserva
P262/P264/P267 behavior). L1 sample interpola no space escolhido.
L3 emit Oklab pipeline preservado P270 — refactor adiado P270.1.
Ver ADR-0091 EM VIGOR.
```

### B.4 — Anotação cumulativa ADR-0054 P270

```
P270 — cluster Gradient extensão ColorSpace runtime cross-variant activado
L1+stdlib (3 variants × 8 spaces); ADR-0083 §ColorSpace revogado
parcialmente; perfil graded preservado.
```

### B.5 — L0 prompt `entities/gradient.md` anotação P270

Adicionar nova secção:

```
## ColorSpace runtime (P270 — cross-variant)

Cada variant (Linear/Radial/Conic) tem campo `space: ColorSpace`:
- Default = ColorSpace::Oklab (preserva P262/P264/P267 behavior).
- Sample interpola no space escolhido.
- Hue-wrap shorter default para HSL/Oklch/HSV.

Stdlib named arg `space: ColorSpace` cross-variant.

L3 emit ainda Oklab pipeline; refactor multi-space adiado P270.1+P270.2.
Ver ADR-0091 EM VIGOR.
```

### B.6 — Hashes propagados

`crystalline-lint --fix-hashes` propaga hashes em L0 + headers L1 afectados. Zero violations.

---

## §3 — Sub-passo P270.C — Materialização L1+stdlib (testes primeiro)

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0091 criada PROPOSTO §2.
3. L0 + ADR-0087/0088/0089/0090/0083/0054 anotações §2.B.
4. `crystalline-lint --fix-hashes`.
5. **Testes-primeiro** — adicionar ~40-50 testes ANTES de qualquer LOC L1/stdlib.
6. L1 código — campo space + sample multi-space + hue-wrap helpers.
7. Stdlib código — named arg parsing 3 variants.
8. ADR-0091 promoção PROPOSTO → IMPLEMENTADO.
9. Verificação final.

### Cap LOC

- L1: ≤ 350 LOC em `01_core/src/entities/gradient.rs` (3 variants struct field + sample multi-space + hue-wrap helpers + interpolate_in_space helper).
- Stdlib: ≤ 50 LOC em `01_core/src/rules/stdlib/gradients.rs` (named arg parsing 3 variants).
- Testes: ≤ 50 novos.

### Estrutura L1 esperada

```rust
// 01_core/src/entities/gradient.rs

pub struct Linear {
    pub stops:  Arc<[GradientStop]>,
    pub angle:  Angle,
    pub space:  ColorSpace,  // P270 — default Oklab
}

pub struct Radial {
    pub stops:        Arc<[GradientStop]>,
    pub center:       Axes<Ratio>,
    pub radius:       Ratio,
    pub focal_center: Axes<Ratio>,
    pub focal_radius: Ratio,
    pub space:        ColorSpace,  // P270 — default Oklab
}

pub struct Conic {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub angle:  Angle,
    pub space:  ColorSpace,  // P270 — default Oklab
}

impl Linear {
    pub fn sample(&self, t: f32) -> Color {
        let (c1, c2, local_t) = find_adjacent_stops(self.stops, t);
        interpolate_in_space(c1, c2, local_t, self.space)
    }
    // Análogo Radial::sample, Conic::sample.
}

// Helper novo cross-variant
fn interpolate_in_space(c1: Color, c2: Color, t: f32, space: ColorSpace) -> Color {
    match space {
        ColorSpace::Oklab => interpolate_oklab(c1, c2, t),  // P262 preserved
        ColorSpace::Oklch => interpolate_oklch_shorter(c1, c2, t),  // P270 novo
        ColorSpace::Srgb => interpolate_srgb(c1, c2, t),  // P270 novo
        ColorSpace::LinearRgb => interpolate_linear_rgb(c1, c2, t),
        ColorSpace::Luma => interpolate_luma(c1, c2, t),
        ColorSpace::Hsl => interpolate_hsl_shorter(c1, c2, t),
        ColorSpace::Hsv => interpolate_hsv_shorter(c1, c2, t),
        ColorSpace::Cmyk => interpolate_cmyk(c1, c2, t),
    }
}

// Hue-wrap helper (polar spaces)
fn interpolate_hue_shorter(h1: f32, h2: f32, t: f32) -> f32 {
    let diff = h2 - h1;
    let wrapped_diff = if diff.abs() > 180.0 {
        if diff > 0.0 { diff - 360.0 } else { diff + 360.0 }
    } else {
        diff
    };
    let result = h1 + wrapped_diff * t;
    result.rem_euclid(360.0)
}
```

### Construtores preservados

```rust
pub fn linear(stops, angle) -> Self {
    Gradient::Linear(Arc::new(Linear {
        stops: stops.into(), angle,
        space: ColorSpace::Oklab,  // P270 default; preserva P262
    }))
}

// linear_with_space construtor novo
pub fn linear_with_space(stops, angle, space) -> Self {
    Gradient::Linear(Arc::new(Linear { stops: stops.into(), angle, space }))
}

// Análogo radial / conic.
```

### Estrutura stdlib esperada

```rust
// 01_core/src/rules/stdlib/gradients.rs

pub fn native_gradient_linear(args) -> SourceResult<Value> {
    // P262 parsing preservado +
    let space = match args.named.get("space") {
        Some(Value::ColorSpace(cs)) => cs.clone(),
        Some(other) => return Err(...),
        None => ColorSpace::Oklab,  // default
    };
    
    // Validação P270: space deve ser um dos 8 P257 (cobrido por enum match).
    
    // Whitelist named estendida com "space".
    for key in args.named.keys() {
        if !["angle", "space"].contains(&key.as_str()) { erro... }
    }
    
    Ok(Value::Gradient(Gradient::linear_with_space(stops, angle, space)))
}

// Análogo native_gradient_radial / native_gradient_conic.
```

### Estrutura testes esperada

**Unit L1 sample multi-space** (24 tests; 8 spaces × 3 variants):
- `p270_linear_sample_oklab_preserva_p262`: defaults preservam P262.
- `p270_linear_sample_srgb`: red↔blue em sRGB.
- `p270_linear_sample_linear_rgb`.
- `p270_linear_sample_luma`.
- `p270_linear_sample_oklch_shorter_hue`.
- `p270_linear_sample_hsl_shorter_hue`.
- `p270_linear_sample_hsv_shorter_hue`.
- `p270_linear_sample_cmyk`.
- (8 análogos Radial; 8 análogos Conic.)

**Unit hue-wrap** (4 tests):
- `p270_hue_shorter_no_wrap`: diff < 180°.
- `p270_hue_shorter_wrap_positive`: diff > 180° → wrap negativo.
- `p270_hue_shorter_wrap_negative`: diff < -180° → wrap positivo.
- `p270_hue_shorter_exactly_180`: edge case ambíguo.

**Unit stdlib** (12 tests; 4 cenários × 3 variants):
- `p270_stdlib_linear_space_named_oklab_preserva_p262`.
- `p270_stdlib_linear_space_named_hsl`.
- `p270_stdlib_linear_space_named_invalido_erro`.
- `p270_stdlib_linear_space_default_oklab`.
- (4 análogos Radial; 4 análogos Conic.)

**Regressão P262/P264/P267/P268/P268.2/P269** (não novos; verificar verdes):
- Todos tests anteriores Gradient devem permanecer verdes literal.

Total esperado: 24 + 4 + 12 = 40 testes (cap 50; folga 10).

---

## §4 — Sub-passo P270.D — Promoção + README + relatório

1. **ADR-0091** PROPOSTO → IMPLEMENTADO (sub-padrão N=6).
2. **ADR-0083** anotação cumulativa P270 fechada.
3. **ADR-0087/0088/0089/0090** anotações cumulativas P270 adicionadas.
4. **ADR-0054** anotação cumulativa P270 adicionada.
5. **README.md** actualizar:
   - Tabela cobertura Visualize (+~1-2pp via ColorSpace runtime L1+stdlib).
   - Entrada P270 ~80-100 linhas (paridade entrada P262 — activação cross-variant grande).
   - Cross-reference ADR-0091 EM VIGOR.
   - Cluster Gradient L1+stdlib: agora 3 variants × 8 spaces materializado.
   - **L3 ainda Oklab hardcoded** registado literal — refactor adiado P270.1+P270.2.
6. **Distribuição ADRs**: total 77 → 78 (ADR-0091 IMPLEMENTADO; EM VIGOR 33 → 33 preservado; IMPLEMENTADO 29 → 30).
7. **Relatório** `00_nucleo/materialization/typst-passo-270-relatorio.md`:
   - Métricas finais (esperado 2456 + 40 = ~2496).
   - Fase A §A.9 hue-wrap shorter confirmado vanilla paridade.
   - Diff L1/stdlib antes/depois.
   - Sub-padrões + N cumulativo.
   - Regressão zero P262-P269.
   - **ADR-0083 §ColorSpace revogado parcialmente** marcado.
   - **ADR-0091 EM VIGOR + IMPLEMENTADO** marcado.

---

## §política de paragem

1. **Fase A §A.5 revela que Oklab está embebido fora de sample()** — refactor L1 estoura cap 350 LOC. Confirmar antes de continuar.

2. **Fase A §A.7 revela que P257 Color falta conversões cross-space**:
   - Gap > 50 LOC em Color para adicionar conversões.
   - Sub-passo "P-Color-Conversions" candidato pre-requisito.

3. **Helpers cross-space (HSL/Oklch hue-wrap) > 80 LOC** — cap L1 ameaçado. Confirmar antes de continuar.

4. **Cap LOC L1 (350) ou stdlib (50) ameaça ser ultrapassado**.

5. **Cap testes (50) ameaça ser ultrapassado**.

6. **Defaults `space: Oklab` não preservam bytes P262/P264/P267 originais** — indica que sample multi-space alterou ordem operacional. §política condição absoluta.

7. **Hue-wrap implementação produz resultados não-determinísticos** — float corner cases.

8. **Crystalline-lint reporta violations** após anotações.

9. **Regressão tests P262/P264/P265/P267/P268/P268.2/P269** — qualquer test antigo falha. §política absoluta.

10. **Validação stdlib space arg revela ambiguidade** — utilizador passa Color em vez de ColorSpace (typed parameter).

11. **ADR-0091 estrutura ambígua** — anotações cross-ADR (4 ADRs Gradient + ADR-0083) revelam conflito que requer arbitragem.

12. **Vanilla validation falha** — defaults cristalino divergem semantica vanilla:
    - Cristalino `space: Oklab` enum value.
    - Vanilla pode usar `Smart<ColorSpace>` resolvido para Oklab.
    - Confirmar §A.1 antes.

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N pós-P270 | Nota |
|---|---|---|
| Auditoria condicional (ADR-0084) | N=12 | + P270 |
| Diagnóstico imutável (ADR-0085) | **N=13** (oitavo consumo directo fonte vanilla) | + P270 |
| ADR PROPOSTO+IMPLEMENTADO mesmo passo | **N=6** | + P270 ADR-0091 |
| ADR scope-out revogado parcialmente | **N=3** (limiar formalização clara) | + P270 ColorSpace |
| Anotação cumulativa cross-ADR | **N=1 inaugural** | P270 anota 5 ADRs simultâneo (0083/0087/0088/0089/0090/0054) |
| Reutilização literal helpers cross-passos | **N=6** | + P270 (Color P257 conversões cross-space) |
| Auto-aplicação ADR-0065 inline | N=11 | + P270 |
| Fase A com industry research proactiva | **N=1 inaugural** | P270 (primeira aplicação preventiva pré-spec) |
| Decomposição L+ em sub-passos | **N=1 inaugural** | P270 + P270.1 + P270.2 (decomposição L+ → M+M+S+) |

### Marco arquitectural P270

**Cluster Gradient L1+stdlib feature-complete em 3 variants × 8 spaces** — paridade vanilla user-facing.

**Sub-padrão "ADR scope-out revogado parcialmente" atinge N=3 cumulativo limiar formalização clara** — candidato meta-ADR formalização futura paridade P260 (ADR-0084/0085 formalizadas em N=5-6 cumulativo).

**Primeira aplicação "Fase A com industry research proactiva"** — pesquisa vanilla typst docs + blog 2023 + issue tracker ANTES de spec, em vez de reactiva pós-divergência (P268.1 e P268.1-correção). Lição operacional P268 aplicada.

**Primeira aplicação "Decomposição L+ em sub-passos"** — P270 + P270.1 + P270.2 decompõe magnitude L+ original em 3 passos M+M+S+ proportional ao cap ADR-0061 (granularidade 1-2 features/passo). Reversibilidade preservada.

### Sequência pós-P270

- **P270.1** — L3 RGB-family + perceptual via Oklab pipeline N=16 (M+; ~300-400 LOC).
- **P270.2** — L3 CMYK directo `/DeviceCMYK` (S+; ~150-200 LOC; revoga ADR-0083 §CMYK).
- **P-Gradient-Relative-Custom** (M).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).

---

## §referências cross-passos

- **P262** — Gradient Linear L1+stdlib (Oklab hardcoded; extendido).
- **P264** — Gradient Radial L1+stdlib (Oklab hardcoded; extendido).
- **P267** — Gradient Conic L1+stdlib (Oklab hardcoded; extendido).
- **P263/P265/P268** — L3 emit Oklab pipeline (preservado P270; refactor P270.1).
- **P268.2** — Adaptive N hybrid (preservado).
- **P269** — Radial focal_* activated (preservado; campo space adicional cross-variant).
- **P257** — Color 8/8 spaces (ADR-0083; §ColorSpace runtime revogado parcialmente P270).
- ADR-0083 — Color paridade (anotada cumulativa P270).
- ADR-0091 — ColorSpace runtime + CMYK strategy (criada PROPOSTO+IMPLEMENTADO P270).
- ADR-0054 — Perfil graded (anotada cumulativa P270).
- ADR-0087/0088/0089/0090 — Variant strategies (anotadas cumulativa P270).

---

## §0.1 — Notas de execução para Claude Code

- **Fase A §A.5 é crítica** — confirmar 3 sítios cristalinos onde Oklab está hardcoded literal antes de refactor.
- **Defaults `space: Oklab` preservam bytes P262/P264/P267 literal** — §política condição 6 absoluta.
- **Regressão tests P262-P269 zero** — `cargo test gradient` antes/depois bate identical.
- **Hue-wrap shorter default** confirmado vanilla §A.9; cristalino paridade.
- **L3 não tocado neste passo** — refactor adiado P270.1; verificar §política condição 11.
- **Industry research consolidada em ADR-0091** — pesquisa proactiva pré-spec; sub-padrão "Fase A com industry research proactiva" N=1 inaugural.
- **Anotações cross-ADR (5 ADRs anotadas P270)** — sub-padrão "Anotação cumulativa cross-ADR" N=1 inaugural; verificar §política condição 11.
- **Relatório final esperado**: 2456 + 40 = ~2496 testes verdes; hash drift L0; lint zero; ADRs 77 → 78 (ADR-0091 IMPLEMENTADO).
- **Marco "Cluster Gradient L1+stdlib feature-complete 8/8 spaces"** documentado em relatório §1 + ADR-0091 §"Consequências".

# Prompt: MathConstants — Constantes OpenType MATH
Hash do Código: c33cafba

## Módulo

`01_core/src/entities/math_constants.rs`

## Contexto

O `MathLayouter` precisa de constantes tipográficas da tabela OpenType MATH
para posicionamento correcto de fracções, superscripts, subscripts e radicais.
Fontes como STIX Two Math, Latin Modern Math, Cambria Math definem estas
constantes com precisão.

Este módulo define o tipo de domínio `MathConstants` em L1 com os valores
em design units. L3 é responsável por preencher a struct a partir de ttf-parser.

## Tipo exportado

```rust
pub struct MathConstants {
    pub upem: f64,
    pub fraction_rule_thickness: f64,
    pub fraction_num_gap: f64,                 // P920 — piso mínimo, não gap directo (ver §P920)
    pub fraction_denom_gap: f64,               // P920 — piso mínimo, não gap directo (ver §P920)
    pub fraction_numerator_shift_up: f64,       // P920
    pub fraction_denominator_shift_down: f64,   // P920
    pub superscript_shift_up: f64,
    pub superscript_shift_up_cramped: f64,     // P915
    pub subscript_shift_down: f64,
    pub radical_vertical_gap: f64,
    pub radical_rule_thickness: f64,
    pub axis_height: f64,
    pub script_percent_scale_down: f64,       // 0.0–1.0
    pub script_script_percent_scale_down: f64, // 0.0–1.0
    pub upper_limit_gap_min: f64,
    pub lower_limit_gap_min: f64,
    pub math_leading: f64,
    pub accent_base_height: f64,              // P922 — cap do gap de acento
    pub flattened_accent_base_height: f64,    // P922 — threshold para variante flattened (não consumido ainda)
}
```

**17 campos públicos** (Passo 255 §3 inconsistência documental
detectada e reconciliada — prompt L0 lista actualizada vs
struct real). Campos adicionais face documentação 2026-03 (10
campos):

- `axis_height` — altura do eixo matemático sobre a baseline em
  design units. Consumida por `MathLayouter::apply_axis_offset`
  (`mod.rs:228-229`) para fracções, delimitadores e sqrt
  (alinhamento ao eixo matemático).
- `upper_limit_gap_min` / `lower_limit_gap_min` — gaps mínimos
  para limites superiores/inferiores em operadores extensíveis.
- `math_leading` — leading vertical entre linhas matemáticas.
- **P915** — `superscript_shift_up_cramped`: valor alternativo de
  `superscript_shift_up` usado quando o superscrito está num
  contexto "cramped" (`entities/layout_types.md` §P915;
  `engine/math/layout/attach.md` §P915). Lido directamente da
  tabela MATH (`ttf_parser::math::Constants::superscript_shift_up_
  cramped()`), mesmo mecanismo dos outros 14 campos — nenhum campo
  novo em `ttf_parser` foi necessário, só nunca tinha sido
  consumido. Confirmado no vanilla (`typst-layout/src/math/
  scripts.rs:325-330`, `compute_script_shifts`): é o **único** termo
  que muda quando cramped=true — nada mais na fórmula é afectado.
- **P922** — `accent_base_height`: cap para o gap entre acento e
  base alta no layout de acentos (`engine/math/layout/accent.md`
  §P922). `flattened_accent_base_height`: threshold para a variante
  "flattened" do acento quando a base é muito alta — campo adicionado
  por paridade de dados, ainda não consumido.

## Comportamento

- `fallback()`: valores baseados em STIX Two Math (upem=1000;
  `axis_height = 500.0`). **P915** — `superscript_shift_up_cramped`
  usa o mesmo valor de `superscript_shift_up` (362.0) como default
  neutro: não foi possível confirmar um valor STIX correspondente
  aos restantes 13 campos do fallback actual (divergem de qualquer
  ficheiro STIX Two Math encontrado, não só deste campo — ver
  `typst-passo-915-relatorio.md`), e o fallback só é exercitado
  sem fonte real (testes/`FixedMetrics`), onde "sem efeito de
  cramped" é o comportamento mais defensável. **P922** —
  `accent_base_height` e `flattened_accent_base_height` usam
  `axis_height` (500.0) como default neutro; o fallback só é
  exercitado sem fonte real, onde o cap não tem efeito observável.
- `to_pt(value, size)`: converte design units para Pt — `size * (value / upem)`.
- Zero I/O de sistema — tipo de domínio puro.

## Critérios de verificação

- `fallback()` retorna valores sãos (upem > 0, espessuras > 0,
  escalas 0–1, `axis_height > 0`).
- `to_pt(500.0, Pt(12.0))` com upem=1000 → `Pt(6.0)`.
- `to_pt(0.0, size)` → `Pt(0.0)` (sem divisão por zero quando
  value=0).
- `to_pt` proporcional ao tamanho de fonte.

## Consumers Layouter (Passo 255 reconciliação)

- `apply_axis_offset` (`rules/math/layout/mod.rs:228`) consome
  `axis_height`; aplica baseline em fracções/delimitadores/sqrt.
- Construtor `MathLayouter::new` (`rules/math/layout/mod.rs:218`)
  obtém `MathConstants` via `metrics.math_constants()`.
- `fraction_rule_thickness` + `fraction_num_gap` +
  `fraction_denom_gap` consumidos em `frac.rs`.
- `superscript_shift_up` + `subscript_shift_down` em `attach.rs`.
- `radical_vertical_gap` + `radical_rule_thickness` em `root.rs`.
- `script_percent_scale_down` + `script_script_percent_scale_down`
  em scripts nested.

Tests sentinela `frac_com_axis_height_nao_regride`,
`delimitado_com_axis_height_nao_regride`,
`sqrt_com_axis_height_nao_regride` em `rules/math/layout/tests.rs`
(linhas 520+) verificam `axis_height > 0` activo via fallback.

## P920 — 2 campos novos: gap real de fracção (piso, não aditivo)

**Contexto** (`typst-passo-906-relatorio.md` achado original, `typst-passo-918-relatorio.md`
achado lateral, `typst-passo-920-relatorio.md` Fase A): dois achados que pareciam pequenos
("gap de `underover` sem constante"; "`fraction_denom_gap` sem consumidor") revelaram, por leitura
directa da fórmula real do vanilla, que a causa é mais funda do que "falta uma constante" — em
ambos os casos o cristalino usava um mecanismo estruturalmente diferente do vanilla.

**`fraction_numerator_shift_up`** / **`fraction_denominator_shift_down`**
(`ttf_parser::math::Constants::fraction_numerator_shift_up()`/`fraction_denominator_shift_down()`).
Consumidas pela fórmula real do vanilla (`typst-layout/src/math/fraction.rs:30-53`): `num_gap =
(shift_up - axis - thickness/2 - num.descent()).max(fraction_numerator_gap_min)`; `denom_gap =
(shift_down + axis - thickness/2 - denom.ascent()).max(fraction_denominator_gap_min)` —
`fraction_num_gap`/`fraction_denom_gap` (campos já existentes) passam de **gap directo** (uso
actual, incorrecto) a **piso mínimo** de uma fórmula computada a partir da tinta real do
numerador/denominador e da posição do eixo. Ver `frac.md` §P920.

**Fallback**: os 2 campos novos usam valores extraídos da fonte real (`NewCMMath-Regular.otf`,
`fontTools`, mesmo `rev` pinado) para o fallback, seguindo a mesma disciplina de P915 — não
inventados. Ver `typst-passo-920-relatorio.md` Fase B para os números exactos e a proveniência.

## P922 — `accent_base_height` e `flattened_accent_base_height`

**Contexto** (`typst-passo-920-relatorio.md`, `typst-passo-922.md`): a fórmula real do gap de
acento no vanilla (`typst-layout/src/math/accent.rs:56-65`) é `gap = -accent.descent() -
base.ascent().min(accent_base_height)`. `accent_base_height` é o cap que limita a contribuição
da base para o gap — bases pequenas ganham mais espaço, bases altas usam o cap. Ambos os
campos são lidos da tabela OpenType MATH:
- `accent_base_height`: `ttf_parser::math::Constants::accent_base_height().value`
  (`ttf-parser-0.25.1/src/tables/math.rs:214`).
- `flattened_accent_base_height`: `ttf_parser::math::Constants::flattened_accent_base_height()
  .value` (`ttf-parser-0.25.1/src/tables/math.rs:220`).

**Consumidor**: `accent_base_height` é consumido por `layout_accent`
(`engine/math/layout/accent.md` §P922). `flattened_accent_base_height` é usada no vanilla para
decidir se o acento é substituído pela sua variante "flattened" quando a base é muito alta
(`accent.rs:24-28`); essa funcionalidade ainda não existe no cristalino, mas o campo é
adicionado por paridade de dados.

**Decisão arquitectural P922**: em vez de alterar o contrato geral de `text_ink_bounds` (que
afectaria dezenas de consumidores de `MathBox::ascent`/`descent`), introduz-se um método novo
`FontMetrics::text_ink_bounds_signed` que devolve `(top, bottom)` com sinal. `layout_accent`
usa esse método só para medir o acento e calcular o gap real; `text_ink_bounds` e a semântica
de `MathBox` permanecem inalteradas. Ver `engine/layout.md` §P922 e `infra/font_metrics.md`
§P922.

## P952 — campo `display_operator_min_height`

**Medição** (`typst-passo-952` Fase A): o vanilla estica glifos de classe
`Large` (∑, ∏, ∫, ⋃, …) em Display para `DisplayOperatorMinHeight` da tabela
MATH (NewCMMath: **1300du**), via variantes verticais (`summation.v1` =
1401du; `integral.v1` = 2223du — medido nos PDFs reais: `∑` vanilla com
advance 1.444em vs 1.056em no cristalino; `∫` 0.999em vs 0.665em). Mecanismo
vanilla: `resolve.rs:350-354` (classe `Large` + `MathSize::Display` →
Y-stretch com `StretchInfo::default()`) + `fragment/glyph.rs:445-451`
(`relative_to = display_operator_min_height`, target = 1.0×).

Novo campo `pub display_operator_min_height: f64` em `MathConstants` (design
units; lido da tabela MATH em `math_constants_from_face`,
`infra/font_metrics.md` §P952; `MathConstants::fallback()` recebe um valor
sensato — 1300, o de NewCMMath, mesmo padrão dos outros campos do fallback
que já são valores de STIX/NCM). Consumido em
`engine/math/layout/_comum.md` §P952.


## P959 — 2 campos novos: `upper_limit_baseline_rise_min` e `lower_limit_baseline_drop_min`

**Medição** (`typst-passo-959` Fase A): a fórmula do vanilla para os shifts
verticais de limites de operadores grandes (`compute_limit_shifts`,
`lab/typst-original/crates/typst-layout/src/math/scripts.rs:290-313`) usa
quatro termos da tabela MATH — os dois `*_gap_min` já existiam; faltavam os
dois termos de baseline (scope-out registado em P944 §4, aqui promovido).
Valores reais em NewCMMath-Book (fontTools, upem 1000):
`UpperLimitGapMin=200`, `UpperLimitBaselineRiseMin=111`,
`LowerLimitGapMin=167`, `LowerLimitBaselineDropMin=600`.

Novos campos em `MathConstants` (design units):
`pub upper_limit_baseline_rise_min: f64` e
`pub lower_limit_baseline_drop_min: f64`. `MathConstants::fallback()`
recebe os valores medidos de NewCMMath (111 / 600 — mesmo padrão de
`display_operator_min_height`=1300 em P952: o fallback documenta valores
reais da fonte de referência). Leitura da tabela MATH em
`math_constants_from_face` — `infra/font_metrics.md` §P959. Consumo em
`engine/math/layout/attach.md` §P959.

## P970 — 4 campos novos: geometria do índice de raiz (degree)

**Gate:** mudança de contrato (campos em entidade) — **confirmada pelo dono
em 2026-08-05** ("Continue" após `typst-passo-970-relatorio.md`, que
registou a Parte 2 como parada no gate ADR-0127).

**Medição** (`typst-passo-970` Fase A): a fórmula do vanilla para a posição
do índice de `root(n, x)` (`layout_radical`,
`lab/typst-original/crates/typst-layout/src/math/radical.rs:86-96,113-114`)
usa quatro termos da tabela MATH que não existiam em `MathConstants`.
Valores reais em NewCMMath-Book (fontTools, upem 1000):
`RadicalKernBeforeDegree=278`, `RadicalKernAfterDegree=−556`,
`RadicalDegreeBottomRaisePercent=60`, `RadicalExtraAscender=48`.

Novos campos em `MathConstants` (design units, salvo o percentual):
`pub radical_kern_before_degree: f64`, `pub radical_kern_after_degree: f64`,
`pub radical_extra_ascender: f64` e
`pub radical_degree_bottom_raise_percent: f64` — este último como razão
0.0–1.0 (mesma convenção de `script_percent_scale_down`).
`MathConstants::fallback()` recebe os valores medidos de NewCMMath
(278 / −556 / 48 / 0.6 — mesmo padrão de P952/P959: o fallback documenta
valores reais da fonte de referência). Leitura da tabela MATH em
`math_constants_from_face` (`infra/font_metrics.md` §P970) — os três kerns
são `MathValueRecord` (`.value`); o percentual é `i16` cru em ttf-parser
0.25 (`radical_degree_bottom_raise_percent()`, lido como percentagem e
dividido por 100, como `script_percent_scale_down`). Consumo em
`engine/math/layout/root.md` §P970 Parte 2.

## P974 — campo `radical_display_style_vertical_gap`

**Gate:** confirmado pelo dono em 2026-08-05 (ADR-0127 ponto 1 — campo em
entidade; pergunta directa após a Fase A de P974). Medição: vanilla `radical.rs:32-36` usa
`RadicalDisplayStyleVerticalGap` em Display e `RadicalVerticalGap` nos
restantes níveis. Valores reais em NewCMMath-Book (fontTools, upem 1000):
`RadicalDisplayStyleVerticalGap=148`, `RadicalVerticalGap=50`. O campo
novo é `pub radical_display_style_vertical_gap: f64` (design units);
fallback 148.0 (valor real medido, padrão P952/P959/P970); leitura em
`math_constants_from_face` (MathValueRecord `.value`, padrão dos
vizinhos). Consumo: `engine/math/layout/root.md` §P974 (Parte B).

## P990 — constantes Display de fracção (GATE APROVADO pelo dono 2026-08-06)

Quatro campos novos (precedente `display_operator_min_height`, P952), lidos
da tabela MATH em L3 e usados por `frac.rs` quando `style.math_size ==
Display` (vanilla `fraction.rs:33-52` selecciona por `MathSize`):

- `fraction_numerator_display_style_shift_up` (NewCMMath-Book: 677du)
- `fraction_denominator_display_style_shift_down` (686du)
- `fraction_num_display_style_gap_min` (120du)
- `fraction_denom_display_style_gap_min` (120du)

**Medição** (achado §8.5 da auditoria; `ρ/ε₀` a 600dpi): em equações de
bloco (Display) o cristalino usava as constantes de texto (394/345/48du) —
gaps acima/abaixo da barra ~0 e até negativos (sobreposição real: o ρ
quase toca a barra); vanilla 2.56/1.43pt+. Fallback: os mesmos valores de
NewCMMath (documentados acima), mesmo padrão dos outros fallbacks.

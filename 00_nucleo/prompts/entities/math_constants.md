# Prompt: MathConstants — Constantes OpenType MATH
Hash do Código: 551090b9

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
}
```

**15 campos públicos** (Passo 255 §3 inconsistência documental
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

## Comportamento

- `fallback()`: valores baseados em STIX Two Math (upem=1000;
  `axis_height = 500.0`). **P915** — `superscript_shift_up_cramped`
  usa o mesmo valor de `superscript_shift_up` (362.0) como default
  neutro: não foi possível confirmar um valor STIX correspondente
  aos restantes 13 campos do fallback actual (divergem de qualquer
  ficheiro STIX Two Math encontrado, não só deste campo — ver
  `typst-passo-915-relatorio.md`), e o fallback só é exercitado
  sem fonte real (testes/`FixedMetrics`), onde "sem efeito de
  cramped" é o comportamento mais defensável.
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

**`accent_base_height` — NÃO adicionado neste passo, destacado para passo dedicado.** A Fase A
tinha originalmente proposto este campo também, para o cap de gap entre acento e base alta
(`layout_accent`, `accent.rs:56-65` do vanilla). Investigação mais funda revelou uma
incompatibilidade real (não um simples erro de fórmula): a fórmula do vanilla usa
`-accent.descent()`, e `accent.descent()` pode ser **negativo** no vanilla (glifo de acento cuja
tinta fica inteiramente acima da própria baseline — comentário explícito do vanilla, `accent.rs:
57-58`: "Descent is negative because the accent's ink bottom is above the baseline"). O contrato
de `FontMetrics::text_ink_bounds` no cristalino garante `ascent`/`descent` **sempre >= 0**
(`engine/layout/metrics.rs:59-61`) — perde exactamente a informação que a fórmula do vanilla
precisa. Uma aproximação (assumir `accent.descent() ≈ 0`) produz sobreposição (gap negativo), não
uma aproximação inofensiva — o termo é estrutural, não cosmético. Precisa de uma decisão
arquitectural própria (estender `FontMetrics` para extensões com sinal nalgum caso, ou mecanismo
equivalente) antes de poder ser portado fielmente — fora do âmbito de "adicionar uma constante".
Achado registado, destacado para passo dedicado. Ver `typst-passo-920-relatorio.md`,
`accent.md`/`underover.md` §P920 (nota de correcção).

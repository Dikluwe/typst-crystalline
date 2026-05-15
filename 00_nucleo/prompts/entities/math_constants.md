# Prompt: MathConstants — Constantes OpenType MATH
Hash do Código: 73380d77

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
    pub fraction_num_gap: f64,
    pub fraction_denom_gap: f64,
    pub superscript_shift_up: f64,
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

**14 campos públicos** (Passo 255 §3 inconsistência documental
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

## Comportamento

- `fallback()`: valores baseados em STIX Two Math (upem=1000;
  `axis_height = 500.0`).
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

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
    pub script_percent_scale_down: f64,       // 0.0–1.0
    pub script_script_percent_scale_down: f64, // 0.0–1.0
}
```

## Comportamento

- `fallback()`: valores baseados em STIX Two Math (upem=1000)
- `to_pt(value, size)`: converte design units para Pt — `size * (value / upem)`
- Zero I/O de sistema — tipo de domínio puro

## Critérios de verificação

- `fallback()` retorna valores sãos (upem > 0, espessuras > 0, escalas 0–1)
- `to_pt(500.0, Pt(12.0))` com upem=1000 → `Pt(6.0)`
- `to_pt(0.0, size)` → `Pt(0.0)` (sem divisão por zero quando value=0)
- `to_pt` proporcional ao tamanho de fonte

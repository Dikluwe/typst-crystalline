# Prompt L0 — `math/layout/cases` — `MathCases`
Hash do Código: 3986027d

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/cases.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathCases` — chaves grandes (`cases`). Usa o despacho e composição do
`MathLayouter` (ver `_comum.md`).

## P912 — Margem de 10% na Altura da Chave de Cases

Em `layout_cases`, a altura da grelha usada para calcular `min_height_du` da chave esquerda
aplica a margem de 10% do vanilla: `grid_height_pt = (grid_box.ascent + grid_box.descent) * 1.1`.

## P918 — cálculo de `min_height_du` migrado para `grid_delim_target_du` partilhado

**Achado** (P918 Fase A, `cases.rs:28-33` vs `matrix.rs:94-99`): o bloco de 5 linhas acima
(margem de 10% + conversão condicional pt→du) é idêntico byte-a-byte ao usado em `layout_matrix`.
Extraído para `grid_delim_target_du(&self, grid_box, style)` em `mod.rs` (ver `_comum.md` §P918)
— `let min_height_du = self.grid_delim_target_du(&grid_box, style);`. Comportamento inalterado.

## P919 — `apply_axis_offset` move-se para `grid_box` isolado, ANTES de juntar o delimitador

**Achado** (`typst-passo-919-relatorio.md` Fase A/B, vanilla `table.rs:188`:
`frame.set_baseline(height/2.0 + axis)` — centra o meio da altura total no eixo, mesma fórmula já
usada por `apply_axis_offset`, `shift = axis_pt - (ascent-descent)/2`): ao contrário de
`root.rs`/`delimited.rs`, o conteúdo da grelha **deve** ficar centrado no eixo — a fórmula bate
exactamente com o vanilla. Até P919, a chamada (no fim de `layout_cases`, sobre `result` já
concatenado com `left_box`) era um no-op visual (bug de omissão em `apply_axis_offset`,
`_comum.md` §P919, nunca deslocava `items`).

**Achado do Agente A (revisão do design inicial deste L0)**: `result.items` mistura os items do
delimitador `{` (`left_box`, já auto-centrado no eixo internamente por
`layout_stretchy_delimiter` — `stretchy.md` §P917) com os items da grelha (`grid_box`, ainda em
baseline própria) **antes** de `apply_axis_offset` correr. Uma vez corrigido o bug de omissão,
chamar `apply_axis_offset(result, ...)` no fim (como o código fazia até aqui) desloca **também**
o `{`, que já estava correcto — desloca-o pela segunda vez. **Correcção de código necessária**
(não só o fix de `apply_axis_offset`): `layout_cases` passa a chamar
`self.apply_axis_offset(grid_box, style.size)` isoladamente, **antes** de o mesclar com
`left_box.items` — usa o `grid_box` já centrado (items + `ascent`/`descent` actualizados) para
compor `result`. `result` deixa de chamar `apply_axis_offset` no fim.

**Critério de regressão**: `x + cases(a, b)` — o centro vertical da grelha (`{a; b}`) fica a
`axis_pt` da baseline partilhada com `x`, não a ~0 como antes de P919; a chaveta `{` mantém-se em
`y=0` local (a sua própria auto-centragem, inalterada, não duplicada). Testes do Agente A:
`axis_bug_cases_conteudo_centra_no_axis_height_nao_a_zero` (guarda explícita de que `{` fica em
`y=0`).

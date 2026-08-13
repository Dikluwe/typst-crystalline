# Prompt L0 — `math/layout/cases` — `MathCases`
Hash do Código: 58607ad1

**Camada**: L1 · **Alvo**: `01_core/src/compiler/math/layout/cases.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathCases` — chaves grandes (`cases`). Usa o despacho e composição do
`MathLayouter` (ver `_comum.md`).

## P923 — Ramos renderizados em estilo de denominador

**Achado** (`typst-layout/src/math/ir/resolve.rs:1088-1098` — `resolve_cases`
→ `resolve_cells`): os ramos de `cases` são resolvidos pelo vanilla com
`style_for_denominator(styles)` (mesmo mecanismo das células de `mat` em
`resolve_mat`). O cristalino aplicava o `style` do ambiente directamente aos
ramos. A correcção segue o mesmo padrão de `matrix.md` §P923: construir
`cell_style = TextStyle { size: style.size * script_percent_scale_down,
cramped: true, ..style.clone() }` e passá-lo a `layout_grid_rows` (e a
qualquer medição futura de cada ramo). O `row_gap` é `style.size * 0.2`
(paridade `DEFAULT_ROW_GAP` do `CasesElem`, resolvido contra o estilo
exterior — ver `matrix.md` §P923b).

## P912 — Folga de 10% na Altura da Chave de Cases

Em `layout_cases`, a altura da grelha usada para calcular `min_height_du` da chave esquerda
aplica uma folga de 10%: `grid_height_pt = (grid_box.ascent + grid_box.descent) * 1.1`.

> **Fundamentação, ao fim de três redacções** (2026-08-13, P1026 Fase C): a folga **é** do
> vanilla. `resolve.rs:1173` (`fn resolve_delimiters`, "vector, matrix, or cases" — o mesmo
> caminho serve `cases`) pede `Rel::new(Ratio::new(1.1), Abs::zero())` com o mesmo
> `DELIM_SHORT_FALL`, que `stretchy.rs:57-61` também aplica. É **paridade**. As duas
> redacções anteriores — P1024 ("multiplica em vez de subtrair") e P1026 Fase A ("inflação
> extra de 10%", citando `resolve.rs:843`, que é a barra da fracção inclinada) — estão
> **refutadas**. Medição completa em `_comum.md` §P912-folga. **Item fechado.**

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

## P945 — células de `cases` com descida de nível MathSize do vanilla

Mesma correcção e mesma medição de `matrix.md` §P945 (a causa raiz é partilhada
— os dois consumidores de `layout_grid_boxes` tinham o mesmo
`size × script_percent_scale_down` incondicional de P923): `layout_cases` passa
a construir o `cell_style` via `denominator_style` de `_comum.md` §P945
(descida **por nível**: `Display→Text` ×1.0, `Text→Script` ×script_percent,
`Script→ScriptScript` ×sscript/script, `ScriptScript→ScriptScript` ×1.0,
sempre com `cramped: true`), em vez de multiplicar incondicionalmente por
`script_percent_scale_down`. Caso medido do documento de 30 secções:
`cases()` de 3 ramos (secção 9) — chaveta assembly curta/desencontrada por a
grelha estar ~30% mais curta que o vanilla.

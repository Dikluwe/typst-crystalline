# Prompt L0 — `math/layout/root` — `MathRoot`
Hash do Código: 95d469dc

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/root.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathRoot` — sqrt + n-th roots. Consome `radical_vertical_gap` +
`radical_rule_thickness` de `MathConstants`. Baseline x-height aplicado via
`MathLayouter::apply_axis_offset` (ver `_comum.md`); test regressão
`sqrt_com_axis_height_nao_regride` (`tests.rs:520+`).

## P901 — correcção de sinal: overline/símbolo/radicando usavam offsets Y invertidos

**Achado** (`typst-passo-894-relatorio.md` → `typst-passo-901-relatorio.md`): a barra horizontal
do radical atravessava o radicando "a meio da altura" (como um traço/strikethrough) em vez de
ficar por cima — confirmado por medição directa (`mutool trace`/`pdftotext -bbox`) num PDF real.

**Causa**: `layout_root` construía `overline_y`/`rad_offset_y`/`sym_dy` assumindo (implicitamente,
nunca declarado no código) a convenção "`y=0` = topo da caixa" — mas a convenção real e já
documentada desde P800 (linha 26-28 acima, `_comum.md`) é "`y=0` = **baseline** da `MathBox`,
`y` cresce para baixo", confirmada de novo por leitura de `hconcat_spaced`/`layout_equation`
(`mod.rs`) — `hconcat_spaced` só desloca `x` ao juntar boxes irmãs, o que só é correcto se todas
partilharem a mesma baseline `y=0`. Sob a convenção real, os offsets tinham o sinal errado: o
radicando ficava deslocado **para baixo** por `gap+thickness` (devia ficar sem deslocamento, já
está na sua própria baseline) e a overline ficava **abaixo** da baseline (perto do radicando) em
vez de **acima** do topo da tinta do radicando.

**Correcção**: `rad_offset_y` removido (radicando sem deslocamento Y); `overline_y =
-(rad_box.ascent + gap + line_thickness/2)`; `sym_dy = radical_box.ascent - total_ascent`
(sinal invertido do original); índice de `root(n,x)` (`idx_dy`) ajustado de `0.0` para
`-total_ascent` para preservar a intenção original ("topo") sob a convenção correcta — achado da
revisão do orquestrador, não coberto pelos 2 testes do Agente A (que só verificam a relação
overline-vs-radicando), registado para não deixar uma regressão nova e não testada.

**Não depende de P893** (`FontMetrics::math_constants` real vs fallback) — o bug é de aritmética/
sinal na fórmula de posicionamento, independente de os valores de `gap`/`line_thickness` virem de
fallback ou da tabela MATH real da fonte; confirmado por leitura de código antes de implementar.

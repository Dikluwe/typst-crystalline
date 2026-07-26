# Prompt L0 — `math/layout/attach` — `MathAttach`
Hash do Código: 87b60816

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/attach.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado (MathLayouter, despacho): ver `math/layout/_comum.md`.

---

`MathAttach` — subscripts/superscripts/primes merged via eval. Consome
`MathGlyphKern` em todos os **4 quadrantes** (top-left, bottom-left, top-right,
bottom-right) via `self.metrics.math_kern(c, style)` (P255 §2 item 1; geometria
correcta sem `.abs()`, kern negativo permitido — `attach.rs:49-208`). **P891** —
`style` passa a ser o segundo argumento (antes só `c`): `FallbackFontMetrics`
(`03_infra/src/font_metrics.rs`) precisa de saber qual face activa cobre `c` para
ler a tabela MATH real; sem `style`, não tem como resolver a face (não pode
assumir uma única face fixa, ao contrário de `FontBookMetrics`). Antes de P891,
`FallbackFontMetrics` não sobrepunha `math_kern` — herdava o default do trait
(kern zero incondicional), confirmado como a causa do gap indevido antes de
expoentes (`i^2` → `i  ²`, achado de P889/P891, `typst-passo-891-relatorio.md`).

Recebe os `MathPrimes` (resolvidos em eval; ver `_comum.md`) pelo arm
superscript regular — não há arm dedicado `MathPrimes`.

**Critério**: `attach.rs` consome `math_kern` em todos os 4 quadrantes.

---

## Empilhamento de limites (`is_limits`) — P772w

`is_limits` decide se `sub`/`sup` empilham verticalmente acima/abaixo da
base (paridade vanilla `Limits::Display`/`Limits::Always`) em vez de ficarem
como scripts laterais à direita (`Limits::Never`). Só considerado quando
`self.block` (modo bloco/display) é `true` — em modo inline nunca empilha.

Condição (`attach.rs`, braço `MathIdent`/`MathText`):

```rust
(symbols::is_large_operator(ch) && !symbols::is_integral_char(ch))
    || symbols::is_limit_function(s.as_str())
```

**P772w — exclusão de integrais**: antes desta correcção, a condição era só
`is_large_operator(ch)` — `∫`/`∬`/`∮`/etc. estavam incluídos no conjunto de
"operadores grandes" (`symbols::is_large_operator`, usado também para
spacing/classe), fazendo `∫_0^1` empilhar `0`/`1` verticalmente em modo
bloco, igual a `∑_0^1`. Paridade vanilla (`Limits::for_char_with_class`,
`math/attach.rs` no vanilla): a classe `Large` só empilha (`Display`) se
**não** for um sinal de integral — integrais têm `Limits::Never`
incondicional, scripts sempre ao lado, mesmo em display style. Corrigido
adicionando `symbols::is_integral_char` (ver `math/symbols.md`) como
exclusão explícita, sem alterar `is_large_operator` (continua a incluir
integrais para outros fins — spacing/classe — só a decisão de limites é
afectada).

`Content::MathOp { limits: true, .. }` (P298, override explícito via
`op("...", limits: true)`) não é afectado — continua a empilhar
incondicionalmente quando `self.block`, independentemente do caractere.

---

## Scripts laterais sub+sup partilham a origem x — P799

No braço não-`is_limits` (scripts laterais à direita), quando existem **sub e
sup em simultâneo**, ambos partem da **mesma origem x** — imediatamente à
direita da base, cada um com o seu kern de quadrante — em vez de serem
compostos em sequência horizontal. Paridade vanilla (`scripts.rs`:
`tr_x = br_x = pre_width + base_width + kern`). A largura total do attach é
`base + max(sup + kern_sup, sub + kern_sub)`, não a soma das larguras dos
dois scripts. A geometria vertical existente (offsets fixos
`superscript_shift_up`/`subscript_shift_down`) mantém-se.

Antes de P799, o cursor avançava depois do sup e o sub era colocado a seguir
a ele (scripts lado a lado, e o elemento seguinte da sequência podia
sobrepor-se ao sub) — `x_1^2` extraía como `x21` com posições erradas.

---

## P914 — Deslocamentos Adaptativos de Sub/Sobrescrito e Ajuste de Gap Simultâneo

Em `layout_attach`, os deslocamentos verticais `shift_up` (sobrescrito) e `shift_down` (subscrito) deixam de ser constantes fixas. São computados dinamicamente via `compute_script_shifts`, calculando o máximo entre a constante da fonte, os termos de queda pela base (`sup_drop_max`/`sub_drop_min` para bases não-texto) e os limites das caixas dos scripts (`sup_bottom_min`/`sub_top_max`).

Quando subscrito e sobrescrito coexistem na mesma base (`(sup, sub)`), se o gap vertical entre a parte inferior do sobrescrito e a parte superior do subscrito for inferior a `sub_superscript_gap_min`, `shift_up` e `shift_down` são expandidos simultaneamente para garantir o espaçamento mínimo exigido.

Kerning em 2 alturas de correção: o kern de cada quadrante é calculado pela soma do kern da base com o kern invertido do script nas duas alturas de conexão (topo e base da caixa delimitadora do script), tomando o valor máximo entre ambas.

## P915 — `cramped`: estilo do subscrito forçado, termo alternativo no shift do superscrito

Achado do vanilla (`scripts.rs:100,318-382` — ver `typst-passo-915-relatorio.md`
Fase A, `file:line` dos dois lados): `cramped` **não** é um conceito
simétrico entre sub e superscrito — só o **subscrito** é forçado a cramped
(`style_for_subscript` = `[style_for_superscript, style_cramped()]`,
vanilla `style.rs:333`); o superscrito herda o `cramped` do estilo
**ambiente** (o contexto em que a própria base+attach está a ser desenhada),
sem forçar.

**Propagação** (`entities/layout_types.md` §P915): `layout_attach` deixa de
construir um único `script_style` partilhado por `tl`/`bl`/`sup`/`sub`.
Passa a dois estilos: `top_style` (`tl`/`sup` — `cramped: style.cramped`,
inalterado, herda do `style` recebido) e `bottom_style` (`bl`/`sub` —
`cramped: true`, forçado). Ambos mantêm `size`/`math_script` inalterados
face ao `script_style` anterior — só `cramped` diverge entre os dois.

**Consumo** (`compute_script_shifts`): `cramped` lido de `style.cramped`
— o parâmetro `style: &TextStyle` já recebido por `layout_attach`/passado a
`compute_script_shifts` (o estilo **ambiente**, não `top_style`/
`bottom_style`) — decide, só quando há superscrito presente (`tl.is_some()
|| tr_box.is_some()`), entre `self.constants.superscript_shift_up` e
`self.constants.superscript_shift_up_cramped` (`entities/math_constants.md`
§P915) na primeira linha da fórmula de `shift_up`
(`sup_shift_up = if cramped { ...cramped } else { ...normal }`, vanilla
`scripts.rs:325-330`). Nenhum outro termo da fórmula (`shift_down`, gaps,
`drop_max`) é afectado por `cramped` — confirmado por leitura literal do
vanilla, não por analogia.

**Critério**: uma base com o mesmo superscrito, uma vez com `style.cramped
= false` e outra com `true`, produz `shift_up` diferente (quando a fonte
tiver `superscript_shift_up_cramped` distinto de `superscript_shift_up` —
teste com fonte real, `NewCMMath-Regular.otf`); `shift_down` idêntico nos
dois casos.

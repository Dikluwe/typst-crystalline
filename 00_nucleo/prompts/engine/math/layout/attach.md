# Prompt L0 — `math/layout/attach` — `MathAttach`
Hash do Código: 503b884d

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

## P992 — override explícito via `Content::MathLimitsOverride` (`limits()`/`scripts()`)

**Diferença crucial face a `Content::MathOp { limits, .. }`**: o override de
`MathOp` continua gated por `self.block &&` no exterior do match — só actua
em modo bloco. `limits(body, inline: true)` (o default do vanilla) tem de
empilhar **mesmo em modo inline** (`LimitsElem.inline`, doc vanilla:
"Whether to also force limits in inline equations"), logo o override de
`MathLimitsOverride` tem de ser verificado **antes** do `self.block &&`
exterior, não dentro do mesmo braço `match`.

Reestruturação de `is_limits` (`attach.rs`, início de `layout_attach`):

```rust
let is_limits = match base {
    Content::MathLimitsOverride(e) => e.limits && (e.inline || self.block),
    _ => self.block
        && match base {
            Content::MathIdent(s) | Content::MathText(s) => { /* inalterado */ }
            Content::MathOp(e) => e.limits,
            _ => false,
        },
};
```

Fórmula `e.limits && (e.inline || self.block)` cobre os 3 casos:
- `scripts(body)` (`limits: false`): `false && (..) = false` sempre —
  nunca empilha, independente de `self.block` (paridade `Limits::Never`).
- `limits(body)` (`limits: true, inline: true`, default): `true && (true
  || ..) = true` sempre — empilha mesmo inline (paridade `Limits::Always`).
- `limits(body, inline: false)` (`limits: true, inline: false`): `true &&
  (false || self.block) = self.block` — só empilha em modo bloco, mesma
  regra do caso natural (paridade `Limits::Display`).

O `base` continua a ser layoutado via `self.layout_node(base, style)`
(`layout_attach.rs:30`) — `Content::MathLimitsOverride` é transparente aí
(`math/layout/_comum.md` §P992), logo a caixa visual do `body` é idêntica
à de um `base` não-embrulhado; só o discriminador `is_limits` muda.

**Critério**: `limits(A)_1^2` empilha 1/2 acima/abaixo de "A" mesmo em modo
inline (`self.block = false`); `scripts(sum)_1^2` mantém 1/2 laterais
mesmo em modo bloco (`self.block = true`) — inverso exacto do
comportamento natural de `sum_1^2`/`A_1^2` nesse mesmo modo.

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

## P945 — nota: manter `math_size` honesto, sem mudar factores

Com a introdução de `TextStyle::math_size` (`entities/layout_types.md`
§P945), este módulo passa a **actualizar o campo** ao construir estilos de
descida (scripts/num/den/radicando/índice/limits), para o nível vanilla
correspondente (`style.rs:315-363` do vanilla), **sem alterar o factor de
tamanho actual** — o comportamento geométrico deste módulo fica inalterado
neste passo. Motivo: consumidores abaixo (ex.: uma matriz dentro de um
subscrito — `matrix.md` §P945) dependem do nível correcto para a sua própria
descida. A correcção dos factores deste módulo (ex.: fracção display a ×1.0,
P944 relatório §8.3 item 6) fica para passo dedicado — scope-out registado.


## P959 — shifts verticais dos limites com os 4 termos da tabela MATH

**Medição** (`typst-passo-959` Fase A; achado da auditoria externa
2026-08-04 — distância operador↔limite 6.3-24.0pt no cristalino vs
1.9-3.8pt no vanilla, 38 ocorrências): a fórmula real do vanilla é
`compute_limit_shifts`
(`lab/typst-original/crates/typst-layout/src/math/scripts.rs:290-313`):

```text
t_shift = base.ascent + max(upper_limit_baseline_rise_min,
                            upper_limit_gap_min + t.descent)
b_shift = base.descent + max(lower_limit_baseline_drop_min,
                             lower_limit_gap_min + b.ascent)
```

(shifts baseline-a-baseline; depois o limite é centrado horizontalmente
com a correcção de itálico — `compute_limit_widths`, já coberta pelo
posicionamento x existente). O cristalino usava só os gaps
(`y_sup = base_ascent + upper_gap_min + sup.descent`,
`y_sub = base_descent + lower_gap_min + sub.ascent`) — sem os termos
`max(rise/drop, …)` (scope-out de P944 §4, aqui fechado). Valores reais
(NewCMMath-Book): rise=111du, drop=600du, gap_up=200du, gap_lo=167du
(`entities/math_constants.md` §P959). A variação larga observada
(6.3-24.0pt) vem de a fórmula gap-only depender inteiramente dos extents
de cada limite e da caixa da base (família multi-termo de P952); a banda
estreita do vanilla vem dos pisos `max()`.

**Correcção**: o braço `is_limits` passa a calcular os shifts pela fórmula
do vanilla acima, com os dois campos novos de `MathConstants`
(`upper_limit_baseline_rise_min`, `lower_limit_baseline_drop_min`).
`y_sup = −t_shift`, `y_sub = +b_shift` (baseline do limite acima/abaixo da
baseline da base). O centro vertical da caixa final e a ascent/descent
declarada passam a derivar destes shifts (a tinta dos limites fica a
`t_shift + t.ascent` acima e `b_shift + b.descent` abaixo). Scripts
laterais (braço não-limits, ex.: integrais) **inalterados**.


## P963 — `is_text_like` do vanilla é sobre o FRAGMENTO (extended_shape), não sobre o Content

**Medição** (`typst-passo-963` Fase A; auditoria externa 3ª ronda: limite
superior ~11.4pt mediana vs 2.37pt vanilla pós-P959): isolado
`$ integral_0^1 $` em bloco — o sup lateral do cristalino ficava ~2.8pt
ABAIXO da baseline da base; o vanilla coloca-o ~12.9pt ACIMA. Causa: o
`is_text_like` de `compute_script_shifts` era
`matches!(base, Content::MathIdent(_) | Content::MathText(_))` — verdadeiro
para `∫`. No vanilla (`fragment/mod.rs:129-135`) `is_text_like` de um glifo
é `!extended_shape`: um operador **esticado** (variante de Display ou
assembly — `FrameItem::Glyph` no cristalino) NÃO é text-like, e o termo
`base_ascent − superscript_baseline_drop_max` do `shift_up` (e
`base_descent + subscript_baseline_drop_min` do `shift_down`) **aplica-se**.
Com o termo zero por engano, o sup lateral de um operador esticado ficava
só com `superscript_shift_up` (~4pt) em vez de ~`ascent − drop_max`
(~13pt para integral.v1) — daí o "limite superior" lido como 5× errado
pela auditoria (era o sup lateral de integrais, não o limite empilhado de
P959, que ficou correcto).

**Correcção**: em `layout_attach`, `is_text_like` passa a ser falso quando
a caixa da base contém `FrameItem::Glyph` (variante esticada ou assembly —
o equivalente cristalino de `extended_shape`). Bases de texto/glifo simples
(itálico de 1 letra, operador não esticado inline) mantêm o termo a zero —
guardas P914/P915 cobrem. Valores medidos (NewCMMath-Book):
SuperscriptShiftUp=363, SuperscriptBaselineDropMax=250,
SubscriptBaselineDropMin=200 (du); `integral.v1` ink: +1361/−861du.

## P971 — termo de itálico do vanilla no subscrito pós-fixado

**Data:** 2026-08-05 · **Gate:** Fase B (método novo no trait
`FontMetrics`) **confirmada pelo dono em 2026-08-05** ("Continue" após
`typst-passo-971-faseA.md`).

**Medição que motiva** (`typst-passo-971` Fase A; auditoria externa
2026-08-05, achado 9.2): `$ integral_a^b f(x) dif x $` — a partir da aresta
esquerda do ∫, o vanilla coloca o subscrito "a" a **6.04pt** e o
sobrescrito "b" a **10.99pt** (colunas diferentes, seguindo a inclinação);
o cristalino coloca ambos a **10.99pt** (mesma coluna). Distâncias
verticais já batem nos dois lados.

**Mecanismo real do vanilla** (lido e confirmado,
`lab/typst-original/crates/typst-layout/src/math/scripts.rs:220-228`,
`compute_post_script_widths`): o kern do subscrito pós-fixado recebe o
termo **`− base.italics_correction()`** ("the base's bounding box already
accounts for its italic correction"); o do sobrescrito fica inalterado.
Aplica-se a **todas** as bases (para a maioria IC=0; para itálicos e
operadores inclinados, IC>0). Não é o caminho de limites empilhados — o ∫
não tem limites móveis por defeito (o cristalino exclui integrais de
`is_limits` correctamente); é o braço de scripts laterais.

**Valores medidos na fonte** (fontTools, NewCMMath-Book, upem 1000):
`MathItalicsCorrectionInfo`: `integral`=180du, `integral.v1` (a variante de
display usada em bloco)=**450du** (= 4.95pt a 11pt — exactamente o delta
medido 10.99−6.04). Kerns MATH dos glifos de integral: **nenhum** (a tabela
MathKernInfo não tem entradas para `integral*`) — todo o efeito vem do
termo de IC. **Proxy refutado por medição**: `ink_xMax − advance` =
**−56du** para `integral` e `integral.v1` — nem o sinal bate com a IC
declarada (+180/+450) — não há caminho honesto via `glyph_ink_bounds`
(P952); a IC é um parâmetro de posicionamento da tabela MATH, não uma
propriedade da tinta.

**O que falta no cristalino** (`attach.rs`, braço de scripts,
`x = scripts_x + kern_sub`): o termo `− IC` do subscrito. A IC tem de ser
lida **do glifo final da base** (a variante esticada `integral.v1` em
display — 450du — não o glifo base `integral` — 180du), pelo que o acessor
tem de ser por **glyph id** (como `glyph_ink_bounds`), não por char:

```rust
// FontMetrics (novo método — CONTRATO, daí o gate):
fn italics_correction(&self, glyph_id: u16, size: Pt, style: &TextStyle) -> Pt
```

e o `attach.rs` precisa de extrair o glyph id real da `base_box`
(`FrameItem::Glyph` já o carrega, P906) — para bases esticadas é o id da
variante/assembly, para bases simples o do glifo resolvido. Subscrito:
`kern_sub − italics_correction(base_gid)`; sobrescrito inalterado. Guarda
de não-regressão: bases com IC=0 (a maioria) ficam bit-a-bit iguais;
distâncias verticais (P959/P914) não são tocadas.

**Escopo implementado (registado à medida)**: o termo aplica-se quando a
`base_box` contém `FrameItem::Glyph` — bases esticadas, o caso do achado
(`∫`/`∮` em display usam a variante v1, IC=450du). O método tem default
`Pt(0.0)` no trait — métricas sintéticas (FixedMetrics) e glifos sem
entrada na tabela ficam inalterados bit-a-bit.

**Residual registado**: bases `Text` de um só carácter com IC>0 — `∫`
inline (glifo base, IC=180du) e letras itálicas com IC própria — ficam
sem o termo neste passo: o item `Text` não carrega glyph id (é
pré-shaping) e o trait não tem resolvedor char→gid; cobri-las exige um
segundo método no trait (contrato adicional, a avaliar em passo próprio
com medição de impacto — as ICs de letras são pequenas, 0–30du).

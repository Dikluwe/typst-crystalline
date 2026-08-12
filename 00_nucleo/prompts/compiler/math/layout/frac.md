# Prompt L0 — `math/layout/frac` — `MathFrac`
Hash do Código: 9b4e958c

**Camada**: L1 · **Alvo**: `01_core/src/compiler/math/layout/frac.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathFrac` — fracções. Consome `fraction_rule_thickness` + `fraction_num_gap` +
`fraction_denom_gap` de `MathConstants`. Baseline x-height própria (P919, ver abaixo — deixou de
usar `MathLayouter::apply_axis_offset`); test regressão `frac_com_axis_height_nao_regride`
(`tests.rs:520+`).

**Critério**: `MathFrac { num: a, den: b }` → sem `[` nos items.

## P905 — offsets Y usavam convenção "topo do box" em vez de baseline-relativa

**Achado** (`typst-passo-905-relatorio.md`, materialização originalmente descrevia o sintoma
como restrito a `/` em argumentos de chamada de função — Fase A confirmou ser mais geral: **todo**
`/` em modo matemático, incluindo fracções soltas fora de qualquer chamada, ex. `$ a/b $`):
confirmado por PDF real (`pdftoppm`) que a linha de fracção atravessava o denominador "a meio da
altura" sempre que o denominador tinha glifo com ascendente alto (`b`, `y`, ...) — o numerador
parecia normal (só o denominador sobrepunha visivelmente), consistente com o sintoma catalogado em
P899 Parte B ("só o primeiro operando aparece, com um glifo estranho por baixo").

**Causa**: `layout_frac` construía `num_y=0.0` e `den_y`/`rule_local_y` a partir de
`num_box.height()` — assumindo (implicitamente, nunca declarado no código) a convenção "`y=0` =
topo da caixa". A convenção real, já confirmada e documentada em P901 (`root.md` acima) via leitura
de `hconcat_spaced`/`layout_equation` (`mod.rs`) e replicada correctamente em `attach.rs`, é
"`y=0` = **baseline própria** da `MathBox`, `y` cresce para baixo" — `frac.rs` nunca tinha sido
auditado contra essa convenção desde a sua introdução original (Passo 9.8/136-137, muito antes de
P800/P901 a terem confirmado). O único teste de posição pré-existente
(`math_frac_numerador_acima_denominador`) só verificava **ordem** (`num.y < den.y`), que continua
verdadeira mesmo com o bug (`0 < positivo`) — por isso a regressão nunca foi apanhada.

**Correcção**: `rule_local_y = 0.0` (a linha fica exactamente na baseline própria do `MathBox` da
fracção — por construção, `ascent`/`descent` já medem a partir daí); `num_y =
-(num_box.descent + gap + rule_thickness/2)` (a baseline do numerador sobe o suficiente para o seu
descent parar `gap` acima do topo da linha); `den_y = gap + rule_thickness/2 + den_box.ascent` (a
baseline do denominador desce o suficiente para o seu ascent parar `gap` abaixo do fundo da linha).
`ascent`/`descent` do `MathBox` resultante **não mudaram** (fórmula já estava correcta,
independente do bug de offset dos items) — por isso `root.rs`/`layout_stretchy_delimiter` já
recebiam a altura correcta para dimensionar `sqrt(frac(...))`, mas o símbolo `√` não escala
(bbox idêntico a `sqrt(x)` de um único carácter) — **achado novo, separado, fora de âmbito de
P905** (ver relatório, candidato a passo dedicado sobre `layout_stretchy_delimiter`).

Testes de regressão (magnitude do gap, não só ordem):
`p905_frac_numerador_tem_gap_acima_da_linha`,
`p905_frac_denominador_tem_gap_abaixo_da_linha_nao_sobrepoe`,
`p905_sqrt_de_fraccao_com_variaveis_nao_produz_saida_malformada`.

## P915 — denominador é cramped, numerador não

Achado do vanilla (`style_for_denominator` = `[style_for_numerator,
style_cramped()]`, `style.rs:362`, vs. `style_for_numerator`, `style.rs:343`,
sem `style_cramped()`) — ver `entities/layout_types.md` §P915,
`typst-passo-915-relatorio.md` Fase A. O `sub_style` único, hoje partilhado
por numerador e denominador (`size: style.size * script_percent_scale_down`,
resto herdado de `style.clone()`), passa a dois: `num_style` (`cramped:
style.cramped` — herda do estilo recebido, inalterado) e `den_style`
(`cramped: true` — forçado, independentemente do estilo recebido). `size`
continua igual nos dois (a redução de tamanho de numerador/denominador não
é o que este passo altera — só `cramped`).

**Efeito prático**: um superscrito dentro do denominador de uma fracção
(ex.: `frac(a, b^2)`) usa `superscript_shift_up_cramped` (via `attach.md`
§P915); o mesmo superscrito no numerador usa `superscript_shift_up` normal.

## P919 — a barra fica fixa a `axis_height` da baseline (fix próprio, não `apply_axis_offset`)

**Achado** (`typst-passo-919-relatorio.md` Fase A, vanilla `fraction.rs:66,69`: `baseline =
line_pos.y + axis; frame.set_baseline(baseline)`): a barra da fracção deve ficar fixa a
`axis_height` acima da baseline do composto — **por construção**, não pelo "meio do
`ascent`/`descent` do box". `apply_axis_offset` (`_comum.md` §P919) usa a fórmula genérica
`shift = axis_pt - (ascent-descent)/2`, que só coincide com o resultado correcto quando
numerador e denominador têm alturas simétricas — diverge em casos assimétricos (`frac(a, b^2)`,
denominador mais alto por causa do subscrito). `layout_frac` **deixa de chamar
`apply_axis_offset`** e passa a aplicar directamente: todos os `items` deslocados por `-axis_pt`
em Y (a barra, já fixada em `rule_local_y=0` por construção — §P905 acima —, passa a `-axis_pt`);
`ascent += axis_pt`; `descent -= axis_pt`. `axis_pt = self.constants.to_pt(self.constants.
axis_height, style.size).val()`, mesmo padrão de conversão já usado no resto do ficheiro.

**Medição que motivou a correcção** (`mutool trace`, fonte real embutida, `frac(a,b)` ao lado de
texto): a barra estava a 0.046pt da baseline partilhada — praticamente zero, quando devia estar a
`axis_height` de distância (bug de omissão em `apply_axis_offset`, nunca deslocava `items` — ver
`_comum.md` §P919).

**Critério de regressão**: `x + frac(a,b)` — a barra da fracção deve estar a `axis_pt` (não ~0)
de distância vertical da baseline partilhada com `x`; testável tanto com `MathConstants::
fallback()` (posição sintética) como com fonte real (`mutool trace`, ground-truth do vanilla).

## P920 — `gap` (numerador/denominador) usa a fórmula real do vanilla, não `fraction_num_gap` directo

**Achado** (`typst-passo-918-relatorio.md` achado lateral: "`fraction_denom_gap` parece não ter
consumidor — `frac.rs` usa `fraction_num_gap` para os dois lados"; `typst-passo-920-relatorio.md`
Fase A: o problema é mais fundo que um campo por ler — **a fórmula toda diverge do vanilla**).
Hoje (`gap = to_pt(fraction_num_gap, size)`, usado directamente como termo aditivo em `ascent`/
`num_y`/`descent`/`den_y`): trata `fraction_num_gap` como o gap em si.

**Fórmula real** (`typst-layout/src/math/fraction.rs:30-53`, vanilla): o gap não é uma constante
directa — é computado a partir da tinta real do numerador/denominador e da posição do eixo, com
`fraction_num_gap`/`fraction_denom_gap` a servirem só de **piso mínimo**:

```
num_gap   = (fraction_numerator_shift_up   - axis - thickness/2 - num.descent() ).max(fraction_num_gap)
denom_gap = (fraction_denominator_shift_down + axis - thickness/2 - denom.ascent()).max(fraction_denom_gap)
```

`fraction_numerator_shift_up`/`fraction_denominator_shift_down` são campos novos em
`MathConstants` (`entities/math_constants.md` §P920) — o cristalino não os tinha. `num.descent()`/
`denom.ascent()` já estão disponíveis (`num_box.descent`/`den_box.ascent`, já lidos por este
ficheiro). A fórmula usa dois gaps DISTINTOS e geometricamente correctos (dependentes da tinta de
cada lado), não o mesmo valor duplicado nos dois. **Exacto sinal/composição com `axis_pt` (já
introduzido em P919, ver acima) a confirmar na Fase B** — a fórmula do vanilla é anterior à
correcção P919 nesta base de código; a integração dos dois (gap por ink-extent + shift fixo para
o eixo) precisa de ser verificada por medição real (`mutool trace`/`fontTools`), não assumida.

**Critério de regressão**: `frac(a,b)` com `a`/`b` de alturas MUITO diferentes (ex.: `frac(x,
y^2)` vs `frac(x^2, y)`) — os gaps de cada lado devem divergir de forma mensurável (não mais o
mesmo valor espelhado); `frac(a,b)` "normal" (glifos de altura típica) — resultado próximo do
actual, dentro do que a fórmula real prevê (não necessariamente idêntico bit-a-bit, já que a
fórmula muda, mas sem salto grande se a fonte não tiver valores extremos).

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

## P952 — numerador/denominador com descida por nível (Display→Text ×1.0)

**Medição** (`typst-passo-952` Fase A): a regra de espaçamento entre blocos de
equação já batia com o vanilla (1.2em colapsado,
`flow/distribute.rs:185-199`); os deltas sistemáticos de +7 a +17pt entre
equações multi-linha do documento de 30 secções decompõem-se em extents de
conteúdo — e o maior componente é a **fracção display mais pequena**:
`$ lr((a/b)) $` (7 gaps na secção 22) com +8.1 a +9.7pt por gap, porque
`num_style`/`den_style` reduziam incondicionalmente
`size × script_percent_scale_down` (P915) — `Display→Text` do vanilla é
**factor 1.0** (`style.rs:343-363`: `style_for_numerator`,
`style_for_denominator = numerator + cramped`).

**Correcção** (supera o scope-out registado em P944 §8.3.6 — aprovado pelo
dono em P952): `num_style`/`den_style` passam a usar a descida por nível de
`_comum.md` §P945 — `denominator_style` (denominador, sempre `cramped`) e o
novo helper simétrico `numerator_style` (mesma descida, `cramped` herdado do
ambiente, nunca forçado — vanilla `style.rs:343-350`), ambos com a tabela:
`Display→Text` ×1.0, `Text→Script` ×`script_percent_scale_down`,
`Script→ScriptScript` ×`sscript/script`, `ScriptScript→ScriptScript` ×1.0.
Os gaps/shifts de P920 (`fraction_numerator_shift_up` etc.) são contra o
tamanho do estilo — seguem automaticamente.

**Impacto esperado e guards**: fracções display ficam ~45% maiores (parity
medida: `a/b` display passa de ~16pt para ~26pt de altura, como o vanilla);
fracções em contexto `Text` (inline) e `Script` mantêm o comportamento actual
(×0.7 — P915/P923 continuam correctos nesses contextos); `root.rs` (radicando
`cramped` mesmo nível, índice `ScriptScript`) **não** muda neste passo —
`sqrt` já estava visualmente próximo do vanilla no documento de 30 secções
(P944 §8.2). Testes existentes que assumem ×0.7 em Display serão revistos caso
a caso (o valor correcto passa a ser o do vanilla, não o anterior).

## P972 — posicionamento do numerador/denominador cobre TODOS os tipos de item (via `offset_item`)

**Data:** 2026-08-05

**Medição que motiva** (`typst-passo-972` Fase A; auditoria externa
2026-08-05, achado 9.3): `$ (n(n+1)) / 2 $` — no cristalino os parênteses
do numerador ficam **3.52pt abaixo** da baseline dos dígitos que envolvem
(no vanilla todos partilham a baseline). Inline e bloco fora de fracção:
alinhados. Só o contexto de fracção quebrava.

**Causa** (confirmada por instrumentação + sonda end-to-end): os ciclos de
posicionamento do numerador e do denominador em `layout_frac` usavam um
`match` que só deslocava `FrameItem::Text`/`TextShaped` — o braço `_ => {}`
deixava `FrameItem::Glyph` (delimitadores stretchy emitidos como glifo,
P906/P952b) **sem o offset** `num_y`/`den_y`. Os parênteses ficavam na
posição relativa da caixa (y≈0) enquanto os dígitos subiam `num_y` =
−(descent + num_gap + thickness/2) = −3.52pt medido — exactamente o delta
observado. Com `FixedMetrics`/sem fontes reais o parêntese sai como `Text`
(mapeamento `glyph_to_char` disponível), pelo que o bug **só se manifesta
com a fonte real** (variante sem mapeamento Unicode → braço Glyph) —
razão pela qual nenhum teste de unidade o apanhou. Também afectaria
`FrameItem::Line` (ex.: overline de um `sqrt` aninhado no numerador) —
mesmo braço `_`.

**Correcção**: os dois ciclos passam a usar `offset_item(item, dx, dy)`
(que cobre todos os tipos — já era usado no map final do eixo, P919), em
vez do `match` restrito a Text/TextShaped. Comportamento inalterado para
Text/TextShaped (mesma fórmula); Glyph/Line/Shape passam a ser deslocados
como sempre deviam ter sido.

## P990-A — constantes Display quando `math_size == Display`

`layout_frac` selecciona shifts/pisos por `style.math_size` (vanilla
`fraction.rs:33-52`): Display ⇒ os 4 campos novos
(`entities/math_constants.md` §P990); caso contrário ⇒ os campos de texto
de P920, inalterados. Inline (`$...$` em texto corrido = Text) não muda.

## P990-B — `FRAC_PADDING = 0.1em` (largura da fracção e barra só com `line_width`)

**Medição** (achado §8.6 da auditoria + investigação P990-B): o "espaço
ausente após − em expoente" NÃO é espaçamento de classes — é o padding
horizontal que o vanilla adiciona à largura de TODA a fracção vertical:
`FRAC_PADDING = Em::new(0.1)` (`math/frac.rs:9`,
`FractionItem::create(..., FRAC_PADDING, ...)` em `ir/resolve.rs:748`),
`width = max(num, denom) + 2 × padding` (`fraction.rs:56` com linha,
`:104` sem linha), resolvido ao `style.size` do CONTEXTO da fracção. A
barra desenha-se só com `line_width = max(num, denom)`, centrada
(`fraction.rs:60-63`) — não de margem a margem da caixa.

**Correcção** em `layout_frac`: `line_width = max(num, den)`;
`padding = 0.1 × style.size`; `width = line_width + 2 × padding`;
`num_x`/`den_x` mantêm `(width − box.width)/2` (o lado mais largo fica a
`padding` da margem); barra de `(width − line_width)/2` a
`(width + line_width)/2`. Aplica-se a todas as fracções (o item seguinte
também afasta 0.1em — era o gap de 0.77pt medido no expoente a 7.7pt).

**Critério conjunto (A+B)**: `ρ/ε₀` (Display) com gaps ≈ vanilla
(≥ piso 120du=1.32pt); `$ e^(-t^2/2) $` com o numerador do expoente a
começar 0.77pt após o advance do −; a barra centrada com `line_width`.

# Prompt L0 — `math/layout/root` — `MathRoot`
Hash do Código: d151b46e

**Camada**: L1 · **Alvo**: `01_core/src/compiler/math/layout/root.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathRoot` — sqrt + n-th roots. Consome `radical_vertical_gap` +
`radical_rule_thickness` de `MathConstants`. **P919**: deixou de chamar
`MathLayouter::apply_axis_offset` (ver `_comum.md` §P919 e secção abaixo) — a baseline própria já
é a baseline do composto, sem ajuste de eixo. Test regressão `sqrt_com_axis_height_nao_regride`
(`tests.rs:520+`).

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

## P915 — radicando e índice são ambos cramped

## P1132j — topo do surd coincide com o topo da barra

O frame do símbolo `√` é posicionado, como em `radical.rs::sqrt_pos`, no
topo da barra: em coordenadas baseline-relativas,
`sym_dy = overline_y - line_thickness/2 + radical_box.ascent`. Ancorá-lo em
`overline_y` (centro da barra) deslocava somente o surd para baixo por meia
`radical_rule_thickness`, mantendo barra e radicando corretos. A espessura
vem da constante OpenType MATH ativa; não há correção empírica.

Achado do vanilla (`resolve_root`, `resolve.rs:1220-1249` — ver
`entities/layout_types.md` §P915, `typst-passo-915-relatorio.md` Fase A):
"o radicando é resolvido em estilo cramped, e o índice em tamanho
scriptscript e estilo cramped" (comentário literal do vanilla, confirmado
pelo código: `cramped_styles = chain(styles, style_cramped())` aplicado ao
radicando; índice usa `cramped_styles.chain(sscript)`, i.e., cramped **e**
scriptscript, não um ou outro).

`layout_root` passa a usar `radicand_style` (`cramped: true`, resto igual a
`style`) em vez de `style` directo para o radicando (linha onde hoje chama
`self.layout_node(radicand, style)`); o `script_style` do índice
(`root(n, x)`) ganha `cramped: true` (já reduzia `size` via
`script_percent_scale_down` — isso não muda, só `cramped` é acrescentado).

**Efeito prático**: um superscrito dentro do radicando (`sqrt(a^2)`) ou do
índice (`root(n^2, x)`) usa `superscript_shift_up_cramped`
(`attach.md` §P915) em vez do valor normal.

## P919 — remoção da chamada a `apply_axis_offset` (nunca devia estar aqui)

**Achado** (`typst-passo-919-relatorio.md` Fase A, vanilla `radical.rs:110`:
`frame.set_baseline(ascent)` — **sem** termo de `axis_height`): ao contrário de `frac`/`cases`/
`matrix`, o radical **não** centra no eixo matemático — a sua baseline é simplesmente o próprio
`ascent`. `layout_root` já constrói `total_ascent`/`total_descent`/`items` correctamente
relativos à baseline própria (§P901 acima) — chamar `apply_axis_offset` no fim (linha final do
método, antes de P919) nunca teve fundamento no vanilla; só não quebrava nada porque o bug de
omissão em `apply_axis_offset` (`_comum.md` §P919) nunca deslocava `items`. Confirmado também
empiricamente: `mutool trace` de `x + sqrt(a) + y` mostra `x`/`a`/`y` já exactamente ao mesmo Y
sem qualquer correcção. A chamada final `self.apply_axis_offset(result, style.size)` é removida —
`layout_root` devolve `result` directamente.

**Critério de regressão**: `x + sqrt(a) + y` — todos os três elementos partilham exactamente a
mesma baseline (mesmo Y), antes e depois desta mudança (não deve haver diferença — é uma remoção
de um no-op, não uma correcção de comportamento visível).

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

## P970 — índice de raiz: tamanho ScriptScript absoluto (implementado) + posição real do vanilla (pendente de gate)

**Data:** 2026-08-05

**Medição que motiva** (`typst-passo-970` Fase A; auditoria externa
2026-08-05, achado 9.1): `root(3, x)` — índice cristalino a 7.7pt (70%),
2.2pt do topo do `√` ("grande e quase no topo, flutuando"); vanilla 5.5pt
(50%), 8.4pt do topo ("encaixado no vinco").

### Parte 1 — tamanho (fluxo contínuo ADR-0127, implementada neste passo)

O vanilla fixa o índice em `MathSize::ScriptScript` **absoluto**
(`resolve.rs:1235-1239`) e o factor de tamanho é aplicado de forma
**absoluta**, não cumulativa — `TextSize::resolve`
(`lab/typst-original/crates/typst-library/src/text/mod.rs:1139-1152`):
`Display|Text → ×1.0`, `Script → ×script_percent`,
`ScriptScript → ×script_script_percent`, sempre sobre o tamanho de texto
declarado. P945 fixou o campo `math_size: ScriptScript` mas deixou o factor
em scope-out ("fica para passo dedicado") — este é esse passo.

Tabela do factor sobre `style.size` (mesma forma dos helpers P945/P952):

| `style.math_size` | factor |
|---|---|
| `Display` / `Text` | ×`script_script_percent_scale_down` |
| `Script` | ×`sscript/script` |
| `ScriptScript` | ×1.0 |

Medido na fonte (NewCMMath-Book): `ScriptPercentScaleDown=70`,
`ScriptScriptPercentScaleDown=50` ⇒ índice a 5.5pt sobre base de 11pt
(era 7.7pt). `cramped: true` mantido (P915).

### Parte 2 — posição e deslocamento horizontal (gate confirmado 2026-08-05, implementada)

Fórmulas reais do vanilla, lidas e confirmadas
(`lab/typst-original/crates/typst-layout/src/math/radical.rs:86-96,113-114`):

```
sqrt_offset = RadicalKernBeforeDegree + index.width + RadicalKernAfterDegree
shift_up    = RadicalDegreeBottomRaisePercent × (inner_ascent − descent)
              + index.descent          // inner_ascent inclui RadicalExtraAscender
index_x     = −min(sqrt_offset, 0) + RadicalKernBeforeDegree
index_baseline = −shift_up             // convenção baseline=0 cristalina
sqrt_x      = max(sqrt_offset, 0)      // o √ e o radicando deslocam-se à
                                       // direita para dar lugar ao índice
ascent      = max(inner_ascent, shift_up + index.ascent)
```

Valores reais medidos na fonte (fontTools, NewCMMath-Book, upem 1000):
`RadicalKernBeforeDegree=278du`, `RadicalKernAfterDegree=−556du`,
`RadicalDegreeBottomRaisePercent=60%`, `RadicalExtraAscender=48du`.

**Gate**: as quatro constantes foram adicionadas a `MathConstants`
(campos em entidade ⇒ paragem obrigatória ADR-0127 ponto 1) — **confirmado
pelo dono em 2026-08-05**; ver `entities/math_constants.md` §P970.

**Decisões de escopo da implementação** (medidas/registadas, não
presumidas):

- A convenção baseline-relativa cristalina traduz `index_pos.y` do vanilla
  para **baseline do índice em `y = −shift_up`** (a posição do vanilla é
  top-anchored: `index_pos.y = ascent − index.ascent − shift_up` ⇒
  baseline = `index_pos.y + index.ascent = ascent − shift_up` ⇒ relativo à
  baseline do composto: `−shift_up`).
- `descent` da fórmula é a profundidade do surd esticado
  (`sqrt.height − sqrt_ascent`, `radical.rs:79`), computada da
  `radical_box` cristalina (`ascent + descent − total_ascent`). O
  `total_descent` declarado do composto **não** muda neste passo (o vanilla
  usa a profundidade do surd; o cristalino usa `radicand.descent` —
  divergência pré-existente de P919, fora do escopo do achado 9.1,
  registada aqui).
- O ajuste de gap do TeXbook p443 item 11 (`radical.rs:76`:
  `gap = max(gap, (sqrt.height − thickness − radicand.height + gap)/2)`)
  **não** é portado neste passo — afecta `sqrt(x)` sem índice (blast
  radius maior que o achado) e fica registado como residual a medir.
- `ascent` do composto com índice: `max(inner_ascent, shift_up +
  index.ascent)` (vanilla `radical.rs:84,95`); sem índice fica inalterado
  (o `extra_ascender` no caso sem índice é outra fatia do mesmo residual).


## P974 — altura do `√`: short_fall=0 para o radical + gap de Display

**Data:** 2026-08-05

**Medição que motiva** (auditoria externa v5, 2026-08-05; reproduzida por
render 600 DPI com medição de tinta por pixel): `√(a²+b²)` — símbolo com
**10.80pt** de tinta no cristalino contra **13.08pt** no vanilla (o vanilla
usa a variante `radical.v1`, 1201du); `³√x`/`√x` — 10.80pt nos dois lados
(glifo base, 1001du). O cristalino desenhava o glifo base em todos os
casos — "altura fixa".

**Causa dupla confirmada** (Fase A, com instrumentação do alvo real:
`sqrt(a²+b²)` dava alvo 936du → curto-circuito 836du → base; só a
combinação das duas correcções empurra o alvo para lá do glifo base):

1. **Gap errado em Display** (`radical.rs:32-36` do vanilla): em Display o
   vanilla usa `radical_display_style_vertical_gap` (**148du** em
   NewCMMath-Book), não `radical_vertical_gap` (50du). O cristalino usava
   sempre o segundo. +98du ao alvo em Display. **Requer campo novo em
   `MathConstants` ⇒ gate ADR-0127** (ver `entities/math_constants.md`
   §P974).
2. **Short-fall não se aplica ao radical** (`resolve.rs:1246` do vanilla:
   `StretchInfo::new(Rel::one(), Em::zero())` — short_fall **zero** para o
   √; a assinatura `StretchInfo::new(target, short_fall)` está em
   `item.rs:1241`; a selecção `target − short_fall ≤ advance ⇒ base` está
   em `glyph.rs:265-271`). O cristalino subtraía `DELIM_SHORT_FALL` (0.1em,
   P912 — correcto para delimitadores de `lr`/`mat`, **não** para o
   radical) em `layout_stretchy_delimiter`. +100du ao limiar efectivo.

**Correcção**: o caminho do radical deixa de aplicar o short-fall
(Parte A — fluxo contínuo, mudança interna sem mudar comportamento nos
casos actuais) e passa a escolher o gap por nível
(`Display → radical_display_style_vertical_gap`, senão
`radical_vertical_gap`) (Parte B — após gate do campo novo).

**Verificação da combinação** (targets a 11pt, glifo base = 1001du):
`√x`: 453+48+148 = 649du → base (ambos). `√(a²+b²)`: 838+48+148 = 1034du
> 1001du → v1 (ambos). Só com as DUAS correcções o cristalino cruza o
limiar no caso grande — cada uma sozinha foi verificada insuficiente nas
contas da Fase A.

## P1130 — TeXbook p443 item 11: redistribuição do excesso de altura do √ no gap (fluxo contínuo, ADR-0127)

**Data:** 2026-08-21 · **Gate:** correcção de fórmula interna / paridade
com o vanilla — fluxo contínuo (sem paragem), per ADR-0127.

**Achado que motiva** (nota externa, secção 1; `typst-passo-1130.md`):
símbolo `√` quase não se move (`+0.26pt`), mas o **radicando** sobe em
relação à barra — o espaço barra→radicando encolhe `-1.18pt` (`sqrt`
simples) e `-2.19pt` (`root` cúbica). Na raiz cúbica isto desalinha
verticalmente o índice `3`.

**Causa confirmada por leitura directa** (`lab/typst-original/crates/
typst-layout/src/math/radical.rs:73-76`): o vanilla selecciona a
variante do `√` a partir de um alvo (`radicand.height() + thickness +
gap`) contra uma tabela de tamanhos **discretos** — a variante
seleccionada é quase sempre **mais alta** que o alvo exacto (raramente
bate em cheio). O TeXbook, p443, item 11, manda redistribuir esse
excesso de volta para o gap:

```
gap = max(gap, (sqrt.height() - thickness - radicand.height() + gap) / 2)
```

Já estava **registado como resíduo não portado** neste ficheiro, §P970
Parte 2, "Decisões de escopo": *"O ajuste de gap do TeXbook p443 item 11
… não é portado neste passo — afecta `sqrt(x)` sem índice … fica
registado como residual a medir"*. O Passo 1130 é essa medição: o
cristalino usa sempre o `gap` mínimo, nunca redistribui o excesso — daí
o espaço barra→radicando ficar sistematicamente mais apertado do que o
vanilla sempre que a variante escolhida (ou o glifo base, no caminho de
fallback) é mais alta que o alvo exacto.

**Cascata sobre o índice (§3 do L0)**: `sqrt_ascent`, `descent_surd` e
`inner_ascent` dependem todos do `gap` já corrigido (linhas seguintes de
`layout_root`); `shift_up` do índice (`root.md` §P970 Parte 2) depende de
`inner_ascent`/`descent_surd`. O desalinhamento do índice em `root(3,x)`
é **consequência** do gap por corrigir, não uma causa própria — confirma-
se corrigindo só o gap e revalidando a posição do índice, sem tocar na
fórmula de `shift_up`.

**Correcção**: em `layout_root`, imediatamente após `radical_box` ser
computado (a variante/glifo já escolhido) e antes de `sqrt_ascent`, o
`gap` é reatribuído:

```rust
let sqrt_height = radical_box.ascent + radical_box.descent;
let radicand_height = rad_box.ascent + rad_box.descent;
let gap = gap.max((sqrt_height - line_thickness - radicand_height + gap) / 2.0);
```

Tradução directa da fórmula do vanilla para os nomes já usados neste
ficheiro (`radical_box.ascent + radical_box.descent` já era usado, antes
deste passo, para computar `descent_surd` — é a mesma medida de
`sqrt.height()`). O `gap` usado para **seleccionar** a variante
(`min_height_du`, antes de `radical_box` existir) fica intocado — a
ordem espelha o vanilla (`radical.rs:27-42` selecciona com o gap
original; só `:76`, depois de escolhida a variante, redistribui).
`overline_y` (mais abaixo na função) lê o `gap` já corrigido por
sombreamento (`let gap = …` na mesma scope) — sem alteração adicional de
código.

**Sem overshoot, sem mudança**: quando a variante escolhida bate no alvo
exacto (`sqrt.height() == radicand.height() + thickness + gap`), a
fórmula devolve o `gap` original inalterado — guarda de não-regressão
para os testes existentes baseados em `FixedMetrics` (que só usam o
glifo base de fallback, tipicamente mais curto que o alvo, logo já sem
overshoot antes deste passo).

**Oráculo**: `radical_gap_redistribution(gap, sqrt_height, thickness,
radicand_height)` adicionado a `01_core/src/testing/math_oracle.rs`
(transcrição literal de `radical.rs:76`) — ver `testing/math_oracle.md`.

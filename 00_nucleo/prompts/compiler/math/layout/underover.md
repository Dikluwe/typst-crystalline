# Prompt L0 — `math/layout/underover` — `MathUnderover`
Hash do Código: 3bddf2b3

**Camada**: L1 · **Alvo**: `01_core/src/compiler/math/layout/underover.rs`
**Origem**: fatiado de `math/layout/mod.rs` em **P909**, completando o padrão de fatiamento
iniciado em P314 (ADR-0104) para `frac`/`root`/`stretchy`/`assembly`/`matrix`/`cases`/
`delimited`. `layout_underover` foi adicionado em **P297**, depois de P314, e nunca tinha sido
movido. Núcleo partilhado: ver `math/layout/_comum.md`.

---

`MathUnderover` — empilha `over` (topo), `base` (meio), `under` (fundo). Cada `Option` é skipped
se `None`. Width final = max das 3 partes; cada parte centrada horizontalmente. Agregação
cristalina per ADR-0054 graded — vanilla typst fragmenta em 12 elementos (`underbrace`/
`overbrace`/`underbracket`/etc., dispatch em `eval/math.rs`); cristalino unifica num único
variant `MathUnderover`.

## P906 — over/under de 1 carácter esticam para cobrir a largura da base

**Achado** (`typst-passo-899-relatorio.md`, Parte C, motivador original): `layout_underover`
centrava `over`/`under` no seu tamanho natural (`let dx = (w - ob.width) / 2.0`), sem esticar para
cobrir a largura da base — `underbrace(a+b+c)` produzia uma chave do tamanho de 1 carácter sobre
uma expressão larga.

**Correcção**: antes de `self.layout_node(c, style)` para `over`/`under`, um guard partilhado com
`layout_accent` (`layout_stretchy_or_node`, definido em `mod.rs` — ver `_comum.md` §guard
partilhado): se o conteúdo é `Content::MathText(s)` com exactamente 1 carácter, calcula
`min_width_du = base_box.width * self.constants.upem / style.size.val().max(0.001)` e chama
`layout_stretchy_glyph_horizontal` (`stretchy.rs`, ver `stretchy.md` §P906) em vez de
`layout_node`. Para qualquer outro conteúdo — incluindo a **anotação** de `underbrace`/`overbrace`
(texto normal, nunca deve esticar) — comportamento **inalterado**.

**Alcance confirmado com o dono**: chaves/colchetes (`⏟`/`⏞`/`⎵`/`⎴`), o caso que a fonte suporta
com dados reais.

**Critério de regressão**: `underover(a+b+c, over: Content::MathText("⏞"))` — a largura do
`MathBox` do `over` é maior que a de `underover(a, over: "⏞")` sozinho (esticou). Anotação
multi-carácter mantém-se centrada no tamanho natural (não estica). Testes:
`p906_layout_underover_over_1char_stretchy_domina_largura_apos_esticar`,
`p906_layout_underover_anotacao_multicaracter_nao_estica` (`tests.rs:2154`, `tests.rs:2181`).

## P906 (cont.) — offsets Y usavam convenção "topo do box"

**Achado, só visível ao confirmar visualmente `underbrace(a+b+c, "soma")`** (com anotação —
estrutura aninhada de 2 `MathUnderover`, `eval.md` §P906): a chave/anotação apareciam sobrepostas,
ilegíveis. `underbrace(a+b+c)` SEM anotação (1 nível, sem aninhamento) já renderizava
correctamente — isolou o problema à COMPOSIÇÃO (`MathUnderover` usado como `base` de outro
`MathUnderover`/`MathAccent`), não ao esticamento em si (secção acima).

**Causa**: mesmo padrão já corrigido em `frac.rs` (P905) e `root.rs` (P901), nunca antes auditado
em `layout_underover` — `over`/`base`/`under` eram posicionados por offsets crescentes a partir de
`Pt(0.0)` (convenção "topo do box"), não pela convenção baseline-relativa confirmada em P901
(`local_y=0` é a BASELINE PRÓPRIA da `MathBox`, y cresce para baixo). Funcionava por coincidência
quando a caixa era o conteúdo de topo da equação (`place()` no topo cancela o termo `-ascent`
independentemente da convenção interna) — quebrava assim que a caixa fosse usada como sub-caixa de
outra (aninhamento, ou `hconcat_spaced` com irmãs).

**Correcção**: `base` fica em `Pt(0.0)` (a sua própria baseline já é `local_y=0`); `over_y =
-(base_box.ascent + over_box.descent)` (a baseline do over sobe o suficiente para o seu descent
parar exactamente no topo da tinta da base); `under_y = base_box.descent + under_box.ascent`
(simétrico, para baixo). `ascent`/`descent` (valores escalares) não mudaram — só os offsets dos
items.

**Efeito colateral**: a ORDEM de push em `items` mudou (base agora primeiro, depois over/under)
— irrelevante para o render, mas quebrou 1 teste pré-existente que assumia posição por índice
(`items.first()`) em vez de por conteúdo — corrigido para procurar por conteúdo.

**Fora de âmbito, achado novo registado**: com a correcção, a estrutura aninhada renderiza
correctamente mas com um gap visualmente maior do que o vanilla entre `base` e a chave —
`layout_underover` não usa nenhuma constante de gap explícita (empilha directamente por
`height()`/ink-to-ink), ao contrário do vanilla que usa `underbar_vertical_gap`/
`overbar_vertical_gap` da tabela MATH. Candidato a passo dedicado.

**Critério**: `MathUnderover { base, under: Some(c1), over: Some(c2) }` → `over` acima e `under`
abaixo da base sem sobreposição, mesmo quando `MathUnderover` é usado como base de outro
`MathUnderover`/`MathAccent` (aninhamento).

## P918 — `over_y` migrado para `stack_tight_above` partilhado

**Achado** (P918 Fase A, `underover.rs:73` vs `accent.rs:65`): `over_y = -(base_box.ascent +
ob.descent)` é idêntica byte-a-byte à fórmula de `accent_y` em `layout_accent`. Extraída para
`stack_tight_above(base_ascent, top_descent)` em `mod.rs` (ver `_comum.md` §P918) — `over_y =
stack_tight_above(base_box.ascent, ob.descent)`. `under_y` (linha 82, espelho "abaixo") **não**
migra — sem segundo consumidor confirmado, fica inline (critério do próprio P918: não generalizar
sem duplicação real).

## P920/P922 — achado original de P906 corrigido (mecanismo real: acento, não underbar/overbar); gap real implementado via `layout_accent`

**Achado** (`typst-passo-906-relatorio.md` achado original: "`layout_underover` empilha por
`height()` sem nenhuma constante de gap da tabela MATH — o vanilla usa `underbar_vertical_gap`/
`overbar_vertical_gap` explícitos"; `typst-passo-920-relatorio.md` Fase A **corrige** a premissa:
`underbrace`/`overbrace`/`underbracket`/etc. resolvem no vanilla como `AccentItem`
(`typst-library/src/math/ir/resolve.rs:1277-1472`, `resolve_underoverspreader`) — o MESMO
mecanismo de `hat`/`tilde` (`accent.rs` §P920) — **não** como `LineItem`
(`underbar_vertical_gap`/`overbar_vertical_gap`, exclusivos de `underline()`/`overline()`, que o
cristalino nem implementa como `MathUnderover` — resolvem para decoração de texto,
`entities/layout_types.md` §P915).

> **Fonte de paridade**: documentação
> `https://typst.app/docs/reference/math/underover/#functions-underbrace` e
> `.../#functions-overbrace` (corpus
> `00_nucleo/corpus-docs/math/underover.typ:15-21`); mecanismo vanilla em
> `lab/typst-original/crates/typst-layout/src/math/ir/resolve.rs:1277-1472`
> e `lab/typst-original/crates/typst-layout/src/math/accent.rs:56-71`;
> guardas em `01_core/src/compiler/math/layout/tests.rs:2200-2500`
> (`p984_tests`/`p985_tests`) e `:4101-4272` (`p922_tests`).

Fórmula real (`typst-layout/src/math/accent.rs:56-71`, dois ramos, POSIÇÕES DIFERENTES):
- **Acima** (`overbrace`/`overbracket`/etc., `over_y` neste ficheiro): `gap = -accent.descent() -
  base.ascent().min(accent_base_height)` — cap. **Nota de correcção**: uma versão anterior desta
  secção dizia "produz gap extra só para bases altas" — errado; o comentário do próprio vanilla
  (`accent.rs:57-60`) diz o oposto ("only if the base is very small, we need a larger gap") — são
  as bases pequenas que ganham mais espaço, o cap em bases altas limita o gap.
  **P922**: a decisão arquitectural de introduzir `FontMetrics::text_ink_bounds_signed` e
  `MathConstants::accent_base_height` resolve a incompatibilidade de sinal destacada em P920.
  `layout_underover` pode reaproveitar a mesma fórmula de `layout_accent` para o `over_y`;
  neste passo o foco é `layout_accent`, deixando `underover` como seguimento natural a validar
  com medição do vanilla real.
- **Abaixo** (`underbrace`/`underbracket`/etc., `under_y` neste ficheiro): `gap = -accent.ascent()`
  — **sem** `accent_base_height`, equivale a empilhamento justo puro. `under_y` (linha 82, `base_
  box.descent + ub.ascent`) **já implementa exactamente isto** — **sem mudança nesta secção**,
  confirma que o achado original de P906 não se aplicava ao lado "abaixo". Esta parte do achado
  fica resolvida (não precisa do passo dedicado): under_y está correcto tal como está.

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


## P961 — anotação de underbrace/overbrace com estilo de script (tamanho reduzido)

**Medição** (`typst-passo-961` Parte A; auditoria externa 2026-08-04, secção
5.4): no vanilla a legenda (`annotation`) de `underbrace`/`overbrace` é
resolvida com **estilo de subscrito/superscrito** —
`resolve_underoverspreader` (`ir/resolve.rs:1441,1455`): under →
`style_for_subscript` (superscript + cramped), over →
`style_for_superscript` — descendo um nível discreto da escada
(`Text→Script`, ×`script_percent_scale_down`; medido no PDF: legenda a
7.7pt ≈ 0.7×11pt). O cristalino marcava só o `math_size` (P945) e mantinha o
tamanho ambiente (11pt) — a nota de P945 ("o factor de tamanho NÃO muda
neste passo") fica **revogada** por esta secção.

**Correcção**: a anotação (conteúdo multi-carácter de `under`/`over`) é
layoutada com o estilo de script completo, mesmo padrão de `attach.rs`:
`size × script_percent_scale_down`, `math_size` um nível abaixo (já
estava), `math_script: true`, e `cramped: true` só no `under`
(`style_for_subscript` = superscript + cramped; `over` não cramped).
**A peça de 1 carácter (a chave ⏟/⏞ que estica, via
`layout_stretchy_or_node`) NÃO recebe a redução** — no vanilla ela é um
acento largo (`AccentItem`, resolve.rs:1427-1432), dimensionado pela
largura da base, não pela escada de scripts. Discriminador: o mesmo de
P906 (1 carácter = peça esticável; multi-carácter = anotação).

## P984 — spreader estica com `short_fall = 0` (não 0.1em)

No vanilla, `resolve_underoverspreader` (`ir/resolve.rs:1430`) configura o
esticamento horizontal da peça ⏟/⏞ com `StretchInfo::new(Rel::one(),
Em::zero())` — short_fall **zero** (ao contrário dos acentos, 0.5em, e dos
delimitadores verticais, 0.1em). `layout_underover` passa agora
`short_fall_em = 0.0` para `layout_stretchy_or_node`. Mecanismo completo
(keep-base, keep-largest) em `stretchy.md` §P984.

> **Fonte de paridade**: vanilla
> `lab/typst-original/crates/typst-layout/src/math/ir/resolve.rs:1430`;
> guarda em `01_core/src/compiler/math/layout/tests.rs:2288`
> (`p984_spreader_short_fall_zero_seleciona_contra_alvo_inteiro`).

## P985 — gaps do spreader: tinta real da peça + fórmula de acento do vanilla

**Medição** (achado §7.2 da auditoria 2026-08-06; bandas de tinta a 600dpi no
documento canónico, `temp/p984/ubc-1.png` vs `ubv-1.png`): `underbrace(a+b+c,
"soma")` — vanilla: conteúdo↔chave **0.7pt**, chave↔legenda **3.4pt**;
cristalino: **8.9pt** e **sobrepostos** (banda única). `overbrace` — vanilla:
legenda↔chave 2.3pt, chave↔conteúdo 0.6pt; cristalino: sobrepostos e 5.4pt.

**Causa (leitura + dados da fonte)**: duas divergências encadeadas.

1. `emit_horizontal_variant` (`stretchy.rs`) dava à caixa da peça o
   `vertical_metrics` da fonte (ascent 0.8em, descent 0) em vez da tinta real
   do glifo. NewCMMath-Book (fontTools): `uni23DF` (⏟) tem bbox y
   **−353..−109du** — tinta toda ABAIXO da baseline (ascent de tinta = 0);
   `uni23DE` (⏞) y **+539..+783du** — tinta toda ACIMA (descent de tinta =
   0). Com ascent inflado, o ⏟ era empurrado ~9.6pt para baixo (gap 8.9pt) e
   a caixa tinha descent 0 com a tinta a sair por baixo → a legenda
   (empilhada "tight" sob a caixa) caía em cima da tinta da chave.
2. A peça de CIMA (⏞) não pode usar `stack_tight_above`: a tinta flutua
   539du acima da baseline; tight com descent=0 deixaria gap de 5.9pt. O
   vanilla trata o spreader como `AccentItem` (`resolve.rs:1416-1472`) e a
   fórmula do ramo "top" de `layout_accent` cancela essa parte oca:
   `baseline_peça = −base.ascent + min(base.ascent, accent_base_height)` —
   **exactamente a fórmula P922** (`accent.md` §P922). Gap de tinta
   resultante: `yMin_peça − min(base.ascent, abh)` = 539−480 = 59du ≈ 0.65pt
   = medido (0.6pt).

**Correcção**:
1. `stretchy.rs` §P985 — `emit_horizontal_variant` passa a usar
   `FontMetrics::glyph_ink_bounds` (método P952b) para ascent/descent da
   caixa: `ascent = ink_up`, `descent = ink_down` (ambos ≥ 0 por construção
   do método).
2. `underover.rs` — peça de 1 carácter em cima usa a fórmula P922
   (`over_y = −base.ascent + min(base.ascent, abh_pt)`); peça de baixo usa
   `under_y = base.descent + max(0, ink_up)` — o `ink_up` vem COM SINAL da
   L3 (−109du para ⏟) e o `max(0, …)` deixa a baseline da peça no fundo da
   tinta da base, com o gap conteúdo↔chave a vir do bearing do próprio
   glifo (−yMax), equivalente ao `gap = −accent.ascent()` do vanilla. Ascent/descent da caixa
   resultante passam a `max(base.*, ∓y + peça.*)` porque a baseline da peça
   de cima agora pode descer abaixo do topo da base.
3. **Anotação (legenda) — CORRECÇÃO pós-revalidação**: a hipótese inicial
   deste §P985 ("tight stacking com caixas honestas já dá o gap certo") foi
   **refutada pela medição** (a 1200dpi o ápice do ⏟ tocava o "m" de "soma";
   gap medido 0pt vs vanilla 3.24pt — em produção a caixa da legenda vem da
   tinta real, não da caixa de linha como no stub). A leitura seguinte do
   vanilla mostrou que a anotação é um **anexo de LIMITE** (`ScriptsItem`
   `top`/`bottom`, `resolve.rs:1438-1460`), posicionada por
   `compute_limit_shifts` (`scripts.rs:295-311`):
   - under: `under_y = base.descent + max(lower_limit_baseline_drop_min,
     lower_limit_gap_min + anotação.ascent)`;
   - over: `over_y = −(base.ascent + max(upper_limit_baseline_rise_min,
     upper_limit_gap_min + anotação.descent))`.
   Valores NewCMMath-Book (fontTools): `LowerLimitGapMin`=167,
   `LowerLimitBaselineDropMin`=600, `UpperLimitGapMin`=200,
   `UpperLimitBaselineRiseMin`=111 (`AccentBaseHeight`=450). Verificação:
   over "soma" (descent 0) → gap = max(111, 200) = 200du = 2.2pt ≈ medido
   2.28pt ✓; under → dominado por drop_min 600du ≈ medido ✓.

**Critério**: com stub de tinta (⏟ ink (0, 353du), ➞ ink (783du, 0)):
`under_y = base.descent` (baseline da chave no fundo do conteúdo); `over_y =
−base.ascent + min(base.ascent, abh)`; descent da caixa =
`base.descent + 353du·scale`; legenda posicionada por limit shifts
(`max(drop_min, gap_min + ascent)` / `max(rise_min, gap_min + descent)`), sem
sobrepor a tinta da chave. Revalidação: gaps medidos ≈ vanilla
(0.7/3.2/2.3/0.5pt, granularidade de variante à parte).

## P1132n — largura exacta do spreader por `TopAccentAttachment`

**Medição antes da decisão** (secção 18, working tree não commitado,
2026-08-22): em `underbrace(overbrace(a+b,"top"),"bottom")`, o cristalino
desenha ambas as chaves com `27.478pt`; o vanilla desenha a superior com
`27.478pt`, deslocada `0.154pt` para a esquerda, e a inferior com `33.000pt`.
As legendas usam a mesma NewCMMath-Book 7.7pt nos dois compiladores.

O mecanismo foi confirmado em `typst-layout/src/math/accent.rs:24-53`:
`resolve_underoverspreader` cria `AccentItem(..., exact_frame_width=true)`;
logo a largura não é `max(base, accent)` com centragem geométrica. O vanilla
usa os attachments MATH:

```text
pre  = accent_attach - base_attach
post = (accent.width - accent_attach) - (base.width - base_attach)
width = max(pre, 0) + base.width + max(post, 0)
pre < 0: base_x=0,    accent_x=-pre
senão:   base_x=pre, accent_x=0
```

Para base composta, `base_attach=base.width/2`; para glifo atómico usa
`FontMetrics::top_accent_attach`, com o mesmo fallback da tabela MATH. A peça
de um caractere usa sempre seu `top_accent_attach` real. A pequena expansão
do frame da chave superior torna-se dinamicamente o alvo da chave inferior;
ela ultrapassa a variante horizontal de 2499du e selecciona a seguinte,
3001du. Nenhuma largura do PDF entra no código.

Esta geometria aplica-se somente ao nível `MathUnderover` cuja peça
`under`/`over` tem um caractere. O nível de legenda multi-caractere continua
a usar `compute_limit_shifts` vertical e centragem horizontal no frame-base.
Regressão da secção 18 fixa os gids/advances escolhidos e as posições das
duas chaves, revalidando também os casos simples de P906/P985.

## P1132p — attachment de assembly horizontal

**Medição antes da decisão** (secção 10, working tree não commitado,
2026-08-22): após P1132n, `underbracket` e `overbracket` mantêm a largura
correta de 42.6238pt, mas a montagem inteira começa em x=104.40351; no
vanilla começa em x=94.81457. A peça é uma assembly de três glifos, não uma
variante única.

Conforme `fragment/glyph.rs` no ramo de assembly horizontal, o attachment é
definido como `(full / 2, full / 2)`. Portanto, `layout_underover` consulta o
attachment tipográfico da variante somente quando a caixa contém exatamente
um `FrameItem::Glyph`; com dois ou mais glifos usa `box.width / 2`. A decisão
deriva da morfologia da caixa emitida e cobre qualquer assembly, sem testar o
caractere, a fixture ou coordenadas do PDF.

Para uma peça de glifo único, a largura propagada pelo próprio nível do
spreader inclui simetricamente `2 * abs(piece_attach - piece.width / 2)`. Isso
conserva no frame a assimetria real do `TopAccentAttachment`, permitindo que
uma peça exterior escolha dinamicamente a variante seguinte. Um wrapper de
legenda não repete essa expansão: a compensação pertence ao nível que contém
a peça. Para assembly, `piece_attach = piece.width / 2`, logo a compensação é
naturalmente zero. Toda a geometria deriva das métricas MATH da fonte; não há
constante posicional ou medida copiada do PDF.

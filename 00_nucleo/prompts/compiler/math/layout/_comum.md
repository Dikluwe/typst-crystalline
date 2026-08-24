# Prompt L0 — `rules/math/layout` — comum (MathLayouter + despacho)
Hash do Código: aa2b038a

## Módulo
`01_core/src/compiler/math/` — motor de layout matemático.

**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Guarda o
que é partilhado (struct, interface, despacho, baseline, primes, handler
MathStyled, critérios gerais); os prompts finos por submódulo citam-no.
**Apontam para aqui**: `math/layout/mod.rs` (MathLayouter + despacho),
`math/layout/tests.rs`.

## Propósito
Recebe `Content::Equation` e produz `Frame`s com `FrameItem::Text` posicionados.

## Estado actual (pós-P96.8 + reconciliação P255)

P96.8 reestruturou `rules/math/layout/` em **8 submódulos** (monólito →
cluster); **P772y** adicionou um nono (`spacing.rs`, prompt próprio
`spacing.md`). `mod.rs` é o núcleo: `MathLayouter` struct + `MathBox` +
métodos coord (`new`, `apply_axis_offset`, `layout_equation`,
`layout_node`, `layout_text_node`, `layout_sequence`, `layout_grid_rows`,
`layout_grid`, `hconcat`, `hconcat_spaced` [P772y]). Os submódulos de
elemento têm prompt próprio (ver índice em `rules/math/layout.md`).

**P800** — `layout_equation` devolve items com posições relativas à
**baseline** da fórmula (y = 0 na baseline); a integração em
`compiler/layout/equation.rs` soma `cursor_y` (ver `compiler/layout/equation.md`).

**P813** — `layout_equation_measured` devolve `(Vec<FrameItem>,
EquationExtent)` com `width`/`ascent`/`descent` da equação, calculados dos
mesmos items de `layout_equation` (não é um segundo caminho de layout):
`width` = limite direito máximo (`pos.x + advance`), `ascent`/`descent` =
limites de tinta acima/abaixo da baseline via `FontMetrics::text_ink_bounds`
(paridade vanilla — frame math usa bboxes de glyphs). Consumidor:
`compiler/layout/equation.rs` (centragem + espaçamento de bloco P813 — ver
`compiler/layout/equation.md`).

**P1140.4-B** — a mesma medição expõe também a geometria de linhas necessária
ao alinhamento vertical do número de equação: `line_count`, baseline/âncora da
primeira linha e da última linha, além da extensão agregada. Esses valores são
produzidos durante o layout da grelha/run matemático; não há segundo layout nem
reconstrução por coordenadas de glyphs. Para uma equação sem quebra,
`line_count = 1` e ambas as âncoras são a baseline única. Consumidor exclusivo:
`compiler/layout/equation.rs`; a geometria não é contrato público da linguagem.

**P921 — `layout_text_node` (a caixa de CADA folha de texto, não só a
extensão agregada da equação) corrigida para usar `text_ink_bounds`,
completando o princípio que P813 já tinha estabelecido**: `layout_text_node`
(`mod.rs:607-628`) construía `ascent`/`descent` de qualquer `MathBox` de
texto via `FontMetrics::vertical_metrics` — métricas OS/2 globais da fonte
(`sTypoAscender`/`sTypoDescender`/`sTypoLineGap`), pensadas para altura de
linha de texto corrido, não para a caixa de um glifo em modo matemático.
Contradiz o que `text_ink_bounds` já documentava desde P813 ("paridade
vanilla: o ascent/descent de um frame math vem das bboxes dos glyphs, não
das métricas globais"), só que essa correcção nunca tinha chegado ao
caminho usado por CADA folha (`layout_text_node`, chamado por
`layout_node` para `MathIdent`/`MathText`) — só a extensão agregada da
equação inteira (`layout_equation_measured`) a usava.

**Achado, não o que o nome do passo original sugeria**: `typst-passo-921.md`
nomeia "`assembly` não atinge a altura-alvo em matrizes de 6+ linhas" — Fase
A confirmou por medição directa com o vanilla real que essa premissa estava
errada (o vanilla TAMBÉM usa `assembly` para 6 linhas, não uma variante
única) e que a causa real do espaçamento de linha excessivo em matrizes
(medido: 27.08pt cristalino vs 23.92pt vanilla, mesmo `.typ`/tamanho) é este
bug em `layout_text_node`, que infla o `descent` de qualquer dígito/letra
sem descendente (medido: `descent=0.394em` fabricado vs `0em` real de tinta,
`NewCMMath-Regular.otf`) — afecta todo o motor de layout matemático, não só
matrizes.

**Correcção**: `ascent`/`descent` de `layout_text_node` passam a vir de
`self.metrics.text_ink_bounds(text, style.size, style)` em vez de
`self.metrics.vertical_metrics(...)`. Testes sintéticos com `FixedMetrics`
(stub sem bbox real) mudam de comportamento como efeito colateral esperado
— `text_ink_bounds` já tinha (desde P813) um default documentado para stubs
sem bbox (`cap_height` acima, zero abaixo, `compiler/layout/metrics.rs:59-71`),
diferente do default de `vertical_metrics` (proporção fixa 0.8/0.4) — não é
uma regressão, é o default já estabelecido a ser exercitado pela primeira
vez neste caminho de código. Ver `typst-passo-921-relatorio.md` para a
medição completa e os 3 testes sintéticos ajustados.

## Restrição arquitectural
L1 puro. Não depende de L3. Usa `FontMetrics` trait injectável. Sem I/O.
`MathLayouter` é genérico sobre `M: FontMetrics`.

## Interface pública

```rust
pub struct MathLayouter<'a, M: FontMetrics> {
    pub(super) metrics:   &'a M,
    pub(super) constants: MathConstants,
    // ... outros campos pub(super) ...
}

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub fn new(metrics: &'a M, block: bool, style: &TextStyle) -> Self;
    pub fn layout_equation(&self, body: &Content, style: &TextStyle) -> Vec<FrameItem>;
    // pub(super): apply_axis_offset, layout_node, layout_text_node,
    // layout_sequence, layout_grid_rows, layout_grid, hconcat.
}
```

**P893** — `new` ganha o parâmetro `style: &TextStyle` (antes só `metrics, block`), correcção
incidental da drift documental pré-existente nesta secção (`block` já era parâmetro real do código
antes de P893, mas não constava aqui; `layout_equation` devolve `Vec<FrameItem>`, não `Frame` —
ambas as correcções feitas agora por serem a mesma linha tocada, não são mudança de comportamento).
Motivo do novo parâmetro: `metrics.math_constants(style)` (ver `infra/font_metrics.md` §P893,
`compiler/layout.md` §P893) precisa de `style` para `FallbackFontMetrics` resolver a face MATH activa
— antes, `constants` era computado sem `style`, sempre com `MathConstants::fallback()` efectivo
nessa variante. Único call site de produção: `compiler/layout/equation.rs` (`compiler/layout/
equation.md` §P893). Os ~48 call sites de teste em `tests.rs` (`MathLayouter::new(&FixedMetrics,
true)`) ganham um terceiro argumento (`&default_style()`) — mudança mecânica, `FixedMetrics` não
sobrepõe `math_constants`, resultado inalterado independentemente do `style` passado.

`MathBox` (4 campos `pub(super)`): caixa intermédia com
`ascent`/`descent`/`width`/`items` para composição hierárquica.

**P825** — a passagem 2 (posicionamento) de `layout_grid_rows` vive em
`layout_grid_boxes(grid_boxes, align, column_gap, row_gap, align_boundaries, style)`:
aceita células já medidas e, por linha/coluna, a marca `align_boundaries`
(limite produzido por `&` — sem `column_gap`; o espaçamento de classe é
incorporado na largura da célula par pelo caller). Consumidor actual:
`matrix.rs` (sub-D de P825 — ver `matrix.md`). `layout_grid_rows` mede e
delega com `align_boundaries` vazio.

**P923b — `row_gap` explicitado**: `layout_grid_rows` e `layout_grid_boxes`
recebem `row_gap: Pt` explicitamente, em vez de deduzirem-no de
`self.constants.math_leading` e do `style` recebido. Isto permite que
`matrix.rs`/`cases.md` usem `DEFAULT_ROW_GAP = 0.2em` do estilo exterior
(paridade vanilla), enquanto `layout_grid` (multiline math `&`/`\\`) pode
continuar a usar o valor que lhe convier. Ver `matrix.md`/`cases.md` §P923b.

**P921 — piso de altura por linha via `(` sintético** (vanilla, `typst-layout/src/math/table.rs:
67-85`: "pad ascent/descent with the paren, to ensure that normal matrices are aligned with others
unless they are way too big"): `layout_grid_boxes` calcula, uma vez, `paren_ascent`/
`paren_descent` de um `(` sintético em estilo de denominador (`size: style.size *
script_percent_scale_down`, `cramped: true` — mesmo padrão de `den_style` em `frac.rs`), via
`layout_text_node(&"(".into(), &denom_style)`. `row_ascent`/`row_descent` de CADA linha (incluindo
a primeira, usada em `total_ascent`/`total_descent` iniciais, e a próxima linha no cálculo de
`advance`/`total_descent` incremental) passam a `.max(paren_ascent)`/`.max(paren_descent)` — sem
isto, uma grelha com conteúdo mais curto que um `(` (ex.: só dígitos) ficava mais baixa/rasa do
que o vanilla, mesmo depois da correcção de `layout_text_node` (`_comum.md` §P921 acima) já ter
corrigido a inflação de `descent` por métricas globais. Efeito colateral, alinhando com a fórmula
do vanilla: o termo de altura da próxima linha no cálculo de `total_descent` passa de `max(ascent+
descent)` por célula para `max(ascent)+max(descent)` (maximizados separadamente, depois somados)
— mesma definição de "altura de linha" que o vanilla usa (`heights[r].0`/`heights[r].1` maximizados
independentemente, `table.rs:84-85`) e que `row_ascent`/`row_descent` já usavam neste mesmo
ficheiro — nunca uma segunda convenção nova.

**Achado residual, não resolvido**: mesmo com os dois mecanismos de P921 (correcção de
`layout_text_node` + piso de `(` sintético), o espaçamento de linha medido (`mat(...)` 6 linhas,
20pt) ainda fica ~4pt abaixo do vanilla por linha (19.96pt cristalino vs 23.92pt vanilla,
`typst-passo-921-relatorio.md` Fase B) — melhoria grande face ao estado anterior (27.08pt, na
direcção errada) mas não exacto. Causa não isolada nesta sessão — candidato a investigação futura,
não bloqueia o fecho deste passo (os dois mecanismos implementados são, cada um, fiéis à fórmula
real do vanilla, confirmados por leitura directa — a paridade é com a fórmula, per ADR-0107/0123,
não com o resultado numérico bit-a-bit de uma implementação mecanicamente diferente).

## Consumers de tipos de domínio (P255 reconciliação)

- **`MathConstants`** → `mod.rs` (construtor `new` via `metrics.math_constants()`);
  `apply_axis_offset` consome `axis_height`; submódulos `frac`/`attach`/`root`
  consomem campos específicos (ver os finos); `accent`/`underover` consomem `upem`
  (conversão `min_width_du`, ver `accent.md`/`underover.md` §P906). `MathConstants::fallback()` é o
  caminho activo via `FixedMetrics` (sem fonte MATH real em testes).
- **`MathGlyphKern`** → `attach.rs` (ver `attach.md`).
- **`GlyphVariants`** → `stretchy.rs` (ver `stretchy.md`).
- **`GlyphAssembly`** → `assembly.rs` (ver `assembly.md`).

## Baseline x-height (P255 §2 item 4)

`MathLayouter::apply_axis_offset` (`mod.rs:228-229`) é o método canónico que
aplica baseline x-height para fracções, delimitadores e sqrt. Usa
`self.constants.axis_height` directamente. Tests regressão
`frac_com_axis_height_nao_regride`, `delimitado_com_axis_height_nao_regride`,
`sqrt_com_axis_height_nao_regride` (`tests.rs:520+`) verificam `axis_height > 0`.

**P919 — correcção (esta secção descrevia um comportamento que nunca existiu de facto — ver
abaixo)**: `apply_axis_offset` **não** aplicava baseline x-height a nada — só ajustava os campos
`b.ascent`/`b.descent` (metadados), nunca `b.items`. Bug de omissão desde a origem da função
(P255): faltava deslocar os items em Y pela mesma quantidade (`shift`), mesmo padrão que
`layout_stretchy_delimiter` já implementa correctamente (`stretchy.rs`, `shift_y` + `offset_item`
por item). Efeito prático: a chamada era um **no-op visual** em todos os 5 call sites
(`frac.rs`, `cases.rs`, `matrix.rs`, `root.rs`, `delimited.rs`) — os testes acima só verificam
`axis_height > 0` ou presença de texto, nunca posição Y real, por isso o no-op nunca foi apanhado.
Achado original: `typst-passo-917-relatorio.md`, "Achado registado, não corrigido" (`x^2_i +
(1/2)` com `(1/2)` desalinhado). Medição real (`mutool trace`, fonte real embutida): a barra de
`frac(a,b)` estava a 0.046pt da baseline partilhada com texto vizinho — devia estar a
`axis_height` de distância. Ver `typst-passo-919-relatorio.md` Fase A para a medição completa.

**Confirmado por leitura do vanilla que os 5 call sites NÃO são uniformes** — cada um tem de ser
avaliado individualmente contra o mecanismo real, não corrigido em bloco:

- **`fraction.rs:51-69`** (vanilla, estilo com barra): `baseline = line_pos.y + axis` — a barra
  fica fixa a `axis_height` da baseline, **por construção**, não pelo "meio do `ascent`/`descent`"
  do frame. Diverge da fórmula genérica de `apply_axis_offset` em fracções assimétricas (ex.:
  `frac(a, b^2)`, denominador mais alto). `frac.rs` deixa de chamar `apply_axis_offset` — ganha
  fix próprio, mais simples: deslocar todos os `items` por `-axis_pt` (a barra, já fixada em
  `local_y=0` por construção — `frac.md` §P905 — passa a `-axis_pt`), `ascent += axis_pt`,
  `descent -= axis_pt`. Ver `frac.md` §P919.
- **`table.rs:188`** (vanilla, matrizes/`cases`): `frame.set_baseline(height/2.0 + axis)` — centra
  o **meio da altura total** no eixo. Bate exactamente com a fórmula genérica já existente em
  `apply_axis_offset` (`shift = axis_pt - (ascent-descent)/2`). `cases.rs`/`matrix.rs` continuam a
  chamar `apply_axis_offset`, agora com o bug de omissão corrigido (desloca `items` também, não só
  metadados) — **mas revisão do Agente A** (Fase B) encontrou que a chamada não pode ficar onde
  estava (no fim, sobre `result` já concatenado com os delimitadores): os delimitadores
  (`left_box`/`right_box`/chaveta) já vêm auto-centrados de `layout_stretchy_delimiter`
  (`stretchy.md` §P917) — deslocar `result` inteiro deslocá-los-ia pela segunda vez. A chamada
  move-se para `grid_box` isolado, **antes** de o mesclar com os delimitadores. Ver `cases.md`/
  `matrix.md` §P919 para o detalhe exacto.
- **`radical.rs:110`** (vanilla): `frame.set_baseline(ascent)` — **sem** termo de `axis` nenhum.
  Confirmado também empiricamente (`mutool trace`, `$x + sqrt(a) + y$`): `x`/`a`/`y` já partilham
  exactamente o mesmo Y no cristalino actual, sem qualquer chamada corrigida — `root.rs` está
  "acidentalmente correcto" hoje só porque o bug de omissão nunca mexeu nos items. Corrigir
  `apply_axis_offset` genericamente e mantê-la em `root.rs` **quebraria** este caso. A chamada é
  **removida** de `root.rs`. Ver `root.md` §P919.
- **`fenced.rs`** (vanilla, `layout_fenced`): não faz nenhuma centragem do grupo delimitado — os
  parênteses/chavetas já se auto-centram no eixo internamente (mesmo mecanismo de
  `layout_stretchy_delimiter`, já correcto — ver `stretchy.md` §P917), o corpo mantém a sua
  própria baseline sem alteração. Confirmado também empiricamente (`$x + (a) + y$`): `x`/`a`/`y`
  já partilham o mesmo Y. A chamada é **removida** de `delimited.rs`. Ver `delimited.md` §P919.

## MathPrimes (P255 §2 item 3 — divergência arquitectural)

`MathPrimes` é **resolvido em eval, não em layout** (`rules/eval/math.rs:85-101`).
Count → glifo `′`/`″`/`‴`/`⁗` (U+2032/2033/2034/2057; n>4 → repetição de `′`)
convertidos para `Content::MathText` e merged como superscript regular.
`attach.rs` recebe-os pelo arm superscript regular; não há arm dedicado.
Paridade observable vanilla per ADR-0033.

## Handler `Content::MathStyled` — Passo 311b.4 (em `mod.rs`)

Handler dedicado em `layout_node` arm `Content::MathStyled` resolve variant glyph
+ flags. Algoritmo:

1. **Pré-transformação recursiva** do body via `apply_math_style` (função livre
   no fim do módulo). Composição **outer-wins** via `Option::or`.
2. **Substituição char-by-char** em `MathIdent`/`MathText`: `map_glyph(c,
   kind.unwrap_or(Plain), bold.unwrap_or(false), italic.unwrap_or(false))`.
3. **Propagação para containers** (MathFrac/MathAttach/MathRoot/MathDelimited/
   MathSequence) — recurse com mesmo context.
4. **`MathOp` passa-through** — operadores texto mantêm aparência normal mesmo
   dentro de `bb(...)`.
5. **Size factor** Script/SScript: `style.size *= kind.size_factor()` antes de
   descer.
6. **Supressão de auto-itálico**: quando wraps aplicados (kind/bold/italic
   Some), `math_style.italic = false` no descent.

**Composição (paridade vanilla)**: `bb(cal(x))` → outer Bb → 𝕩 ·
`bold(bb(x))` → kind Bb preserved, bold flag · `upright(italic(x))` → outer
upright · `script(sscript(x))` → outer-wins via `Option::or`, **NÃO**
multiplicativo (refuta diagnóstico P311a §3.5).

**8 unit tests P311b.4**: `bb_substitutes_chars`, `bold_italic_orthogonal`,
`bb_cal_outer_wins`, `upright_italic_outer_wins`, `bold_preserves_inner_bb`,
`recurses_through_mathfrac`, `size_variant_passthrough_glyph`,
`math_op_passthrough`.

**Integração com `is_single_letter_var`**: pré-P311b.4 `MathIdent` aplica
itálico automático a variáveis de 1 letra; pós-P311b.4 wraps `MathStyled`
suprimem auto-itálico via `math_style.italic = false`; itálico explícito honrado
via codepoint já transformado. Sem regressão.

## Itálico matemático por defeito via codepoint — P809

Antes de P809, o "itálico" das variáveis de 1 letra era só a **flag de fonte**
`italic: true` — invisível à extracção (`x` vs `𝑥` U+1D465 do vanilla).
P809 aplica a regra do vanilla (codex `MathStyle::select`, medida).

**P812 — onde o default é aplicado (correcção da arquitectura P809)**:
`layout_equation` chama `apply_math_default(body)` (não
`apply_math_style` — essa versão consumia os nós `MathStyled` e destruía
display/script/sscript, regressão medida em P812-A). `apply_math_default`:

1. Mapeia folhas de 1 carácter com `is_math_italic_default` (latin ou grego
   minúsculo — cobre `$x$`, que o lexer entrega como `MathText`, e símbolos
   resolvidos como `alpha`→`α`) para o codepoint math italic. Grego
   maiúsculo fica upright (medido: `ΓΔΩ𝛼`).
2. **Preserva os nós `MathStyled` intocáveis** — a composição corre no
   handler `MathStyled` de `layout_node` (via `apply_math_style`), que
   também aplica o factor de tamanho.

**Composição por eixos ortogonais (P812, paridade vanilla)**: em
`apply_math_style`, tamanho e glifo são eixos independentes — outer
size-variant + inner glyph-variant → o glyph do inner prevalece
(`script(bb(R))` → ℝ a 0.7×); ambos size-variants → outer vence (regra
P311b.4); variants de tamanho (Display/Inline/Script/SScript) não têm
mapping de glifo próprio e tratam-se como `Plain` no eixo glifo — o
itálico por defeito atravessa wrappers de tamanho (`$script(x)$` → 𝑥,
medido). `bold(x)` compõe para **bold-italic** (U+1D499, medido no
vanilla) porque o default de `italic` nas folhas de 1 carácter é
`unwrap_or(is_math_italic_default)`; `upright(x)` continua plain
(Some(false) explícito).

## Espaçamento automático por `MathClass` (P772y — ver `spacing.md`)

`layout_sequence` computa `spacing::compute_gaps` sobre os nós filtrados
(antes do layout) e concatena via `hconcat_spaced` em vez de `hconcat`
plano. Handler `Content::MathClassOverride(e) => self.layout_node(&e.body,
style)` em `layout_node` — a classe forçada por `math.class(class, body)`
só entra no cálculo de espaçamento (`spacing::node_math_class`), o layout
do body é normal. Detalhe completo, tabela de espaçamento e critérios de
verificação: `spacing.md`.

## Handler `Content::HSpace` — P895 (espaçamentos nomeados de modo math)

`layout_node` ganha um arm dedicado para `Content::HSpace(e)`, colocado
**antes** do catch-all genérico (`other => plain_text()...`). Motivo:
`thin`/`med`/`thick`/`quad`/`wide` (registados em `make_math_module()`,
`stdlib/structural.rs`, como `Value::Content(Content::h_space(...))` —
paridade vanilla `math/mod.rs:98-102`) resolvem, via `lookup_math_op`
(`eval/math.rs`), a um nó `Content::HSpace` dentro da sequência math. Sem
este arm, `HSpace` caía no catch-all: `other.plain_text()` é vazio para
`HSpace` (não tem texto), produzindo `MathBox { width: 0.0, .. }` — os 5
nomes compilavam sem erro mas **não produziam nenhum espaço visível**
(achado do catálogo de terceiros, `typst-passo-895-relatorio.md`, Parte B).

Resolução: só `Spacing::Absolute(Length)` é honrado (`len.resolve_pt(style.
size.val())` → `width` do `MathBox`, `ascent`/`descent` = 0, sem items).
`Spacing::Fractional` (`1fr`) fica `width: 0.0` — scope-out registado, não
silencioso: não há "espaço restante" bem definido dentro de uma sequência
math de largura própria, e nenhum dos 5 nomes registados usa fracção.

## Critérios de verificação (gerais)

- `MathIdent("x")` → `Frame` com `FrameItem::Text { text:"x", .. }` não vazio.
- `MathSequence([x,+,y])` → `Frame` com 3 items.
- Integração: `Content::Equation` delega ao `MathLayouter`.
- `apply_axis_offset` consome `axis_height` real (não zero).
- Primes (`′″‴⁗`) renderizam correctamente (resolvidos em eval).
- Suite math layout: ~50 tests pré-P311b.4 + 8 = ~58 verdes; auto-itálico para
  `MathIdent` sem wrap preservado (caminho default não-MathStyled).

## P906/P909 — guard partilhado `layout_stretchy_or_node` (esticamento de over/under/accent)

**P909** — `layout_accent`/`layout_underover` fatiados para `accent.rs`/`underover.rs` (arquivos
próprios, completando o padrão de P314 que os tinha deixado para trás). O guard partilhado entre
os dois, `layout_stretchy_or_node`, **fica em `mod.rs`** (`pub(super)`, chamado por ambos os
arquivos novos — decisão P909: não duplicar, mesmo tratamento já dado a
`layout_stretchy_delimiter`/`apply_axis_offset`). Conteúdo P906 (esticamento horizontal + correcção
de convenção baseline-relativa) migrado para `accent.md`/`underover.md` — ver aí.

## P918 — núcleo geométrico partilhado (Fase A: duplicação real confirmada, não semelhança superficial)

**Contexto** (ADR-0123, "próximo passo natural"): auditar os módulos de `math/layout/` que ainda
não tinham passado pela checagem de fidelidade geométrica ao vanilla, e extrair o que for
duplicação **real** (fórmula idêntica, 2+ consumidores) — não duplicação superficial (mesma
convenção de eixo, fórmula distinta, como `frac.rs` vs `root.rs` — mantidos separados, ver
`frac.md`/`root.md` §P918).

Dois candidatos confirmados por leitura directa (`file:line` dos dois lados, P918 Fase A) e
extraídos para cá, mesmo padrão de `layout_stretchy_or_node` (P909) — free function/método
`pub(super)` em `mod.rs`, consumido pelos módulos-irmãos via `use super::{..}`:

1. **`stack_tight_above(base_ascent: f64, top_descent: f64) -> f64`** — **REMOVIDA em P985**:
   o último consumidor real (`underover.rs`, `over_y`) deixou de usar tight stacking (peça de
   cima passou à fórmula P922 do acento e a legenda a `compute_limit_shifts` — ver
   `underover.md` §P985); `accent.rs` já não a chamava desde P922 (o import ficou esquecido).
   Sem consumidores, a free function e os dois imports foram removidos. Registo histórico:
   devolvia o `local_y` da baseline de uma caixa `top` empilhada rente ao topo da tinta de
   `base` (`-(base_ascent + top_descent)`), convenção `y=0=baseline própria` (ADR-0123).
2. **`grid_delim_target_du(&self, grid_box: &MathBox, style: &TextStyle) -> f64`** — converte a
   altura de tinta de uma grelha (`ascent+descent`, com a folga de 10% descrita em §P912-folga)
   para design units, para dimensionar o delimitador esticável que a envolve. Método
   `pub(super)` (precisa de `self.constants.upem`). Confirmado idêntico byte-a-byte em
   `cases.rs` (`layout_cases`) e `matrix.rs` (`layout_matrix`) — bloco de 5 linhas
   (`grid_height_pt = (ascent+descent)*1.1`, conversão condicional para du) duplicado sem
   variação.

### §P912-folga — a folga de 10% **é** o `Ratio::new(1.1)` do vanilla (medido 2026-08-13, P1026 Fase C)

Esta secção foi escrita duas vezes com a conclusão errada, das duas por ler o vanilla no
sítio errado. As duas ficam registadas, porque a **forma** do erro é reutilizável:

1. **P1024** afirmou que o cristalino "multiplica **em vez de** subtrair" o short fall.
   Falso: aplicam-se em sequência.
2. **P1026 Fase A** corrigiu (1), mas afirmou que o ×1.1 era "inflação extra de 10%",
   citando `math/ir/resolve.rs:843` como o alvo do vanilla. Esse call site é a **barra da
   fracção inclinada** (`resolve_skewed_frac`) — não o delimitador de grelha. Conclusão
   tirada do call site vizinho errado, sem confirmar que era o do construto em causa.

**Medição decisiva** — vanilla `math/ir/resolve.rs:1164-1174`, `fn resolve_delimiters`,
documentada como *"Resolves the delimiters around the body of a vector, matrix, or cases"*:

```rust
let target = Rel::new(Ratio::new(1.1), Abs::zero());
let stretch = Stretch::new().with_y(StretchInfo::new(target, DELIM_SHORT_FALL));
```

O vanilla pede **110%**, com o mesmo `DELIM_SHORT_FALL`. A fórmula do cristalino
(`math/layout/mod.rs:390` `(ascent+descent) * 1.1`, depois `−0.1em` em `stretchy.rs:57-61`) é
**paridade**, não divergência. Não há nada a alinhar: alinhar o alvo a 100% **introduziria**
a divergência que se julgava estar a remover.

**O short fall está portado** — `layout_stretchy_delimiter_impl` (`stretchy.rs:57-61`) subtrai
`DELIM_SHORT_FALL = 0.1em` antes de escolher a variante, com a distinção fina do vanilla: só
delimitadores, porque o radical usa `short_fall = 0` (vanilla `resolve.rs:1248`; ver `root.md`
§P974).

**Verificação de âmbito** — o 1.1 tem de estar neste caminho e em mais nenhum, dos dois lados:

| construto | cristalino | vanilla ratificado (`a51e02804`) |
|---|---|---|
| grelha (vector/matriz/cases) | `mod.rs:390`, chamado **só** por `cases.rs:41` e `matrix.rs:113` | `resolve.rs:1173` — `Ratio::new(1.1)` |
| `binom` | não usa `grid_delim_target_du` | `resolve.rs:754` — `Rel::one()` |
| fracção inclinada | idem | `resolve.rs:843` — `Rel::one()` |
| `lr` / `mid` | idem | `resolve.rs:881`, `:917` — tamanho pedido; `Rel::one()` por defeito (`lr.rs:26`) |
| radical | `layout_radical_symbol`, `short_fall = 0` | `resolve.rs:1248` — `Rel::one()`, `Em::zero()` |
| acento | eixo horizontal | `resolve.rs:1432` — `Rel::one()` |

`grep` de `1.1` em todo o `compiler/math/layout/` do cristalino devolve **uma** ocorrência
(`mod.rs:390`). Âmbito idêntico ao do vanilla.

**A montagem por peças recebe o alvo cru nos dois.** O vanilla calcula
`short_target = target - short_fall` (`glyph.rs:271`) e usa-o só na busca de variante;
`glyph.rs:312` passa **`target`** — não `short_target` — a `assemble`. O cristalino faz o
mesmo: passa `min_height_du` (pré-short-fall) a `layout_assembly`.

### Medição de output — os dois produzem o mesmo delimitador

Proveniência: `HEAD = 0f8487b9d`, árvore limpa (0 ficheiros alterados), 2026-08-13 07:46.
Método: mesmo documento nos dois binários → PDF → PGM 300dpi (`pdftoppm -gray`), banda
horizontal do delimitador esquerdo **auto-calibrada** (da coluna de tinta mais à esquerda até
um intervalo de ≥20 colunas vazias, para não cortar a ponta do glifo, que curva para a
direita); contagem de glifos desenhados por `mutool trace`.

| `mat` (11pt) | vanilla | cristalino | Δ | glifos v / c |
|---|---:|---:|---:|---|
| 2 linhas | 26,160pt | 26,160pt | 0,000 | 4 / 4 |
| 3 linhas | 40,800pt | 40,800pt | 0,000 | 11 / 11 |
| 4 linhas | 55,200pt | 55,440pt | +0,240 | 18 / 18 |
| 5 linhas | 69,840pt | 69,840pt | 0,000 | 25 / 25 |
| 6 linhas | 84,240pt | 84,240pt | 0,000 | 30 / 30 |
| 8 linhas | 113,040pt | 113,280pt | +0,240 | 44 / 44 |
| 10 linhas | 142,080pt | 142,080pt | 0,000 | 57 / 57 |
| 12 linhas | 171,120pt | 171,120pt | 0,000 | 73 / 73 |

0,240pt é **1 pixel** a 300dpi. Junta-se às 30 configurações de P1026 Fase A (2-12 linhas a
11pt; 3 e 5 linhas a 8/9/10/11/12/14/16/18/20/24pt), com o mesmo resultado. A explicação
correcta não é "a quantização absorve a inflação" — é que **não há inflação a absorver**.

**Refutado: os dois montam o delimitador alto da mesma maneira.** P1026 Fase A registou, a
partir de extracção de texto, que "o vanilla estica um glifo variante e o cristalino monta a
partir de `U+239B…U+23A0`". Falso. `mutool trace` a 5 linhas dá 20 glifos dos dois lados
(cristalino `1×⎛ + 8×⎜ + 1×⎝` por delimitador; vanilla `1×(` + peças sem mapeamento
ToUnicode), 36 a 8 linhas, 58 a 12 linhas — **contagem de peças igual em todas as
configurações**. Os dois usam a `GlyphAssembly`; só difere o `ToUnicode` do PDF (mecânica,
livre por ADR-0107).

**Estado**: **paridade confirmada na fonte e no output**. Item fechado — não reabrir sem
medição nova que contrarie `resolve.rs:1173`. Guarda de regressão:
`layout/tests.rs::p945_grid_delim_target_du_e_altura_vezes_1_1`.

**Não extraído nesta ronda** (avaliado e rejeitado por P918 Fase A, registo para não repetir a
mesma pergunta em passo futuro):
- `frac.rs` (offset de numerador/denominador) vs `root.rs` (overline do radicando) — mesma
  *forma* (`termo + gap + espessura/2`), mas `frac` desloca a própria caixa deslocada
  (`.descent`/`.ascent` do numerador/denominador), `root` desloca uma linha fixa sobre o
  radicando parado em `y=0` — consumidores estruturalmente distintos, não a mesma função.
- `attach.rs` (`compute_script_shifts`) — `.max()` de 4-5 termos, incomparável estruturalmente
  às fórmulas acima.
- Gap de empilhamento de `underover.rs` — confirmado ainda sem constante explícita de
  `MathConstants` (achado de P906 continua em aberto); não resolvido nesta ronda, não é
  duplicação (não há segundo módulo com o mesmo gap para comparar), fica registado para passo
  próprio se/quando `underover.rs` ganhar essa constante.
- `covering()`/resolução de fonte MATH (candidato 3) e `hor_advance`/`advance` (candidato 4):
  confirmados já correctos (P912/P917 respectivamente), nada a extrair.
- `compute_math_kern` (candidato 5, P914): single-consumer confirmado (`attach.rs`, 4 call
  sites internos), mantido onde está.

**Achados adicionais fora do escopo original dos 5 candidatos**, vistos de passagem na Fase A e
incorporados por decisão do dono (não fazem parte da checagem ADR-0123, são duplicação de
código comum, categoria "mecânica" per ADR-0107 — livre para reorganizar, sem implicação de
fidelidade geométrica): `resolve_assembly_repeat` (interno a `assembly.rs`, ver `assembly.md`
§P918) e `apply_delim_short_fall` (interno a `stretchy.rs`, ver `stretchy.md` §P918) — ambos
partilhados **dentro do próprio ficheiro** (vertical/horizontal do mesmo módulo), não precisam
de viver em `mod.rs`.

## P945 — `denominator_style` (descida por nível) + `total_descent` de grelhas

**Contexto e medição**: ver `matrix.md` §P945 (tabela vanilla × cristalino para
a matriz 3×3 em Display). Duas correcções em código partilhado deste módulo,
mais uma confirmação anti-deriva.

### 1. `denominator_style(&self, style: &TextStyle) -> TextStyle` (novo helper, `pub(super)`)

Descida de **um nível MathSize** do vanilla
(`lab/typst-original/crates/typst-library/src/math/style.rs:343-363`:
`style_for_denominator = style_for_numerator + cramped`), usando o campo
`TextStyle::math_size` (`entities/layout_types.md` §P945):

| `style.math_size` | novo `math_size` | factor sobre `style.size` |
|---|---|---|
| `Display` | `Text` | ×1.0 |
| `Text` | `Script` | ×`script_percent_scale_down` |
| `Script` | `ScriptScript` | ×`sscript/script` |
| `ScriptScript` | `ScriptScript` | ×1.0 |

`cramped: true` sempre (é o denominador). Consumidores neste passo:
`matrix.rs` e `cases.rs` (células). `frac.rs`/`root.rs`/`attach.rs`/
`underover.rs` **não** mudam os seus factores de tamanho actuais neste passo —
só passam a manter `math_size` honesto (ver os L0s respectivos, nota §P945).

### 2. `layout_grid_boxes` — `total_descent` acumulado a mais

**Medição** (leitura + simulação numérica, `typst-passo-945` Fase A): o laço de
posicionamento acumulava `total_descent += row_descent + line_gap +
next_row_ascent + next_row_descent` por transição — o `next_row_descent` é
contado **duas vezes** por linha intermédia (para N linhas, o total fica
inflado em `d_1 + … + d_{N-1}`, ≈4-6pt em matrizes de 3+ linhas). O vanilla
(`lab/typst-original/crates/typst-layout/src/math/table.rs:103-106`):
`total_height = Σ(ascent_r + descent_r) + gap.y × (nrows-1)`, com a baseline na
primeira linha — ou seja `total_descent = d_1 + Σ_{r≥2}(a_r + d_r) + gap ×
(nrows-1)`, sem duplicação. **Correcção**: acumular pela forma do vanilla (ou
equivalentemente: `baseline_offset` final + `last_row_descent`). Afecta
`layout_matrix`, `layout_cases` e `layout_grid` (math multilinha) — os três
caminhos convergem para o valor do vanilla; a revalidação visual (Fase C do
passo) cobre os dois primeiros e a suíte cobre o terceiro.

### 3. CONFIRMADO, não mexer (anti-deriva): `grid_delim_target_du`

A fórmula `(ascent+descent) × 1.1` **está correcta** para matrizes/casos —
medido contra o vanilla
(`lab/typst-original/crates/typst-library/src/math/ir/resolve.rs:1168-1186`,
`resolve_delimiters`: alvo `Rel::new(Ratio::new(1.1))`, `balanced = false` →
`relative_to = height = ascent+descent`). A fórmula balanceada
`2×max(ascent−axis, descent+axis)` aplica-se **só** a grupos delimitados
simples (`MathDelimited`, `balanced = true`, `resolve.rs:976`) — já
implementada em `delimited.rs` (P912). Não "corrigir" uma com a outra: os
dois caminhos têm fórmulas diferentes no próprio vanilla.

### `numerator_style(&self, style: &TextStyle) -> TextStyle` (novo helper, P952)

Simétrico de `denominator_style` (§P945), mesma descida de nível da tabela
(`Display→Text` ×1.0, `Text→Script` ×script_percent, `Script→ScriptScript`
×sscript/script, `ScriptScript→ScriptScript` ×1.0) mas **sem forçar
`cramped`** — herda-o do ambiente (vanilla `style.rs:343-350`:
`style_for_numerator`, sem `style_cramped()`). Consumidor: `frac.rs`
(numerador — P952).

## P952 — operadores grandes (`MathClass::Large`) esticados em Display

**Medição** (`typst-passo-952` Fase A — leitura do vanilla + avanços medidos
nos PDFs reais): no vanilla, qualquer glifo de classe `Large` em
`MathSize::Display` recebe Y-stretch para `display_operator_min_height`
(1300du em NewCMMath), seleccionando a primeira variante vertical com
`advance >= target` (`summation.v1`=1401, `integral.v1`=2223) — `∑`/∫
renderizam ~37%/100% maiores que o glifo base. O cristalino usava sempre o
glifo base (advance 1.056em/0.665em vs 1.444em/0.999em do vanilla),
contribuindo para os deltas sistemáticos de altura das equações display.

**Correcção**: nos braços `Content::MathIdent`/`Content::MathText` de
`layout_node`, quando o texto é **um único carácter** e
`symbols::is_large_operator(c)` e `self.block` (Display — paridade com a
condição `math_size == Display` do vanilla), o glifo passa por um novo helper
`layout_large_operator_display(c, style)` que:
- selecciona a primeira variante vertical com `advance >=
  to_du(display_operator_min_height)` — **sem** `DELIM_SHORT_FALL` (o
  `StretchInfo::default()` do vanilla tem `short_fall = Em::zero()`);
- emite a variante como `FrameItem::Glyph` (mesma emissão do caminho de
  variante de `stretchy.rs`, P917: `hor_advance` para `x_advance`/largura);
- `ascent`/`descent` da MathBox vêm das métricas da variante (altura
  `advance`, centrada na baseline como o vanilla — a variante substitui o
  glifo no run com as suas próprias métricas, `update_glyph`);
- sem variante suficiente: glifo base (comportamento anterior, inalterado).

`is_integral_char` **não** exclui: no vanilla, integrais são `Large` para o
stretch (só não empilham limites) — `∫` display usa `integral.v1` (2223du).

**Guards**: inline (`Text`) e scripts mantêm o glifo base (vanilla só estica
em Display); a família coberta é exactamente `is_large_operator`
(`symbols.rs` — já paridade com a classe `Large` do vanilla, P772w).

### P1136 — operadores `Large` são sempre centrados no eixo

**Medição** (2026-08-23, working tree não commitado sobre HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`; vanilla ratificado
`a51e02804`): na secção 17, os glifos `⋃`/`⋂` e seus limites coincidem, mas
o `A_i` seguinte diverge verticalmente em sentidos opostos: −0,121pt após
`⋃` e +0,121pt após `⋂`. O padrão refuta deslocamento global e aponta para a
baseline lógica do glifo `Large`, distinta da posição da tinta.

**Causa medida na fonte antes da decisão**: o vanilla chama
`glyph.center_on_axis()` incondicionalmente quando
`glyph.class == MathClass::Large`
(`lab/typst-original/crates/typst-layout/src/math/text.rs:109-121`), além da
chamada feita ao esticar no eixo Y. `GlyphFragment::align_on_axis`
(`fragment/glyph.rs:328-340`) muda a baseline para
`height/2 + axis_height` e compensa a posição interna da tinta pela diferença
entre a baseline antiga e a nova. Assim a tinta permanece no mesmo lugar,
mas o fragmento passa a alinhar conteúdo adjacente pela baseline correcta.
O cristalino só seleccionava variante Display em
`layout_large_operator_display` e nunca aplicava essa centragem ao glifo
base nem à variante.

**Decisão**: todo `MathIdent`/`MathText` de um carácter reconhecido por
`symbols::is_large_operator` passa pelo layouter de operador grande em todos
os `MathSize`. Em Display continua a seleccionar a variante vertical como em
P952; em Text/Script mantém o glifo base. Em ambos os caminhos, a `MathBox`
final passa por `apply_axis_offset`, que implementa a mesma fórmula dinâmica
`shift = axis_height − (ascent − descent)/2` e compensa os items por
`−shift`. Nenhuma coordenada ou constante de fixture entra na produção.

**Critério de língua**: `⋃_(i in I) A_i` e `⋂_(i in I) A_i` alinham `A_i`
como o vanilla; operadores Display continuam a seleccionar a mesma variante
e limites, apenas com baseline de classe `Large` correcta.

**Contraprova de escopo**: a secção 31 também tem uma diferença de 0,022pt
na altura lógica após um somatório inline, mas a aplicação desta regra não a
altera. Portanto essa diferença não tem a centragem de classe `Large` como
causa e permanece para diagnóstico separado; não integra a aceitação de
P1136.

### P952b — `layout_grid_boxes`: baseline das células em `baseline_offset`

**Medição** (`typst-passo-952` Fase A + revisão cética retroativa): o vanilla
posiciona cada linha da grelha em `pos.y = size.y + row_ascent − sub.ascent`
(`lab/typst-original/crates/typst-layout/src/math/run.rs:137`) — mas aí
`pos.y` é o **topo** do frame da célula (origem top-left), enquanto os items
de `MathBox` cristalino são **baseline-relativos**. A tradução directa
(`dy = baseline_offset + row_ascent − cell_box.ascent`, primeira versão deste
passo) colocava cada célula a uma altura diferente — regressão medida na
revisão retroativa (`mat(a, b; c, d)`: 2.77pt de desalinhamento intra-linha;
pitch não-uniforme em `mat(a; b; c)`). Para items baseline-relativos, a forma
correcta é simplesmente **`dy = baseline_offset`** — o acumulador (ascents +
descents + gap entre linhas, P921/P923/P945) já é, por construção, a baseline
da linha relativa à baseline da `MathBox`, e **todas** as células partilham a
baseline da linha independentemente do seu ascent.

**Antes de P952b** (`dy = baseline_offset − row_ascent`): a grelha inteira
flutuava ~uma altura-de-linha acima da baseline da equação e a
`ascent`/`descent` declarada da `MathBox` (correcta) ficava inconsistente com
os items (extent de P813 errado para grelhas → espaçamento entre blocos mais
apertado que o vanilla — parte do padrão sistemático +7 a +17pt de P952).

**Correcção final**: `dy = baseline_offset` — mantém o benefício (extent
correcto, centragem de `apply_axis_offset` no eixo real) **e** o alinhamento
intra-linha (medido após a correcção: `mat(a, b)` alinhado, pitch uniforme).


## P961 — `apply_math_default` recursa em `MathAccent` e `MathUnderover`

**Medição** (`typst-passo-961` Parte B; auditoria externa 2026-08-04, secção
5.4 + achado #5 de P906, reconfirmado no código actual): `apply_math_default`
não tinha braços para `Content::MathAccent` nem `Content::MathUnderover` —
caíam em `other => other.clone()`, logo a base de `hat(x)`/`tilde(x)`/
`dot(x)` nunca recebia o itálico automático por codepoint (saía `x` latino;
o vanilla desenha `𝑥` U+1D465).

**Correcção**: dois braços novos, mesma recursão dos outros containers —
`MathAccent`: recursão na `base`; o campo `accent` (o combining mark)
passa **inalterado** (não é letra — e mesmo que fosse, o acento nunca
recebe itálico no vanilla). `MathUnderover`: recursão em `base`, `under`
e `over` (a anotação de 1 letra, ex.: `underbrace(x, n)`, recebe itálico
como qualquer identificador; a peça ⏟ não é letra, inalterada na prática).


## P966 — recursão em containers de markup dentro de math (conteúdo de função de utilizador)

**Medição** (`typst-passo-966` Fase A): com `#let bra(x) = [⟨#x\|]` invocado
como `bra(phi)` dentro de `$…$`, a árvore produzida é (debug print real):

```text
equation(body: math.sequence([
  sequence([text("⟨"), math.text("φ"), text("|")]),   // bra(phi)
  sequence([text("|"), math.text("ψ"), text("⟩")])    // ket(psi)
]))
```

O conteúdo de função de utilizador dentro de math chega como
**`Content::Sequence` de markup** (não `MathSequence`) com filhos mistos
`Text`/`MathText` — e o `MathText("φ")` (já avaliado em contexto math) nunca
recebia o mapeamento itálico porque `apply_math_default` (layout, P812) só
recursava em containers nativos `Math*`. No vanilla, o tratamento acontece
na **resolução** (`ir/resolve.rs:127-146`: `resolve_into_self` chama
`(routines.realize)(RealizationKind::Math, …)` — o output de funções de
utilizador é re-realizado COMO math, e `resolve_text` trata cada carácter
como glifo math com o default). **A diferença de camada (resolve vs layout)
é a causa raiz confirmada** (Fase A.3 do passo).

**Decisão de direcção (Fase A.1 — à espera da confirmação do dono)**:
**(a) estender `apply_math_default`**, não (b) mover para o eval. Razões:
(a) é contida numa função de L1 e casa exactamente com a árvore medida
(as folhas `MathText` já chegam correctas; só falta a recursão chegar
lá); (b) contradiz a decisão de arquitectura de P812 (o default vive no
layout precisamente para preservar os wrappers `MathStyled` — regressão
P812-A) e tem blast radius muito maior (pipeline de avaliação inteiro).

Desenho de (a): braços novos para `Content::Sequence` e `Content::Styled`
(recursão nos filhos/corpo — os dois containers que templates de markup
produzem). As folhas transformáveis continuam a ser só
`MathIdent`/`MathText` de 1 carácter (regra existente); **`Content::Text`
nunca é transformado** (texto literal de markup dentro de math fica reto —
paridade com o `resolve_text` do vanilla, que não italiciza texto). Os
wrappers `MathStyled` continuam intocados dentro da recursão (o `dif`
upright de P962 sobrevive). Aninhamento de funções de utilizador resolve-se
pela recursão (Sequence dentro de Sequence).

Guardas: função de utilizador FORA de math é inafectada por construção
(`apply_math_default` só corre a partir de `layout_equation`).


## P1132m — colapso de `HSpace` fraco nas bordas do run matemático

Depois de achatar `Sequence`/`MathSequence`, `layout_sequence` remove um
`Content::HSpace` com `weak=true` quando não existe item material antes ou
depois dele no run (ignorando align points e quebras). Entre dois itens ele
permanece e resolve `Length` pelo `style.size` activo. Isto reproduz o
`HElem(THIN, weak)` que prefixa `dif`: `$dif x$` não ganha margem inicial,
mas `$f dif x$` ganha `1/6em` imediatamente antes do `d`. Espaços fortes e
fraccionais permanecem com as regras existentes.

## P967b — espaço de texto no limite `&` (item espaçado na grelha multiline e em matrizes)

**Medição** (`typst-passo-967` Fase A; auditoria externa 2026-08-05 secção
8.2 — secção real do documento: `$ (3x+y)/7 &= 9 && "dado" \\ … $`): o
vanilla insere um **espaço de texto** (3.65pt medido = largura do espaço da
fonte, NewCMMath-Book) entre o fim da célula matemática (`9`) e a anotação
entre aspas (`"dado"`) — mecanismo: strings em math são "items espaçados"
(`process.rs::spacing()` do vanilla, ramo `l.is_spaced() || r.is_spaced()
=> return space`), e o espaço é computado **através** do limite `&`
(a grelha do vanilla é um único run — o espaçamento entre fragmentos
adjacentes não para no align point). O cristalino colava "9dado" (0pt).

Duas lacunas na tradução cristalina da regra:

1. **`align_boundary_spacing` sem o fallback de item espaçado**: o helper
   (P825, `matrix.rs`) usa `spacing_between` (catch-all = 0.0). Passa a
   ter a mesma semântica de `compute_gaps` (P903):
   `spacing_between_class(...)` e, se o match cair no catch-all E um dos
   dois lados da aresta for `Content::Text` (ou `Fence` como rclass —
   paridade `is_spaced()` do vanilla, `spacing.rs:211-222`), devolve a
   largura real do espaço (`metrics.advance(" ", size, style)`). Regras
   explícitas de 0.0 (pontuação/abertura/fecho) continuam a ganhar.
2. **A grelha multiline (`layout_grid`) nunca aplicava espaçamento de
   limite**: passava `&[]` para `layout_grid_boxes` (boundary gap = 0).
   Como `partition_grid` só parte em `MathAlignPoint`, **todos** os
   limites da grelha multiline são limites `&` por construção. O
   `layout_grid` passa a medir as células, incorporar
   `align_boundary_spacing(left, right)` na largura da célula esquerda
   (mesmo modelo de P825 para matrizes) e a passar `align_boundaries`
   (todos `true`) para `layout_grid_boxes`. O caminho de matrizes sem `&`
   (`layout_grid_rows`) fica inalterado.

## P973 — convenção: matches sobre `FrameItem` em math/layout são exaustivos (sem braço `_`)

**Data:** 2026-08-05

**Origem**: a varredura de P973 (motivada pela lição de P972 — o braço
`_ => {}` de `frac.rs` escondia `Glyph`/`Line`) inventariou todos os
`match`/`if let` sobre `FrameItem` em `01_core/src/compiler/math/layout/`
(6 sítios: `place`, `offset_item`, extent de `layout_equation_measured`,
`hconcat_spaced`, o find_map de P971 em `attach.rs`, e o split de
`Content` em `matrix.rs`). Veredicto: todos exaustivos e com braços no-op
documentados (`Image`/`Shape`/`Group`/`Link` — "não ocorrem em contexto
math"); o único braço defeituoso era o de `frac.rs`, corrigido em P972.

**Convenção em vigor** (prevenção, decisão registada per o passo):

1. Todo `match` sobre `FrameItem` em `math/layout/` **lista as 8 variantes
   explicitamente** — nunca `_ =>`. A exaustividade fica assim verificada
   pelo compilador: uma variante nova no enum quebra o build no ponto
   certo, em vez de ser engolida por um catch-all.
2. Braços que são intencionalmente no-op (variantes que não ocorrem em
   contexto math) mantêm o comentário a dizer porquê.
3. Translações usam sempre `offset_item` (o único ponto onde cada
   variante sabe transladar-se) — nunca um `match` local novo para
   deslocar items.
4. Decisão sobre lint dedicado (Fase C.3 do passo): **não implementado** —
   custo desproporcionado (6 sítios, todos correctos; a convenção + a
   revisão bastam). Se o padrão voltar a falhar, reavaliar.

**Suspeito fora de escopo registado** (não é math/layout; para passo
futuro): `01_core/src/compiler/layout/sub_frame.rs:306` — o cálculo de
altura de um sub-frame (`layout_sub_frame`) só considera
`Text`/`TextShaped` (`_ => {}` para o resto, com fallback a `line_h`).
Um sub-frame cujo conteúdo é math (items `Glyph`/`Line`) ficaria com
altura subestimada. Precisa de reprodução e investigação próprias — não
confirmado como bug, não corrigido neste passo.

## P990-C — `apply_math_default` cobre `MathCancel`; braço math para `Content::Strike`

**Medição** (achado §8.1 da auditoria; repro `$ cancel(a+b) $` /
`$ std.strike(a+b) $` vs vanilla): o cancel renderizava o corpo em glifo
RETO (ASCII `a+b`) em vez de itálico matemático (`𝑎+𝑏`), e o strike
renderizava o corpo reto E SEM a linha. Causa: `apply_math_default`
(P961/P966) não tinha braço para `Content::MathCancel` — caía em
`other.clone()` e o corpo nunca recebia o itálico por defeito (mesma
família de P966); `Content::Strike` caía no catch-all `plain_text()` do
`layout_node` math, que perdia o itálico E a linha.

**Correcção**:
1. `apply_math_default` ganha braço
   `Content::MathCancel(e) => Content::math_cancel(apply_math_default(&e.body))`
   (espelho do braço `MathAccent` de P961).
2. `layout_node` (math) ganha braço para `Content::Strike` e layouta o corpo
   como math (já com itálico via braço análogo de `apply_math_default`).
   **Retificação P1132q por medição do PDF ratificado**: o show rule do
   vanilla converte `StrikeElem` em decoração de `TextElem`; os glifos
   matemáticos não formam um run textual decorável e, no corpus
   `$ std.strike(a+b) $`, o stream PDF contém os três glifos e zero linhas.
   Logo o braço math preserva o corpo, mas não desenha uma linha própria.
   A afirmação anterior de que o vanilla confirmava a linha foi uma leitura
   visual incorreta e fica revogada. O comportamento textual fora de math
   permanece no layouter de texto.

**Critério**: `$ cancel(a+b) $` e `$ std.strike(a+b) $` com corpo em
itálico matemático; `cancel` contém uma diagonal e `std.strike` em math não
contém `FrameItem::Line`, como o vanilla ratificado.

## P991 — `layout_grid` sem `&`: `GridAlign::Center` em vez de `Alternating` (paridade com o default `CENTER` de `equation.rs`)

**Medição** (`typst-passo-991.md`; achado externo 2026-08-08, secção 7 do
documento de teste, `(n \ k) = n!/(k!(n-k)!)`): na construção manual
`(n \ k)` (quebra de linha `\` dentro de um `MathDelimited`, sem `&`), o
conteúdo de cada linha fica com folga horizontal assimétrica dentro do
delimitador ("n": 0.00pt/0.00pt esq/dir; "k": 0.70pt/0.17pt), em vez de
centrado como `binom(n, k)` (referência, 1.10pt/1.10pt e 1.45pt/1.62pt).

**Causa confirmada por leitura** (`mod.rs::layout_grid`, chamado por
`layout_sequence` quando `needs_grid_layout` — `MathAlignPoint` ou
`Linebreak` presente — e `self.block`): `layout_grid` particiona os nós
com `partition_grid` (linhas por `Linebreak`, colunas por
`MathAlignPoint`) e chama sempre `layout_grid_boxes` com
`GridAlign::Alternating` hardcoded, **independentemente de existir algum
`MathAlignPoint`** nos nós de origem. Para `(n \ k)` — só `Linebreak`,
zero `&` — isto produz 1 coluna por linha, e `Alternating` (coluna par →
alinha à direita) empurra cada linha para a direita da largura da coluna
em vez de a centrar.

**Confirmado no vanilla** (`lab/typst-original/crates/typst-library/src/
math/equation.rs:200`: `out.set(AlignElem::alignment, Alignment::CENTER)`
— o default de alinhamento de uma equação é `CENTER`;
`typst-layout/src/math/run.rs:113-140` `stack_rows`: `has_alignment =
!points.is_empty()`, onde `points` vem de `cumulative_alignment_points
(widths)` sobre as colunas — com 1 coluna só (nunca houve `&`), `points`
fica vazio, `has_alignment = false`, e a linha é posicionada por
`pos.x = align.position(total_width - sub.width())` — o alinhamento
**resolvido** (`CENTER` por omissão numa equação), não a alternância
esquerda/direita por paridade de coluna. `FixedAlignment::Center::
position(extent) = extent / 2.0` — folga simétrica. A alternância
esquerda/direita só entra em jogo quando `has_alignment` é `true`, ou
seja, quando existe pelo menos um `&` real.

**Correcção**: `layout_grid` passa a escolher o `GridAlign` consoante o
número de colunas da grelha particionada (`n_cols`, o mesmo valor já
calculado para `align_boundaries`): `n_cols <= 1` (nenhum `&` nos nós de
origem — só `Linebreak`, caso `(... \ ...)` e multiline math sem `&`) →
`GridAlign::Center`; `n_cols > 1` (pelo menos um `&`) → `GridAlign::
Alternating` (comportamento existente, inalterado). `align_boundaries`
continua a marcar todos os limites internos como `&` (P967b) — nunca
`true` quando `n_cols <= 1`, porque não há limite nenhum a marcar.

**Não afecta** `binom()`/`mat`/`cases` (caminho `matrix.rs` /
`layout_grid_rows`, já `GridAlign::Center` desde a origem — ver
`matrix.md`) nem multiline math com `&` (continua `Alternating`, guarda
de não-regressão: `p967b_grid_limite_com_string_leva_espaco_de_texto`
mantém-se verde).

**Critério**: `(n \ k)` com folga horizontal simétrica em cada linha,
igual em ordem de grandeza à de `binom(n, k)` na mesma posição; `binom()`
inalterado (guarda); multiline math com `&` mantém a alternância
esquerda/direita.

## P1134 — leading de math multilinha vem do estilo de parágrafo

**Medição** (2026-08-23, working tree não commitado sobre HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`; fixture
`.typ/sec_07.typ`, vanilla ratificado `a51e02804`): todos os elementos da
secção 7 coincidem verticalmente até à última fórmula
`$ (n \ k) = n! / (k!(n-k)!) $`. A segunda linha começa no cristalino
5,456pt antes do vanilla e a página auto fica exactamente 5,456pt mais
baixa (288,707pt contra 294,163pt). A divergência é integralmente o gap
entre as duas linhas; `binom(n,k)` já coincide.

**Causa medida na fonte antes da decisão**: o vanilla escolhe o leading em
`lab/typst-original/crates/typst-layout/src/math/run.rs:49-53`: para
`EquationElem::size >= MathSize::Text`, resolve `ParElem::leading`; apenas
em Script/ScriptScript usa `TIGHT_LEADING = 0.25em` (`run.rs:15`). O default
de `ParElem::leading` é `0.65em`
(`typst-library/src/model/par.rs:210`). O cristalino, em `layout_grid`,
usava `MathConstants::math_leading`, constante OpenType da fonte
(NewCMMath: cerca de 0,154em), que não representa esta propriedade da
linguagem.

**Decisão**: `layout_grid` resolve o `row_gap` dinamicamente pelo estilo:

- Display/Text: `style.leading.resolve_pt(style.size)` quando definido;
  caso contrário, o default canónico `PAR_LEADING * style.size`;
- Script/ScriptScript: `0.25 * style.size`, espelhando `TIGHT_LEADING`.

O valor `PAR_LEADING` vem da constante canónica já existente em
`compiler/layout/vanilla_defaults.rs`, com proveniência no vanilla; não é
uma constante empírica da fixture. Matrizes e `cases` continuam a usar o
seu `DEFAULT_ROW_GAP = 0.2em`, pois passam `row_gap` explicitamente por
caminho próprio (P923b).

**Critério de língua**: a última fórmula da secção 7 tem a segunda linha e
a altura de página iguais ao vanilla; alterar `#set par(leading:)` altera o
gap de math multilinha Display/Text pelo mesmo valor resolvido; `binom()` e
matrizes permanecem inalterados.

## P992 — `Content::MathLimitsOverride` (`limits()`/`scripts()`): layout transparente + itálico por defeito

**Contexto**: achado externo 2026-08-07 (secção 32) — `attach()`/`limits()`/
`scripts()` não reconhecidas, viram texto literal e perdem os argumentos
(`typst-passo-992.md`). `limits()`/`scripts()` fecham 2 dos 4 casos —
`attach()` de 6 cantos (`t`/`b` novos em `MathAttachElem`) fica para passo
próprio (ADR-0127: mudança de contrato maior, escopo separado por decisão
do dono). Ver `entities/elements/math_limits_override.md` para o struct.

`layout_node` ganha braço transparente, mesmo padrão de
`Content::MathClassOverride` (§ acima): `Content::MathLimitsOverride(e) =>
self.layout_node(&e.body, style)` — o `body` é layoutado normalmente, sem
caixa/decoração própria do wrapper.

`apply_math_default` ganha braço **recursivo** (ao contrário de
`MathClassOverride`, que não tem — gap pré-existente, fora de escopo):

```rust
Content::MathLimitsOverride(e) => Content::math_limits_override(
    apply_math_default(&e.body),
    e.limits,
    e.inline,
),
```

Necessário porque o caso de uso canónico do achado é uma base de 1 letra
(`limits(A)_1^2`) — sem recursão, "A" nunca recebia o itálico por defeito
(mesma família de bug de P961/P966/P990-C: wrapper novo sem braço de
recursão). `apply_math_style` (`bb()`/`bold()`/etc., P311b.4) **não** ganha
braço — cai no catch-all `other => other.clone()`, mesmo estado que
`MathClassOverride` já tem aí; fora do escopo dos 4 casos do achado
(nenhum combina `limits()`/`scripts()` com wrappers de estilo).

`base_math_class` (`math/layout/spacing.md` §P992) é transparente ao
`body` — `limits()`/`scripts()` não afectam a classe/espaçamento
(diferente de `MathClassOverride`, que força a classe). O discriminador
`is_limits` de `layout_attach` (`math/layout/attach.md` §P992) é onde o
override de facto actua.

**Critério**: `limits(A)^alpha_beta`/`scripts(A)^alpha_beta` — "A" continua
itálico (𝐴); `is_empty`/`plain_text`/`map_content` transparentes ao body.

## P994 — `layout_external`: o catch-all de `layout_node` delega ao `Layouter` normal (Opção β)

**Diagnóstico**: `diagnostico-math-aninhado-layout-fase-a-passo-993.md` —
o catch-all `plain_text()` de `layout_node` achatava variantes
não-matemáticas de `Content` (`Styled`/`Box`/`Align`/`Pad`/`Block`),
matando itálico, `^`/`_`, tamanhos e caixas. Desenho aprovado pelo dono
(`typst-passo-994.md`, Opção β). Vanilla: `ExternalItem`
(`ir/resolve.rs:228-230` → `layout_external`, `typst-layout/src/math/
mod.rs:585-603`) + `BoxItem` (`resolve.rs:192-194`).

**Respostas da Fase A (leitura, registadas antes de código)**:

1. **`Layouter` genérico**: `Layouter::new(metrics: M, sizer: S,
   font_size: f64, introspector: Tracked<dyn Introspector>)`
   (`compiler/layout/mod.rs:642`); existe `impl FontMetrics for &dyn
   FontMetrics` (`metrics.rs:369`) — o `MathLayouter` passa
   `self.metrics` (`&'a M`) coagido a `&dyn FontMetrics`, mesmo padrão de
   `measure_content_real` (`compiler/layout/mod.rs:2123`). Geometria real,
   não `FixedMetrics`.
2. **`layout_sub_frame`** devolve `(height, items, deco, orphaned_x,
   orphaned_y)`; região `SubLayoutRegion { origin_x: 0.0, width:
   f64::INFINITY, height: None, align_rtl: false, unconstrained_height:
   true }` (precedente `measure_content_real:2142-2151`; `deco`/órfãos
   descartados com a instância, nota P908). Largura ilimitada — decisão
   registada: vanilla usa `ctx.region`; em página auto o efeito coincide.
3. **`FrameItem::Group`** (`entities/layout_types.rs:430`) chega como
   está (`pos`, `matrix: TransformMatrix::identity()`, `clip_mask: None`,
   `inner_width`, `inner_height`, `items`) — padrão de construção em
   `block.rs:206`. **`MathBox` não precisa de contrato novo**:
   `items: Vec<FrameItem>` já aceita `Group`; `offset_item` já trata
   `Group` (`math/layout/mod.rs:206`). O walker de estilo math
   (`mod.rs:92,1147`) ignora `Group` — desejado: o conteúdo embutido
   gere os seus próprios estilos.
4. **Cadeia de estilos**: reconstruída de `TextStyle` —
   `StyleChain::default_chain()` + `push_styles` com os campos suportados
   por `Style` (`Size`, `Font`, `Weight`, `Tracking`, `Fill`, `Lang`, …)
   lidos do `style` corrente; `layouter.style = style.clone()` por cima.
   É o que faz o `text(size: 20pt)` aplicar-se ao conteúdo embutido.
5. **Introspector**: `TagIntrospector::empty()` (precedente `measure`) —
   labels/links dentro de conteúdo externo em math não resolvem;
   limitação registada (igual à de `measure()`).

**Sem mudança de contrato público** (ADR-0127 — fluxo contínuo):
`Content` (enum fechado) intocado; `MathBox` inalterado; API pública do
`MathLayouter` (`layout_equation`) inalterada; o novo método é
`pub(super)` interno.

**Mecanismo** (novo método `pub(super) fn layout_external` em
`math/layout/mod.rs`, chamado pelo braço final de `layout_node` em vez
do `plain_text()`):

1. Construir `Layouter` temporário com `self.metrics` coagido a `&dyn
   FontMetrics`, `NullImageSizer`, `style.size`, introspector vazio;
   cadeia reconstruída (item 4 acima).
2. `layout_sub_frame(content, região ilimitada)` → `(height, items)`.
3. `width = metrics.line_content_right(&items)` (método já existente,
   usado por `measure_content_real`).
4. Baseline do bloco embutido: vanilla `layout_external` usa
   `height/2 + axis_height` quando o frame não declara baseline
   (`math/mod.rs:596-599`). `ascent = height/2 + axis_pt`,
   `descent = (height/2 − axis_pt).max(0.0)`, `axis_pt` do já existente
   `self.constants.axis_height` convertido por `to_pt`.
5. Devolver `MathBox { width, ascent, descent, items:
   vec![FrameItem::Group { pos: (0, −ascent), matrix: identity,
   clip_mask: None, inner_width: width, inner_height: height, items }] }`
   — o topo do frame embutido fica `ascent` acima da baseline math,
   em espaço Y-down local.

**Guarda**: o catch-all continua a devolver caixa vazia para conteúdo
cujo `plain_text` é vazio E cujo layout externo não produz itens
(preserva o comportamento para `HSpace`-like desconhecidos); a matemática
pura (sem funções de layout) não é tocada — os braços específicos de
`layout_node` ganham sempre antes do catch-all.

**Critério**: os 6 casos do diagnóstico (A1–A4c) corrigidos — itálico,
sobrescrito e tamanhos preservados em `text()`, caixa visível em
`box()`, `align`/`pad`/`block` com conteúdo matemático real; não-regressão
da suíte. O fix da causa secundária (`align` confundir o ident de
alinhamento com o corpo) é separado — ver `compiler/eval.md` §P994.

## P994 (causa secundária) — idents de alinhamento em chamadas math de funções de layout

`eval_math_arg_value` (`compiler/eval/math.rs:193`) avaliava todo o
posicional como `Value::Content` — o ident `center` virava body de
`native_align` (que toma o primeiro `Content` como corpo). Correcção: na
avaliação de argumentos posicionais de chamadas em modo math, um
`Expr::MathIdent` que resolve no scope para um valor **não-Content**
(ex.: `Alignment`/`Value::Auto`/etc.) é avaliado pelo caminho de scope
(`eval_math_callee`) em vez de empacotado como Content; idents que
resolvem para `Value::Func`/bindings de utilizador mantêm o caminho
actual. Vanilla resolve `center` como `Alignment` no mesmo ponto
(`ir/resolve.rs` — o arg de `align` não é Content). Critério:
`$ #align(center)[$a+b$] $` produz o corpo `a+b` itálico (sem vazamento
de "center"), com o alinhamento aplicado.

### P994 — adenda pós-Fase B (medição do Agente B, ADR-0108: refuta a Fase A no item 3)

Duas correcções à Fase A, medidas pelo Agente B antes de código (protocolo
respeitado: parou e reportou em vez de alargar âmbito por conta própria):

1. **Visibilidade**: `layout_sub_frame` e `SubLayoutRegion` são
   `pub(super)` = `pub(in crate::compiler::layout)` (`sub_frame.rs:48,21`) —
   `compiler::math` é módulo-irmão, não descendente. Alargamento mínimo:
   `pub(crate)` nos dois itens + `pub(crate) use sub_frame::SubLayoutRegion`
   em `compiler/layout/mod.rs`. Mudança de visibilidade interna — não é
   contrato público (fluxo contínuo mantém-se).
2. **Consumidor final descarta `Group`**: a integração dos items da equação
   na página (`compiler/layout/equation.rs:241`) tem `FrameItem::Group { ..
   } => {} // grupos não ocorrem em math inline` — o Group seria
   **descartado** mesmo com `layout_external` perfeito. Braço novo aí:
   push do `Group` com `pos + (offset_x, offset_y)` e avanço do cursor por
   `inner_width` (mesmo padrão do braço `Text`). Idem `Shape` (`:240`,
   borda do `box()`). E `hconcat` (`math/layout/mod.rs:1147`) ganha braço
   `pos.x += x` para `Group` (hoje não desloca em X).
3. **Guarda extra proposta pelo Agente B e aceite**: `items` vazio do
   sub-frame MAS `plain_text` não vazio → fallback ao antigo
   `layout_text_node` (não perder texto silenciosamente — o caso não
   estava coberto no desenho original).

### P994 — adenda 2 (desvios da implementação, medidos pelo Agente B)

1. **Discriminador do catch-all é deny-list, não delegação cega** — a
   rotação de TUDO para `layout_external` foi **refutada por medição**
   (quebrou 10 testes pré-existentes; e não era artefacto de harness:
   `Content::Text`/markup em math, ex. `"dado"` em `$ 9 & "dado" $`,
   ficava centrado no eixo (−4.5pt da baseline vizinha) — regressão real
   face ao vanilla, que re-resolve markup como math e só cria
   `ExternalItem` para o não-resolúvel, `ir/resolve.rs:127-146`). Regra
   final: `needs_external_layout` sobe para `layout_external` só conteúdo
   que contém (recursivamente, via `Sequence`/`Styled`)
   `Equation`/`Boxed`/`Align`/`Pad`/`Block`; o resto mantém o caminho de
   texto. Mais próximo do vanilla que o desenho original.
2. **`layout_sub_frame` ficou `pub(in crate::compiler)`** (não
   `pub(crate)`): `pub(crate)` disparava `private_interfaces` (a
   assinatura devolve tipos `pub(in crate::compiler)`). É o mínimo que
   alcança `compiler::math`.
3. **`equation.rs`**: braços `Group`/`Shape` na integração dos items da
   equação (pos absoluta + avanço do cursor por `inner_width`).
4. **Cadeia reconstruída**: `Size`/`Bold`/`Italic` sempre;
   `Fill`/`HeadingLevel`/`Weight`/`Tracking`/`Leading`/`Lang`/`Font`
   quando `Some`; sem variante `Style` ficam fora (`dir`, edges,
   `baseline_offset`, `variations`, flags math).
5. **Correcção dos testes P994**: codepoint do b itálico era U+1D45F
   (que é 𝑟) no teste do Agente A — corrigido para U+1D44F (𝑏) pelo
   Agente B, verificado via unicodedata e contra o snapshot do
   teste-guarda.

### P994 — adenda 3 (fix de posicionamento vertical, revisão do orquestrador)

Duas afirmações do desenho original foram **refutadas por medição** na
revisão do orquestrador (caso composto `text()`∋`box()`∋math + renders de
e1/e3) e corrigidas pelo Agente B:

1. **Âncora vertical é a baseline declarada do frame, não
   `height/2 + axis` incondicional** — os items de `layout_sub_frame` são
   baseline-ancorados (o `pos.y` de um `Text` é a baseline da linha) e o
   `height` devolvido mede desde a primeira baseline (`sub_frame.rs:239`).
   O vanilla só usa `height/2 + axis` quando o frame **não** declara
   baseline (`if !frame.has_baseline()`, `math/mod.rs:596`) — conteúdo com
   texto declara. Implementação: `scan_external_verticals` (primeiro
   `Text`/`Glyph` por ordem do documento = baseline; extents de tinta via
   `text_ink_bounds`/`cap_height`); com texto: `ascent = baseline −
   ink_top`, `descent = ink_bottom − baseline`; sem texto: fórmula do
   vanilla sobre extents reais. e1 ficou idêntico ao vanilla ao centésimo
   de ponto (𝑏 yMin 26.108 vs 26.1065).
2. **Embutimento ACHATADO, não `FrameItem::Group`** — medição no content
   stream cru provou incoerência PRÉ-EXISTENTE no exportador PDF para
   `Group` com filhos de texto (`03_infra/src/export/stream.rs:1348-1361`:
   o `cm` não inverte Y com matriz identidade; erro = exactamente
   2×y_local; o renderer raster assume o contrário) — `#box(height: 6pt,
   clip: true)[hello clip]` fora de math já perde o texto hoje.
   **Candidato a passo próprio** (afeta clip groups em geral, não só
   math). O `layout_external` achata os items do sub-frame directamente
   na `MathBox` (translados por `−anchor` via `offset_item`), fluindo
   pelos caminhos de emissão já provados; braços `Shape` novos em
   `hconcat` (`pos.x += x`) e no extent de `layout_equation_measured`.
   Os braços `Group` ficam para grupos aninhados vindos de dentro do
   conteúdo embutido (estado pré-existente, dentro/fora de math por
   igual).
3. Ressalva pré-existente medida e fora de scope: a altura do box inline
   (`boxed.rs:186-196`, `outer_h = line_h`, inset só horizontal) deixa o
   texto junto à borda inferior — **igual fora de math** (render de
   comparação `#box(stroke:)[hello]` em texto corrido tem a mesma
   geometria). Débito de `boxed.rs`, não de P994.

### P994 — adenda 4: o custo da deny-list, medido e fechado (P1026/P1027, 2026-08-13)

A adenda 2 item 1 fixou a regra: `needs_external_layout` (`math/layout/mod.rs:249-262`)
sobe para `layout_external` **só** conteúdo que contém, recursivamente,
`Equation`/`Boxed`/`Align`/`Pad`/`Block`. `Content::Styled(Text, …)` **não** estava na
lista → ficava no caminho de texto de math, que **descarta o override de tamanho**.

O custo dessa regra nunca foi medido. Foi medido em P1026 e fechado em P1027.

**Proveniência das medições**: `HEAD = 0f8487b9d`, árvore limpa, 2026-08-13.
Extensão da tinta da página, PDF → PGM 300dpi, mesmo documento nos dois binários
(vanilla ratificado `/usr/local/bin/typst` e `lab/typst-original/target/release/typst`;
cristalino `./target/release/typst`).

| documento | vanilla | cristalino |
|---|---|---|
| `#text(size: 40pt)[x]` **fora** de math | 18,24 × 17,28pt | 18,24 × 17,04pt ✔ |
| `$ #text(size: 40pt)[x] $` | 20,16 × 17,28pt | **5,52 × 4,80pt** ✘ |
| `$ mat(#text(size: 40pt)[x]) $` | 31,20 × 22,80pt | **14,40 × 10,80pt** ✘ |
| `$ lr(( #text(size: 40pt)[x] )) $` | 30,24 × 24,48pt | **12,24 × 10,80pt** ✘ |
| `$ cases(#text(size: 40pt)[x]) $` | 27,12 × 23,28pt | **11,28 × 10,80pt** ✘ |

**Fase B (P1027) — outras propriedades de `text()` em markup bare dentro de `$…$`**,
medição via `pdftotext -bbox` e rasterização 300dpi:

| propriedade | vanilla | cristalino | conclusão |
|---|---|---|---|
| `#text(size: 40pt)[x]` | ink 21,12 × 40,00pt | ink 5,81 × 11,00pt | **perdido** ✘ |
| `#text(fill: red)[x]` | sRGB (cor), ink 5,81 × 11,00pt | Grayscale (sem cor), ink 5,81 × 11,00pt | **cor perdida** ✘ |
| `#text(weight: "bold")[x]` | ink 6,68 × 11,00pt | ink 5,81 × 11,00pt | **peso perdido** ✘ |
| `#text(style: "italic")[x]` | ink 5,81 × 11,00pt | ink 5,81 × 11,00pt | sem diferença visual (math já itálico) ✔ |
| `#text(style: "normal")[x]` | ink 5,50 × 11,00pt | ink 5,50 × 11,00pt | sem diferença visual mensurável ✔ |

Fora de math todas as propriedades aplicam-se; **dentro** de math, `size`, `fill` e
`weight` são ignorados. `style` não evidencia perda porque o texto matemático já é
itálico por defeito.

**Porque é que os testes-guarda de P994 passavam**: `p994_text_size_aplica_tamanho_e_mantem_math`
e `p994_text_size_com_superscript_real` (`compiler/layout/tests.rs:19777,19801`) usam
`#text(size: …)[$…$]` — com equação **aninhada**, que entra na allow-list por
`Equation`. O caso sem `$…$` aninhado nunca foi coberto.

#### Respostas às três perguntas deixadas em P1026

1. **Onde o override de tamanho entra no fluxo vanilla?**  
   O vanilla re-resolve markup como math (`ir/resolve.rs:127-146`,
   `resolve_into_self`) e só cria `ExternalItem` para o não-resolúvel. No
   cristalino, a via equivalente é a deny-list `needs_external_layout`; o
   `layout_external` já reconstrói a cadeia de estilos a partir do `TextStyle` da
   equação e aplica os estilos do `Content::Styled` via `StyleChain`
   (`math/layout/mod.rs:766-796`). Logo, basta que `needs_external_layout` reconheça
   o `Styled` como "não realizável como texto math".

2. **O alargamento pode ser restrito a `Styled` cujo `Style` contém as propriedades
   que o caminho de texto de math não consegue aplicar?**  
   **Sim.** O catch-all (`math/layout/mod.rs:713-719`) chama
   `layout_text_node(&text, style)`, onde `style` é o estilo da **equação**; o
   `Styles` do `Content::Styled` é descartado quando o conteúdo é achatado para
   `plain_text()`. Portanto, qualquer `Styled` que carregue um delta tipado de
   texto deve subir por `layout_external`. Medições de posicionamento vertical
   confirmam que, para `fill` e `weight`, o vanilla mantém o alinhamento na
   baseline; para `size`, o vanilla coloca o texto grande centrado no eixo (comportamento
   de paridade, não regressão).

3. **Qual o efeito no posicionamento vertical que a adenda 2 mediu como regressão?**  
   A regressão da adenda 2 (delegação cega de markup puro para `layout_external`)
   aplica-se apenas a conteúdo **sem** estilos de texto — o `"dado"` de
   `$ 9 & "dado" $` e o output de funções de utilizador. Restringir o
   alargamento a `Styled` com delta tipado de texto mantém esse conteúdo no
   caminho baseline-alinhado. Medições de P1027 para `$ a + #text(size: 40pt)[x] + b $`:
   o vanilla coloca o `x` grande centrado no eixo (`yMin=-15,0`, `yMax=25,0`) enquanto
   `a`/`b` ficam na baseline (`yMin=8,374`, `yMax=19,374`); esse é o alvo de paridade.

#### Decisão (gate ADR-0127 categoria 2 — mudança de comportamento por defeito)

Alargar `needs_external_layout` para que `Content::Styled(body, styles)` retorne
`true` quando `styles.delta()` tiver qualquer propriedade tipada de texto definida
que o catch-all de math não consiga aplicar: `size`, `fill`, `weight`, `font`,
`tracking`, `leading`, `lang`, `bold`, `italic`. O corpo continua a ser verificado
recursivamente (`Equation`/`Boxed`/`Align`/`Pad`/`Block`).

**Não-regressão**: markup puro sem `Styled` (`"dado"`, funções de utilizador) e
`Styled` semanticamente vazio (transporte `custom`) mantêm o caminho de texto
baseline-alinhado — exatamente o comportimento protegido pela adenda 2.

**Testes-guarda**: os 6 testes de P994 (`#text(size: …)[$…$]` com equação aninhada)
continuam a passar porque a equação aninhada já activava a allow-list.

**Critério de aceitação P1027**:
- `$ #text(size: 40pt)[x] $` → tamanho aplicado, batendo com vanilla.
- `$ #text(fill: red)[x] $` → cor aplicada.
- `$ #text(weight: "bold")[x] $` → peso aplicado.
- `$ #text(style: "italic")[x] $` e `$ #text(style: "normal")[x] $` → sem regressão
  (math já itálico por defeito).
- `$ 9 & "dado" $` e markup puro sem estilos → mantêm alinhamento baseline.
## P1132o — spacing de sequência respeita `MathSize`

Ao chamar `spacing::compute_gaps`, `layout_sequence` marca `in_script` se
`TextStyle::math_script` estiver ativo ou se `math_size` for `Script` ou
`ScriptScript`. Assim frações aninhadas seguem a condição tipográfica do
vanilla e não recebem espaços de classe em tamanhos de script.

## P1132s — largura de texto literal em matemática

`Content::Text` corresponde ao `TextItem` vanilla. Somente quando o sucessor é
um `MathDelimited` (fronteira texto → delimitador de abertura), o avanço
terminal reservado pelo caminho textual (a largura de `" "` no estilo ativo)
é retirado antes da concatenação; o espaço semântico entre itens continua
exclusivamente em `compute_gaps`. Nas demais fronteiras e quando é terminal, a
largura completa participa da extensão e da centralização de scripts/limites.
Assim `"Res"(f)` encosta à abertura, enquanto `p "prime"` recebe exatamente
um espaço e permanece centrado sob `∏`. Todas as grandezas vêm da fonte e da
posição estrutural, não da string ou da fixture.

**Guarda de regressão P1132t:** conteúdos textuais adjacentes produzidos por
funções de markup, como `bra(phi) ket(psi)`, não são delimitadores matemáticos
estruturais e não podem ser apertados. Conservam a largura completa de cada
frame, tal como antes de P1132s.

A normalização de sequência preserva essa morfologia: `MathSequence` é sempre
achatada; `Content::Sequence` permanece atômica somente quando todos os filhos
são folhas textuais (`Text`/`MathText`/`MathIdent`), como no corpo de
`bra`/`ket`. Sequências técnicas com `HSpace` (`dif`) e sequências com nós
matemáticos compostos (`hat`, attach, delimitados etc.) são abertas, para não
perder sua semântica de layout. Assim os fragmentos internos de markup não
adquirem `is_spaced()` entre si, sem transformar acentos em texto plano. A
decisão depende das variantes estruturais, não dos caracteres ou coordenadas.

A sequência textual atômica conserva, como unidade, a propriedade `spaced` do
`TextItem` e a largura completa do seu frame; `compute_gaps` insere o espaço
externo. O aperto de cola terminal continua reservado a `Content::Text`
diretamente seguido por `MathDelimited`, não a caixas de markup. Portanto o
interior de `⟨φ|` permanece compacto, mas as fronteiras `bra→ket`, `bra→hat` e
`hat→ket` recebem um espaço tipográfico.

## P1132w — avanço de glifo matemático inclui a correção itálica

**Medição antes da decisão.** No vanilla ratificado, a construção de qualquer
`GlyphFragment` lê `MathItalicsCorrectionInfo` e soma a correção ao
`x_advance` quando o glifo não é `extended_shape`
(`lab/typst-original/crates/typst-layout/src/math/fragment/glyph.rs:207-216`).
O cristalino lia essa mesma métrica apenas para anexos e esticamento, mas
`layout_text_node` usava somente `FontMetrics::advance`
(`01_core/src/compiler/math/layout/mod.rs:1233`). Na sonda da seção 40, `⋆`
tem correção de `25du = 0,275pt` a 11pt: cada fronteira `⋆ → +` ficava
`0,275pt` curta; quatro repetições reduziam dinamicamente a base e a chave em
`1,1pt`. A tinta do glifo tinha a mesma caixa nos dois renders; a divergência
era de avanço, não de escala do desenho.

**Contraprova de escopo.** Aplicar a soma indiscriminadamente em
`layout_text_node` alterou também as letras das equações aninhadas em `box()`;
essas caixas já coincidiam com o vanilla antes da mudança. Portanto essa via
já transporta a largura efetiva por outra fronteira e uma soma global duplica
a métrica. A perda medida ocorre na transparência de `MathClassOverride`, que
descartava a propriedade do fragmento ao devolver apenas o box do corpo.

**Decisão.** Quando `MathClassOverride` envolve diretamente uma folha
`Text`/`MathText`/`MathIdent` com exatamente um caractere, o box do wrapper conserva
`char_italics_correction` no avanço. Tal como no vanilla, não soma quando o
caractere é uma forma extensível: essa condição é derivada das construções
MATH verticais/horizontais (`GlyphVariants` ou `GlyphAssembly` não vazias),
sem lista de caracteres nem constante empírica. Outros corpos e folhas fora
do wrapper conservam a via existente.

**Aceitação linguística.** Símbolos reutilizados por `math.class` conservam o
avanço definido pela própria fonte; delimitadores/operadores extensíveis e
equações aninhadas em caixas não recebem correção duplicada. Construções
dependentes da largura, como `underbrace`, continuam a derivar sua extensão
exclusivamente da base resultante.

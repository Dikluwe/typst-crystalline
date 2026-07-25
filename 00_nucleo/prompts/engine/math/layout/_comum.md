# Prompt L0 — `rules/math/layout` — comum (MathLayouter + despacho)
Hash do Código: 8f57fff8

## Módulo
`01_core/src/engine/math/` — motor de layout matemático.

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
`engine/layout/equation.rs` soma `cursor_y` (ver `engine/layout/equation.md`).

**P813** — `layout_equation_measured` devolve `(Vec<FrameItem>,
EquationExtent)` com `width`/`ascent`/`descent` da equação, calculados dos
mesmos items de `layout_equation` (não é um segundo caminho de layout):
`width` = limite direito máximo (`pos.x + advance`), `ascent`/`descent` =
limites de tinta acima/abaixo da baseline via `FontMetrics::text_ink_bounds`
(paridade vanilla — frame math usa bboxes de glyphs). Consumidor:
`engine/layout/equation.rs` (centragem + espaçamento de bloco P813 — ver
`engine/layout/equation.md`).

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
`engine/layout.md` §P893) precisa de `style` para `FallbackFontMetrics` resolver a face MATH activa
— antes, `constants` era computado sem `style`, sempre com `MathConstants::fallback()` efectivo
nessa variante. Único call site de produção: `engine/layout/equation.rs` (`engine/layout/
equation.md` §P893). Os ~48 call sites de teste em `tests.rs` (`MathLayouter::new(&FixedMetrics,
true)`) ganham um terceiro argumento (`&default_style()`) — mudança mecânica, `FixedMetrics` não
sobrepõe `math_constants`, resultado inalterado independentemente do `style` passado.

`MathBox` (4 campos `pub(super)`): caixa intermédia com
`ascent`/`descent`/`width`/`items` para composição hierárquica.

**P825** — a passagem 2 (posicionamento) de `layout_grid_rows` vive em
`layout_grid_boxes(grid_boxes, align, column_gap, align_boundaries, style)`:
aceita células já medidas e, por linha/coluna, a marca `align_boundaries`
(limite produzido por `&` — sem `column_gap`; o espaçamento de classe é
incorporado na largura da célula par pelo caller). Consumidor actual:
`matrix.rs` (sub-D de P825 — ver `matrix.md`). `layout_grid_rows` mede e
delega com `align_boundaries` vazio.

## Consumers de tipos de domínio (P255 reconciliação)

- **`MathConstants`** → `mod.rs` (construtor `new` via `metrics.math_constants()`);
  `apply_axis_offset` consome `axis_height`; submódulos `frac`/`attach`/`root`
  consomem campos específicos (ver os finos). `MathConstants::fallback()` é o
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

## P906 — `layout_underover`/`layout_accent` esticam over/under/accent de 1 carácter

Ver `engine/layout.md` §P906 (contexto completo — vanilla, dados da fonte, decisão de design) e
`math/layout/stretchy.md` §P906 (`layout_stretchy_glyph_horizontal`, o método consumido aqui).

**Achado** (`typst-passo-899-relatorio.md`, Parte C adiada): `layout_underover` centra `over`/
`under` no seu tamanho natural (`let dx = (w - ob.width) / 2.0`); `layout_accent` faz o mesmo para
`accent`. Nenhum dos dois estica para cobrir a largura da base — `hat(a+b)` produz um circunflexo do
tamanho de 1 carácter sobre uma expressão larga, sem relação visual com ela.

**Correcção**: antes de `self.layout_node(c, style)` para `over`/`under` (em `layout_underover`) e
para `accent` (em `layout_accent`), um guard: se o conteúdo é `Content::MathText(s)` com exactamente
1 carácter, calcula `min_width_du = base_box.width * self.constants.upem /
style.size.val().max(0.001)` e chama `layout_stretchy_glyph_horizontal(char, min_width_du, style)`
em vez de `layout_node`. Para qualquer outro conteúdo (multi-carácter, sequência, etc.) — incluindo
a **anotação** de `underbrace`/`overbrace` (texto normal, nunca deve esticar) — comportamento
**inalterado** (continua `layout_node`). Não precisa de verificar antecipadamente se o char tem
dados de esticamento: `layout_stretchy_glyph_horizontal` já faz fallback a `layout_text_node` quando
não há variantes nem assembly (mesmo padrão de `layout_stretchy_delimiter`, chamado incondicionalmente
para qualquer delimitador desde P255) — chamar sempre e deixar a função decidir é mais simples e não
duplica a lógica de detecção.

**Alcance confirmado com o dono**: cobre os dois casos que a fonte suporta com dados reais — chaves/
colchetes (`layout_underover`, motivador original de P899 Parte C) e acentos largos `hat`/`tilde`
sobre base multi-carácter (`layout_accent`, já aceite como limitação cosmética menor em P899 Parte A,
agora corrigida como efeito colateral barato do mesmo mecanismo).

**Critério**: `hat(a+b)`/`tilde(a+b)` — a largura do `MathBox` do accent é maior que a de `hat(a)`
sozinho (esticou, não é o glifo de 1 carácter). `underover(a+b+c, over: Content::MathText("⏞"))` —
idem para `over`. Anotação (texto multi-carácter) mantém-se centrada no tamanho natural, não estica
(regressão explícita a testar — é o caso que distingue "esticar sempre" de "esticar só quando é o
guard de 1 carácter stretchy").

## P906 (cont.) — `layout_underover`/`layout_accent` usavam convenção "topo do box"

**Achado, só visível ao confirmar visualmente `underbrace(a+b+c, "soma")`** (com anotação — estrutura
aninhada de 2 `MathUnderover`, `eval.md` §P906): a chave/annotação apareciam sobrepostas, ilegíveis.
`underbrace(a+b+c)` SEM anotação (1 nível, sem aninhamento) já renderizava correctamente — isolou o
problema à COMPOSIÇÃO (`MathUnderover` usado como `base` de outro `MathUnderover`/`MathAccent`), não
ao esticamento em si (secção acima).

**Causa**: mesmo padrão já corrigido em `frac.rs` (P905) e `root.rs` (P901), nunca antes auditado em
`layout_underover`/`layout_accent` — `over`/`base`/`under` (e `accent`/`base`) eram posicionados por
offsets crescentes a partir de `Pt(0.0)` (convenção "topo do box"), não pela convenção
baseline-relativa confirmada em P901 (`local_y=0` é a BASELINE PRÓPRIA da `MathBox`, y cresce para
baixo). Funcionava por coincidência quando a caixa era o conteúdo de TOPO da equação (P899 Parte A,
`hat(a)` sozinho) — `place()` no topo cancela o termo `-ascent` independentemente da convenção
interna — mas quebrava assim que a caixa fosse usada como sub-caixa de outra (aninhamento, ou
`hconcat_spaced` com irmãs).

**Correcção** (`layout_underover`): base fica em `Pt(0.0)` (a sua própria baseline já é `local_y=0`);
`over_y = -(base_box.ascent + over_box.descent)` (a baseline do over sobe o suficiente para o seu
descent parar exactamente no topo da tinta da base); `under_y = base_box.descent + under_box.ascent`
(simétrico, para baixo). `layout_accent`: mesmo princípio, `accent_y = -(base_box.ascent +
accent_box.descent)`. `ascent`/`descent` (valores escalares de ambas) **não mudaram** — só os
offsets dos items.

**Efeito colateral**: a ORDEM de push em `items` mudou (base agora primeiro, depois over/under —
antes era over primeiro) — puramente um detalhe de implementação, irrelevante para o render (é só
uma lista de items desenháveis), mas quebrou 1 teste pré-existente que assumia posição por índice
(`items.first()`) em vez de por conteúdo — corrigido para procurar por conteúdo, mesmo padrão já
usado nos outros testes desta família.

**Fora de âmbito, achado novo registado**: com a correcção, a estrutura aninhada renderiza
correctamente (chave visível, anotação legível, sem sobreposição) mas com um gap visualmente maior
do que o vanilla entre `base` e a chave — `layout_underover` não usa nenhuma constante de gap
explícita (empilha directamente por `height()`/ink-to-ink), ao contrário do vanilla que usa
`underbar_vertical_gap`/`overbar_vertical_gap` da tabela MATH. Candidato a passo dedicado.

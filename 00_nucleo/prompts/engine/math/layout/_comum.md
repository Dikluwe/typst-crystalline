# Prompt L0 — `rules/math/layout` — comum (MathLayouter + despacho)
Hash do Código: 18fdb0f0

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

1. **`stack_tight_above(base_ascent: f64, top_descent: f64) -> f64`** — devolve o `local_y` da
   baseline de uma caixa `top` empilhada rente ao topo da tinta de `base` (`-(base_ascent +
   top_descent)`), convenção `y=0=baseline própria` (ADR-0123). Free function (sem `self` — pura
   aritmética, mesmo padrão de `offset_item`). Confirmado idêntica byte-a-byte em
   `underover.rs` (`over_y`, layout de `over`) e `accent.rs` (`accent_y`) — ambas
   implementavam `-(base_box.ascent + <top>.descent)` separadamente. **Não** inclui o espelho
   "abaixo" (`under_y` de `underover.rs`): esse termo não tem segundo consumidor confirmado —
   critério do próprio P918 ("não forçar generalização se só houver um consumidor real hoje")
   — fica inline em `underover.rs`.
2. **`grid_delim_target_du(&self, grid_box: &MathBox, style: &TextStyle) -> f64`** — converte a
   altura de tinta de uma grelha (`ascent+descent`, margem de 10%, P912) para design units, para
   dimensionar o delimitador esticável que a envolve. Método `pub(super)` (precisa de
   `self.constants.upem`). Confirmado idêntico byte-a-byte em `cases.rs` (`layout_cases`) e
   `matrix.rs` (`layout_matrix`) — bloco de 5 linhas (`grid_height_pt = (ascent+descent)*1.1`,
   conversão condicional para du) duplicado sem variação.

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

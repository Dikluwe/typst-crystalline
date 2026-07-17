# Prompt L0 — `rules/math/layout` — comum (MathLayouter + despacho)
Hash do Código: 45c00e9e

## Módulo
`01_core/src/rules/math/` — motor de layout matemático.

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
    pub fn new(metrics: &'a M, style: &TextStyle) -> Self;
    pub fn layout_equation(&mut self, body: &Content, style: &TextStyle) -> Frame;
    // pub(super): apply_axis_offset, layout_node, layout_text_node,
    // layout_sequence, layout_grid_rows, layout_grid, hconcat.
}
```

`MathBox` (4 campos `pub(super)`): caixa intermédia com
`ascent`/`descent`/`width`/`items` para composição hierárquica.

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

## Espaçamento automático por `MathClass` (P772y — ver `spacing.md`)

`layout_sequence` computa `spacing::compute_gaps` sobre os nós filtrados
(antes do layout) e concatena via `hconcat_spaced` em vez de `hconcat`
plano. Handler `Content::MathClassOverride(e) => self.layout_node(&e.body,
style)` em `layout_node` — a classe forçada por `math.class(class, body)`
só entra no cálculo de espaçamento (`spacing::node_math_class`), o layout
do body é normal. Detalhe completo, tabela de espaçamento e critérios de
verificação: `spacing.md`.

## Critérios de verificação (gerais)

- `MathIdent("x")` → `Frame` com `FrameItem::Text { text:"x", .. }` não vazio.
- `MathSequence([x,+,y])` → `Frame` com 3 items.
- Integração: `Content::Equation` delega ao `MathLayouter`.
- `apply_axis_offset` consome `axis_height` real (não zero).
- Primes (`′″‴⁗`) renderizam correctamente (resolvidos em eval).
- Suite math layout: ~50 tests pré-P311b.4 + 8 = ~58 verdes; auto-itálico para
  `MathIdent` sem wrap preservado (caminho default não-MathStyled).

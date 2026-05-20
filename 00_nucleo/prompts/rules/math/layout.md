# Prompt L0 — motor de equações
Hash do Código: 7a10f542

## Módulo
`01_core/src/rules/math/`

## Propósito
Motor de layout matemático — recebe `Content::Equation` e produz `Frame`s
com `FrameItem::Text` posicionados.

## Estado actual (pós-P96.8 + reconciliação P255)

P96.8 reestruturou `rules/math/layout/` em **8 submódulos**
(de monólito → cluster). Submódulos canónicos:

- `mod.rs` — `MathLayouter` struct + `MathBox` + métodos coord
  (`new`, `apply_axis_offset`, `layout_equation`, `layout_node`,
  `layout_text_node`, `layout_sequence`, `layout_grid_rows`,
  `layout_grid`, `hconcat`).
- `attach.rs` — `MathAttach` (subscripts/superscripts/primes
  merged via eval). Consome `MathGlyphKern` em todos os 4
  quadrantes (top-left, bottom-left, top-right, bottom-right)
  via `self.metrics.math_kern(c)` (P255 §2 item 1).
- `root.rs` — `MathRoot` (sqrt + n-th roots). Consome
  `radical_vertical_gap` + `radical_rule_thickness` de
  `MathConstants`.
- `frac.rs` — `MathFrac` (fracções). Consome
  `fraction_rule_thickness` + `fraction_num_gap` +
  `fraction_denom_gap` de `MathConstants`.
- `matrix.rs` — `MathMatrix` (matrizes).
- `cases.rs` — `MathCases` (chaves grandes).
- `stretchy.rs` — operadores extensíveis. Consome
  `GlyphVariants` via `self.metrics.vertical_glyph_variants(c)`
  com `.select(min_advance)` (P255 §2 item 2).
- `assembly.rs` — assembly por partes para delimitadores
  grandes. Consome `GlyphAssembly` via
  `self.metrics.vertical_glyph_assembly(c)` (P255 §2 item 2).
- `delimited.rs` — `MathDelimited` (par de delimitadores fixos).

## Restrição arquitectural
L1 puro. Não depende de L3. Usa `FontMetrics` trait injectável.
Sem I/O de sistema. `MathLayouter` é genérico sobre `M: FontMetrics`.

## Interface pública

```rust
pub struct MathLayouter<'a, M: FontMetrics> {
    pub(super) metrics:   &'a M,
    pub(super) constants: MathConstants,
    // ... outros campos pub(super) ...
}

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub fn new(metrics: &'a M, style: &TextStyle) -> Self;
    pub fn layout_equation(
        &mut self,
        body:  &Content,
        style: &TextStyle,
    ) -> Frame;
    // Outros métodos pub(super): apply_axis_offset, layout_node,
    // layout_text_node, layout_sequence, layout_grid_rows,
    // layout_grid, hconcat.
}
```

`MathBox` (4 campos `pub(super)`): caixa intermédia com
`ascent`/`descent`/`width`/`items` usada para composição
hierárquica (fracções, attachments, etc).

## Consumers de tipos de domínio (P255 reconciliação)

Mapping consumer ↔ tipo (todos pós-P96.8 + integration):

- **`MathConstants`** → `mod.rs` (construtor `new` via
  `metrics.math_constants()`); `apply_axis_offset` consume
  `axis_height`; submódulos `frac.rs`/`attach.rs`/`root.rs`
  consomem campos específicos.
- **`MathGlyphKern`** → `attach.rs:49-208` em 4 quadrantes
  (geometria correcta sem `.abs()`; kern negativo permitido).
- **`GlyphVariants`** → `stretchy.rs:22` para operadores
  extensíveis via `select(min_advance)`.
- **`GlyphAssembly`** → `assembly.rs:14, 20` para
  delimitadores grandes por assembly de partes.

`MathConstants::fallback()` é o caminho activo via
`FixedMetrics` (sem fonte MATH real exigida em testes).

## Baseline x-height (P255 §2 item 4)

`MathLayouter::apply_axis_offset` (`mod.rs:228-229`) é o método
canónico que aplica baseline x-height para fracções,
delimitadores e sqrt. Usa `self.constants.axis_height`
directamente (não hardcode). Tests regressão
`frac_com_axis_height_nao_regride`,
`delimitado_com_axis_height_nao_regride`,
`sqrt_com_axis_height_nao_regride` em `tests.rs:520+` verificam
`axis_height > 0` activo via fallback.

## MathPrimes (P255 §2 item 3 — divergência arquitectural)

`MathPrimes` é **resolvido em eval, não em layout**
(`rules/eval/math.rs:85-101`). Count → glifo `′`/`″`/`‴`/`⁗`
(U+2032/2033/2034/2057; n>4 → repetição de `′`) convertidos
para `Content::MathText` e merged como superscript regular.
Layouter `attach.rs` recebe-os pelo arm superscript regular;
não há arm dedicado `MathPrimes`.

Paridade observable vanilla preservada per ADR-0033 (output
PDF correcto; glifos visualmente idênticos).

## Critérios de verificação

- `MathIdent("x")` → `Frame` com `FrameItem::Text { text: "x", .. }` não vazio.
- `MathSequence([x, +, y])` → `Frame` com 3 items.
- `MathFrac { num: a, den: b }` → sem `[` nos items.
- Integração no layouter principal: `Content::Equation` delega ao `MathLayouter`.
- `apply_axis_offset` consome `axis_height` real (não zero).
- `attach.rs` consume `math_kern` em todos os 4 quadrantes.
- `stretchy.rs` selecciona variant via `select(min_advance)`.
- `assembly.rs` constrói assembly de partes para delimitadores.
- Primes (`′″‴⁗`) renderizam correctamente (resolvidos em eval).

## Variant `Content::MathStyled` — Passo 311b.4 (handler + apply_math_style)

Handler dedicado em `layout_node` arm `Content::MathStyled` resolve
variant glyph + flags. Algoritmo:

1. **Pré-transformação recursiva** do body via `apply_math_style`
   (função livre no fim do módulo). Composição outer-wins via
   `Option::or` — outer set ganha sobre inner.
2. **Substituição char-by-char** em `MathIdent`/`MathText`:
   `map_glyph(c, kind.unwrap_or(Plain), bold.unwrap_or(false),
   italic.unwrap_or(false))`.
3. **Propagação para containers** math (MathFrac/MathAttach/MathRoot/
   MathDelimited/MathSequence) — recurse com mesmo context.
4. **`MathOp` passa-through** — operadores texto (`sin`/`lim`/etc.)
   mantêm aparência normal mesmo dentro de `bb(...)` (paridade
   vanilla).
5. **Size factor** para `Script`/`SScript`: aplica
   `style.size *= kind.size_factor()` antes de descer para body.
6. **Supressão de auto-itálico**: quando wraps math style são
   aplicados (kind/bold/italic Some(_)), `math_style.italic = false`
   no descent para evitar duplicar variant em fontes que já encoded
   no codepoint.

### Composição (paridade vanilla)

- `bb(cal(x))` → outer Bb ganha → MathIdent("𝕩").
- `bold(bb(x))` → kind preserved (Bb inner), bold flag aplicado.
- `upright(italic(x))` → outer upright (italic=Some(false)) ganha.
- `script(sscript(x))` → outer-wins via Option::or; **NÃO**
  multiplicativo (refuta diagnóstico P311a §3.5 — confirmado vanilla
  outer-wins em testes empíricos).

### Tabela de testes P311b.4 (8 unit tests)

| Teste | Cenário |
|---|---|
| `bb_substitutes_chars` | `bb(x)` → 𝕩 (U+1D569) |
| `bold_italic_orthogonal` | Plain+bold+italic → Bold Italic codepoint |
| `bb_cal_outer_wins` | `bb(cal(x))` → outer Bb DS |
| `upright_italic_outer_wins` | `upright(italic(x))` → 'x' literal |
| `bold_preserves_inner_bb` | `bold(bb(x))` → DS lowercase plane |
| `recurses_through_mathfrac` | `bb(frac(a,b))` → DS num + DS den |
| `size_variant_passthrough_glyph` | `script(x)` → 'x' literal |
| `math_op_passthrough` | `bb(op("sin"))` → MathOp("sin") inalterado |

### Integração com `is_single_letter_var`

Pre-P311b.4: `MathIdent` handler aplica itálico automático a
variáveis de 1 letra via `is_single_letter_var`.

Pos-P311b.4: wraps `MathStyled` SUPRIMEM auto-itálico via
`math_style.italic = false`. Itálico explícito (`italic(x)`) é
honrado via codepoint Bold/Italic plane já transformado em
`apply_math_style`. Tests existentes preservados (sem regressão).

### Tests preservados

Suite math layout pré-P311b.4: ~50 tests. Pós-P311b.4 todos verdes
mais 8 novos P311b.4 = ~58. Auto-itálico para `MathIdent` sem wrap
preservado (caminho default não-MathStyled).

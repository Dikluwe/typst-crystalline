# L0 — math/layout/tests
Hash do Código: 8c88f08f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c

**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/tests.rs:16` locates `fn default_style() -> TextStyle`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Test-only ownership

Own only `01_core/src/compiler/math/layout/tests.rs` and its cfg(test)
integration. Test obligations do not authorize productive geometry.
Exercise dispatch, styles, baseline/extents, spacing and feature regressions
with injected metrics and attributable oracles. Record formula/observable
provenance for new cases; fixture coordinates and bytes are not contracts.

Root fixtures preserve index_explicit: square-root controls use None/false,
explicit indices use true, without changing independent geometry oracles.

Fraction coverage distinguishes vertical/skewed/horizontal, explicit versus
omitted style, style-chain precedence, slash associativity, both
deparenthesization bits and the barless binomial. Observe axis, operand order,
separation and bar/slash presence rather than freezing font names, private
frames or operation order. Independent feature suites keep their own owners
under `00_nucleo/prompts/compiler/math/layout/`.

Acceptance observes morphology and attributable font-derived geometry;
historical RED/GREEN reports or aggregate test totals are not fresh parity
evidence.

For `lr` around a fraction, compare the same fenced body with and without
the explicit wrapper using injected OpenType-like vertical variants. The
fraction's unbalanced raw height and axis-balanced target must straddle a
variant threshold, so a test fails if explicit `lr` uses raw height while
implicit fences use the balanced extent. Observe selected glyph advance and
declared box extents for both parentheses and bars; the pinned vanilla
source's `FencedItem::relative_to_from_fragments` supplies the independent
axis-balancing rule, and bilateral `sec_22.typ` checks its product effect.

For Display Large operators, verify that a multi-part assembly carries one
source-character TextReplacement around its visual parts; the wrapper does
not alter part positions, width or extents. Preserve base italics-correction
behavior when scripts consume the wrapped visual glyphs. Verify that a
sufficient base glyph remains
unchanged before consulting MATH constructions. Exercise the case where the font's largest
vertical variant remains below DisplayOperatorMinHeight and has no assembly:
retain that largest variant, including its native horizontal advance and
ink-derived extents, rather than reverting to the base glyph. Also cover the
assembly path when the font supplies one. Attribute expected selection to the
active font's OpenType MATH construction and the pinned vanilla algorithm.

## Verificação de identidade extraível

Para `cases`, acentos e spreaders, verificar no L1 que `TextReplacement`
retém o caráter da linguagem e preserva as peças visuais; sondar extração
PDF nos exemplos locais após RED→GREEN. Diferenças de espaçamento
introduzidas apenas pelo extrator, como `.typ/sec_27.typ`, não autorizam
inserir whitespace na árvore sem causa linguística.

Testar também dois acentos combinantes aninhados: a identidade deve ser
base+marca interna+marca externa com somente um envelope no resultado final.

Em `hat(mu)`, verificar que a folha grega `MathText` entra no acento como
`𝜇` matemático, enquanto texto de vários caracteres não vira letras
matemáticas por aproximação. A sonda PDF deve preservar a composição
base+marca e medir o raster contra o vanilla.

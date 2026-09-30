# L0 — math/layout/p1430-tests

Hash do Código: f5c6fd93

**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/p1430_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/p1430_tests.rs:23` locates `struct TestWorld`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Independent cases tests

Own only `01_core/src/compiler/math/layout/p1430_tests.rs`.
Observe attributable FrameItems with deterministic metrics.
Verify one initial row per child, late MathAlignPoint splitting, denominator
style, left/right delimiter chosen by reverse, omitted None sides,
relative gap resolved against outer style and explicit > cascade > default
without carrier changes.

Retain regression controls for natural alignment, one axis-centering operation,
no redundant padding, baseline-relative composition and full delimiter extents
(the established P919/P923/P945/P1042/P1132 behaviors).
Acceptance observes geometric relationships and frame morphology, not bytes.

Unattributable font, carrier, StyleChain, glyph, frame, metric or geometry is
Unknown; fail closed and do not count it as killing a mutant.

## Leitura de delimitador por ocorrência

Após `cases` envolver o delimitador visual em `TextReplacement`, os testes
de direção e extents seguem medindo o `Text`/`TextShaped` filho e suas
coordenadas; o helper de observação atravessa o envelope sem usar a string
extraível como substituto geométrico. RED integrado: quatro leituras de
`{`/`}` não encontravam os itens filhos no nível superior.

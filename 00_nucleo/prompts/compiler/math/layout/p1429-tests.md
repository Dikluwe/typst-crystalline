# L0 — math/layout/p1429-tests

Hash do Código: 625f010a

**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/p1429_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/p1429_tests.rs:29` locates `struct NullWorld`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Independent accent tests

Own only `01_core/src/compiler/math/layout/p1429_tests.rs`.
Observe attributable FrameItems, not serialized bytes.
Upper accents stay above cramped bases and preserve attachment/baseline
regressions; Unicode Below and the four explicit lower support symbols stay
below non-cramped bases. Only the upper path performs dotless realization.

Effective Rel<Length> size resolves against base width with shortfall 0.5em.
Synthetic identifiable variants distinguish 100% from 150%. Size and dotless
independently obey explicit > cascade > default, without manufacturing carrier
presence. Both transformations preserve every causal field. Dotless morphology
covers i and j with both polarities.

Preserve the exact-threshold witness: base=1200du, first variant=750du.
Shortfall 0.5em gives target=700du and selects that variant; 0.4em gives
800du and selects the larger variant; 0.6em gives 600du and retains the base.
This discriminates the actual threshold, not merely percentage differences.

Unattributable font, variant, glyph, frame, cascade or geometry is Unknown.
Unknown fails closed in mandatory cases and never kills a mutant.

## Leitura de identidade por ocorrência

Após o carrier visual de acentos ganhar `TextReplacement`, as observações
de posição/tamanho continuam a medir os filhos `Text`/`TextShaped` e `Glyph`
originais. O helper de teste percorre o envelope de forma transparente, sem
substituir a assertiva de geometria pela string `ActualText` nem alterar o
caso sem envelope. RED integrado: os testes de orientação/dotless e de
precedência de size deixaram de encontrar suas folhas diretas.

A base `i`/`j` sem dotless dentro de matemática é `𝑖`/`𝑗` após a
aplicação vigente de itálico por defeito; o teste compara essa morfologia
realizada, não o token ASCII de entrada. A forma dotless continua `ı`/`ȷ`.
O acento inferior também conserva o itálico da base; sua orientação é
medida pelas posições dos filhos visuais.

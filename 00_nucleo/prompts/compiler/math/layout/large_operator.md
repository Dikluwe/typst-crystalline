# L0 — math/layout/large_operator
Hash do Código: a168fb88

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/large_operator.rs`

## Medição e classificação

No vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`,
`crates/typst-library/src/math/ir/resolve.rs:350-352` pede stretch vertical
para um glifo Large em Display com `StretchInfo::default()`; `item.rs:1271-1275`
define alvo relativo 1 e short fall zero. Em
`crates/typst-layout/src/math/fragment/glyph.rs:265-300`, o glifo base suficiente
fica, a primeira variante suficiente é usada, a maior variante insuficiente é
usada se faltar assembly, e uma assembly disponível é montada quando todas as
variantes são insuficientes. `glyph.rs:452-454` resolve o alvo relativo a
`display_operator_min_height`, e `text.rs:115-118` centra Large no eixo.

Em `NewCMMath-Book.otf` deste repositório, upem=1000, o mínimo normal é
1300du; `∑` tem variantes de 1001/1401du sem assembly e `∫` tem variantes
de 1112/2223du com assembly. Uma cópia de diagnóstico da fonte com somente
`DisplayOperatorMinHeight=3000` expõe ambos os ramos: o PDF do vanilla usa a
variante 1401du de `∑` e uma montagem de três glifos para `∫`. A escolha do
glifo e sua extensão são morfologia visual/semântica observável da linguagem;
IDs de glifos, bytes PDF e estrutura de frames não são o contrato. Esta
correção repara paridade interna; não muda contrato público, fase nem padrão
de produto segundo ADR-0127. A medição não prova paridade de todas as fontes;
um contraexemplo seria uma fonte cujo fallback vanilla divergisse destes ramos.

## Escolha composta do operador

Own only the Large glyph path under math layout. Keep `MathClass::Large`
identity; do not turn the operator into a delimiter or a textual MathOp.
In Display, use the active font's `display_operator_min_height` without
delimiter short fall. Preserve a sufficient base glyph. Otherwise choose the
first sufficient vertical variant. If none suffices, use the font's vertical
assembly when present; if absent, retain the largest available variant.
Only when neither variants nor assembly exists, use the base glyph. Inline
and script modes keep the base glyph.

For a selected variant, derive width/x-advance from native horizontal advance
and ascent/descent from real ink. Center the resulting real ink on the font's
math axis. For assembly, reuse the existing vertical assembly owner and its
font-supplied parts, overlap, target and axis; do not build a second assembly
algorithm. Avoid centering twice. Keep the same language identity through
glyph or assembly emission. A multi-part Large assembly is one textual
occurrence: wrap its already-positioned visual Glyph parts in the existing
TextReplacement envelope with `text` equal to the source operator character.
Do not infer identity from part GIDs or remap ToUnicode globally. The envelope
adds no advance, width, height, drawing or pipeline phase; variants and other
stretchy constructions retain their existing representation.

Acceptance covers sufficient variant, largest insufficient variant without
assembly, assembly after insufficient variants, no construction and inline.
Use injected metrics for deterministic layout tests and the pinned vanilla
plus modified-font PDF probe as independent evidence. No hardcoded font
height, glyph ID or fallback order belongs in productive logic.

# L0 — core/compiler/eval/math_root_evaluation_tests
Hash do Código: 9669cb1f

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/math_root_evaluation_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:16` — `struct TestWorld {`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:57` — `fn evaluate`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:62` — `fn find_root`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:75` — `fn evaluated_root`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:87` — `fn p1426_sintaxe_bare_preserva_vazio_none_e_conteudo`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:106` — `fn p1426_root_bare_exige_slots_e_preserva_presenca`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:119` — `fn p1426_observation_relation_observa_presenca_do_indice`.

## Closed fixture observer evidence

Source inspection at commit `059d1a0ffa9523dbe21af6860a215acaeef55d73` on
`2026-10-19T11:42:01.899108+00:00` found `find_root` at
`01_core/src/compiler/eval/math_root_evaluation_tests.rs:62` returning the exact MathRoot carrier and
using a wildcard at `01_core/src/compiler/eval/math_root_evaluation_tests.rs:71` for unsupported paths.
The full source SHA-256 at inspection was `1e39fb19a3a2096480b0f1bc81e84cab9d5a0cbe683377232d12a20338d1a30d`;
the owning L0 before this revision was `937d416e170b41c955fab25da6e7d2a4e1f7d7ea30d845e14575e8775cc4cfe3`.
All line references in the original evidence above identify this inspection
checkpoint, not later line positions.

`01_core/src/entities/content.rs:233` declares the closed Content domain.
The observer does not call a general Content traversal. Its fixture scope is
therefore narrower than semantic descendability: a stopped wrapper may contain a
matching carrier. Existing real-source fixtures at `01_core/src/compiler/eval/math_root_evaluation_tests.rs:87`
exercise constructor observation; existing argument, presence and failure
assertions remain authoritative. Content constructors in
`01_core/src/entities/content/construction.rs:27` (Styled),
`01_core/src/entities/content/construction.rs:40` (Strong),
`01_core/src/entities/content/construction/layout.rs:17` (Equation) and
`01_core/src/entities/content/construction/math.rs:97` (MathDelimited) permit
controlled wrapper fixtures without introducing another productive visitor.

This is evidence of current test-observer mechanics, not evidence that all
wrappers share language behavior or that vanilla intends this traversal policy.
The preservation claim is refuted by a changed hit, order, wrapper disposition,
or existing language-level constructor expectation. New wrapper needs require a
measured fixture and an owner revision; they must not silently expand this search.

## Contract

Own only tests of bare-root syntax and its observation relation. Evaluate real
sqrt(x) as None/not-explicit; root(,x) as Some(Empty)/explicit; present index as
Some(_)/explicit; evaluated none as None/explicit. root(x) remains a missing
radicand error, not a sqrt alias. Complete identical carriers compare Same;
carriers differing only in index_explicit compare Different in both directions,
never Unproven. Syntax belongs to 00_nucleo/prompts/core/compiler/eval/math.md and
comparison to 00_nucleo/prompts/core/compiler/eval/observation/math.md. Ambiguous
observations are Unknown and fail closed; auxiliary test structure is mechanics.

## Closed fixture observer contract

`find_root` is private and test-only. Match Content nominally and exhaustively at
this decision, with no wildcard, catch-all binding, lint suppression, classifier
escape, or generic productive traversal.

- `MathRoot` is a terminal hit. Return that carrier itself; do not search its
  children for a competing nested hit.
- `Equation` descends into its body.
- `Sequence` and `MathSequence` search left to right, depth first, stopping at
  the first successful child. Empty sequences and paths with no target miss.
- `Styled` and `MathDelimited` are deliberately stopped here, unlike the other
  math fixture observers. Do not broaden root observation to those wrappers.

The following current variants are terminal misses (`None`), with no descent,
even if their payload contains a target:

`Source`, `ShowRecipe`, `Realized`, `Located`, `Empty`, `Text`, `Space`,
`SpaceFromNewline`, `Parbreak`, `HtmlElem`, `HtmlFrame`, `Par`, `Heading`, `Title`,
`Strong`, `Emph`, `Raw`, `ListItem`, `EnumItem`, `Link`, `MathIdent`,
`MathText`, `MathFrac`, `MathAttach`, `MathPrimes`, `MathStretch`, `MathDelimited`, `MathLr`,
`MathMid`, `MathOverbrace`, `MathUnderbrace`, `MathUnderbracket`, `MathUnderparen`, `MathUndershell`, `MathOverbracket`,
`MathOverparen`, `MathOvershell`, `MathAlignPoint`, `Linebreak`, `MathMatrix`, `MathCases`, `MathAccent`,
`MathCancel`, `MathUnderline`, `MathOverline`, `MathVec`, `MathClassOverride`, `MathLimitsOverride`, `MathUnderover`,
`MathOp`, `MathStyled`, `Label`, `Ref`, `CounterDisplay`, `CounterUpdate`, `Outline`,
`OutlineEntry`, `Figure`, `FigureCaption`, `Image`, `Shape`, `Curve`, `Transform`,
`Grid`, `GridHeader`, `GridFooter`, `GridCell`, `SetPage`, `PageRun`, `Align`,
`Place`, `Flush`, `Styled`, `Divider`, `Terms`, `TermItem`, `Quote`,
`Document`, `Asset`, `SmartQuote`, `PdfAttach`, `PdfArtifact`, `Underline`, `Strike`,
`Overline`, `SmallCaps`, `Pad`, `Hide`, `HSpace`, `VSpace`, `Pagebreak`,
`Colbreak`, `Stack`, `Boxed`, `Block`, `TableCell`, `Bibliography`, `Cite`,
`Footnote`, `FootnoteEntry`, `TableHeader`, `TableFooter`, `GridHLine`, `GridVLine`, `TableHLine`,
`TableVLine`, `Table`, `Repeat`, `Columns`, `Metadata`, `State`, `StateUpdate`,
`StateDisplay`, `CounterDisplayCallback`, `ContextBlock`, `Dynamic`.

They may share a negative arm because the rationale is identical **for this
fixture observer**: each is outside its authorized fixture path. This grouping
does not classify them as semantically empty, equivalent, or universally opaque.
Payload patterns may ignore fields only after naming their Content variant.

### Acceptance and residual scope

Retain every existing real-source constructor and observation assertion. Add
controls for a direct hit, all allowed wrapper paths, nesting, both sequence
kinds with distinguishable first and later targets, empty/no-target inputs, and
blocked wrappers containing real target decoys. Place a blocked decoy before a
valid later hit to prove that unsupported paths are skipped without changing
encounter order. Use Strong, MathStyled, Styled and MathDelimited as blocked
wrappers containing target decoys. A target containing another target must return
the outer carrier. Pointer identity is permitted only to
identify which fixture carrier the test-local observer returned; it is not a
language parity criterion. The existing semantic/morphological assertions
continue to decide constructor behavior.

Runtime preservation controls must pass against the wildcard baseline and the
closed implementation. Separately, in a disposable full workspace, extend the
real Content enum with a temporary sentinel: the baseline observer must have no
non-exhaustiveness error at this decision, while the repaired observer must
produce E0004 at this exact match. Unrelated enum-extension errors do not satisfy
this requirement. Never add the sentinel or a suppression to productive sources.

This internal observer correction follows ADR-0127 continuous flow after L0
resealing. It grants no new constructor, traversal, default, or pipeline behavior
and cannot close broader Typst parity. An unobservable fixture remains Unknown
and blocks acceptance; it must not be converted into success or ignored.

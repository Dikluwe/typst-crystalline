# L0 — core/compiler/eval/math_frac_evaluation_tests
Hash do Código: c74eada6

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/math_frac_evaluation_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:17` — `struct TestWorld {`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:58` — `fn evaluate`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:63` — `fn find_frac`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:76` — `fn evaluated_frac`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:85` — `fn frac`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:97` — `fn p1427_set_style_valida_e_respeita_precedencia`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:115` — `fn p1427_frac_bare_e_slash_convergem_no_carrier`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:142` — `fn p1427_slash_associa_esquerda_e_preserva_parenteses_causais`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:172` — `fn p1427_observation_relation_compara_todos_os_campos`.

## Closed fixture observer evidence

Source inspection at commit `059d1a0ffa9523dbe21af6860a215acaeef55d73` on
`2026-10-19T11:42:01.899108+00:00` found `find_frac` at
`01_core/src/compiler/eval/math_frac_evaluation_tests.rs:63` returning the exact MathFrac carrier and
using a wildcard at `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:72` for unsupported paths.
The full source SHA-256 at inspection was `a2fd2b097b969c05a3b23b226c719e163f45ab3e057ff6cc90cae2aa42e329a0`;
the owning L0 before this revision was `b6187b015201359b5f73bda75773627db1fc8a33f12c7d3273bb2b9afba22c28`.
All line references in the original evidence above identify this inspection
checkpoint, not later line positions.

`01_core/src/entities/content.rs:233` declares the closed Content domain.
The observer does not call a general Content traversal. Its fixture scope is
therefore narrower than semantic descendability: a stopped wrapper may contain a
matching carrier. Existing real-source fixtures at `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:115`
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

Own independent fraction evaluation tests from the frozen contract, not productive
implementation. Evaluate real sources for explicit/cascaded/default vertical
precedence and invalid set-rule failures, bare/qualified/slash convergence,
left-associative a/b/c and causal deparenthesization. Only slash supplies exactly
one origin bit per removed exterior pair; bare and qualified calls invent none.
Compare complete optional style, both bits and line in both observation directions;
any isolated difference is Different. Do not infer success from render bytes,
consult a candidate for expected results or declare overall parity from this suite.

## Closed fixture observer contract

`find_frac` is private and test-only. Match Content nominally and exhaustively at
this decision, with no wildcard, catch-all binding, lint suppression, classifier
escape, or generic productive traversal.

- `MathFrac` is a terminal hit. Return that carrier itself; do not search its
  children for a competing nested hit.
- `Equation` descends into its body.
- `Sequence` and `MathSequence` search left to right, depth first, stopping at
  the first successful child. Empty sequences and paths with no target miss.
- `Styled` and `MathDelimited` descend into their bodies.

The following current variants are terminal misses (`None`), with no descent,
even if their payload contains a target:

`Source`, `ShowRecipe`, `Realized`, `Located`, `Empty`, `Text`, `Space`,
`SpaceFromNewline`, `Parbreak`, `HtmlElem`, `HtmlFrame`, `Par`, `Heading`, `Title`,
`Strong`, `Emph`, `Raw`, `ListItem`, `EnumItem`, `Link`, `MathIdent`,
`MathText`, `MathAttach`, `MathPrimes`, `MathRoot`, `MathStretch`, `MathLr`, `MathMid`,
`MathOverbrace`, `MathUnderbrace`, `MathUnderbracket`, `MathUnderparen`, `MathUndershell`, `MathOverbracket`, `MathOverparen`,
`MathOvershell`, `MathAlignPoint`, `Linebreak`, `MathMatrix`, `MathCases`, `MathAccent`, `MathCancel`,
`MathUnderline`, `MathOverline`, `MathVec`, `MathClassOverride`, `MathLimitsOverride`, `MathUnderover`, `MathOp`,
`MathStyled`, `Label`, `Ref`, `CounterDisplay`, `CounterUpdate`, `Outline`, `OutlineEntry`,
`Figure`, `FigureCaption`, `Image`, `Shape`, `Curve`, `Transform`, `Grid`,
`GridHeader`, `GridFooter`, `GridCell`, `SetPage`, `PageRun`, `Align`, `Place`,
`Flush`, `Divider`, `Terms`, `TermItem`, `Quote`, `Document`, `Asset`,
`SmartQuote`, `PdfAttach`, `PdfArtifact`, `Underline`, `Strike`, `Overline`, `SmallCaps`,
`Pad`, `Hide`, `HSpace`, `VSpace`, `Pagebreak`, `Colbreak`, `Stack`,
`Boxed`, `Block`, `TableCell`, `Bibliography`, `Cite`, `Footnote`, `FootnoteEntry`,
`TableHeader`, `TableFooter`, `GridHLine`, `GridVLine`, `TableHLine`, `TableVLine`, `Table`,
`Repeat`, `Columns`, `Metadata`, `State`, `StateUpdate`, `StateDisplay`, `CounterDisplayCallback`,
`ContextBlock`, `Dynamic`.

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
encounter order. Use Strong and MathStyled as blocked wrappers containing target
decoys. A target containing another target must return the outer carrier. Pointer identity is permitted only to
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


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.


## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

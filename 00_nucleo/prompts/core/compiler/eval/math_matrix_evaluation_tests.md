# L0 — core/compiler/eval/math_matrix_evaluation_tests
Hash do Código: 6ffea87e

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:16` — `struct TestWorld {`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:53` — `fn id`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:56` — `fn evaluate`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:60` — `fn find_mat`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:72` — `fn evaluated_mat`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:80` — `fn direct`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:94` — `fn p1431_bare_e_qualificado_convergem_sem_pre_resolver_alignpoint`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:117` — `fn p1431_bare_rows_arrays_padding_e_invalidos_usam_construtor_comum`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:150` — `fn p1431_avaliacao_ocorre_uma_vez_e_na_ordem_causal`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:208` — `fn p1431_cascata_nao_fabrica_presenca_e_explicito_ganha_por_eixo`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:244` — `fn p1431_set_rejeita_named_e_tipos_invalidos`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:271` — `fn p1431_observacao_de_ordem_opaca_falha_fechado`.

## Closed fixture observer evidence

Source inspection at commit `059d1a0ffa9523dbe21af6860a215acaeef55d73` on
`2026-10-19T11:42:01.899108+00:00` found `find_mat` at
`01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:60` returning the exact MathMatrix carrier and
using a wildcard at `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:69` for unsupported paths.
The full source SHA-256 at inspection was `c545af4c89d5a09484b1367f2bac80faf223ae8e584ca8c7cbc04c422d4ba1ef`;
the owning L0 before this revision was `85b506acf010f69eb2a8f92ccc54e88ae10601f7cce9400bbee6c52191f46926`.
All line references in the original evidence above identify this inspection
checkpoint, not later line positions.

`01_core/src/entities/content.rs:233` declares the closed Content domain.
The observer does not call a general Content traversal. Its fixture scope is
therefore narrower than semantic descendability: a stopped wrapper may contain a
matching carrier. Existing real-source fixtures at `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:94`
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

Own independent mat evaluation tests from real sources, requiring bare/qualified
convergence for scalar/array rows, complete named values and rejections. Every
positional/named occurrence evaluates once in causal source order. A successful
mutable-array witness records one label per occurrence; a separate arguments
spread fixture proves expansion/order but is not used as an execution counter.
First error ends construction; cascade cannot manufacture presence/repr fields,
and explicit values win per axis. Unobservable parser/order/counter yields
UNKNOWN_EVALUATION_OBSERVATION and fails closed.

Set-rule negatives cover unknown, rows:, invalid delim/align/augment/gap/row-gap/
column-gap and duplicates, including mandatory #set math.mat(column-gap: 1).
Bare-route negatives cover mixed rows and invalid/unknown/duplicate named values;
no rejection may rely solely on testing the qualified route. This consumer owns
tests only, not the constructor or style schema.

## Closed fixture observer contract

`find_mat` is private and test-only. Match Content nominally and exhaustively at
this decision, with no wildcard, catch-all binding, lint suppression, classifier
escape, or generic productive traversal.

- `MathMatrix` is a terminal hit. Return that carrier itself; do not search its
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
`MathText`, `MathFrac`, `MathAttach`, `MathPrimes`, `MathRoot`, `MathStretch`, `MathLr`,
`MathMid`, `MathOverbrace`, `MathUnderbrace`, `MathUnderbracket`, `MathUnderparen`, `MathUndershell`, `MathOverbracket`,
`MathOverparen`, `MathOvershell`, `MathAlignPoint`, `Linebreak`, `MathCases`, `MathAccent`, `MathCancel`,
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

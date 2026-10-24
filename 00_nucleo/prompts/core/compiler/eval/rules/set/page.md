# L0 — compiler/eval/rules/set/page — Page settings and geometry transport
Hash do Código: 6b580833

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/set/page.rs`

## Evidence before decision

At the historical post-extraction state, the dispatcher in `01_core/src/compiler/eval/rules/set.rs` retained the page arm as a contiguous block. The L1 baseline in `lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz` and the post-extraction suite passed with 6401 tests and 2 ignored. This measures structure; it is neither a benchmark nor a new bilateral upstream measurement.

## Language contract

Preserve width/height/paper/flipped/binding, margins, numbering, header/footer, bleed/fill/background/foreground, casts, lexical style and the produced SetPageDelta. em sizes resolve against the current eval style; subsequent geometry belongs to layout.

The dispatcher calls this owner statically without changing arm order. Preserve errors, spans, argument evaluation, value identity, absent versus present styles and execution phase. Historical decisions and measurements remain in `00_nucleo/prompts/core/compiler/eval/rules.md`.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Preserve language parity under ADR-0107/0108, 1:1 ownership and pins under ADR-0129/0130.

## Closed-domain decision and preservation

### Evidence before decision

At checkpoint `370d78355bd646d1abead5bcacef215288cc8834`,
`01_core/src/compiler/eval/rules/set/page.rs:11–553`
and the caller/producer evidence recorded in
`lab/results/v16-corrections-2026-10-19/batch-c/agents/structural/l0-audit.json`
show the current decision and its refusal path. That receipt records UTC,
working-tree diffstat and exact source/owner SHA-256 before this amendment.
The existing clauses above establish the behavior to preserve; an open Rust
match is structural evidence, not proof of additional language admission.

### Exclusive policy

Dict validates side keys/values and logical/physical precedence; Auto errors expected
relative length or dictionary. In the wildcard, Relative/Ratio/Length/Float/Int become
uniform bleed via extract_rel_length; all other listed complement Values error expected
relative length at the existing span. Float/Int point interpretation is retained local
legacy behavior, not fresh vanilla certification.

Retain the closed Dict and Auto cases. Explicitly list all other Value variants as
delegation to the existing relative-length cast; this is not a blanket acceptance. That
cast admits Relative/Ratio/Length/Float/Int and rejects the rest at the same span.
Existing numeric point interpretation is local legacy behavior, not new upstream
certification. Test accepted uniform values, invalid/Auto distinction, dictionary side-
over-axis-over-rest precedence, logical/physical conflict and invalid dictionary values.

The explicit nominal complement is: `None`, `Bool`, `Int`, `Float`, `Str`, `Array`, `Module`, `Datetime`, `Func`, `Content`, `LocatedContent`, `Length`, `Relative`, `Ratio`, `Angle`, `Color`, `Stroke`, `Fraction`, `Align`, `Location`, `Gradient`, `Regex`, `Tiling`, `Bytes`, `Decimal`, `Duration`, `Version`, `Selector`, `Symbol`, `Args`, `State`, `Counter`, `Label`, `Dir`, `Path`, `Type`, `SpotColorant`.

### Acceptance and refutation

Preservation controls run before and after the repair. A real temporary enum
variant in a disposable complete workspace must fail with E0004 at this exact
decision after repair; failures at other decisions alone are insufficient.
Exercise nested closed domains separately where present. The structural RED is
the original V16/open evolution boundary, not a fabricated runtime defect.
Any changed result, message, span, callback count, argument evaluation or refusal
route refutes preservation. Retain informational V19 groups with their rationale;
add no suppression, hidden catchall, linter/configuration weakening or new policy.

Internal continuous ADR-0127: no public field/trait/signature, product default,
phase or compatibility change. L0 precedes code; reseal only valid owner pairs
after V15/V26. Require focused and full workspace tests, build/all-targets, fmt,
strict Clippy/Rustdoc, path checks, direct/reverse lineage and differential SARIF
without new errors or warnings outside the selected repairs. Heavy checks are
serialized under one job, 12 GiB maximum, 8 GiB high and zero swap. Historical
parity claims and unresolved scope exclusions above remain bounded.

## Measured remaining HTML and Length parity correction

### Evidence before decision

`lab/results/remaining-parity-2026-10-22/agents/html-length/measurement-before-decision.json`
pins receipt SHA-256 `96025a9ff9d2c5a6057e9873285e43e4e81501bb40b55fe43b1244743e8ed09c`,
UTC `2026-10-22T11:53:10.657191+00:00`–`2026-10-22T11:53:35.064880+00:00`,
uncommitted HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, complete dirty stat,
source archive, commands/fonts/fixtures/build/tool and verified oracle pins.
All provenance checks pass. Oracle commit is `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Length rendering, numeric Raw paint, page warnings, native page aliases and two
container refusals differ. False-condition/uninserted/scalar controls pass. The
coordinator froze exact source rejection in `lab/results/remaining-parity-2026-10-22/html-negative-policy.json`
before implementation; no failed render is waived. Pinned `foundations/value.rs:192–205`,
`text/raw.rs:563–606` and `typst-realize/src/lib.rs:635–655` distinguish actual
content morphology, token styles, target-specific warnings and container errors.

### Exclusive correction and acceptance

Attach the actual SetRule span to its emitted SetPage through the existing Source
carrier after successful validation. No line/text guesses, warning at argument
evaluation, dormant marginal execution or new field is allowed. Preserve the delta,
lexical styles, argument order/errors and source identity. Verify direct and native
alias real source spans at subsequent HTML warning/refusal boundaries.

Internal continuous ADR-0127 parity correction: no public field/trait/signature,
new mode/default, phase or compatibility break. L0 precedes code and meaningful
RED→GREEN; validate paths, valid owner/Nucleus graph, restricted hashes, tests,
build, strict Clippy/rustdoc, reverse lineage and complete unsuppressed Tekt.

## Complete-suite Source transparency regression — 2026-10-22

### Evidence before correction

`lab/results/remaining-parity-2026-10-22/candidate-7/validation/tests-domain-infra.log`
SHA-256 `e5900986d7df6f773083e7b95a967d76d451bccb7afe12a93e326d04b98ac8da`
records the complete unfiltered core/infra run. Its frozen inputs are retained in
`candidate-7/validation/validation-state.json.gz` and the unchanged-input check in
`candidate-7/validation/validation-freeze.json` at
`2026-10-22T14:53:49.321659+00:00`, under that same result root. The source-bound
build receipt `candidate-7/candidate-build/receipt.json`, SHA-256
`bcdd2171a19fd1e93bc4934388c35e8fb79563c957994fd9d6bae176930dd9fe`, retains
UTC `2026-10-22T14:44:18.153459+00:00`–`2026-10-22T14:44:36.818708+00:00`,
exact HEAD/uncommitted diffstat, inputs, source archive and product SHA-256
`91cba2e532c185020023aed9a0c031540e57f4b966673649b9a5b1cd8935ec95`.
The existing test bodies are unchanged from the initial baseline archive.
Six page-column layout regressions show missing implicit Columns after actual
SetPage gained its proven Source origin: four one-page requirements observe two
pages, and two RTL requirements begin in the wrong column. Bare-only SetPage
classification in content/page_structure.rs prevents the transform. Four further
observer failures stop at Source instead of examining the actual SetPage/Heading
body. Three separate paragraph failures have another owner and are not evidence
to change page configuration or remove Source. These actual failures supersede
focused-suite confidence; no warning/test/geometry threshold is waived.

### Exclusive correction and acceptance

The two existing bleed tests must inspect Value::Content's source_body to reach
the actual SetPageDelta, because Source is causal metadata rather than a language
element. Keep every original cast, dictionary-axis/side precedence and error
assertion intact. Add an equality check of the retained source span to the actual
SetRule AST span before observing the delta. No producer/runtime behavior change
is authorized by this test-observation amendment.


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

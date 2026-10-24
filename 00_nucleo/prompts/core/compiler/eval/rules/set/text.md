# L0 — compiler/eval/rules/set/text — Set text and lexical transport
Hash do Código: ab6d1f4a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:41e38095ab33ab1710ba11f43d6a5ca50def5a703f77a0001a066b72964c4165
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:d6c54bdd10c4ee5884a9a46bbdabaad3ffdea3aa0e4ad905411f6b43d1b6286b
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:dc3d1eff8516cab7b246430e867fd906289164509c4533e071be543f33b210b6
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:c8570ec5f4f32c0805587230cafc796f35e5ee7c9f8e634e13d807db2afb6e00

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/set/text.rs`

## Extraction evidence and contract

set.rs dispatcher after historical P1499 extraction keeps text arm contiguous.
L1 baseline in
`lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz`
and suite after that extraction passed6401 tests/two ignored. Structural evidence,
not benchmark/fresh vanilla comparison.

Preserve property schemas/type validation/defaults/precedence/scope-out warnings
and custom text.* transport transparent to morphology. text.hyphenate accepts
bool/auto; costs/lexical policy follow pinned nuclei. Dispatcher calls statically
without arm reorder. Keep errors/spans/argument evaluation/value identity/style
absence-presence/execution phase. Historical decisions/measurements remain in
`00_nucleo/prompts/core/compiler/eval/rules.md`. L1/crystalline-lint without
regression; language ADR-0107/0108,1:1 ownership/pins ADR-0129/0130.

## Ratified complete text size

### Evidence before decision

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`,
before-measurement.json.gz, morph-before-measurement.json.gz and audit.json
preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC, status/diffstat, sources/L0/binaries/argv and historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. named-em measures A at 0pt versus
24pt; affine 3pt versus 27pt; nested over 18pt measures 0pt versus 36pt. All six
Size/Color/body permutations are accepted by the pin and rejected by Tekt.
Positional set 18pt is ignored (12 versus 18pt). Pinned text/mod.rs:290,303,
924–930,1136–1142 and args.rs:178–247 establish typed search, named preference,
remaining body and composition. style_chain.rs:513–517 and constructor.rs:123
discard em; collapse at style_chain.rs:394–395,419–422 uses top-wins where size
needs a fold. Size/scope is observable language; Rust representation is an
internal choice. Infer no historical intent. Restoration, Raw, heading, math,
capture/show, duplicates/spans, zero/negative or metric failures can refute it.
The equality probe refuted comparing declared values: the pin returns true for
text(size:2em)[A] == text(size:24pt)[A]. Pinned StyledElem::eq at
content/mod.rs:779–782 compares child. That equality defect requires independent
correction; a complete size type does not close that axis.

### Ratification and limits

The human approved the complete-size package and its two public contracts in
manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba`.
Require implementation and S1–S16 tests under 12 GiB/zero swap; no release/merge/
push. Reseal only after implementation and a valid graph. RG1 remains independent.

### Exclusive property selection and unresolved Paint boundary

Set text no longer ignores positional Size Length/Color: typed selection/named
preference, validate leftovers with once-only evaluation/order/causal occurrences.
Transport text.size as Length without resolving em. StyleChain owns fold, not
this dispatcher. Keep capture/lexicality/other properties.

Explicitly classify all three Arg variants when selecting font property's AST.
Uniform V19 arguments lacking matching named font expression use evaluated
occurrence without reevaluating unrelated positional/spread/named args. Validate
named solid fill; None/other nonpaints use Paint cast error. Gradient/Tiling and
legacy font validators are independent work. Preserve earlier partial Gradient/
Tiling transport as explicit uniform V19 no-effect group here until Paint work;
this scope-out is no invalid cast. None/nonpaints now fail. Solid color or
contradictory message does not close Paint.

Initial complete-size SARIF has two new V17 here (AST lookup/font-array validation).
Split exhaustive variant selection/conditions into arm body, no suppression,
preserving diagnostics/evaluation. Residual Pos/Spread V19 is uniform: no named
font AST expression, use evaluated occurrence.

## Shared selection-nucleus pin evidence

Measurements/shape-RTL refinement in
`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18`
precede new shared selection pin. This owner receives only pin; executable code/
API/phase/other responsibilities remain unchanged in that refinement.

## Closed-domain decision and preservation

### Evidence before decision

At checkpoint `370d78355bd646d1abead5bcacef215288cc8834`,
`01_core/src/compiler/eval/rules/set/text.rs:11–466`
and the caller/producer evidence recorded in
`lab/results/v16-corrections-2026-10-19/batch-c/agents/structural/l0-audit.json`
show the current decision and its refusal path. That receipt records UTC,
working-tree diffstat and exact source/owner SHA-256 before this amendment.
The existing clauses above establish the behavior to preserve; an open Rust
match is structural evidence, not proof of additional language admission.

### Exclusive policy

Str yields a family; Dict yields one only when its name member is Str. Dict with Regex
or absent/non-string name yields no literal lookup. All listed complement Value entries
yield None at this stage; most are disallowed by the earlier ordinary Str/array
validator or font dictionary parser. That reachability must be documented from producers
rather than assumed from the filter itself.

Close both the outer item Value and the inner optional dictionary name Value. None is a
separate Option arm; only Str gives a literal family. Regex names, missing names and
every non-Str name skip literal lookup. The earlier producers accept ordinary Str/Str
arrays and parser-built dictionaries; the defensive complement must not turn into a new
font schema. Test actual set text literal and dictionary literal warnings, regex
dictionary absence of literal warnings, and prior rejection of invalid array/scalar
inputs; preserve lowercase family diagnostics and spans.

The explicit nominal complement is: `None`, `Bool`, `Int`, `Float`, `Array`, `Module`, `Datetime`, `Func`, `Content`, `LocatedContent`, `Auto`, `Length`, `Relative`, `Ratio`, `Angle`, `Color`, `Stroke`, `Fraction`, `Align`, `Location`, `Gradient`, `Regex`, `Tiling`, `Bytes`, `Decimal`, `Duration`, `Version`, `Selector`, `Symbol`, `Args`, `State`, `Counter`, `Label`, `Dir`, `Path`, `Type`, `SpotColorant`.

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

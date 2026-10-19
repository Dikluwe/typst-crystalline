# L0 — compiler/eval/rules/set/page — Page settings and geometry transport
Hash do Código: 2bef3c09

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

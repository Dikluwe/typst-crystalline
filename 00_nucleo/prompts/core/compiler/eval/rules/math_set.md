# L0 — compiler/eval/rules/math_set — Mathematical set rules
Hash do Código: ecf2f031

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/math_set.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz` contains this family's routines in `01_core/src/compiler/eval/rules.rs`. The L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not a benchmark or new bilateral vanilla parity evidence. Historical measurements and pin qualifications remain in `00_nucleo/prompts/core/compiler/eval/rules.md`.

## Language contract

Validate and transport math parameters through canonical casts without creating implicit morphological presence. Properties still without effect preserve measured warnings and do not acquire new behavior.

Preserve observed signatures, identities, casts, defaults, error precedence and evaluation phase. The parent retains shared diagnostics and static bindings; selector matching, termination and textual realization retain their exclusive owners. Do not duplicate those algorithms.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Parity concerns semantics, syntax and morphology (ADR-0107/0108); preserve exclusive ownership and valid pins (ADR-0129/0130).


## Issue 8973 — measured lr sizing and linebreak correction (2026-10-31)

### Measurement before decision

`lab/results/construction/diagnostics/issue-8973-2026-10-31/before-state.json`,
before-sources.json, before/receipt.json and before-relative/receipt.json
bind dirty HEAD, exact diffstat, UTC, sources and freshly built product bytes.
The oracle bytes are verified against SHA-256
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34,
commit 64c5e8c02fe0394d913ead36e42ef8c1334d2107. The Tekt baseline warns
that set lr.size is ignored; even without set, its two fences enclose the
whole two-row stack rather than their own first/last segments. The pinned
oracle itself wrongly uses inherited size over explicit size in multiline;
that defect is not an acceptance oracle. Unset explicit-size controls,
unequal-height rows and percentage/mixed probes distinguish the healthy
sizing contract from that defect. Source: rules.rs:200-216 omits lr.size;
math_set.rs:170-195 drops it; delimited.rs:193-205 pairs across Linebreak.
Pinned math/lr.rs:25-28 defines size relative to wrapped content;
math/ir/multiline.rs:58-111 splits rows/cells; fenced.rs:63-100 takes the
maximum natural fragment extent, not the complete stacked height. Explicit
precedence is already ratified in the owning delimited L0 and MathLrElem
approval of 2026-09-17. Geometry, row semantics, casts, diagnostics and
morphological presence are language obligations; upstream IR/cache/storage
and output bytes are not. New measurements that lose a row, leak size,
resize non-boundary delimiters or change authored content refute closure.

### Classification

This is an internal parity correction under ADR-0127 continuous flow. Keep
all existing public entities, fields, signatures, defaults, phases and
compatibility. In particular preserve MathLrElem's original body, size and
size_explicit; no eval-to-layout migration or new public contract is needed.
L0 and seals precede code; preserve meaningful RED→GREEN and valid V15/V26.

### Exclusive correction

Validate lr.size through the existing relative-length domain (Relative,
Ratio or Length), retaining the existing type vocabulary and expression
span. Store the supplied typed value at math.lr.size without manufacturing
explicit element presence. Evaluate a supplied expression once; empty set
is neutral and lexical scopes restore. Stage this lr-only delta until the
whole rule passes duplicate/unknown/type validation. Preserve all other
math schemas and existing diagnostic precedence. General positional/spread
math set completion and alias-target dispatch remain outside this repair.

Source tests must fail before implementation for actual ignored transport
and invalid-value acceptance, then pass for absolute/ratio/mixed/zero/negative
values, duplicate/unknown/type errors, lexical restoration, single expression
evaluation, empty set, explicit size versus inherited size and morphology.
The official input must compile without the ignored-property warning.

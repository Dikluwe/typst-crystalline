# Circle constructor origin and radius correction

Status: implementation under validation. This draft does not certify final tests,
lineage, path checks, lint deltas or language parity. Final validation receipts
and the eventual commit must be bound before closing the Circle correction.

## Authorization and baseline

The user explicitly approved the proposal adding `ShapeOrigin` and
`ShapeElem.origin`, and authorized implementation with agents on the existing
branch, preserving Tekt contracts and validating tests, paths and lineage.
`approval.json` and `approval-receipt.json` preserve that authorization.
The approved proposal is
`lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md`,
SHA-256 `5b7136bc18f419004113d8d83c9c54adb816e3b70389039aeeed35e1c6f772e8`.
It satisfies ADR-0127 section 2 for this specific public type/required field,
consumer migration and explicitly listed accidental language equalities.

The baseline is commit `954d13185e26a1ed90f6d7c60f6ba27760ca9889` on
`codex/tekt-mature-layout`. Measurements bind their exact UTC, HEAD, dirty-tree
stat, input hashes, binary hashes and commands in their own receipts.
`baseline-build-receipt.json`, `baseline-sources.json.gz`, `l0-before.json` and
`l0-before-sources.json.gz` preserve the preimplementation state.
The oracle remains commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Neither a version string nor the current product executable is the oracle.

## Cause and approved correction

The prior Circle constructor reduced a Length radius to its absolute scalar,
erasing its em component. Circle and generic Ellipse shared a ShapeElem carrier
without a constructor distinction. Simply retaining full Length had already
been refuted: the earlier candidate broke numeric-radius equivalence and made
some Circle/Ellipse values equal. The historical rejected candidate, tests,
receipts and proposal remain unchanged in batch F.

The approved repair adds `ShapeOrigin::{Geometry, Circle}` and required
`ShapeElem.origin`. Existing generic constructors and direct ShapeElem literals
initialize Geometry; the native Circle constructor produces Circle origin with
ShapeKind::Ellipse. Equal dimensions never infer Circle. Existing public
Content constructor signatures and ShapeKind remain unchanged. Derived structural
equality and the existing structural hash retain origin; conservative observation
compares it nominally and exhaustively alongside every prior field. Clone and
terminal maps preserve it. Rendering still dispatches by ShapeKind.

For explicit Int/Float radius, the supported point-number extension is retained.
Explicit Length retains both absolute and em components until the existing
layout resolver handles each occurrence's effective font. All these radius
forms store twice the full Length in equal width and height. Numeric/float/Length
forms of the same point radius remain equal. Distinct em and mixed radii no
longer collapse, and Circle differs from a generic Ellipse even when rendered
dimensions coincide. The approved compatibility correction includes accidental
Circle-versus-numeric-Ellipse equality; it does not delete numeric radii.

Malformed radius values are rejected by an exhaustive Value decision, with the
measured `expected length or auto, found {type}` diagnostic. The last matching
radius occurrence supplies its value span, then argument span, then aggregate
argument span when the more specific anchor is detached. Unknown-name validation
still precedes radius validation, which still precedes paint/stroke parsing.
The existing optional-stroke nucleus, Paint fields and fill/stroke behavior
remain binding. No suppression, relaxed threshold, fallback-to-zero for malformed
values or hidden match-to-if-let conversion is authorized.

## Preserved debt and exclusions

Explicit Auto deliberately retains zero diameter through canonical
`Value::Length(Length::ZERO)`. Its established equality with radius 0, 0.0 and
0pt is preserved. Omission retains absent dimensions and remains distinct from
explicit Auto. This bounded preservation is known default-sizing debt; it is
not correct oracle Auto/default sizing, permission to hardcode 30pt, or complete
Circle parity. Negative/nonfinite explicit radius values are not silently
clamped. Any unapproved previously matching language regression refutes this
implementation.

Circle function identity/reflection, fields, selector/show/set support,
body/inset/outset, width/height syntax and complete automatic/default sizing
remain separate work. Rect, Square, Polygon and Curve identities are not made
complete by the Geometry origin. Manually constructed inconsistent origin/kind
pairs are outside the certified native-constructor domain; the required public
field necessarily requires external Rust struct-literal migrations. Content's
compact hash may collide for unequal shapes, and the existing structural
signed-zero caveat is not repaired here. No phase change, new Content variant,
trait method, product flag, I/O, shared mutable state or layout origin dispatch
is introduced. Figure remains separately unresolved and is not changed by this
Circle authorization.

## Ownership, implementation and independent tests

`scope.json` identifies the fourteen exclusive source/owner pairs after the
measured Label-selector prerequisite below. `scope-circle-only-13.json` preserves
the original thirteen-pair Circle scope. Active L0s
were updated before tests and productive edits; `l0-before-code.json` records
that boundary. The initial diagnostic wording was corrected to the measured
length-or-auto order before tests or productive code; the initial receipt is
preserved rather than silently replaced. Only this approved addition qualifies
older clauses excluding public-field changes; unrelated contracts and nucleus
pins remain in force.

The implementation agent handled L0, productive code and literal migration.
A separate test agent supplied language, actual-layout, diagnostic, API and
observation controls. The integration agent owns actual execution and resealing;
the audit agent freezes bilateral expectations and reviews lint. The recorded
semantic RED precedes the implementation authorization. Do not count a compile
failure or a postimplementation API test as that semantic RED.

`mechanical-migration-audit.json` verifies that consumers outside the three
semantic bodies recover their baseline source after removing only lineage
header changes and Geometry initialization. `implementation-ready.json` records
the source state before post-API tests. `contract-test-review.json` reviews the
initial nineteen controls and explicitly identifies the remaining invalid-Value
coverage gap; its disposition and any supplemental tests must be attached to
final validation. Merely entering one grouped rejection arm or proving future
E0004 does not execute every current alternative.

Language controls cover equality/inequality, aliases, numeric/Length equivalence,
Circle/Ellipse distinctions, mixed radii, arrays, dictionaries, membership and
Source/ordinary semantic wrappers. Actual layout controls cover explicit point,
em and mixed radii at different occurrence fonts and through Block/Box. API
controls verify constructor invariants, terminal maps, conservative observation
and unchanged kind-selected rendering. The independent paint corpus checks
physical output; local stored dimensions alone cannot certify painted bounds.
Retain all preexisting acceptance conditions and frozen case-specific policies.

## Measured Label-selector prerequisite

The candidate replay retained only the first of two shared Circle measurements
under different fonts. Independent controls in
`agents/audit/context-investigation/receipt.json` reproduce the same loss for
direct metadata without Circle or Context. The cause is the Label selector's
first-only projection, not the new Circle carrier or occurrence font resolver.
The frozen replay policy remains unchanged.

The queries owner was updated under ADR-0127 continuous parity flow before
independent RED and code. Selector::Label must use the existing complete label
registry, retain distinct occurrence identities, sort by recorded causal order
and deduplicate only repeated Locations. The separate query_by_label trait
projection remains first-insertion/Option<Location> for compatibility. Other
selector branches, registry behavior, pipeline phases and callbacks remain
unchanged. `label-query-before.json`, its source archive and
`label-query-l0-before-code.json` bind this bounded scope extension. Implementation
and accepted validation of this prerequisite remain pending in this draft.

## Final validation — pending

The final report must cite exact accepted state and receipts for:

- Semantic RED and complete focused GREEN, including supplemental nominal cases.
- Real Value/ShapeOrigin future-variant E0004 proofs in disposable copies.
- Frozen bilateral language, layout and paint replay, preserving explicit debts.
- Full tests, build/all targets, strict Clippy and Rustdoc, and formatting.
- Canonical paths, ownership, V15/V26, direct and reverse lineage.
- Exact full-linter delta, including retained V19 notes and unrelated V16 debt.
- Source freeze, final evidence manifest, commit identity and checkout status.

Final counts and all-pass claims are pending. This draft does not close batch F,
Figure, or overall vanilla parity. Existing batch-F evidence remains immutable;
this follow-up records new evidence under `circle-origin/`.

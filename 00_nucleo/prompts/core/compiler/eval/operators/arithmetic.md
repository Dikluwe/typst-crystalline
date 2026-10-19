# L0 — compiler/eval/operators/arithmetic
Hash do Código: d6a9123b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/arithmetic.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/operators/arithmetic.rs:27` — `fn apply_binary`.
- `01_core/src/compiler/eval/operators/arithmetic.rs:518` — `fn sanitize_length_nan`.
- `01_core/src/compiler/eval/operators/arithmetic.rs:538` — `fn eval_unary_op`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/args.rs:468-482`.
- `foundations/ops.rs:344-359`.
- `foundations/ops.rs:45-53`.
- `foundations/ops.rs:80-84`.
- `lab/typst-original/crates/typst-library/src/foundations/ops.rs:94–95`.
- `typst-utils/src/scalar.rs:30-32`.

## Contract

Own Add/Sub/Mul/Div/And/Or, unary evaluation and sanitize_length_nan.
Before division dispatch, reject zero Int/Float/Decimal/Length/Relative/Ratio/Angle
with `cannot divide by zero`; retain Duration's own zero checks. A Relative
divisor is zero only when both components are zero. Keep this precedence even
when the dividend would otherwise be incompatible.

Preserve the supported binary domains and result types:

- Int addition/subtraction/multiplication are checked (`number too large`);
  Int division returns Float, not a truncated integer. Float and mixed Int/Float
  arithmetic propagates IEEE nonfinite values without a general NaN/Inf guard.
- Decimal arithmetic is homogeneous or mixed with Int converted exactly via
  Decimal::from(i64), never f64; checked failure is `value is too large`.
  This does not enable Decimal/Float coercion or a generic numeric tower.
- Duration addition/subtraction is checked in nanoseconds; integer/float scaling
  and division retain the current checked conversions, errors and nonfinite
  guards; Duration/Duration returns the nanosecond ratio as Float.
- Str+Str concatenates; Symbol+Symbol and Str/Symbol mixed addition returns Str
  preserving full graphemes (variation selectors and ZWJ included).
  Content with Content/Str/Symbol returns the existing content sequence.
- Array+Array concatenates without deduplication. Dict+Dict keeps first-key
  position and right-hand values. String/array repetition accepts Int on either
  side; negative repeats fail `number must be at least zero`; size overflow fails
  `cannot repeat this string {n} times` or `cannot repeat this array {n} times`.
- Length addition/subtraction keeps abs/em components separate. Scalar scaling
  and division sanitize each NaN component to zero, but preserve infinity;
  do not move this eval-specific behavior into the domain Length operators.
  Length/Length succeeds only on compatible pure axes, otherwise
  `cannot divide these two lengths`.
- Length+Color or Length+Gradient in either order constructs Stroke using the
  absolute thickness and the corresponding paint. Relative+Color in either
  order retains its current absolute-thickness stroke construction.
- Relative addition/subtraction with Relative/Length and scalar scaling/division
  preserve both components. Relative/Relative returns a Float only for compatible
  pure absolute or relative axes; otherwise
  `cannot divide these two relative lengths`.
- Ratio arithmetic and scalar scaling/division preserve Ratio; Ratio/Ratio
  returns Float. Ratio with Length in the supported + and - orders produces
  Relative. Ratio*Fraction in either order produces Fraction; Fraction+Fraction
  produces Fraction.
- Angle subtraction, scalar scaling/division and Angle/Angle preserve the current
  radians-based results. Alignment addition merges compatible axes to Align2D;
  preserve `cannot add two horizontal alignments`,
  `cannot add two vertical alignments` and `cannot add two 2D alignments`.
- Bool And/Or operate on already evaluated boolean operands; short-circuit is
  coordinated by 00_nucleo/prompts/core/compiler/eval/mod.md, not here.

Do not infer reverse division or additional pairings from the word 'scaling'.
Unsupported pairs keep the canonical mismatch diagnostic. Mixed Length/Relative
and Ratio/Relative division, Length*Ratio, Dict*Int, remainder and // are not
enabled by this contract.

Args+Args Add delegates exclusively to
00_nucleo/prompts/core/compiler/eval/operators/join.md. Args+None and None+Args return
the same Args, preserving aggregate span, occurrences and views; do not reconstruct
through the detached-span Args join. Do not route every join-compatible pair into Add.

Unary Neg retains Int checked overflow and the supported Float/Decimal/Length/
Relative/Angle/Ratio/Fraction/Duration results. Pos is identity on the current
Int/Float/Decimal/Length/Angle/Ratio/Relative/Fraction domains; Not requires Bool.
Mismatch wording delegates to error_formatting. Tests distinguish checked
overflow, zero precedence, IEEE behavior, repetition, grapheme preservation,
component compatibility, paint construction and Args metadata conservation.

## Provided Stroke components for Length+Color

Cell-style physical-complete.json.gz/stroke-with-body-recipe measures literal
red+2pt rendered black 1pt in Tekt versus red 2pt vanilla even after borders repair.
Collection retains HEAD/tree/sources/binaries/UTC/argv/PDF/SVG.
arithmetic.rs:336–344 uses ..Stroke::default() without provided bits, letting
fold_stroke replace components by Table default. Pinned foundations/ops.rs uses
Stroke::from_pair with both Custom; geometry model retains other omissions.
Color/thickness/getter is language, not Rust equality requirement.

Internal continuous ADR-0127: existing Length+Color/Color+Length arms set only
specified.paint/specified.thickness true. Keep values/casts/absolute thickness/
overhang/signature/other arms/unspecified components. Expand no Gradient/Relative/
em resolution here. RED→GREEN both orders before Table fallback, physical correct
color/thickness vector/full suite. No new API/phase/mode/default/compatibility.
Rich strokes/relative/em stay incomplete outside certification.

## Ratified compact Value carrier adaptation

Before deciding,
`lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/before.json.gz`
and inventory.json freeze this consumer/all Value::Stroke file:line occurrences.
sizes.json measures external Value model272→144 bytes, no RSS proof. New public
carrier belongs only to `00_nucleo/prompts/core/entities/value.md`; this unit
consumes without shared ownership. Earlier implementation gate was satisfied by
human `confirmo` on2026-10-08, manifest
`6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237`;
proposed clauses are current for materialization.

Three existing Stroke-producing arms use retained From<Stroke> conversion.
Change no operations/provided bits/casts/defaults or Gradient/Relative scope.
Keep tests/oracles, pertinent RED→GREEN if new isolation behavior exists,
owner-only reseal/revalidation. No semantic/default/phase expansion.

## Content concatenation delegation

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c` and working tree/UTC/
diffstat/sources in state-stack-2026-10-17/before.json.gz.
morphology-before.json.gz/morphology-measured.json.gz compare loop/+ repr at
`bd5ebec285505889e8ff1d4f68804ab147a699f3`: Tekt nests Sequence per join;
vanilla concatenates immediate children. debug-root.log locates stack failure
while scanning64-update-produced chain. Language form/failure, no copying Rust
shape or stack increase required.

Five Content Add arms with Content/Str/Symbol delegate after current coercions
to concat_content owned by
`00_nucleo/prompts/core/compiler/eval/operators/join.md`. Duplicate no body or
route every join-compatible type to Add. Preserve full domain/numeric/string-
symbol/array/Args/repetition/error matrix above. Test both operand sides and
Str/Symbol-to-Content coercions including empty text with existing P900 controls.
L0 before code, RED→GREEN/workspace/build/Clippy/Rustdoc/lineage/fmt under
12GiB/High 8 GiB/swap0/jobs1. Internal continuous ADR-0127, no public API/phase/
default/compatibility change.

## Closed-domain decision and preservation

### Evidence before decision

At checkpoint `370d78355bd646d1abead5bcacef215288cc8834`,
`01_core/src/compiler/eval/operators/arithmetic.rs:28–507`
and the caller/producer evidence recorded in
`lab/results/v16-corrections-2026-10-19/batch-c/agents/structural/l0-audit.json`
show the current decision and its refusal path. That receipt records UTC,
working-tree diffstat and exact source/owner SHA-256 before this amendment.
The existing clauses above establish the behavior to preserve; an open Rust
match is structural evidence, not proof of additional language admission.

### Exclusive policy

Int(0), Float ==0 (including negative zero), Decimal.is_zero, Length.is_zero, Relative
with BOTH abs and rel zero, Ratio zero and Angle zero return cannot divide by zero
before binary dispatch. All guard-refused payloads of those same variants plus all
listed complement variants continue. Duration zero remains governed by later operation-
specific checks, while invalid operand kinds later get type/operator errors.

Move zero checks inside the seven nominal numeric arms. Same-kind nonzero values
continue normally, including Float NaN and infinities; Relative requires both components
zero. Other families preserve later operator validation (Duration zero remains its own
rule). Test every zero family with an incompatible dividend to prove precedence; test
nonzero counterparts and Relative with either nonzero component, signed Float zero, IEEE
controls and invalid nonnumeric types.

The explicit nominal complement is: `None`, `Bool`, `Str`, `Array`, `Dict`, `Module`, `Datetime`, `Func`, `Content`, `LocatedContent`, `Auto`, `Color`, `Stroke`, `Fraction`, `Align`, `Location`, `Gradient`, `Regex`, `Tiling`, `Bytes`, `Duration`, `Version`, `Selector`, `Symbol`, `Args`, `State`, `Counter`, `Label`, `Dir`, `Path`, `Type`, `SpotColorant`.

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

# Informational-note review — batch C initial candidate

This review compares `baseline.sarif.gz` with
`initial-candidate-lint/after.sarif.gz`; it does not certify a later final scan.
The baseline was measured at 2026-10-19T12:37:19.791186+00:00 on clean HEAD
`370d78355bd646d1abead5bcacef215288cc8834`. The candidate was measured at
2026-10-19T13:29:03.510547+00:00 on the same HEAD with an uncommitted working tree.
Exact changed-file stat, input hashes, linter identity, command and end times are
in `baseline-state.json.gz` and `initial-candidate-lint/lint-state.json.gz`.
`note-review-delta.json.gz` retains those provenance links/hashes, full counts and
the complete added/removed note objects including original source positions.

Comparison uses a multiset of rule ID, severity, source URI and exact message
text. Only source positions are ignored when deciding whether content changed;
duplicate notes remain separate occurrences. No notes or errors are suppressed.

| Diagnostic | Baseline | Initial candidate | Content additions | Content removals |
|---|---:|---:|---:|---:|
| V19 informational grouped alternatives | 1166 | 1195 | 30 | 1 |
| V20 informational pattern depth | 878 | 878 | 1 | 1 |
| V16 errors | 83 | 64 | 0 | 19 |
| V14 errors | 0 | 1 | 1 | 0 |

The V14 is the newly detected direct regex escape dependency and remains a real
error in this scan. The coordinator reports a separate producer-side repair in
progress; this note review does not classify that error as informational or closed.

## What the V19 changes mean

Each new V19 reports an explicit grouped nominal policy. Executing a shared arm
proves neither every variant nor every payload/guard state. These notes therefore
remain useful after wildcard removal and must stay enabled. Their growth is not
an independent quality metric, and the smaller V16 count is not semantic closure.
The added note occurrences by owner are:

| Source under compiler/ | Added V19 occurrences |
|---|---:|
| `eval/bindings/field_access/fields.rs` | 1 |
| `eval/bindings/value_methods/element_where.rs` | 1 |
| `eval/bindings/value_methods/selector.rs` | 2 |
| `eval/call_dispatch/content_methods.rs` | 1 |
| `eval/call_dispatch/instance_methods.rs` | 2 |
| `eval/call_dispatch/value_methods.rs` | 3 |
| `eval/expression_dispatch.rs` | 3 |
| `eval/markup_stream.rs` | 2 |
| `eval/modules.rs` | 1 |
| `eval/operators/arithmetic.rs` | 1 |
| `eval/operators/equality.rs` | 2 |
| `eval/rules/intercept.rs` | 1 |
| `eval/rules/realize.rs` | 1 |
| `eval/rules/set.rs` | 1 |
| `eval/rules/set/page.rs` | 1 |
| `eval/rules/set/text.rs` | 2 |
| `eval/selector_matching.rs` | 5 |

The groups have different responsibilities and cannot be justified by one blanket
“all other cases are unsupported” statement:

- Matrix/Cell field presence is a narrow family discriminator. Other Content
  keeps ordinary field ownership; snapshots remain authoritative. Selector
  locatability terminals only lack native metadata in that validator; a separate
  contextual validator recursively rejects nonlocatable boundaries and Regex.
- Operand conversion and receiver admission are separate gates. Refused Value
  groups do not run method arguments; accepted element operands preserve actual
  identities. Content/instance dispatch complements defer to other handlers.
  The nested Type complement preserves Counter's static-method exception.
- Include's placeholder group supplies diagnostic context before the existing
  rejecting dispatcher. Division's complement bypasses only the preliminary
  zero gate. Outline inheritance's complement supplies no inherited indent.
  Page bleed's scalar complement delegates to the existing converter, preserving
  both successful conversions and type errors. Font-name groups omit only a
  literal warning name, not validation. The Expr group distinguishes evaluated
  targets from static name paths without prohibiting expression evaluation.
- Static discarded-update inspection separates update nodes from mere state or
  counter values/displays. Its broad Content complement permits normal static
  child traversal rather than asserting that descendants contain no update.
  Metadata's Value complement leaves Dict/functions opaque; Content/arrays have
  their explicit recursion. Ref/RawLine/constructor Heading and Page-body policy
  are deliberate local boundaries. The removed V19 was the older conflated
  State/StateUpdate/CounterUpdate/CounterDisplay/CounterDisplayCallback group.
  Figure/Equation supplement provenance remains a documented semantic debt.
- Display's Counter/None pair has no display output; its remaining bound Value
  alternatives use the preexisting repr conversion, while rich transported
  content retains separate display semantics. These are not universal empty
  fallbacks.
- Paragraph recognition preserves the existing Source/ShowRecipe/Styled stage
  partition. Its explicit complement does not prove that every completed carrier
  is globally unreachable or resolve the documented raw-flow grouping debts.
- Selector/equality groups encode different constructor or ABI identity, retain
  separate Format witnesses, and preserve ordered recursive comparisons. Native
  matcher complements retain existing unsupported/ambiguous identity limits;
  native field projection delegates to get_field rather than discarding fields.
  State-like kinds lack the specified native show bridge. No Shape/dynamic
  identity certification follows from listing their variants.

`agents/structural/v19-review.md` gives the detailed owner-by-owner rationale for
the structural cases. The final selector/equality and heading-snapshot reviews
record the boundary audits and known limits, independently of these note counts.

## V20: unchanged count, different content

The removed note was the nested font warning pattern `Some(Value::Str(s))`.
The producer now separates Option presence from an exhaustive nested Value
match, making future Value review explicit. The added note is the bound display
pattern `other @ (Value::Int(..) | …)`: the binding preserves the original Value
for repr conversion while all current variants are named. Its depth remains
visible; it is not a reason to hide the pattern in a wildcard or helper.

Both occurrences report depth three over the informational threshold two. The
net-zero total does not mean the note set was unchanged: one actual removal and
one actual addition are retained in the delta. The other V20 contents were
unchanged under the position-independent comparison.

## Evidence and certification limits

Preservation controls check prior positive/refusal behavior, error precedence,
spans, unevaluated arguments and snapshot/field authority. Semantic corrections
have separate source-level RED controls and current-pin oracle evidence; internal
fabricated carriers alone do not establish public language reachability. Existing
positive and negative tests supplement the new controls rather than being
replaced by a grouped-arm execution count.

The planned real-domain enum mutation tests answer a narrower question: does a
new nominal variant fail compilation at each selected decision until its owner
classifies it? Pair axes must be isolated; nested Value/Type/FuncRepr domains have
separate targets. Such E0004 evidence cannot prove correct semantics for every
current variant, payload, tuple combination or runtime path. The original19
baseline witnesses and new supplemental matches must not be conflated.

Final compile/test, structural, oracle, paths and lineage receipts remain the
coordinator's gates. Recompare the final SARIF after the V14 repair and any later
source changes; this initial-candidate review does not imply zero repository
violations, full parity, or closure of the explicitly retained debts.

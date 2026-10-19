"""Publish the current summary only after the closure audit succeeds."""
from pathlib import Path
import json

out = Path(__file__).resolve().parent
ledger = json.loads((out / 'closure-ledger.json').read_text())
assert ledger['full_gates'] == 12 and ledger['remaining_v16'] == 20
ended = ledger['validation_end']
inputs = ledger['input_count']
report = f'''# Circle origin and explicit radius: validated correction

V16-065 is closed on `codex/tekt-mature-layout`. Batch F now has 17 validated
repairs and one pending item, Figure V16-055. The full pinned linter reports
**21 → 20 V16 errors**; this is not an overall zero-violation or full-parity claim.

## Approval, cause and implementation

The user explicitly approved `ShapeOrigin` and required `ShapeElem.origin`.
[Approval](approval.json) binds proposal SHA-256
`5b7136bc18f419004113d8d83c9c54adb816e3b70389039aeeed35e1c6f772e8`.
The baseline is `954d13185e26a1ed90f6d7c60f6ba27760ca9889`; every measurement
records its dirty-tree stat, UTC and input/executable hashes. The baseline binary
is the prior validated build whose 2132 source/config inputs were verified equal
to this clean baseline, not a claimed rebuild at this HEAD. Its original build
provenance remains in [the baseline receipt](baseline-build-receipt.json).

Circle previously discarded the em component of radius. Retaining full Length
alone was already refuted by language equality regressions. Native Circle now
constructs origin Circle with kind Ellipse; generic constructors and literals
use Geometry. Origin participates in structural equality/hash and conservative
observation and survives clones/maps. Rendering still dispatches on ShapeKind.
Int/Float retain their point-number extension; explicit Length retains abs/em
until layout and stores the full doubled Length. Malformed radius values are
rejected exhaustively at their causal value/argument span, before paint parsing.
Existing name precedence, fill/stroke and signed/nonfinite controls remain.

The frozen shared-occurrence replay exposed an additional Label-selector defect:
both Context measurements existed, but the selector returned only the first.
[Independent probes](agents/audit/label-cardinality-cause.json) reproduced it
without Circle or Context. The compiler-owned Label query now returns all matches
in causal order and deduplicates only identical Locations. The separate
query_by_label first-result API is unchanged. This measured internal parity
correction followed ADR-0127 continuous flow, its own L0 and semantic RED.
No Context, pipeline-phase or additional public API change was needed.

## Validation

Three agents separated implementation/contracts, independent tests, and audit.
Owning L0s preceded code; [scope](scope.json) contains fourteen exclusive pairs.
The original thirteen-pair Circle scope is preserved separately.

- The original fourteen tests produced six semantic RED failures and eight
  preservation passes. Label added two semantic RED failures and one preservation
  pass. Six additional post-API/domain controls complete **23 new tests**, all
  observed passing in the final full suite, including all 35 rejected Value
  classes and all four ShapeOrigin observation pairs.
- Real copied-workspace enum extensions yield **five exact primary E0004
  witnesses** for Value, ShapeOrigin and the bounded test observer's FrameItem.
  [Independent raw-byte review](agents/tests/structural-review.json) verifies
  these and baseline absence; unrelated errors do not count as witnesses.
- All **twelve integrated gates** finished at `{ended}` with **{inputs} unchanged
  inputs**: CI/path checks, strict forward/reverse lineage, V15/V26, workspace
  build/tests, wiring, doctests, all targets/benches, Clippy, fmt and Rustdoc.
  Product packages were cleaned after copied-workspace probes.
- **224 frozen language cases**, **6 exported-ink cases**, and **18 additional
  Label controls** pass their unchanged case-specific policies. Paint measures
  saturated fill/stroke bounds after SVG rasterization at 288 dpi, with a fixed
  2-pixel (0.5pt) tolerance; it does not compare output bytes.

[Commands](validation/validation-commands.json),
[frozen state](validation/validation-state.json.gz),
[accepted binary](accepted-build-receipt.json),
[language replay](accepted-replay/receipt.json),
[paint replay](accepted-paint/receipt.json), and
[Label replay](accepted-label-controls/receipt.json) bind these results.
The oracle remains commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Build/test resource limits were one Cargo job, 12 GiB maximum, 8 GiB high and zero swap.

## Alerts, failed attempts and remaining obligations

The pinned linter and scan scope are unchanged. Its full scan still exits 1 for
the other twenty V16 errors. [Independent comparison](agents/audit/lint-review.json)
finds exactly V16-065 removed, no new error/warning, and identical diagnostics
outside the fourteen consumers. V19 changes 1259 → 1262; V20 remains 877.
Every prior note is retained. Three new grouping notes remain enabled: the 35
invalid Value alternatives are exercised; the bounded frame observer does not
claim runtime coverage of every recursive/ignored wrapper alternative.

The first candidate passed 19 tests but failed the shared-occurrence replay
(223/224), and its test observer introduced a new wildcard V16. Both attempts
remain archived. Repairs made the observer nominally exhaustive and fixed Label
cardinality; no frozen acceptance policy, old expectation or severity was weakened.
Original batch F history is unchanged.

Explicit Auto still means canonical zero and differs from omission; complete
automatic/default sizing remains debt. Circle func/fields/reflection, native
selectors/show/set, body/inset/outset and width/height syntax remain out of scope.
Other shape identities, unrelated selector(label)/locate behavior, contextual
style debt and Figure V16-055 remain visible in their existing obligations.
The public required field needs migration in external Rust struct literals.
The [closure ledger](closure-ledger.json) binds all accepted source/owner hashes.
'''
(out / 'README.md').write_text(report)

progress = Path('00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md')
text = progress.read_text()
old = 'Status: batches A–E are closed; batch F has 16 validated repairs and two\nexplicit pending items. The remaining 21 V16 findings remain open'
new = 'Status: batches A–E are closed; batch F has 17 validated repairs and one\nexplicit pending item (Figure V16-055). The remaining 20 V16 findings remain open'
assert old in text
text = text.replace(old, new, 1)
old = '''Circle065 has an
[independently reviewed proposal](../../lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md)
for typed constructor origin and full explicit radius. It awaits ADR-0127
approval for the public entity change; its source/L0 remain identical to baseline.'''
new = '''At that checkpoint, Circle065 had an
[independently reviewed proposal](../../lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md)
for typed constructor origin and full explicit radius, pending ADR-0127 approval.
The approved follow-up below supersedes that pending status; the original batch F
receipts remain unchanged.'''
assert old in text
text = text.replace(old, new, 1)
section = f'''## Circle follow-up: approved origin and full explicit radius

The user approved ShapeOrigin and ShapeElem.origin after the original batch F
checkpoint. Baseline HEAD is `954d13185e26a1ed90f6d7c60f6ba27760ca9889` on the
same branch. Native Circle now retains constructor identity and full abs/em
radius through the existing layout resolver; generic shapes retain Geometry
origin. Numeric/Length equivalence, fill/stroke and explicit Auto-as-zero debt
remain. Malformed values are rejected by an exhaustive decision with causal spans.

The frozen shared-occurrence replay additionally exposed Label-selector
first-only projection. Independent controls reproduced it without Circle or
Context. Its compiler-owned branch now preserves all distinct causal occurrences,
while query_by_label keeps its first-result API. This internal parity repair had
its own L0 and semantic RED; no Context or phase change was introduced.

The full pinned scan at `2026-10-19T22:28:29.489373+00:00` reports **21 → 20 V16**,
exactly V16-065 removed, no new error or warning and identical diagnostics outside
fourteen source/owner pairs. V19 is 1262 and V20 is 877; all prior notes and the
three new grouping reminders remain visible. The bounded test observer's complete
runtime alternative coverage is not claimed. State, hashes and independent review
are in [the follow-up report](../../lab/results/v16-corrections-2026-10-19/circle-origin/README.md).

All **23 new tests** pass in the fresh full suite: eight observed semantic REDs,
nine preservation controls and six additional post-API/domain controls. Five
exact compiler witnesses passed independent raw-span review. All twelve integrated
gates finished at **{ended}**, with **{inputs} unchanged inputs**. Paths, strict
direct/reverse lineage, V15/V26, builds, tests, Clippy, formatting and Rustdoc pass.
The 224 frozen language cases, six exported-ink controls and eighteen Label
controls pass their case-specific policies against the ratified oracle. The first
223/224 replay and observer-lint failure remain preserved, with no weaker expected
result or suppression. Accepted source/owner hashes are in
[the closure ledger](../../lab/results/v16-corrections-2026-10-19/circle-origin/closure-ledger.json).

Batch F now has 17 repairs and Figure V16-055 pending. Auto/default sizing,
Circle reflection/selectors/show/set and the other explicitly deferred surfaces
remain obligations; this is not complete Circle or language parity.

'''
anchor = '## Remaining obligations and organizational result\n'
assert anchor in text
text = text.replace(anchor, section + anchor, 1)
text = text.replace('The two pending F items and batch G remain open.', 'The pending Figure item in F and batch G remain open.', 1)
progress.write_text(text)
print('Published validated report and current diagnostic; historical batch F unchanged.')

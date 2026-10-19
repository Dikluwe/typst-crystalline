# Batch F geometry and Figure audit

Status: measured proposals; no productive edits authorized by this report.
Source provenance is in `initial-audit-state.json`; oracle eval and realized-query
receipts record exact UTC, argv, sources, binary SHA, HEAD and working-tree diff.
The oracle commit is bd5ebec285505889e8ff1d4f68804ab147a699f3 and its binary SHA is
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9. The initial
oracle-only receipts do not claim product behavior or successful compilation of
any candidate. Their repr observations identify accepted input and exposed values,
not a byte-level rendering acceptance gate.

## Evidence before classification

- `stdlib/layout/grid.rs:11-33,60-61` projects track values then silently filters
  failed array members; `stdlib/structural/table_grid.rs:71-72` shares it.
  Pinned `layout/grid/mod.rs:455-463` casts every array member fallibly and accepts
  positive integer counts. `layout/container.rs:476-525` preserves relative
  dimensional tracks. Oracle eval cases for grid columns/rows and table columns
  reject None, Boolean, wrong String and invalid array members; 0/-1 counts fail
  with `number must be positive`. Empty tuples are accepted. Float and `"auto"`
  reject upstream, but their existing Tekt extension must not be removed casually.
- `stdlib/layout/place.rs:34-51` projects Float/Int/Length absolute part and sends
  every other Value to zero. Its public `PlaceElem` dx/dy fields are f64; pinned
  `layout/place.rs:163,169` uses relative lengths. Oracle accepts Length/em/Ratio/
  Relative and rejects None/Auto/Boolean/String/Fraction/Array. Existing Tekt
  numeric extension is explicit in current L0. This separates malformed arguments
  from an unresolved dimensional-carrier limitation.
- `stdlib/shapes/ellipse.rs:84-100` projects circle radius to points and doubles
  it, losing em and silently converting invalid values to zero. Pinned
  `visualize/shape.rs:272-294` casts Smart<Length> and doubles the entire length;
  its radius accepts Auto. Oracle measured 2em at 10pt and 20pt to produce 40pt
  and 80pt dimensions, and 2pt to produce 4pt at either font. The existing Shape
  Value dimension carrier and layout `shape.rs:28-51` already preserve Length.em;
  no public carrier or phase change is needed for that correction.
- `stdlib/shapes/line.rs:30-37,106-140` reuses a zero-return projection for legacy
  dx/dy and already-guarded polar length. Pinned line has no dx/dy arguments:
  all oracle dx cases are `unexpected argument: dx`. Current shapes L0 explicitly
  retains this extension, numeric points, and its zero/zero omitted default. A
  malformed legacy argument correction must be grounded in that typed local
  contract, not described as upstream dx parity. Existing length/angle/end
  precedence and errors must remain intact.
- `stdlib/figure_image.rs:60-71,128-145` infers Image/Table/Raw through Sequence
  only and uses Option<String>. Pinned `model/figure.rs:345-363` synthesizes typed
  FigureKind and `foundations/content/mod.rs:408-464` traverses content-valued
  fields/arrays. Ten realized show-rule probes identify table/raw through Strong,
  Emph, Hide, Box, Block and nested Figure; their kind type is function, and the
  fallback is image. Raw unsynthesized kind getters reject until synthesis.
  The active Figure L0 explicitly leaves this recursion/phase/typed-kind work
  under FG1; the referenced follow-ups.md explicitly says this V16 remains open
  and prohibits inventing neutral container leaves to silence it.

## Proposed bounded correction and honest limits

The selected matches are mechanics, but argument acceptance, diagnostics,
cardinality, lengths and typed synthesized kind are language semantics. Source
inspection and oracle behavior do not by themselves establish historical intent.

For tracks, validate every supplied member before extraction and reject malformed
inputs with causal supplied-value span; preserve array order, omission/empty tuple
defaults, positive-count expansion, legacy Float and `"auto"` acceptance. Both Grid
and Table owners must participate. Do not silently discard Ratio/Relative merely
because the current public TrackSizing cannot retain them, and do not replace a
previously accepted dimensional limitation with a new error while calling it full
parity. The minimal implementation boundary remains to be chosen by the parent.

For Place and legacy Line, make the point projection fallible for non-dimensional
values, preserve numeric extensions and existing supported absolute lengths, and
name the unresolved relative/em projection in the owning L0. RM1 explicitly
prohibits calling the truncation final semantics or replacing accepted percentages
by errors. Full dimensional support requires a separately reviewed public carrier
and layout plan. Do not change end/polar priority, missing-value defaults, stroke
validation priority, callback counts, or mixed-invalid precedence accidentally.

For Circle, preserve the entire doubled Length in existing Value dimensions and
handle explicit Auto through the existing Auto dimension path. Retain numeric
point extensions; reject unsupported radius values with measured expected-length-
or-auto vocabulary. Preserve omission, negative/zero dimensions, stroke/fill
fallbacks and their validation order. This is continuous internal parity flow
under ADR-0127; no public field/signature/default mode/phase changes are needed.

For Figure, do not remove V16-055 in this bounded batch by a fabricated exhaustive
negative list. A complete FG1 solution needs typed-kind/presence/synthesis API and
phase decisions already identified by the owner. A reviewable follow-up must read
all those owners, supply L0s, migration list and tests, and obtain its actual gate.
The earlier user approval for Context occurrence identity does not authorize FG1.

Refuters: a source or current L0 proving invalid inputs intentionally mean zero;
a previously accepted non-dimensional extension documented elsewhere; a Circle
layout consumer that loses Length.em; a Grid/Table caller bypassing validation;
future enum variants accepted silently; counter/selector/show results that differ
from the claimed bounded repair. Tests and bilateral product runs remain required.

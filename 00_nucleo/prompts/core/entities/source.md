# L0 — In-memory source and syntax tree
Hash do Código: ace32789

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/source.rs`
**Governance:** ADR-0016, ADR-0031, ADR-0045.

## Evidence and responsibility

The historical reference is
`lab/typst-original/crates/typst-syntax/src/source.rs`. Source owns loaded text
and its CST. L3 acquires file bytes/text; this pure type accepts text already
in RAM and parses it. The earlier world_types stub and staged implementation
chronology are archived rather than a second current owner.

## Contract

Preserve private Source backed by Arc<SourceInner>, with fields id: FileId,
text: String, root: SyntaxNode. The original Arc<LazyHash<SourceInner>> was
replaced with Arc and Clone under ADR-0016. Preserve public signatures:

```rust
impl Source {
    pub fn new(id: FileId, text: String) -> Self;
    pub fn detached(text: impl Into<String>) -> Self;
    pub fn id(&self) -> FileId;
    pub fn text(&self) -> &str;
    pub fn root(&self) -> &SyntaxNode;
    pub fn len_bytes(&self) -> usize;
}
```

new/detached parse to construct root. detached uses
FileId::from_raw(NonZeroU16::new(1).unwrap()) as fixed sentinel (or the
corresponding detached helper convention), with no filesystem access.
Retain the later public new_with_parser/detached_with_parser, content_hash,
span_to_line_col, and span_byte_range methods. span_byte_range resolves direct
raw ranges or numbered spans through LinkedNode::find().range(); detached,
other-file or missing spans yield None. The call-trace and shell rendering
consumers rely on this range behavior, analogous to upstream world.range.

## Acceptance and remaining coverage

new(id,"Hello *world*") has non-erroneous Markup root, original text and id.
Empty text has zero bytes. detached("= Heading") has a Heading child;
detached("#{{{broken") is erroneous. Preserve parser injection, content hash,
line/column and range behavior with their current integration tests.
Incremental reparse was excluded in the original bounded scope. The old
blanket exclusion of LinkedNode in find is superseded specifically for the
later span-byte-range method. The old future Hash/Eq/comemo-isolation plan
is historical, not permission to remove current hash APIs or alter L1 purity;
any unresolved trait/representation mismatch needs its own owner review.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/source.md`. Historical observations are not a new parity certification.


## Approved fallibility contract — 2026-10-22

Supersede only the four infallible Source construction signatures with:

```rust
pub fn new(id: FileId, text: String) -> SourceResult<Self>;
pub fn new_with_parser(id: FileId, text: String, parser: fn(&str) -> SyntaxNode) -> SourceResult<Self>;
pub fn detached(text: impl Into<String>) -> SourceResult<Self>;
pub fn detached_with_parser(text: impl Into<String>, parser: fn(&str) -> SyntaxNode) -> SourceResult<Self>;
```

SourceResult is the existing `Result<T, Vec<SourceDiagnostic>>`; add no error entity,
trait, mode or flag. Numbering failure returns one error diagnostic
`cannot number source spans`, anchored with the real FileId to the input's raw
range via existing Span::from_range. Do not publish an unnumbered/partial Source,
construct a detached success, substitute empty text, catch a panic or exit the
process. Keep content hashes/id/text/AST and valid-source spans unchanged.

Use NonZeroU16::MIN for the existing detached sentinel. Propagate invalid internal
numbering intervals, overflow and invalid child-range failures via existing
NumberingResult before the constructor maps that failure. This does not authorize
synthetic success for invalid SyntaxNode kinds or ignore their separate constructor
contract. Parser callbacks keep their existing function-pointer signature; this
phase does not certify arbitrary external callback implementations.

Migrate productive callers to propagate the existing Result channel, including
expression eval. Test helpers may assert a known fixture succeeded. No productive
caller may restore unwrap/expect, unsafe, a default Source or catch_unwind.
Source/byte cache identity and diagnostics must retain actual causal origin.


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

The private constructor may accept a numbering interval so unit controls can
exercise the real capacity/error mapping without allocating an enormous tree.
Public constructors always supply the immutable Span::FULL domain interval;
this introduces no runtime mode, flag, limit or production fallback.


## Approved laboratory revision API — 2026-11-12

### Evidence before decision

Frozen streammemo source, HEAD48e5a2570159c6fc325babe883bac7d1c1fbd4b2,
uncommitted laboratory tree (evidence/freeze-streammemo.json): Source::new at55
and with_numbering_interval at76–95 parse and number the entire CST; the stride
at syntax_node/representation.rs:425–470 depends on descendant count. The
source_session owner removes a changed entry and calls Source::new again at65.
incremental-boundary-audit/receipt.json binds the current core rlib and observer:
a51-node document preserves the for-loop span under a marker/text-length edit,
but adding a heading before or after creates57nodes and preserves zero exact
(kind,text,span) records, even though the loop text remains identical. This is
mechanical identity evidence, not proof that spans are a complete cache key.
The existing replace_children at representation.rs:485–562 already retains
unchanged ordered edges and renumbers a bounded neighborhood when necessary.

### Exact proposed public addition

```rust
impl Source {
    pub fn reparse(&self, text: String) -> SourceResult<Self>;
}
```

The user approved this exact laboratory proposal (SHA-256
ed79ec83553c1a8aac50b3aa3675701a75df6168db5e76aed7d573f116622b35),
then instructed: "Para o laboratório não precisa autorização". These approvals
cover laboratory materialization only; integration remains excluded.
Preserve every current fallible constructor and other public signature. Return
an immutable revision with the same FileId; the old Source and all its clones
remain unchanged. Retain the original pure parser function and numbering interval
as private immutable provenance in SourceInner, including custom code/math parsers.
No new public field, parser mode, global state, file access, clock or dependency.
Hash remains eagerly computed from exact new text under ADR0031; Source Eq/Hash
behavior is unchanged and is never used alone as revision/reuse proof.

### Revision construction and bounded reuse

Identical bytes may return the existing immutable Source. For changed text, run
the retained parser on the complete new text first: this phase establishes the
current syntax and diagnostics. Reconcile that new tree with a private clone of
the old numbered tree. Use exact spanless syntax equality (kind/text/structure/
error payload), not hashes, to retain unchanged ordered prefix/suffix children.
Use existing replace_children numbering for changed sibling ranges, including
its bounded-neighborhood expansion. Where a changed pair has the same inner
kind and compatible child topology, descend and update parent length/descendant/
error metadata through existing internal operations; never reuse an old node's
payload merely because its old span or byte length matches. No approximate
matching, hash-only admission, moved-node identity promise or fixture markers.

If preservation is not valid for a subtree, replace/renumber that actual subtree.
If reconciliation cannot produce a valid numbered tree, discard the private
attempt and number the fresh parsed tree in the retained interval. If that also
fails, return the existing origin-qualified numbering diagnostic; no partially
modified Source, panic, empty/default source or stale tree. This is identity
preservation, not a claim of incremental parsing speedup: full parse is retained.
Every produced root must be spanlessly identical to a cold parse with the same
parser, have valid searchable spans and
current byte ranges. For the normal parsers, reconstruct the complete new text;
an arbitrary custom parser is compared with its own cold output, without a new
runtime restriction requiring it to preserve input bytes. Identity retained outside an edit is not authority to reuse
any eval/layout product without checking all causal inputs.

### Acceptance and falsification

Before implementation, demonstrate changed unrelated structure invalidating the
intact loop span; that RED is separate from compilation failure for the missing
API. Test repeated insert/delete/replace sequences, prefix/suffix/middle, nested
content/code/math, duplicate equal siblings, Unicode boundaries, errors/warnings
and recovery, custom parsers, same text, old clones retained, numbering pressure
and fallback. Check every numbered span's current range and payload, uniqueness,
parent metadata and equivalence to cold parse. Add deterministic edit sequences
covering nested topology and fresh World/FileId boundaries. Runtime assertion
failures must reject mutants that retain changed text or stale ranges.

The L3 source-session owner alone selects prior revisions by canonical path and
current FileId and preserves existing bounds. Complete core/infra suites,
workspace builds, strict lint/Clippy/Rustdoc/fmt and real watch diagnostics/Unicode/
imports/font-change controls are required. Timing is measured separately; this
foundation alone does not satisfy the eval/layout reuse or vanilla CPU objective.
Laboratory only. No integration, commit or public product release is approved.


## Laboratory ordered interior anchors — 2026-11-12

subtree-validation/two-edge-observer.json binds the actual observer and core
library: an unchanged loop preserves its span after prefix insertion (4→6 root
children), but loses it after prefix and suffix insertion together (4→8).
The subtree-session runtime test fails, although same-source reuse passes.
Source::reparse currently delegates the entire differing-count range to
replace_children, whose prefix/suffix trimming cannot see an interior island.

Before replacing a differing-count child range, scan ordered exact spanless
anchors within at most32 old and32 fresh children ahead, preferring the smallest
combined skip distance. Replace the gaps between anchors with existing checked
replace_children; retain the anchor then advance. On no bounded anchor, replace
the remaining range through that same primitive, preserving its edge trimming.
No fuzzy/hash matching, unbounded quadratic diff, moved-node guarantee or
fixture-specific kind/name. Each failed candidate is discarded by the existing
fallback; immutable old sources, full-parser equivalence, uniqueness/current
ranges and numbering errors remain mandatory. Both-edge insertion is now an
explicit identity regression case; repeat deterministic edit controls.

## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.

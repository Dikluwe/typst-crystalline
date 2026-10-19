# Prompt L0 — `entities/position`
Hash do Código: 4c766ec7

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/position.rs`
**References:** ADR-0033, ADR-0073. The earlier ADR-0066 runtime deferral is superseded.

## Evidence before contract

The historical introspection audit identified upstream PagedPosition as
{ page:NonZeroUsize, point:Point }. Upstream builds its PagedIntrospector after
finalizing pages; crystalline intentionally records locations during layout.
This is an implementation difference, with equivalent location-to-position
observables required for a document, not a new equality measurement.

For HTML, HEAD `b5abe9b288ce168c494a4c5172830e325b9aedae` still had only mandatory
page/point Position. The then-pinned upstream distinguished paged and HTML
positions, projecting coordinates only inside html.frame. Simple/multiple/
nested probes found local points without a page; exposing intrinsic page 1
would contradict location.page() == none.

## Paged contract and integration

Position has public page:NonZeroUsize (one-based) and point:Point, deriving
Debug/Clone/Copy/PartialEq. Hash is manual through f64 coordinate to_bits,
not derive over Pt(f64). Preserve Send+Sync, deterministic hashing, no I/O or
mutable globals and no new external dependencies (std::num::NonZeroUsize).

LayouterRuntimeState.positions is HashMap<Location,Position>.
advance_locator_if_locatable inserts at the same gating that sets
current_location; insert replaces the entry on relayout/TOC fixpoint.
Introspector::position_of returns Option<Position>, superseding Option<()>.
TagIntrospector returns None because positions live in the layout runtime;
consumers needing them access the Layouter. A future runtime-aware/post-layout
implementation may return Some through its own contract.

## Ratified HTML carrier

Human ratification on 2026-09-24 authorizes HtmlPosition: a pure L1 Copy
carrier containing only Point, with PartialEq and manual x/y Hash. Normalize
signed zeros before hashing bits to preserve equality-implies-equal-hash in
tracked returns. It represents html.frame-local position, without page,
DOM path or origin fallback. Paged Position and all fields stay intact.
External absence stays None, never HtmlPosition::ZERO.

## Acceptance

Verify the exact Position shape and runtime map, matching locatability gating,
Option<Position> trait result, an entry for locatable content and none for
nonlocatable content. Sentinel tests assert type/field existence; structural
comparisons test manual coordinate hashing. Preserve HTML page absence and
signed-zero hash consistency. Historical source:
`lab/typst-original/crates/typst-library/src/introspection/position.rs`.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/position.md`. Historical observations are not a new parity certification.

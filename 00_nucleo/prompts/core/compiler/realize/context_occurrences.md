# L0 — compiler/realize/context_occurrences
Hash do Código: 9882f3a5

**Layer:** L1. **Exclusive consumer:** `01_core/src/compiler/realize/context_occurrences.rs`.
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.
**Status:** Ratified; materialization in progress.

## Context — ratified occurrence identity, 2026-10-18

### Measurement before decision and gate

`before.json.gz` in `lab/results/construction/diagnostics/context-paridade-2026-10-18` preserves HEAD `b7e7c83223e5daa67bbdf124c36b5b7f93200731`, sources, UTC, diffstat, argv, and binary SHA; `baseline-build-resources.json` contains the new build recipe. Vanilla pin: `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Two equations produce EQ EQ Q1/Q2; a Heading prefix produces H EQ Q0/Q1; query of `it.location` returns 00/01. `context_stabilization.rs:257` restarts `EvalContext` per producer; `eval/mod.rs:438` restarts Locator; `realize/mod.rs:41` allocates without a shared domain; `introspect/walk.rs:375–390` deduplicates by Location. This is language identity. Upstream `typst-realize/src/lib.rs:353–418` prepares identity before the recipe but does not require copying representation, vtables, or algorithm. The collision hypothesis is refuted by failing count, alias, callback, or replay controls.

The human approved proposal `review.md` in `lab/history/construction/records/00_nucleo/materialization/context-occurrences-2026-10-18`, SHA-256 `500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388`; `approval-receipt.json` is in the same directory. ADR-0127 §2.1 is satisfied for the specified additive API. This does not authorize unrelated phase or API changes.

### Exclusive responsibility

A pure arena of Locations for contextual producers within one Session. Additive public API and opaque types, without `Clone`:

```rust
pub struct ContextOccurrenceArena { /* private */ }
pub struct ContextOccurrenceFrame { /* private */ }
impl ContextOccurrenceArena {
    pub fn new(origin: &Content) -> Self;
    pub fn frame(&mut self) -> ContextOccurrenceFrame;
    pub fn evaluate<R>(
        &mut self,
        frame: &mut ContextOccurrenceFrame,
        ctx: &mut EvalContext,
        operation: impl FnOnce(&mut EvalContext) -> R,
    ) -> R;
}
```

`new` reserves Locations already carried by `origin` and those assigned by canonical introspection of that same origin. Do not use another pass's results as an increasing offset. Reservation order does not change identity: `observe` takes a maximum; it does not allocate. The arena has one Locator; a frame has private `Vec<Location>` slots and an execution cursor. An immutable `Arc` token may verify that a frame came from the same arena; its address never creates a Location.

The crate-private `next` helper delegates to the ordinary Locator when there is no active frame. With a frame, it reuses the slot in the order allocations are actually reached; when a new slot is needed, allocate from the single Locator and append the slot. Already `Located` aliases do not call `next`. Do not derive Location from morphological equality, span, text, hash, or Context ID. Slots unused after a retraction remain reserved, without phantom tags or contributions. Growth allocates at the shared free tail.

In `evaluate`, temporarily install the Locator and slots into the existing `EvalContext`, reset the cursor, call `operation` once, recover Locator/slots, and restore prior context state on every normal return, including `Result::Err`. Do not catch panic or alter compiler unwind policy. A frame belongs to its actual producer and persists across that producer's re-executions; do not clone it for another producer. Do not retain frames/arenas across compilations or create global state, I/O, unsafe code, or dynamic dispatch.

### Acceptance

Integrated RED for count and `it.location`, then bilateral GREEN on the pinned version. Local tests cover reservation of the real origin, producer disjointness, slot reuse/growth/retraction, and restoration after `Result::Err`, using real locatables and introspection. Pipeline controls cover aliases, nesting, and selective replay.

Validate build, full suite, strict Clippy/Rustdoc, formatting, lineage, and SARIF with no new error/warning, under 12 GiB/High8 GiB/swap0/jobs1. Do not claim general parity.

## Documentary provenance

The pre-consolidation owner is preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/realize/context_occurrences.md`.
The ratified API, current materialization status and acceptance remain authoritative.

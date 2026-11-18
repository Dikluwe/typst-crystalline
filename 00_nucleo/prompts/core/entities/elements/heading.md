# Prompt L0 — `entities/elements/heading` — `HeadingElem`
Hash do Código: 722975bb

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/heading.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The recorded reflection fixtures distinguish heading[H].has("level") false,
heading(level:2)[H].has("level") true, and absent level lookup error:
`field "level" in heading is not known at this point and no default was specified`.
Markup `[= H].fields()` returns depth:1 and body:[H], not level. Historical
Content anchors 1267,1573,2021–2022 and extract_payload.rs:21 support clamp,
text, fields and payload. This is retained evidence, not a fresh current-pin probe.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HeadingElem {
    pub level: u8,
    pub body: Content,
    pub outlined: bool,
    pub bookmarked: Option<bool>,
    pub set_fields: u8,
}
```

Preserve Content::Heading(Arc<HeadingElem>) and constructor level.clamp(1,6).
outlined defaults true; bookmarked None follows outlined (language auto).
Explicit-field mask bits are HEADING_SET_LEVEL=1 (positional integer or named
level), HEADING_SET_DEPTH=2 (markup via new/new_with_outlined), and
HEADING_SET_OUTLINED=4 (named outlined). bookmarked tracks presence with Option.
Upstream offset remains excluded; exposed depth equals level.
Native heading constructors set bits through heading_native/
heading_numbered_native; markup new/Content::heading use DEPTH. with_body,
both maps and introspection-time reconstruction preserve the whole mask.

## Behavior and acceptance

Plain text delegates to body; is_empty is false for this structural element.
Both maps recurse into body, preserving level, outlined, bookmarked and mask;
the older illustrative map snippets omitted newer fields and are superseded
by the explicit preservation rule. get_field("body") yields Value::Content,
"level" yields Value::Int(level as i64), "outlined" yields Value::Bool;
unknown fields yield None. Show-rule field access returns baked values without
consulting the mask; only has/at/fields read presence, in their evaluator owner.

Locatable kind is Heading. Payload is Heading { depth: level,
body_hash: hash_content(&body), counter_update: CounterUpdate::Step }.
The exhaustive extraction arm delegates to h.to_payload; ElementKind and
ElementPayload remain for other variants. Preserve the existing payload and
`heading_produz_some_payload` expectation. Layout uses level/body under its
own owner without a logic change. Verify text, fields, maps with all metadata,
reflection-presence distinction and identical own payload; if the locatable
boundary fails, revisit this L0 before changing its contract.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/heading.md`. Historical observations are not a new parity certification.


## Laboratory stable heading origin during generic mapping — 2026-11-16

Evidence before decision: origin-trace.json observes the referenced Location(2)
with a different live Heading owner at every generation, then first-failed
"miss element_at replaced". eval/rules/outline.rs:67–111 calls map_content on
the complete source even when only Outline is replaced. traversal.rs:471
unconditionally wraps HeadingElem::map_content in Some; heading.rs:136 allocates
a fresh Arc even if its body had no replacements. pipeline.rs body! is a second
independent reconstruction boundary addressed by its own owner. These are
mechanics; no changed language identity/equality rule is authorized.

Continuous internal lab refinement: extend the existing explicit no-replacement
proof to Heading. HeadingElem privately to entities provides map_content_if_changed
with the same callback signature, delegating once to body.map_content_if_changed.
Return None only when that actual traversal reports no replacement. On Some,
allocate a Heading with with_body, preserving all level/outlined/bookmarked/
set_fields. Content's exhaustive dispatcher delegates to this owner, reusing its
actual original Content Arc only on None. No equality/hash/Debug/pointer test
proves no change; Some with equal value IS a change. Preserve callback execution
order/count, errors, source/Located/Realized suppression, all metadata and original/
visual rules. Trait/public map_content signatures and behavior remain compatible;
map_content_pre/map_text unchanged. Helper visibility is entities descendants
only, no compiler bridge. No phase/default/API/field/compatibility change.

Tests first in Heading consumer via public Content.map_content: repeated no-op
retains the exact original owner and nested Strong/Emph; equal leaf replacement
rebuilds its path and all fields; callback visits are once and ordered; first
error stops later siblings and does not mutate original; equal independent
origins remain independent. The unchanged strict Weak certificate still rejects
a genuinely replaced heading, fields, body, origin or expired owner. Full core
suite plus existing L3 controls and same source/build/ELF receipts are required.
No integration or commits. This is a tested source-origin preservation experiment,
not a certificate synthesized from matching text or Location.


Test-observer refinement: the callback records Text and Heading visits with
conditional observation, without a wildcard match silently classifying Content.
Keep exactly the same visits, replacement condition and assertions.

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

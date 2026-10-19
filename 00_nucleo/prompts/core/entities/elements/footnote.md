# Prompt L0 — `entities/elements/footnote` — `FootnoteElem`
Hash do Código: b128c1df

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/footnote.rs`

## Evidence and scope

Historical upstream `e0e8ca4d`, `footnote.rs:224-231`, associates a footnote
with a counter location. Its Element counter key differs intentionally from
the crystalline string key. At `footnote.rs:114-124,174-178`, a reference
footnote reuses a number: `(!is_ref).then(Step)` excludes a new counter step.
The 2026-08-13 observation at HEAD `4f64e4e69` (working tree containing only
L0 edits) used that upstream: `A#footnote[Um] <fn> B#footnote(<fn>) C#footnote[Dois]`
yielded A1 B1 C2 and entries 1 Um / 2 Dois; the local result was
`footnote() espera content ou string, recebeu label`. This is historical
language evidence, not certification of the current pin.

## Contract

`Footnote` carries `body: Content` and `numbering: Option<EcoString>` and derives
Debug/Clone/PartialEq/Hash. `Content::Footnote` holds Arc. Preserve
`footnote(body)` with numbering None and `footnote_with_numbering(body, numbering)`.
The native constructor accepts numbering; marker formatting remains separately
owned. `plain_text` delegates to body; `is_empty` remains false because the marker
is visible even when body is empty. Both content/text maps recurse into body and
preserve numbering. Preserve derived field-sensitive equality/hash; the original
body-only equality summary does not authorize dropping numbering.

The locatable payload is `Some(Footnote { counter_update: Step })` and the flat
`"footnote"` counter advances once, without a caption/numbering gate.
`counter(footnote)` resolves this counter. This supersedes the original
marker-only scope. Default `get_field` and `element_kind` behavior remains;
locatability does not itself introduce a new kind result.

## Acceptance and unresolved work

Preserve constructors, traversal, payload, counter step and derived equality.
Label arguments/reference footnotes remain an explicit language gap, requiring
a separately approved public contract; their historical counter exception is
not implemented by this editorial consolidation. Numbering acceptance alone
must not be reported as marker-formatting closure.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/footnote.md`. Historical observations are not a new parity certification.

# Prompt L0 — `entities/elements/cite` — `CiteElem`
Hash do Código: d5ece443

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/cite.rs`
**References:** ADR-0105 model D; ADR-0107/0108/0109.

## Contract

CiteElem derives Debug/Clone/PartialEq/Hash with public key:String,
supplement:Option<Content>, form:Option<CitationForm>. Content::Cite holds Arc;
Content::cite(key,supplement,form) constructs it. CitationForm must provide
Hash alongside Copy/Eq through its own owner. The original four-unit-variant
rationale applies to Normal/Prose/Author/Year and does not erase later form
extensions specified by that owner.

plain_text starts format!("[{}]",key) and appends supplement plain text if
present. is_empty defaults false; both maps recurse into supplement while
preserving key/form. get_field defaults None. This entity is locatable:
element_kind returns Citation and to_payload returns Citation{key:key.clone()}.
Keep payload-based from_tags/walk consumption; equality/hash include all fields.

CSL formatting belongs to compiler/layout/cite.rs (static free-function form B),
not the data entity. Introspector bib_entry_for_key/bib_number_for_key remain
lookups; enriching data requires their own contracts. Prelayout from_tags
population supports forward references, a crystalline mechanism. The rendered
text is language-observable; shared hayagriva implementation is mechanics.
Advanced forms beyond the originally bounded four and native CSL supplement
formatting (rather than simple `, supp`) remain separately owned pending scope.

## Evidence and public key gap

Historical upstream e0e8ca4d, model/cite.rs:39-42, supports reference syntax
whose label is the bibliography key. Lines 44-46 declare key as label, and 31-37
allow special characters through label("…"). Forms are documented at 133-147.

The 2026-08-13 probe used that vanilla and crystalline HEAD 4f64e4e69 with only
L0 edits. `A #cite("netwok") B` gave upstream
`error: expected label, found string`, versus local `A [netwok] B`.
`A #cite(<x>, form: none) B` accepted the upstream label then reported
`error: the document does not contain a bibliography`; local reported
`error: cite() espera key como string, recebeu label`.
This is a public argument-morphology gap, not permitted mechanical divergence.
Changing public CiteElem.key/default behavior requires a separately approved
ADR-0127 contract. The archived gap remains pending here; these historical
observations do not certify the current pin.

## Acceptance

Preserve supplement traversal, locatable Citation payload, field-sensitive
equality/hash, plain text and current constructor transport. Do not claim
key-label or advanced CSL/form closure from an entity extraction or cleanup.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/cite.md`. Historical observations are not a new parity certification.

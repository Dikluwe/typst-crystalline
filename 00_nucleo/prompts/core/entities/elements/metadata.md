# Prompt L0 — `entities/elements/metadata` — `MetadataElem`
Hash do Código: 6adefe9f

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/metadata.rs`

## Evidence before specification

Historical upstream `e0e8ca4d`, `metadata.rs:3-28`, requires a Value and marks
metadata Locatable. It exposes arbitrary query/CLI data without visual output,
can be labelled, and supports retrieving the value through a query. This is
source evidence for the required field and query observable. Empty plain text
is a contextual inference, not a claim of identical upstream implementation.

## Contract

`Metadata` carries `value: Box<Value>` and derives Debug/Clone/PartialEq;
Value supplies its manual Debug/Hash handling, including f64. Box is pure
indirection. `Content::Metadata` holds Arc and `Content::metadata(Value)`
constructs it. Preserve these signatures and the required payload.

`plain_text` is empty; `is_empty` remains false. Content/text maps treat this
entity as terminal. The entity's default `get_field` remains None;
`element_kind` is Some(Metadata) and its payload is
Some(Metadata { value: value.clone() }). The Content hub exposes the `value`
field, supporting `query(...).first().value`.

## Acceptance and boundary

Keep the entity's derived PartialEq and the Content hub's existing always-false
Metadata equality distinct. The latter is Rust mechanics, not language parity;
this contract does not add a hub equality arm. Preserve value transport,
queryability, terminal traversal and no visible text. No own rendering or I/O
belongs here.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/metadata.md`. Historical observations are not a new parity certification.

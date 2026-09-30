# L0 — introspect/extract_payload
Hash do Código: d819ab1a

```text
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/extract_payload.rs`.
The function at `:18` delegates to element-owned to_payload implementations;
it ends in `_ => None`. The parent walk at
`01_core/src/compiler/introspect.rs:1915` consumes its result before recursion.
The current function is not compiler-exhaustive merely because its dispatch is
static. The exhaustive classification owner is
`00_nucleo/prompts/compiler/introspect/locatable.md`.

## Contract

Preserve `pub fn extract_payload(content: &Content) -> Option<ElementPayload>`.
It is pure, deterministic for the same modeled input, independent of counters,
walk context, Locator and Engine. Cloning domain data is permitted; do not
perform I/O, walk descendants or execute callbacks.

Delegate directly to the native element for Strong, Emph, Heading, Figure,
Cite, Metadata, State, StateUpdate, StateDisplay, CounterDisplayCallback,
Outline, Bibliography, Equation, Table, Footnote, CounterUpdate and ContextBlock.
Dynamic uses its existing dyn_to_payload boundary, not a new dispatch mechanism.
All remaining variants return None in this dispatcher.

Payload formulas and domain fields belong to their entity L0s, not to a
duplicated table in this module. Effective lexical numbering/supplement/alt
capture belongs to `00_nucleo/prompts/compiler/introspect.md`, after extraction.
Do not invent payloads from bold/italic flags or descend through Styled:
the child's normal walk discovers its own occurrence.

## Acceptance and alignment boundary

Check native delegate equivalence, own Strong/Emph nodes including empty and
nested bodies, and None for Text, Empty, both Space variants, Sequence and
Styled. A new locatable producer requires an explicit coordinated review of
classification, extraction and causal traversal.

The intended classification/extraction alignment is not proven universal:
OutlineEntry is classified true but falls through this dispatcher, while its
queriable entries are synthesized after the walk. This observed discrepancy is
recorded in `00_nucleo/diagnosticos/sanitization-introspection-columns-history.md`.
Do not silently add an arm, fabricate payloads or remove its classification
under this documentation-only contract.

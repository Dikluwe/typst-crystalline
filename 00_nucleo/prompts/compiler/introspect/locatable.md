# L0 — introspect/locatable
Hash do Código: c37d093a

```text
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/locatable.rs`.
The exhaustive Content match in is_locatable at `:20` classifies the current
node, not its descendants. `01_core/src/compiler/introspect/extract_payload.rs:18`
is a separate delegated extractor with an open None fallback.
Layout and introspection share classification for causal allocation; allocation
and traversal themselves are owned by their respective productive modules.

## Contract

Preserve `pub fn is_locatable(content: &Content) -> bool` as a pure,
deterministic, stateless classification over the closed enum, without a wildcard.
A new enum variant requires an explicit classification decision; adding a
locatable producer also requires extractor/traversal review.

The native true branches are Strong, Emph, Heading, Figure, Cite, Metadata,
State, StateUpdate, StateDisplay, CounterDisplayCallback, ContextBlock,
Outline, OutlineEntry, Bibliography, Equation, CounterUpdate, Table and Footnote.
Dynamic is true only when dyn_element_kind() is present.
The remaining variants are false, including Styled, Label, Sequence, Columns,
text/spacing/flow sentinels, mathematical carriers, FigureCaption and
FootnoteEntry. A non-locatable wrapper can still have traversed locatable children;
false is not a command to discard its descendants.

Own Strong/Emph occurrences remain true even for empty or repeated bodies.
Styled surrounding them stays false. Footnote itself is true independently
of column layout. SpaceFromNewline remains false without origin normalization.
No boolean classifies an arbitrary wrapper as Heading or invents identity from
old show flags.

## Alignment and acceptance

The intended ordinary-producer invariant is
is_locatable(c) == extract_payload(c).is_some(), supporting synchronized
Location allocation rather than a second independent occurrence count.
Do not claim universal verified coverage: OutlineEntry is currently true here
but returns None from the extractor and has a post-walk synthesized query path.
Its direct-input/traversal alignment is unresolved; see
`00_nucleo/diagnosticos/sanitization-introspection-columns-history.md`.
Dynamic alignment also depends on its existing kind/payload boundary.

Cover native true branches and representative false wrappers/leaves, own
Strong/Emph, both Space variants, and delegation alignment for the ordinary
producer domain. Preserve the explicit OutlineEntry discrepancy as an audited
pending issue; fixing either consumer is outside documentation sanitization.

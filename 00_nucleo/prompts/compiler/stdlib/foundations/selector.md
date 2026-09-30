# L0 — stdlib/foundations/selector
Hash do Código: 57456261

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/selector.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/selector.rs:23`, `01_core/src/compiler/stdlib/foundations/selector.rs:269`, `01_core/src/compiler/stdlib/foundations/selector.rs:286` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own native_selector, element_kind_of_native_func and parse_selector_arg used by query/locate. Use static identity mapping, not callbacks, dynamic registry, vtables or function-name matching.

Preserve Location, Selector and Label semantic identity. Preserve the entire Element selector including function identity, empty filter group, group order and Values: clone it rather than reconstructing Kind or retaining only the first filter. Construction does not require locatability; text.where remains a valid show selector even when query/counter reject it. Known kind strings, label strings "<sec>", and the established bare heading/figure/table/metadata element functions retain their conversions. Non-element functions fail `only element functions can be used as selectors`. Recognize native_outline_entry by address as OutlineEntry, not outline.entry.body.

Accept an existing Selector unchanged and a Regex after empty checks: empty string `text selector is empty`; empty regex `regex selector is empty`; non-empty regex matching empty text `regex matches empty text`. Valid regex returns Selector::Regex. Preserve arity, types and exact diagnostics. The registry owns the established Type::Selector binding; construction/parsing must not change it or promote additional bare functions implicitly.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

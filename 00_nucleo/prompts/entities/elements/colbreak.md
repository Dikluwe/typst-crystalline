# L0 — ColbreakElem
Hash do Código: cefb38a2

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/colbreak.rs`

## Evidence

Retained historical anchors `content.rs:1790,1617` distinguish empty text from
a nonempty flow command. They are not current upstream parity measurements.

## Contract

`ColbreakElem` has public bool fields weak and weak_explicit with
Debug/Clone/PartialEq/Hash. Both fields participate in equality/hash and remain
preserved by terminal map_content/map_text. plain_text is empty; is_empty is
always false. get_field, element_kind and to_payload retain default None and
the element remains non-locatable.
Presence transports whether the named argument was supplied. Existing
`Content::colbreak(weak)` keeps its signature: true is explicit, false omitted.
Evaluation uses the existing presence-aware path, while layout reads only weak.
Those operations belong to their constructor/eval/layout owners; this entity
preserves the carrier and its distinctions, not argument validation or layout.
The trait owner is `00_nucleo/prompts/entities/elements/_comum.md`.
Verify empty text, nonempty command identity, terminal maps and both equality components.

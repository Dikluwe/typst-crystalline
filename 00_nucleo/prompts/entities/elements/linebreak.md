# L0 — LinebreakElem
Hash do Código: 2a86c30d

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/linebreak.rs`

## Evidence before specification

At historical upstream `a51e02804`, `repr(linebreak())` is `linebreak()`;
explicit false is `linebreak(justify: false)`. Fields/has also distinguish
omission from explicit false. Presence is therefore necessary for language
observables, not merely a Rust representation choice. Historical text evidence
is `content.rs:1683`; no new current-pin probe is claimed.

## Contract

`LinebreakElem` has public bool fields justify and justify_explicit, deriving
Debug/Clone/PartialEq/Hash. Both fields participate in equality and hashing.
Text projection is one newline. `is_empty` is the default false; both maps are
terminal and preserve both fields. `get_field("justify")` returns Value::Bool
only when justify_explicit; reflection distinguishes unset from undeclared.
Other fields and element_kind/to_payload retain default None.

The unchanged no-argument Content constructor produces false/false. The existing
presence-aware construction path transports explicit values; constructors belong
to `00_nucleo/prompts/entities/content/construction.md` and evaluation/reflection
to their eval owners. Layout's true/false behavior and bidi boundary belong to
`00_nucleo/prompts/compiler/layout/linebreak.md`, not this entity. The previous
staged layout obligations remain recorded in the diagnostic, not waived.
Verify terminal maps, newline, nonempty command identity, both equality/hash
components and preservation of omitted versus explicit false.

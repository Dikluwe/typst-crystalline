# L0 — stdlib/structural/heading
Hash do Código: 5fe69f10

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/heading.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/heading.rs:34` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own heading construction and presence transport, not numbering or rendering. Preserve four forms: positional level+body, body-only (level 1), named level+positional body, and named level/body. Initial Int is level with required second body; initial Content|Str is body; other initial types fail `heading(): primeiro argumento deve ser int (level) ou content/string (body), recebeu {type}`. Missing body fails `heading() exige body`. Named level overrides the body-only default, requires Int and range 1..6; preserve `heading(level:): espera int, recebeu {type}` and `heading(): level deve estar entre 1 e 6, recebeu {n}`.

Named numbering:Str|none defaults None; outlined:Bool|none defaults true; bookmarked:Bool|none defaults None (follows outlined). Keep outlined versus bookmarked separate (document outline versus PDF bookmarks). set_fields marks HEADING_SET_LEVEL for explicit positional Int or named level, and HEADING_SET_OUTLINED whenever outlined is named, including true. bookmarked Option retains its presence distinction. Emit heading_numbered_native when numbering exists, otherwise heading_native, with all fields/bits intact. Preserve type diagnostics and show-selector integration.

has/at/fields distinguish explicit defaults from omission; these bits are an introspection contract, not expendable mechanics. Context/World/file parameters remain pass-through; numbering here would violate phase ownership. No new helper or render logic.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

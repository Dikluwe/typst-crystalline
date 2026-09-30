# L0 — stdlib/emoji
Hash do Código: 047dd8d3

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/emoji.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/emoji.rs:14`, `01_core/src/compiler/stdlib/emoji.rs:26`, `01_core/src/compiler/stdlib/emoji.rs:48` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own pure build_emoji_module returning Value::Module named emoji with the full codex::EMOJI catalog from the existing git pin `2e43ab90079cf9f9fa670d421c8c38233143b376`. Preserve the data snapshot SHA-256 `8691ca68e09b6fedca61e00e824648e79f7502772eefe7af367e404e26161489`; do not infer unchanged sym data from unchanged emoji.txt.

Simple entries use Symbol::new and variant groups Symbol::with_variants. Preserve complete Unicode sequences, VS15/VS16/ZWJ, modifier names/order and repr without truncation/normalization. Parents without bare values use the established best match: greatest modifier match, fewest extra modifiers, first variant on ties, not merely first source row. Preserve face 😀, heart ❤️, heart.arrow 💘, heart.excl ❣️, apple 🍏 and arrow ↙️; invalid modifiers remain `unknown symbol modifier` at the modifier span. All 772 symbols and 1,386 value/variant records of the preserved data snapshot remain required, not a current parity percentage. Registry and Symbol rendering/pretty-printing belong to their owners; this adapter does not change because another catalog in the dependency changes.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

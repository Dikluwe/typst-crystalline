# L0 — Shaping shared glyph regressions
Hash do Código: 99599d88

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/tests/shared_glyphs.rs`

## C06: immutable glyph storage

Evidence at baseline `0a15ccebf861e3cc0ea16d0397d55dc057664556`: `shaper/shaping.rs:441,545` clones cached/new glyph payloads; `layout_types.rs:411` owns the occurrence glyph Vec. Regressions must distinguish immutable payload sharing from independent occurrence metadata without asserting new vanilla parity.
The known outline semantic/render divergence against pinned vanilla remains open; these local controls do not certify its parity.


Use versioned real-font fixtures and public metrics/document shaping. Compare
fresh and shared cache paths in both orders, preserving full glyph signatures,
source face, language, axes exact bits, features, tracking, size/style, fallback,
None versus successful empty and native-script refusal. Repeated occurrences must
share payload pointers while retaining independent metadata; clones survive cache
and document destruction. Different keys must not be globally interned. Assert
sharing unconditionally in the integrated candidate. Retain all existing assertions.

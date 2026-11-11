# L0 — Shared glyph payload regressions
Hash do Código: 48e2dcbe

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/shared_glyphs/tests.rs`

## C06: immutable glyph storage

Evidence at baseline `0a15ccebf861e3cc0ea16d0397d55dc057664556`: `shaper/shaping.rs:441,545` clones cached/new glyph payloads; `layout_types.rs:411` owns the occurrence glyph Vec. Regressions must distinguish immutable payload sharing from independent occurrence metadata without asserting new vanilla parity.


Verify every field, order, list Debug, immutable iteration, content equality of
separate allocations, shared Clone pointers, empty storage, final-owner release
and retained ownership across threads. Private tests may inspect backing ownership.

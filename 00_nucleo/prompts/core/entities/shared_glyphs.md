# L0 — Immutable shaped glyph payload
Hash do Código: 86b21cdd

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/shared_glyphs.rs`

## C06: immutable glyph storage

Evidence at baseline `0a15ccebf861e3cc0ea16d0397d55dc057664556`: `layout_types.rs:411` owns a mutable Vec while `shaper/shaping.rs:545` clones its payload into the cache. The preceding C06 experiment supports freezing the completed payload; no uniform CPU gain or vanilla parity is inferred.

The user authorizes this public immutable wrapper. Restrict construction to finished Vec payloads, with private Arc<[ShapedGlyph]> backing. No DerefMut, AsMut, COW, public Arc extraction, unsafe, globals or I/O. Clone shares storage; equality compares contents and Debug formats the glyph list. Retain every field and order, including empty payloads. The final owner releases storage; retained clones are thread safe.


Provide public SharedGlyphs with private Arc<[ShapedGlyph]> storage, Clone,
PartialEq, Default, list Debug, From<Vec<ShapedGlyph>>, Deref<Target=[ShapedGlyph]>,
AsRef<[ShapedGlyph]> and IntoIterator for &SharedGlyphs. Own no mutable interface.
Declare a private cfg(test) child tests; its separate owner governs regression tests.

# L0 — Shaped glyph carrier
Hash do Código: b9e0e19f

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/shaped_glyph.rs`

## Evidence and contract

The exclusive consumer is the standalone shaped-glyph entity, not layout_types.
Preserve its public Debug/Clone/PartialEq data carrier:

```rust
pub struct ShapedGlyph {
    pub glyph_id: u16,
    pub x_advance: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub cluster: u32,
    pub char_code: char,
}
```

Advance/offset values are font units, not points: points = units/upm × font size.
Cluster is the UTF-8 byte index in the original text and determines char_code.
Glyph IDs support the existing FrameItem::Glyph and PDF CID path. The L0's
u16 representation differs from the u32 form in ADR-0120; this documentary
cleanup does not approve truncation safety or resolve that discrepancy.

`units_per_em` belongs to the whole TextShaped run, not individual glyphs.
The existing TJ projection is `-(x_advance / units_per_em * 1000)`; emission,
font tracking and shaping remain under their own owners. Do not add a run
field here or move shaping to L1.

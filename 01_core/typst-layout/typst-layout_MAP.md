# 🗺️ Module: typst-layout

| File | Purpose |
|---|---|
| `Cargo.toml` | — |
| `src/flow/block.rs` | — |
| `src/flow/collect.rs` | — |
| `src/flow/compose.rs` | — |
| `src/flow/distribute.rs` | — |
| `src/flow/mod.rs` | Layout of content into a [`Frame`] or [`Fragment`]. |
| `src/grid/layouter.rs` | — |
| `src/grid/lines.rs` | Indicates which priority a particular grid line segment should have, based |
| `src/grid/mod.rs` | — |
| `src/grid/repeated.rs` | — |
| `src/grid/rowspans.rs` | All information needed to layout a single rowspan. |
| `src/image.rs` | Layout the image. |
| `src/inline/box.rs` | — |
| `src/inline/collect.rs` | — |
| `src/inline/deco.rs` | — |
| `src/inline/finalize.rs` | Turns the selected lines into frames. |
| `src/inline/line.rs` | — |
| `src/inline/linebreak.rs` | — |
| `src/inline/mod.rs` | — |
| `src/inline/prepare.rs` | A representation in which children are already layouted and text is already |
| `src/inline/shaping.rs` | — |
| `src/lib.rs` | Typst's layout engine. |
| `src/lists.rs` | — |
| `src/math/accent.rs` | — |
| `src/math/attach.rs` | Can be re-enabled once `Option::map_or_default` is stable in our MSRV. |
| `src/math/cancel.rs` | — |
| `src/math/frac.rs` | — |
| `src/math/fragment.rs` | — |
| `src/math/lr.rs` | Lays out an [`LrElem`]. |
| `src/math/mat.rs` | — |
| `src/math/mod.rs` | — |
| `src/math/root.rs` | Lays out a [`RootElem`]. |
| `src/math/run.rs` | — |
| `src/math/shaping.rs` | — |
| `src/math/shared.rs` | How much less high scaled delimiters can be than what they wrap. |
| `src/math/stretch.rs` | Lays out a [`StretchElem`]. |
| `src/math/text.rs` | — |
| `src/math/underover.rs` | — |
| `src/modifiers.rs` | Frame-level modifications resulting from styles that do not impose any |
| `src/pad.rs` | Layout the padded content. |
| `src/pages/collect.rs` | An item in page layout. |
| `src/pages/finalize.rs` | Piece together the inner page frame and the marginals. We can only do this |
| `src/pages/mod.rs` | Layout of content into a [`Document`]. |
| `src/pages/run.rs` | — |
| `src/repeat.rs` | Layout the repeated content. |
| `src/rules.rs` | — |
| `src/shapes.rs` | — |
| `src/stack.rs` | — |
| `src/transforms.rs` | — |

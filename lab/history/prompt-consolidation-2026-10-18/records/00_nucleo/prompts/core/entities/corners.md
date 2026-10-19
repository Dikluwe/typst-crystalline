# L0 — Generic corner values
Hash do Código: 115f54e7

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/corners.rs`

## Contract

`Corners<T>` stores public `top_left`, `top_right`, `bottom_right` and
`bottom_left` values in clockwise order from top-left. Derive Debug, Clone,
Copy, PartialEq and Eq when supported by T.
`new(top_left, top_right, bottom_right, bottom_left)` preserves each value;
`uniform(value)` requires Clone and repeats it in all corners; Default requires
T: Default and defaults each corner independently.

For `new(1,2,3,4)` the clockwise values are exactly 1,2,3,4; uniform(5) has
four 5 values; default f64 corners are zero. The upstream reference is
`lab/typst-original/crates/typst-library/src/layout/corners.rs`.
This pure carrier does not resolve radii or generate exporter paths; those
belong to geometry/layout/export owners. Sides and corners remain distinct.

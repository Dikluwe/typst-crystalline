# Contract: Layout

> The interface for elements that can be laid out into frames.

## Purpose

Elements that produce visual output implement layout capabilities. The layout contract defines how elements respond to available space and produce positioned output.

## Interface

```rust
pub trait Layout {
    /// Layout into the given regions.
    fn layout(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>;
}
```

## Regions

```rust
pub struct Regions {
    /// Available size for current region.
    pub size: Size,
    /// Whether the element can expand.
    pub expand: Axes<bool>,
    /// Remaining regions after current.
    pub backlog: &[Size],
}
```

## Fragment

```rust
pub struct Fragment(Vec<Frame>);
```

A fragment contains one `Frame` per region occupied.

## Invariants

| Invariant | Description |
|-----------|-------------|
| **I1** | Layout is pure given same inputs |
| **I2** | Returned frames fit within region sizes |
| **I3** | Number of frames ≤ 1 + backlog.len() |

## Layout Contract

### Input
- `regions`: Available space (current + backlog)
- `styles`: Inherited styles
- `engine`: Mutable context

### Output
- `Fragment`: One frame per region used
- May use fewer regions than available
- May fail with `SourceResult::Err`

## Example

```rust
impl Layout for Packed<TextElem> {
    fn layout(&self, engine, styles, regions) -> SourceResult<Fragment> {
        // Shape text
        // Break into lines
        // Return frames
    }
}
```

## Related

- [ADR-010: Region-Based Layout](../adr/ADR-010-region-based-layout.md)
- [StyleChain Contract](stylechain.md)

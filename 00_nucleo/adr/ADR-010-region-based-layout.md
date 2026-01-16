# ADR-010: Region-Based Layout Model

## Status
**Accepted**

## Context

Page layout requires:
- Breaking content across pages
- Respecting margins and columns
- Handling elements that span regions

A model was needed to describe available space for layout.

## Decision

Use **Regions** to describe available layout space. Elements layout into regions and return Frames for each region they occupy.

### Region Structure

```rust
struct Region {
    size: Size,          // Available width/height
    expand: Axes<bool>,  // Can element grow?
}
```

### Layout Contract

```rust
fn layout(&self, regions: Regions) -> Vec<Frame>
```

Each element receives regions and returns one Frame per region occupied.

## Consequences

### Positive
- **Flexible**: Handles pages, columns, containers
- **Composable**: Nested layouts pass sub-regions
- **Natural breaks**: Multi-frame return handles page breaks

### Negative
- **Complexity**: Unfamiliar model for developers
- **Prediction**: Hard to know how many frames result

## References
- [typst-layout/](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/01_core/typst-layout/)

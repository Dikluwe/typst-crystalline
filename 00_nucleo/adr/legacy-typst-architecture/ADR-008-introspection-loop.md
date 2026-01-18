# ADR-008: Introspection Loop

## Status
**Accepted**

## Context

Document layout can depend on the layout itself:
- Table of contents needs page numbers
- Counters need element positions
- Cross-references need target locations

This creates circular dependencies: layout determines positions, but positions are needed for layout.

## Decision

Implement an **introspection loop** that runs layout iteratively until results stabilize, with a maximum of **5 iterations**.

### Mechanism

```
Loop (max 5 iterations):
  1. Layout document with current introspection state
  2. Collect new introspection data (positions, counters)
  3. Compare with previous iteration
  4. If stable → done
  5. If changed → continue with new data
```

### Example

```typst
// First pass: TOC has placeholder page numbers
// Second pass: Real page numbers from first layout
// Third pass: Verify stability (text didn't reflow)
```

## Consequences

### Positive
- **Resolves cycles**: Converging iteration handles dependencies
- **Correct results**: Most documents stabilize in 1-2 iterations
- **Bounded**: Maximum 5 prevents infinite loops

### Negative
- **Non-determinism risk**: Some layouts never stabilize
- **Performance**: Up to 5x layout cost for complex documents
- **Complexity**: Users may not understand iteration behavior

## References
- [legacy-docs/dev/architecture.md](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/00_nucleo/legacy-docs/dev/architecture.md)

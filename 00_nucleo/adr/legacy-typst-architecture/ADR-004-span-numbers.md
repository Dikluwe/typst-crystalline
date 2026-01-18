# ADR-004: Span Numbers for Source Tracking

## Status
**Accepted**

## Context

Error messages need to point back to source code locations. During compilation, source is transformed through multiple phases (CST → Content → Frame), but errors can occur at any stage.

A mechanism was needed to trace errors back to their source location across all phases.

## Decision

Assign **span numbers** to syntax nodes after parsing, and propagate them through all compilation phases.

### Properties
- Unique identifier per syntax node
- Ordered: node lookup is O(log n)
- Stable: incremental reparsing preserves distant spans
- Small: fits in a few bytes

### Propagation

```
SyntaxNode.span() → Content.span() → Frame elements
        ↓                ↓                ↓
   All carry span numbers for error tracing
```

## Consequences

### Positive
- **Precise errors**: Point to exact source location
- **Incremental-friendly**: Stable spans improve cache hits
- **Lightweight**: Minimal overhead per node

### Negative
- **Pervasive**: Spans must be carried through all data structures
- **Renumbering**: Local changes cause local renumbering

## References
- [specs/typst-syntax/span.md](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/00_nucleo/specs/01_core/typst-syntax/span.md)

# ADR-006: Capture by Value in Closures

## Status
**Accepted**

## Context

Typst closures need to access variables from their enclosing scope. Two strategies exist:

1. **Capture by reference**: Variables point to original locations
2. **Capture by value**: Variables are cloned at closure creation

Typst needed a capture strategy compatible with:
- Immutability (Typst values are immutable)
- Memoization (closures must be deterministic)
- Simplicity (predictable behavior)

## Decision

Implement **capture by value**: when a closure is created, all referenced external variables are cloned into the closure.

### Mechanism

```rust
// At closure definition:
1. Walk closure body
2. Find free variables (not defined locally)
3. Clone their current values
4. Store with closure definition
```

### Example

```typst
#let x = 1
#let f() = x  // x=1 is captured
#let x = 2
#f()          // Returns 1, not 2
```

## Consequences

### Positive
- **Predictable**: Closure behavior is determined at definition
- **Memoization-friendly**: Same captures = same closure
- **Simple mental model**: No hidden references

### Negative
- **Clone overhead**: Large values are copied
- **No mutation**: Can't observe changes to captured variables

## References
- [legacy-docs/dev/architecture.md](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/00_nucleo/legacy-docs/dev/architecture.md)

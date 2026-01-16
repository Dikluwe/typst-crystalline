# ADR-002: Incremental Compilation with comemo

## Status
**Accepted**

## Context

Typst documents are frequently edited iteratively. Recompiling an entire document on each small edit is inefficient, especially for large documents with many imports, images, and calculations.

A strategy was needed to accelerate recompilations without sacrificing correctness.

## Decision

Adopt the **comemo** framework for incremental compilation based on memoization of pure functions.

### Mechanism

```rust
#[comemo::memoize]
fn layout(content: &Content, regions: Regions) -> Vec<Frame> {
    // ...
}
```

comemo:
1. **Tracks** arguments of marked functions
2. **Stores** results in cache
3. **Invalidates** selectively when inputs change
4. **Reuses** results when inputs are identical

### Cache Granularity

| Level | Unit | Example |
|-------|------|---------|
| File | Module | Import re-evaluation |
| Function | Closure | Memoized function calls |
| Element | Layout | Individual block layout |

## Consequences

### Positive
- **Fast recompilations**: Only modified parts are recalculated
- **Transparency**: Code doesn't need to manage cache explicitly
- **Enforced purity**: Functions must be pure for memoization to work

### Negative
- **Memory overhead**: Cache uses RAM
- **Invalidation complexity**: Span numbers must be stable for cache hits
- **Debugging**: Behavior can be non-obvious when cache is used

## References
- [comemo](https://github.com/typst/comemo)

# Contract: Span

> Source location tracking across compilation phases.

## Purpose

`Span` identifies source code locations for error reporting and introspection. It survives all compilation phases (parsing → evaluation → layout).

## Interface

```rust
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span(NonZeroU64);

impl Span {
    /// The detached span (no source location).
    pub const DETACHED: Span;
    
    /// Resolve a path relative to this span's file.
    pub fn resolve_path(&self, path: &str) -> StrResult<FileId>;
    
    /// Get the source file ID.
    pub fn id(&self) -> Option<FileId>;
}
```

## Invariants

| Invariant | Description |
|-----------|-------------|
| **I1** | Spans are ordered within a file |
| **I2** | `DETACHED` has no source location |
| **I3** | Spans are stable across incremental reparsing |

## Properties

### Uniqueness
- Each syntax node has a unique span
- Span encodes both file ID and position

### Stability
- Incremental parsing preserves distant spans
- Enables cache hits in memoization

## Usage

```rust
// Error with location
bail!(span, "undefined variable: {}", name);

// Attach span to content
content.spanned(span)
```

## Related

- [ADR-004: Span Numbers](../adr/ADR-004-span-numbers.md)

# ADR-012: Custom VTable Implementation

## Status
**Accepted**

## Context

Typst's `Content` type needs to:
- Store any element type (type erasure)
- Access element-specific methods (dispatch)
- Support multiple traits (Debug, Clone, Hash, LocalName, etc.)

Rust's built-in trait objects (`dyn Trait`) support only one trait at a time.

## Decision

Implement a **custom vtable** that combines multiple capabilities in a single dispatch table.

### Structure

```rust
#[repr(C)]
struct ContentVtable {
    name: &'static str,
    fields: &'static [FieldVtable],
    debug: unsafe fn,
    clone: unsafe fn,
    hash: unsafe fn,
    capability: fn(TypeId) -> Option<NonNull<()>>,
    // ... more
}
```

### Capability Query

```rust
// Check if element implements a trait
if let Some(ptr) = vtable.capability(TypeId::of::<dyn LocalName>()) {
    // Use trait
}
```

## Consequences

### Positive
- **Multiple traits**: Single vtable, many capabilities
- **Field introspection**: Element fields accessible dynamically
- **Documentation**: Metadata stored in vtable

### Negative
- **Complexity**: Manual unsafe implementation
- **Maintenance**: Must update vtable for new traits

## References
- [specs/content/vtable.md](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/00_nucleo/specs/01_core/typst-library/foundations/content/vtable.md)

# Contract: Content

> The central type-erased container for typographic content.

## Purpose

`Content` is Typst's fundamental type representing any piece of document content. It provides a unified interface for all elements while hiding their concrete types.

## Interface

```rust
pub struct Content(pub(super) RawContent);

impl Content {
    /// Check if this is a specific element type.
    pub fn is<T: NativeElement>(&self) -> bool;
    
    /// Try to cast to a specific packed element.
    pub fn to_packed<T: NativeElement>(&self) -> Option<&Packed<T>>;
    
    /// Get the element descriptor.
    pub fn elem(&self) -> Element;
    
    /// Get the source span.
    pub fn span(&self) -> Span;
    
    /// Get the label, if any.
    pub fn label(&self) -> Option<Label>;
    
    /// Concatenate contents.
    pub fn add(self, other: Content) -> Content;
}
```

## Invariants

| Invariant | Description |
|-----------|-------------|
| **I1** | `is::<T>()` ⟺ `to_packed::<T>().is_some()` |
| **I2** | `span()` always returns a valid span |
| **I3** | `Content + Content` produces a sequence |
| **I4** | Reference counted: clone is cheap |

## Operations

### Composition
```rust
let combined = heading + paragraph + image;
// Produces SequenceElem([heading, paragraph, image])
```

### Type Check
```rust
if content.is::<HeadingElem>() {
    // Handle heading
}
```

### Downcasting
```rust
if let Some(heading) = content.to_packed::<HeadingElem>() {
    let level = heading.level();
}
```

## Memory Model

- Clone-on-write semantics
- Atomic reference counting
- `Packed<T>` provides zero-cost typed access

## Related

- [ADR-011: Content as Central Type](../adr/ADR-011-content-type.md)
- [Packed Contract](packed.md)

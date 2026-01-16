# Contract: Element

> The fundamental trait for all Typst elements.

## Purpose

`Element` is the runtime type descriptor for content elements. It provides metadata and vtable access for type-erased content operations.

## Interface

```rust
pub struct Element {
    vtable: &'static ContentVtable,
}

impl Element {
    /// Element's string name (e.g., "heading", "text").
    pub fn name(&self) -> &'static str;
    
    /// Element's title for documentation.
    pub fn title(&self) -> &'static str;
    
    /// Check if this is a specific element type.
    pub fn is<T: NativeElement>(&self) -> bool;
    
    /// Access field by ID.
    pub fn field(&self, id: u8) -> Option<&'static FieldVtable>;
}
```

## Invariants

| Invariant | Description |
|-----------|-------------|
| **I1** | `is::<T>()` returns true iff content was created as `T` |
| **I2** | `name()` matches the `#elem` macro name |
| **I3** | Field IDs are stable across compilations |

## Properties

### Identity
- Each element type has a unique `Element` value
- Comparison is pointer equality on vtable

### Introspection
- Fields are accessible by ID
- Documentation strings available at runtime

## Related

- [NativeElement Contract](native-element.md)
- [ADR-012: Custom VTable](../adr/ADR-012-custom-vtable.md)

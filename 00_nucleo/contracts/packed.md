# Contract: Packed\<T\>

> Type-safe wrapper for accessing Content of a known element type.

## Purpose

`Packed<T>` provides compile-time type safety when working with `Content` that is known to be of type `T`. It enables direct field access without runtime type checks.

## Interface

```rust
#[repr(transparent)]
pub struct Packed<T: NativeElement>(Content, PhantomData<T>);

impl<T: NativeElement> Packed<T> {
    /// Create from element.
    pub fn new(element: T) -> Self;
    
    /// Try to cast from content.
    pub fn from_ref(content: &Content) -> Option<&Self>;
    
    /// Get the source span.
    pub fn span(&self) -> Span;
    
    /// Unpack back to element.
    pub fn unpack(self) -> T;
}

impl<T: NativeElement> Deref for Packed<T> {
    type Target = T;
}
```

## Invariants

| Invariant | Description |
|-----------|-------------|
| **I1** | `repr(transparent)`: same layout as `Content` |
| **I2** | Inner content is always of type `T` |
| **I3** | `Deref` enables direct field access |

## Usage Pattern

```rust
// Safe construction
let packed = Packed::new(HeadingElem { body, level: 1 });

// Direct access via Deref
let level = packed.level;  // No runtime check

// Safe downcast
if let Some(packed) = content.to_packed::<HeadingElem>() {
    process_heading(packed);
}
```

## Guarantees

### Type Safety
```
Packed<T>::from_ref(content) → Some(_) 
  ⟺ content.is::<T>()
```

### Memory Safety
- `repr(transparent)` ensures transmute is safe
- Type invariant maintained by construction

## Related

- [ADR-013: Packed Wrapper](../adr/ADR-013-packed-wrapper.md)
- [Content Contract](content.md)

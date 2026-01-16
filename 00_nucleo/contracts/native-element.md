# Contract: NativeElement

> The trait implemented by all element structs.

## Purpose

`NativeElement` is the compile-time trait that element structs implement to participate in Typst's content system.

## Interface

```rust
pub trait NativeElement: Debug + Clone + Hash + Send + Sync + 'static {
    /// Pack this element into type-erased Content.
    fn pack(self) -> Content;
    
    /// Get the element descriptor.
    fn elem() -> Element;
}
```

## Derived Implementations

Elements defined via `#[elem]` macro get:

| Trait | Auto-derived |
|-------|--------------|
| `Debug` | ✅ |
| `Clone` | ✅ |
| `Hash` | ✅ |
| `Send + Sync` | ✅ (required) |

## Invariants

| Invariant | Description |
|-----------|-------------|
| **I1** | `T::elem().is::<T>()` always true |
| **I2** | `pack()` preserves all field values |
| **I3** | `elem()` returns same value for all instances of `T` |

## Usage Pattern

```rust
// Define element
#[elem]
pub struct HeadingElem {
    #[required]
    pub body: Content,
    pub level: u8,
}

// Use element
let heading = HeadingElem { body, level: 1 };
let content: Content = heading.pack();
```

## Related

- [Element Contract](element.md)
- [Packed Contract](packed.md)

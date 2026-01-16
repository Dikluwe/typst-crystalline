# ADR-013: Packed\<T\> for Type-Safe Content Access

## Status
**Accepted**

## Context

`Content` is type-erased for flexibility, but internal code often knows the exact element type and needs type-safe access without runtime checks on every operation.

## Decision

Create **`Packed<T>`** as a type-safe wrapper around `Content` that guarantees the underlying element is of type `T`.

### Structure

```rust
#[repr(transparent)]
pub struct Packed<T: NativeElement>(Content, PhantomData<T>);
```

### Guarantees

| Property | Guarantee |
|----------|-----------|
| `Deref` | Safe access to `&T` |
| `DerefMut` | Safe mutable access (with COW) |
| Construction | Only via type-checked cast |

### Usage

```rust
// Type check once
if let Some(heading) = content.to_packed::<HeadingElem>() {
    // Safe access without further checks
    let level = heading.level();
    let body = heading.body();
}
```

## Consequences

### Positive
- **Zero-cost abstraction**: Same as `Content` at runtime
- **Type safety**: Compiler enforces correct usage
- **Ergonomic**: Deref enables method access

### Negative
- **Unsafe internals**: Transmute requires careful handling

## References
- [specs/content/packed.md](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/00_nucleo/specs/01_core/typst-library/foundations/content/packed.md)

# Contract: Value

> The runtime representation of Typst values.

## Purpose

`Value` is the runtime type for all Typst values. It's an enum that can hold any value type supported by the language.

## Interface

```rust
pub enum Value {
    None,
    Auto,
    Bool(bool),
    Int(i64),
    Float(f64),
    Length(Length),
    Str(Str),
    Content(Content),
    Array(Array),
    Dict(Dict),
    Func(Func),
    // ... more variants
}

impl Value {
    /// Get the type name.
    pub fn ty(&self) -> Type;
    
    /// Try to cast to a specific type.
    pub fn cast<T: FromValue>(self) -> StrResult<T>;
}
```

## Invariants

| Invariant | Description |
|-----------|-------------|
| **I1** | All values are immutable |
| **I2** | Clone is shallow (reference counted where applicable) |
| **I3** | Hash and Eq are value-based |

## Type Casting

```rust
// Safe casting
let n: i64 = value.cast()?;

// Type check
if value.ty() == Type::of::<i64>() {
    // ...
}
```

## Related

- [Content Contract](content.md)
- Specs for individual types

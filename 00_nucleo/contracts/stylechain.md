# Contract: StyleChain

> Immutable chain of styles for property inheritance.

## Purpose

`StyleChain` provides efficient style inheritance without mutation. Styles are chained so that lookups traverse from child to parent until a value is found.

## Interface

```rust
pub struct StyleChain<'a> {
    head: Option<&'a Styles>,
    tail: Option<&'a StyleChain<'a>>,
}

impl<'a> StyleChain<'a> {
    /// Get a property value.
    pub fn get<P: Property>(&self, prop: P) -> P::Value;
    
    /// Get a reference to a property value.
    pub fn get_ref<P: Property>(&self, prop: P) -> &P::Value;
    
    /// Chain additional styles.
    pub fn chain(&self, styles: &'a Styles) -> StyleChain<'a>;
}
```

## Invariants

| Invariant | Description |
|-----------|-------------|
| **I1** | Lookup is O(n) where n = chain length |
| **I2** | Immutable: chaining creates new view |
| **I3** | Default returned if property not in chain |

## Property Lookup

```
chain.get(TextElem::size)
  → Search head
  → Search tail.head
  → Search tail.tail.head
  → ... 
  → Return default
```

## Usage Pattern

```rust
// Base styles
let base = StyleChain::default();

// Add styles
let styled = base.chain(&[
    TextElem::size.set(12.pt()),
    TextElem::fill.set(Color::BLACK),
]);

// Read value
let size = styled.get(TextElem::size);  // 12pt
```

## Performance

- No allocation on chain
- Shared backing between chains
- Default values are static

## Related

- [Styles type](../specs/01_core/typst-library/foundations/styles.md)

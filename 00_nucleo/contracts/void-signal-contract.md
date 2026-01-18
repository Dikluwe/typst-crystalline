# VoidSignal Contract

**Version**: 1.0  
**Location**: `01_core/shared/src/diagnostics.rs`

## Purpose

Define the interface for pure, I/O-free error signals.

## Contract

```rust
pub trait VoidSignal: std::fmt::Debug + Send + Sync {
    fn accept(&self, visitor: &mut dyn NarrativeVisitor);
}
```

## Laws

### Law I: Purity
- Signals MUST NOT contain formatted messages
- Signals MUST NOT perform I/O
- Signals MUST be `Send + Sync`

### Law II: Stability
- Signal struct fields are immutable
- Signal data represents the raw collapse event

### Law III: Double Dispatch
- All signals implement `accept()` for visitor pattern
- This enables type-safe narrative extraction

## Implementation Requirements

Every signal MUST:
1. Derive `Debug`, `Clone`
2. Implement `VoidSignal`
3. Have a corresponding hook in `NarrativeVisitor`
4. Be documented in ADR-003

## Example

```rust
#[derive(Debug, Clone)]
pub struct TypeMismatchSignal {
    pub expected: String,
    pub found: String,
}

impl VoidSignal for TypeMismatchSignal {
    fn accept(&self, visitor: &mut dyn NarrativeVisitor) {
        visitor.on_type_mismatch(self);
    }
}
```

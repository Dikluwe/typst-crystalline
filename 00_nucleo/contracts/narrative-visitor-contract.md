# NarrativeVisitor Contract

**Version**: 1.0  
**Location**: `01_core/shared/src/diagnostics.rs`

## Purpose

Define the visitor interface that Shell implementations use to transform signals into narratives.

## Contract

```rust
pub trait NarrativeVisitor {
    fn on_access_denied(&mut self, signal: &AccessSignal);
    fn on_type_mismatch(&mut self, signal: &TypeMismatchSignal);
    fn on_missing_field(&mut self, signal: &MissingFieldSignal);
    // Extend as signals are added
}
```

## Laws

### Law I: Completeness
- Every `VoidSignal` MUST have a corresponding visitor method
- Implementations MUST handle ALL methods

### Law II: Independence
- Visitor methods MUST NOT call other visitor methods
- Each method is self-contained

### Law III: Stateless Output
- Visitor may accumulate state (message, severity)
- Final output is produced externally

## Implementation Requirements

Shell implementations MUST:
1. Implement all visitor methods
2. Produce user-facing messages
3. Handle localization if needed

## Example Implementation

```rust
impl NarrativeVisitor for DiagnosticShield {
    fn on_type_mismatch(&mut self, signal: &TypeMismatchSignal) {
        self.message = format!(
            "expected {}, found {}",
            signal.expected, signal.found
        );
    }
}
```

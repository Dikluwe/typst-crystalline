//! Pure diagnostic signals for the Crystalline Architecture.
//!
//! This module defines the "Void Signals" - logical collapse indicators that are
//! completely free from I/O, formatting, and geography (Span) concerns.
//!
//! # Laws
//! - **Law II**: Signals are stable, invariant data objects (no messages)
//! - **Law III**: Double dispatch via `accept()` maintains purity

/// The NarrativeVisitor defines hooks that the Shell (L2) must implement
/// to give a narrative to technical signals.
pub trait NarrativeVisitor {
    fn on_access_denied(&mut self, signal: &AccessSignal);
    fn on_type_mismatch(&mut self, signal: &TypeMismatchSignal);
    fn on_missing_field(&mut self, signal: &MissingFieldSignal);
}

/// Law II: A signal is a stable, invariant data object.
/// It contains no messages, only the raw data of the logical collapse.
pub trait VoidSignal: std::fmt::Debug + Send + Sync {
    /// Law III: Double Dispatch to avoid downcasting and maintain purity.
    fn accept(&self, visitor: &mut dyn NarrativeVisitor);
}

// --- Signal Implementations (Examples of Geometric Collapses) ---

/// Signal for access control violations.
#[derive(Debug, Clone)]
pub struct AccessSignal {
    pub target: String,
    pub property: String,
}

impl VoidSignal for AccessSignal {
    fn accept(&self, visitor: &mut dyn NarrativeVisitor) {
        visitor.on_access_denied(self);
    }
}

/// Signal for type mismatches.
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

/// Signal for missing required fields.
#[derive(Debug, Clone)]
pub struct MissingFieldSignal {
    pub parent_type: String,
    pub field_name: String,
}

impl VoidSignal for MissingFieldSignal {
    fn accept(&self, visitor: &mut dyn NarrativeVisitor) {
        visitor.on_missing_field(self);
    }
}

/// Utility type for the Core to return failures without string interruptions.
pub type CrystalResult<T> = Result<T, Box<dyn VoidSignal>>;

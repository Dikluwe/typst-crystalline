//! Diagnostic Shield - Adapter that transforms pure signals into user narratives.
//!
//! The Shield is the Primary Adapter (L2) that transforms technical collapse
//! into an intelligible social experience.

use lexicon::Span;
use primitives::diag::SourceDiagnostic;
use shared::diagnostics::{
    AccessSignal, MissingFieldSignal, NarrativeVisitor, TypeMismatchSignal, VoidSignal,
};

/// The Shield transforms Core signals into user-facing diagnostics.
pub struct DiagnosticShield {
    /// Temporary state during visitation.
    message: String,
}

impl DiagnosticShield {
    /// Entry point for Space Projection.
    /// Transforms a Signal (L1) + Span (L0) into a Diagnostic (L2).
    pub fn project(signal: &dyn VoidSignal, span: Span) -> SourceDiagnostic {
        let mut shield = Self {
            message: String::new(),
        };

        // Law III: The signal "inhabits" the visitor to extract the narrative.
        signal.accept(&mut shield);

        SourceDiagnostic::error(span, shield.message)
    }
}

/// Implementation of the Visitation Facet.
/// Here resides the "Voice" that the Core (L1) does not possess.
impl NarrativeVisitor for DiagnosticShield {
    fn on_access_denied(&mut self, signal: &AccessSignal) {
        self.message = format!(
            "access denied: the property '{}' on '{}' is strictly guarded by the crystal logic",
            signal.property, signal.target
        );
    }

    fn on_type_mismatch(&mut self, signal: &TypeMismatchSignal) {
        self.message = format!(
            "geometric mismatch: expected a facet of type {}, but encountered {}",
            signal.expected, signal.found
        );
    }

    fn on_missing_field(&mut self, signal: &MissingFieldSignal) {
        self.message = format!(
            "the volume '{}' does not contain the required field '{}'",
            signal.parent_type, signal.field_name
        );
    }
}

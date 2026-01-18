//! Diagnostic Bridge - The Composition (L4) that performs the "Weld" between layers.
//!
//! This module acts as the terminal of Law IV, pairing the purely logical signal
//! with the geographic coordinate (Span) that the Core is forbidden to know.
//!
//! # Architecture Notes
//! The Bridge uses a trait-based approach to avoid cyclic dependencies:
//! - `shared::diagnostics` provides the pure `VoidSignal` and `NarrativeVisitor` traits
//! - The Shell (L2) implements `NarrativeVisitor` for user-facing messages
//! - The Bridge receives a projector function, decoupling from specific implementations

use lexicon::Span;
use primitives::diag::SourceDiagnostic;
use shared::diagnostics::{CrystalResult, VoidSignal};

/// A projector function that transforms a VoidSignal into a SourceDiagnostic.
/// This enables dependency injection from the Shell layer without direct coupling.
pub type SignalProjector = fn(&dyn VoidSignal, Span) -> SourceDiagnostic;

/// The Bridge is the Composition (L4) that performs the "Weld" between layers.
/// 
/// It provides utilities for converting Core's pure signals into Shell's diagnostics
/// without creating direct dependencies between layers.
pub struct DiagnosticBridge;

impl DiagnosticBridge {
    /// Resolves a geometric collapse from the Core into a diagnostic ready for the Shell.
    ///
    /// This function acts as the terminal of Law IV, pairing the purely logical signal
    /// with the geographic coordinate (Span) that the Core is forbidden to know.
    ///
    /// # Arguments
    /// * `result` - The CrystalResult from Core containing either data or a VoidSignal
    /// * `geography` - The Span (source location) for the diagnostic
    /// * `projector` - A function that converts VoidSignal to SourceDiagnostic
    pub fn resolve<T>(
        result: CrystalResult<T>,
        geography: Span,
        projector: SignalProjector,
    ) -> Result<T, SourceDiagnostic> {
        result.map_err(|signal| projector(signal.as_ref(), geography))
    }

    /// Executes a logical operation and converts any failure immediately.
    pub fn perform<F, T>(
        logic: F,
        span: Span,
        projector: SignalProjector,
    ) -> Result<T, SourceDiagnostic>
    where
        F: FnOnce() -> CrystalResult<T>,
    {
        let outcome = logic();
        Self::resolve(outcome, span, projector)
    }
}

/// Helper trait for building projectors from NarrativeVisitor implementations.
/// This is implemented by the Shell layer.
pub trait DiagnosticProjector {
    /// Creates a projector function from this visitor implementation.
    fn as_projector() -> SignalProjector;
}

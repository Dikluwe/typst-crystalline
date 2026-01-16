use crate::core::diagnostics::{NarrativeVisitor, VoidSignal, AccessSignal, TypeMismatchSignal, MissingFieldSignal};
use typst::diag::{SourceDiagnostic, Severity};
use typst::syntax::Span;

/// O Shield é o Adaptador Primário (L2) que transforma o colapso técnico
/// em uma experiência social inteligível.
pub struct DiagnosticShield {
    /// O estado temporário da projeção durante a visitação.
    message: String,
    severity: Severity,
}

impl DiagnosticShield {
    /// Ponto de entrada para a Projeção de Espaço.
    /// Transforma um Signal (L1) + Span (L0) em um Diagnostic (L2).
    pub fn project(signal: &dyn VoidSignal, span: Span) -> SourceDiagnostic {
        let mut shield = Self {
            message: String::new(),
            severity: Severity::Error,
        };

        // Law III: O sinal "habita" o visitante para extrair a narrativa.
        signal.accept(&mut shield);

        SourceDiagnostic::new(shield.severity, span, shield.message)
    }
}

/// Implementação da Facet de Visitação.
/// Aqui reside a "Voz" que o Core (L1) não possui.
impl NarrativeVisitor for DiagnosticShield {
    fn on_access_denied(&mut self, signal: &AccessSignal) {
        self.message = format!(
            "Access denied: the property '{}' on '{}' is strictly guarded by the crystal logic.",
            signal.property, signal.target
        );
    }

    fn on_type_mismatch(&mut self, signal: &TypeMismatchSignal) {
        self.message = format!(
            "Geometric mismatch: expected a facet of type {}, but encountered {}.",
            signal.expected, signal.found
        );
    }

    fn on_missing_field(&mut self, signal: &MissingFieldSignal) {
        self.message = format!(
            "The volume '{}' does not contain the required field '{}'.",
            signal.parent_type, signal.field_name
        );
    }
}

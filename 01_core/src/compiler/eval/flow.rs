//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval.md
//! @prompt-hash 10314082
//! @layer L1
//! @updated 2026-07-09
//!
//! Eventos de controlo de fluxo: `FlowEvent` e mensagens de erro fora de
//! contexto. Extraído para módulo próprio no Passo 635, espelhando
//! `typst-eval/src/flow.rs` do vanilla.

use crate::entities::source_result::SourceDiagnostic;
use crate::entities::span::Span;
use crate::entities::value::Value;

/// A control flow event that occurred during evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum FlowEvent {
    /// Stop iteration in a loop.
    Break(Span),
    /// Skip the remainder of the current iteration in a loop.
    Continue(Span),
    /// Stop execution of a function early, optionally returning an explicit
    /// value. The final boolean indicates whether the return was conditional.
    Return(Span, Option<Value>, bool),
}

impl FlowEvent {
    /// Return an error stating that this control flow is forbidden.
    pub fn forbidden(&self) -> SourceDiagnostic {
        match *self {
            Self::Break(span) => {
                SourceDiagnostic::error(span, "cannot break outside of loop")
            }
            Self::Continue(span) => {
                SourceDiagnostic::error(span, "cannot continue outside of loop")
            }
            Self::Return(span, _, _) => {
                SourceDiagnostic::error(span, "cannot return outside of function")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn break_forbidden_message() {
        let diag = FlowEvent::Break(Span::detached()).forbidden();
        assert_eq!(diag.message, "cannot break outside of loop");
    }

    #[test]
    fn continue_forbidden_message() {
        let diag = FlowEvent::Continue(Span::detached()).forbidden();
        assert_eq!(diag.message, "cannot continue outside of loop");
    }

    #[test]
    fn return_forbidden_message() {
        let diag = FlowEvent::Return(Span::detached(), None, false).forbidden();
        assert_eq!(diag.message, "cannot return outside of function");
    }

    #[test]
    fn return_with_value_stores_value() {
        let value = Value::Int(42);
        let event = FlowEvent::Return(Span::detached(), Some(value.clone()), false);
        assert_eq!(
            event,
            FlowEvent::Return(Span::detached(), Some(Value::Int(42)), false)
        );
    }
}

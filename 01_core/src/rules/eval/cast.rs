//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval/cast.md
//! @layer L1
//! @updated 2026-06-25
//!
//! P469 — Casts implícitos de `Value` para tipos concretos.
//!
//! No eval puro, `Value::Relative` não pode ser resolvido para `Length`
//! porque falta o contexto de layout (largura/altura do container).
//! Consumers em Trilha 7 recebem `Rel<Length>` e resolvem com contexto.

use crate::entities::layout_types::Length;
use crate::entities::value::Value;

/// Erro de cast de `Value` para tipo concreto.
#[derive(Debug, Clone, PartialEq)]
pub enum CastError {
    /// Tipo incompatível.
    TypeMismatch,
    /// Valor relativo precisa de contexto de layout para resolver.
    NeedsContext(&'static str),
}

/// Cast de `Value` para `Length`.
pub fn cast_length(value: Value) -> Result<Length, CastError> {
    match value {
        Value::Length(l) => Ok(l),
        Value::Relative(_) => Err(CastError::NeedsContext(
            "relative length needs layout context to resolve",
        )),
        _ => Err(CastError::TypeMismatch),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::Length;
    use crate::entities::rel::Rel;

    #[test]
    fn cast_length_from_length() {
        assert_eq!(cast_length(Value::Length(Length::cm(2.0))).unwrap(), Length::cm(2.0));
    }

    #[test]
    fn cast_length_relative_needs_context() {
        let result = cast_length(Value::Relative(Rel::<Length>::from_percent(50.0)));
        assert!(matches!(result, Err(CastError::NeedsContext(_))));
    }

    #[test]
    fn cast_length_from_int_fails() {
        let result = cast_length(Value::Int(42));
        assert!(matches!(result, Err(CastError::TypeMismatch)));
    }
}

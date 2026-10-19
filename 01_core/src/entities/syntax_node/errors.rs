//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/syntax_node/errors.md
//! @prompt-hash 4e42dd8f
//! @layer L1
//! @updated 2026-09-26

use crate::entities::span::Span;
use crate::entities::syntax_text::SyntaxText;
use std::fmt::{self, Display, Formatter};

/// Categoria de um erro sintático.
///
/// **P649** — usada para decidir quais erros de lexer devem ser
/// propagados pelo eval, em vez de depender da comparação frágil de
/// strings com o texto exacto da mensagem.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SyntaxErrorKind {
    /// Erro genérico ou não categorizado.
    #[default]
    Other,
    /// Literal numérico hexadecimal malformado (ex: `0xZZ`).
    InvalidHexNumber,
    /// Escape Unicode inválido (ex: `\u{FFFFFFFF}`).
    InvalidUnicodeCodepoint,
    /// **P786a** — par `**` sem conteúdo (warning; roteado ao sink no eval,
    /// ver `prompts/compiler/eval.md` §P786a e `prompts/compiler/parse.md` §P786a).
    NoTextWithinStars,
    /// **P786a** — par `__` sem conteúdo (warning; idem).
    NoTextWithinUnderscores,
}

/// A syntactical error.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxError {
    /// The node's span.
    pub span: Span,
    /// Categoria do erro.
    pub kind: SyntaxErrorKind,
    /// The error message.
    pub message: SyntaxText,
    /// Additional hints to the user.
    pub hints: Vec<SyntaxText>,
}

impl SyntaxError {
    /// Create a new detached syntax error.
    pub fn new(message: impl Into<SyntaxText>) -> Self {
        Self {
            span: Span::detached(),
            kind: SyntaxErrorKind::Other,
            message: message.into(),
            hints: vec![],
        }
    }

    /// Create a new detached syntax error with a specific kind.
    pub fn with_kind(message: impl Into<SyntaxText>, kind: SyntaxErrorKind) -> Self {
        Self {
            span: Span::detached(),
            kind,
            message: message.into(),
            hints: vec![],
        }
    }

    pub(super) fn spanless_eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.message == other.message
            && self.hints == other.hints
    }
}

/// Result of numbering a node within an interval.
pub(crate) type NumberingResult = Result<(), Unnumberable>;

/// Indicates that a node cannot be numbered within a given interval.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct Unnumberable;

impl Display for Unnumberable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad("cannot number within this interval")
    }
}

impl std::error::Error for Unnumberable {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_keep_all_categories_and_detached_defaults() {
        assert_eq!(SyntaxError::new("error").kind, SyntaxErrorKind::Other);
        for kind in [
            SyntaxErrorKind::Other,
            SyntaxErrorKind::InvalidHexNumber,
            SyntaxErrorKind::InvalidUnicodeCodepoint,
            SyntaxErrorKind::NoTextWithinStars,
            SyntaxErrorKind::NoTextWithinUnderscores,
        ] {
            let error = SyntaxError::with_kind("error", kind);
            assert_eq!(error.kind, kind);
            assert_eq!(error.message.as_str(), "error");
            assert!(error.span.is_detached());
            assert!(error.hints.is_empty());
        }
    }

    #[test]
    fn spanless_error_equality_keeps_hints_and_kind() {
        let original = SyntaxError::new("error");
        let mut other = original.clone();
        let id = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        other.span = Span::from_number(id, 10).unwrap();
        assert!(original.spanless_eq(&other));
        other.hints.push("hint".into());
        assert!(!original.spanless_eq(&other));
        other.hints.clear();
        other.kind = SyntaxErrorKind::InvalidHexNumber;
        assert!(!original.spanless_eq(&other));
    }

    #[test]
    fn numbering_failure_display_stays_stable() {
        assert_eq!(Unnumberable.to_string(), "cannot number within this interval");
    }
}

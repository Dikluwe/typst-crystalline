//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/syntax_diagnostics.md
//! @prompt-hash a3eeb842
//! @layer L1
//! @updated 2026-10-17

use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::{LinkedNode, SyntaxErrorKind, SyntaxNode};

/// Validate an untyped tree before evaluation, preserving source diagnostics.
pub(super) fn validate_syntax(
    root: &SyntaxNode,
    mut emit_warning: impl FnMut(SourceDiagnostic),
) -> SourceResult<()> {
    let mut errors = Vec::new();
    for error in root.errors() {
        let warning_parent = match error.kind {
            SyntaxErrorKind::NoTextWithinStars => Some(SyntaxKind::Strong),
            SyntaxErrorKind::NoTextWithinUnderscores => Some(SyntaxKind::Emph),
            SyntaxErrorKind::Other
            | SyntaxErrorKind::InvalidHexNumber
            | SyntaxErrorKind::InvalidUnicodeCodepoint => None,
        };
        let mut diagnostic = if let Some(kind) = warning_parent {
            let span = LinkedNode::new(root)
                .find(error.span)
                .filter(|node| node.kind() == SyntaxKind::Error && node.is_empty())
                .and_then(|node| {
                    node.parent()
                        .filter(|parent| parent.kind() == kind && parent.len() == 2)
                        .map(|parent| parent.span())
                })
                .unwrap_or(error.span);
            SourceDiagnostic::warning(span, error.message.to_string())
        } else {
            SourceDiagnostic::error(error.span, error.message.to_string())
        };
        diagnostic.hints = error.hints.iter().map(ToString::to_string).collect();
        if warning_parent.is_some() {
            emit_warning(diagnostic);
        } else {
            errors.push(diagnostic);
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

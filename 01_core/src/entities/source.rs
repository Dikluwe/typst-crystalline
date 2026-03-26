//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/source.md
//! @prompt-hash 95e8aee1
//! @layer L1
//! @updated 2026-03-25

use std::hash::{Hash, Hasher};
use std::num::NonZeroU16;
use std::sync::Arc;

use crate::entities::file_id::FileId;
use crate::entities::span::Span;
use crate::entities::syntax_node::SyntaxNode;
use crate::rules::parse::parse;

/// Ficheiro de texto carregado em memória com a sua CST associada.
///
/// `Source` é domínio puro: recebe texto já carregado e chama `parse()`
/// internamente. O carregamento do filesystem acontece em L3.
///
/// Clone é barato — partilha o interior via `Arc`.
/// `Hash` é implementado manualmente baseado em `(id, text)` — necessário
/// para `comemo::track` em `TrackedWorld`. Sem `LazyHash` (ADR-0016):
/// o hash é calculado por invocação, não cached. Se performance for
/// relevante, um campo `content_hash: u64` será adicionado no Passo 10.
#[derive(Clone)]
pub struct Source(Arc<SourceInner>);

impl Hash for Source {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.id.hash(state);
        self.0.text.hash(state);
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id && self.0.text == other.0.text
    }
}

impl Eq for Source {}

struct SourceInner {
    id: FileId,
    text: String,
    root: SyntaxNode,
}

impl Source {
    /// Cria `Source` com `FileId` explícito — usado por L3 ao carregar ficheiros.
    pub fn new(id: FileId, text: String) -> Self {
        let mut root = parse(&text);
        root.numberize(id, Span::FULL).unwrap();
        Self(Arc::new(SourceInner { id, text, root }))
    }

    /// Cria `Source` sem `FileId` real — para testes e contextos sem filesystem.
    ///
    /// Usa um `FileId` sentinel (1). Dois `detached()` partilham o mesmo id —
    /// comportamento aceitável fora de um `World` real.
    pub fn detached(text: impl Into<String>) -> Self {
        let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
        Self::new(id, text.into())
    }

    /// O `FileId` desta source.
    pub fn id(&self) -> FileId {
        self.0.id
    }

    /// O texto completo da source.
    pub fn text(&self) -> &str {
        &self.0.text
    }

    /// O nó raiz da CST (tipo `SyntaxKind::Markup` para ficheiros normais).
    pub fn root(&self) -> &SyntaxNode {
        &self.0.root
    }

    /// Comprimento do texto em bytes.
    pub fn len_bytes(&self) -> usize {
        self.0.text.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        file_id::FileId,
        syntax_kind::SyntaxKind,
    };

    fn test_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    #[test]
    fn source_root_markup() {
        let src = Source::new(test_id(), "Hello *world*".into());
        assert_eq!(src.root().kind(), SyntaxKind::Markup);
        assert!(!src.root().erroneous());
    }

    #[test]
    fn source_vazia() {
        let src = Source::new(test_id(), "".into());
        assert_eq!(src.len_bytes(), 0);
        assert_eq!(src.text(), "");
    }

    #[test]
    fn source_detached_heading() {
        let src = Source::detached("= Heading");
        let has_heading = src.root()
            .children()
            .any(|n| n.kind() == SyntaxKind::Heading);
        assert!(has_heading);
    }

    #[test]
    fn source_com_erros() {
        let src = Source::detached("#{{{broken");
        assert!(src.root().erroneous());
    }

    #[test]
    fn source_id_roundtrip() {
        let id = test_id();
        let src = Source::new(id, "text".into());
        assert_eq!(src.id(), id);
    }

    #[test]
    fn source_text_preservado() {
        let src = Source::new(test_id(), "Hello *world*".into());
        assert_eq!(src.text(), "Hello *world*");
    }
}

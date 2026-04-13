//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/source.md
//! @prompt-hash 4fd9dae5
//! @layer L1
//! @updated 2026-03-25

use std::hash::{Hash, Hasher};
use std::num::NonZeroU16;
use std::sync::Arc;

use rustc_hash::FxHasher;

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
/// `Hash` é implementado manualmente baseado em `(id, content_hash)` — O(1)
/// graças ao hash pré-computado em `new()` (ADR-0031).
#[derive(Clone, Debug)]
pub struct Source(Arc<SourceInner>);

impl Hash for Source {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.id.hash(state);
        self.0.content_hash.hash(state);
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id && self.0.content_hash == other.0.content_hash
    }
}

impl Eq for Source {}

#[derive(Debug)]
struct SourceInner {
    id:           FileId,
    text:         String,
    root:         SyntaxNode,
    content_hash: u64,   // ADR-0031 — pré-computado em new(), nunca muda
}

impl Source {
    /// Cria `Source` com `FileId` explícito — usado por L3 ao carregar ficheiros.
    pub fn new(id: FileId, text: String) -> Self {
        let content_hash = {
            let mut h = FxHasher::default();
            text.hash(&mut h);
            h.finish()
        };
        let mut root = parse(&text);
        root.numberize(id, Span::FULL).unwrap();
        Self(Arc::new(SourceInner { id, text, root, content_hash }))
    }

    /// Hash do conteúdo — O(1), pré-computado na construção (ADR-0031).
    pub fn content_hash(&self) -> u64 {
        self.0.content_hash
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

    // ── Passo 26 — Early hashing ADR-0031 ───────────────────────────────────

    #[test]
    fn source_hash_o1_apos_construcao() {
        let s = Source::detached("hello world");
        // Múltiplas chamadas retornam o mesmo hash — pré-computado
        assert_eq!(s.content_hash(), s.content_hash());
    }

    #[test]
    fn source_eq_mesmo_conteudo() {
        let id = test_id();
        let s1 = Source::new(id, "hello".into());
        let s2 = Source::new(id, "hello".into());
        assert_eq!(s1, s2);
    }

    #[test]
    fn source_neq_conteudo_diferente() {
        let id = test_id();
        let s1 = Source::new(id, "hello".into());
        let s2 = Source::new(id, "world".into());
        assert_ne!(s1, s2);
    }

    #[test]
    fn source_pode_ser_chave_de_hashmap() {
        use rustc_hash::FxHashMap;
        let mut map: FxHashMap<Source, &str> = FxHashMap::default();
        let s = Source::detached("test");
        map.insert(s.clone(), "value");
        assert_eq!(map.get(&s), Some(&"value"));
    }
}

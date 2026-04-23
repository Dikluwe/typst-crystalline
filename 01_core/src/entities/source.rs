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
use crate::entities::syntax_node::{LinkedNode, SyntaxNode};
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

    /// Resolve `span` para `(linha, coluna)` 1-indexadas (Passo 111, ADR-0045).
    ///
    /// Devolve `None` se:
    /// - o span é detached (`Span::is_detached()`);
    /// - o span refere-se a outro ficheiro (`span.id() != Some(self.id())`);
    /// - o span é numbered e o nó correspondente não é encontrado;
    /// - o byte offset excede o texto (defensivo).
    ///
    /// Coluna em Unicode code points (chars), não bytes — convenção
    /// editor seguida por gcc/clang/rustc. `\r\n` conta como 1 newline
    /// (avança linha em `\n`; `\r` conta como char na linha anterior).
    pub fn span_to_line_col(&self, span: Span) -> Option<(u32, u32)> {
        if span.is_detached() {
            return None;
        }
        if span.id() != Some(self.0.id) {
            return None;
        }

        // Raw-range spans: o range dá byte offset directo.
        // Numbered spans: resolver via LinkedNode::find → offset do nó.
        let offset = if let Some(range) = span.range() {
            range.start
        } else {
            LinkedNode::new(&self.0.root).find(span)?.offset()
        };

        let text = &self.0.text;
        if offset > text.len() {
            return None;
        }

        // Contar linhas/colunas em chars até offset.
        let mut line: u32 = 1;
        let mut col: u32 = 1;
        for (i, ch) in text.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        Some((line, col))
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

    // ── Passo 111 (ADR-0045) — span_to_line_col ────────────────────────

    fn raw_span(src: &Source, range: std::ops::Range<usize>) -> Span {
        Span::from_range(src.id(), range)
    }

    #[test]
    fn span_to_line_col_inicio_do_texto() {
        let src = Source::new(test_id(), "abc".into());
        let sp = raw_span(&src, 0..0);
        assert_eq!(src.span_to_line_col(sp), Some((1, 1)));
    }

    #[test]
    fn span_to_line_col_depois_de_newline() {
        // "abc\nde" — offset 4 é o 'd' (segunda linha, coluna 1).
        let src = Source::new(test_id(), "abc\nde".into());
        let sp = raw_span(&src, 4..4);
        assert_eq!(src.span_to_line_col(sp), Some((2, 1)));
    }

    #[test]
    fn span_to_line_col_multibyte_unicode_coluna_em_chars() {
        // "áéí" — cada char tem 2 bytes em UTF-8.
        // Offset 2 é início do 'é' (coluna 2 em chars).
        let src = Source::new(test_id(), "áéí".into());
        let sp = raw_span(&src, 2..2);
        assert_eq!(src.span_to_line_col(sp), Some((1, 2)));
    }

    #[test]
    fn span_to_line_col_detached_devolve_none() {
        let src = Source::new(test_id(), "qualquer coisa".into());
        assert_eq!(src.span_to_line_col(Span::detached()), None);
    }

    #[test]
    fn span_to_line_col_ficheiro_diferente_devolve_none() {
        use std::num::NonZeroU16;
        let src = Source::new(test_id(), "hello".into());
        let other_id = FileId::from_raw(NonZeroU16::new(42).unwrap());
        let sp = Span::from_range(other_id, 0..0);
        assert_eq!(src.span_to_line_col(sp), None);
    }

    #[test]
    fn span_to_line_col_fim_do_texto() {
        // "abc" — offset 3 == len → posição após o 'c' (coluna 4).
        let src = Source::new(test_id(), "abc".into());
        let sp = raw_span(&src, 3..3);
        assert_eq!(src.span_to_line_col(sp), Some((1, 4)));
    }

    #[test]
    fn span_to_line_col_span_numbered_via_linked_find() {
        // Spans numbered: obter o span dum nó real da CST e confirmar
        // que span_to_line_col devolve a posição do nó.
        let src = Source::new(test_id(), "hello world".into());
        let root = src.root();
        // Pegar no primeiro filho com span numbered.
        let first_child = root.children().next().expect("root tem filhos");
        let sp = first_child.span();
        assert!(!sp.is_detached(), "filho deve ter span numbered");
        // Posição esperada: início da source → (1, 1) para o primeiro leaf.
        let pos = src.span_to_line_col(sp);
        assert!(pos.is_some(), "span_to_line_col deve resolver spans numbered reais");
    }

    #[test]
    fn span_to_line_col_offset_fora_de_limites_retorna_none() {
        // Raw-range com offset além do texto — defensivo.
        let src = Source::new(test_id(), "abc".into());
        let sp = raw_span(&src, 100..200);
        // start=100 excede text.len()=3 → None.
        assert_eq!(src.span_to_line_col(sp), None);
    }
}

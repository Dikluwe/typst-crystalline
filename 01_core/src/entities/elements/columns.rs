//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/columns.md
//! @prompt-hash e5100460
//! @layer L1
//! @updated 2026-06-11
//!
//! `ColumnsElem` — Lote 8 P323 (por largura). `columns(count, gutter, body)`.
//! Contentor: recurse no body em map_content E map_text.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// Distribui o `body` por `count` colunas, com `gutter` entre elas.
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnsElem {
    pub count:        usize,
    pub gutter:       Option<Length>,
    pub body:         Content,
    /// **P552** — `true` quando este `ColumnsElem` foi produzido por
    /// `#set page(columns: N)` (via `wrap_page_columns`). Distingue a
    /// semântica de footnotes: por coluna (página) vs empilhadas (contentor).
    pub page_columns: bool,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Length` carrega `f64`
// e não implementa `Hash`. Ressalva `-0.0` vs `0.0` aceitável (`…Elem` não são
// chaves de mapa).
impl std::hash::Hash for ColumnsElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for ColumnsElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Columns(Arc::new(ColumnsElem {
            count:        self.count,
            gutter:       self.gutter,
            body:         self.body.map_content(transform)?,
            page_columns: self.page_columns,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Columns(Arc::new(ColumnsElem {
            count:        self.count,
            gutter:       self.gutter,
            body:         self.body.map_text(transform),
            page_columns: self.page_columns,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn ex() -> ColumnsElem {
        ColumnsElem { count: 2, gutter: None, body: Content::text("a"), page_columns: false }
    }

    #[test]
    fn plain_text_delega_ao_body() {
        assert_eq!(ex().plain_text(), "a");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        assert!(ColumnsElem { count: 2, gutter: None, body: Content::Empty, page_columns: false }.is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_count() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Columns(e) => {
                assert_eq!(e.count, 2);
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Columns"),
        }
    }

    fn h(e: &ColumnsElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        assert_ne!(h(&ex()), h(&ColumnsElem { count: 3, gutter: None, body: Content::text("a"), page_columns: false }));
    }
}

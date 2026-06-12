//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/table.md
//! @prompt-hash bc89d250
//! @layer L1
//! @updated 2026-06-11
//!
//! `TableElem` — Lote 12 P327 (bloco grid/table cell). `table(columns, rows, …)`.
//! Contentor: recurse em cada child em map_content E map_text. Não-locatável.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{Color, TrackSizing};
use crate::entities::source_result::SourceResult;

/// Tabela: `children` distribuídos em `columns`×`rows`; `stroke`/`fill` globais.
#[derive(Debug, Clone, PartialEq)]
pub struct TableElem {
    pub columns:  Vec<TrackSizing>,
    pub rows:     Vec<TrackSizing>,
    pub children: Vec<Content>,
    pub stroke:   Option<Stroke>,
    pub fill:     Option<Color>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `TrackSizing`/`Stroke`/
// `Color` carregam `f64`. Ressalva `-0.0` vs `0.0` aceitável.
impl std::hash::Hash for TableElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for TableElem {
    fn plain_text(&self) -> String {
        self.children.iter().map(|c| c.plain_text()).collect::<Vec<_>>().join(" ")
    }

    fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let new_children: SourceResult<Vec<Content>> =
            self.children.iter().map(|c| c.map_content(transform)).collect();
        Ok(Content::Table(Arc::new(TableElem {
            columns:  self.columns.clone(),
            rows:     self.rows.clone(),
            children: new_children?,
            stroke:   self.stroke.clone(),
            fill:     self.fill,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Table(Arc::new(TableElem {
            columns:  self.columns.clone(),
            rows:     self.rows.clone(),
            children: self.children.iter().map(|c| c.map_text(transform)).collect(),
            stroke:   self.stroke.clone(),
            fill:     self.fill,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn ex() -> TableElem {
        TableElem {
            columns: vec![], rows: vec![],
            children: vec![Content::text("a"), Content::text("b")],
            stroke: None, fill: None,
        }
    }

    #[test]
    fn plain_text_concatena_children() {
        assert_eq!(ex().plain_text(), "a b");
    }

    #[test]
    fn is_empty_so_sem_children() {
        assert!(!ex().is_empty());
        assert!(TableElem { columns: vec![], rows: vec![], children: vec![], stroke: None, fill: None }.is_empty());
    }

    #[test]
    fn map_content_recurse_children() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s, _) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Table(e) => assert!(matches!(&e.children[0], Content::Text(s, _) if s.as_str() == "Z")),
            _ => panic!("esperado Table"),
        }
    }

    fn h(e: &TableElem) -> u64 {
        let mut s = DefaultHasher::new(); e.hash(&mut s); s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let outro = TableElem { columns: vec![], rows: vec![], children: vec![], stroke: None, fill: None };
        assert_ne!(h(&ex()), h(&outro));
    }
}

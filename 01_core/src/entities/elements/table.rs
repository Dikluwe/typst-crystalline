//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/table.md
//! @prompt-hash 7f7bde85
//! @layer L1
//! @updated 2026-06-25
//!
//! `TableElem` — Lote 12 P327 (bloco grid/table cell). `table(columns, rows, …)`.
//! P459: `caption` opcional para numeração automática via `table.numbering`.
//! Contentor: recurse em cada child e no caption em map_content E map_text. Não-locatável.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{Color, TrackSizing};
use crate::entities::source_result::SourceResult;

/// Tabela: `children` distribuídos em `columns`×`rows`; `stroke`/`fill` globais.
/// **P459**: `caption` opcional para numeração automática via `table.numbering`.
#[derive(Debug, Clone, PartialEq)]
pub struct TableElem {
    pub columns:  Vec<TrackSizing>,
    pub rows:     Vec<TrackSizing>,
    pub children: Vec<Content>,
    pub stroke:   Option<Stroke>,
    pub fill:     Option<Color>,
    pub caption:  Option<Content>,
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
        let cells = self.children.iter().map(|c| c.plain_text()).collect::<Vec<_>>().join(" ");
        let cap = self.caption.as_ref().map(|c| c.plain_text()).unwrap_or_default();
        match (cells.is_empty(), cap.is_empty()) {
            (false, false) => format!("{} {}", cap, cells),
            (false, true)  => cells,
            (true,  false) => cap,
            (true,  true)  => String::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.children.is_empty() && self.caption.as_ref().is_none_or(|c| c.is_empty())
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
            caption:  self.caption.as_ref().map(|c| c.map_content(transform)).transpose()?,
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
            caption:  self.caption.as_ref().map(|c| c.map_text(transform)),
        }))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Table)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        // P461: `is_counted` placeholder = caption.is_some(); o gate
        // `table.numbering` é ANDado pela chain no walk top (espelho
        // Figure P365). O elemento não baka o padrão.
        Some(ElementPayload::Table {
            counter_update: CounterUpdate::Step,
            is_counted:     self.caption.is_some(),
        })
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
            stroke: None, fill: None, caption: None,
        }
    }

    #[test]
    fn plain_text_concatena_children() {
        assert_eq!(ex().plain_text(), "a b");
    }

    #[test]
    fn is_empty_so_sem_children() {
        assert!(!ex().is_empty());
        assert!(TableElem { columns: vec![], rows: vec![], children: vec![], stroke: None, fill: None, caption: None }.is_empty());
    }

    #[test]
    fn map_content_recurse_children() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Table(e) => assert!(matches!(&e.children[0], Content::Text(s) if s.as_str() == "Z")),
            _ => panic!("esperado Table"),
        }
    }

    fn h(e: &TableElem) -> u64 {
        let mut s = DefaultHasher::new(); e.hash(&mut s); s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let outro = TableElem { columns: vec![], rows: vec![], children: vec![], stroke: None, fill: None, caption: None };
        assert_ne!(h(&ex()), h(&outro));
    }
}

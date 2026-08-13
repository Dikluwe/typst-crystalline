//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/grid.md
//! @prompt-hash 084f0a33
//! @layer L1
//! @updated 2026-06-11
//!
//! `GridElem` — Lote 12 P327 (bloco grid/table cell). `grid(columns, rows, …)`.
//! Contentor: recurse em cells (Vec), header e footer. Não-locatável.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{Align2D, Color, Length, TrackSizing};
use crate::entities::sides::Sides;
use crate::entities::source_result::SourceResult;

/// Grade: `cells` distribuídas em `columns`×`rows`; `header`/`footer` opcionais;
/// `gutter`/`align`/`inset`/`stroke`/`fill` cosméticos.
/// **P512**: `hlines`/`vlines` são renderizadas pelo layout_grid.
#[derive(Debug, Clone, PartialEq)]
pub struct GridElem {
    pub columns: Vec<TrackSizing>,
    pub rows: Vec<TrackSizing>,
    pub cells: Vec<Content>,
    pub hlines: Vec<crate::entities::elements::grid_hline::GridHLineElem>,
    pub vlines: Vec<crate::entities::elements::grid_vline::GridVLineElem>,
    pub gutter: Option<Length>,
    pub align: Option<Align2D>,
    pub inset: Sides<Length>,
    pub header: Option<Content>,
    pub footer: Option<Content>,
    pub stroke: Option<Stroke>,
    pub fill: Option<Color>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `TrackSizing`/`Length`/
// `Align2D`/`Sides<Length>`/`Stroke`/`Color` carregam `f64`.
impl std::hash::Hash for GridElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for GridElem {
    fn plain_text(&self) -> String {
        self.cells
            .iter()
            .map(|c| c.plain_text())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let new_cells: SourceResult<Vec<Content>> =
            self.cells.iter().map(|c| c.map_content(transform)).collect();
        Ok(Content::Grid(Arc::new(GridElem {
            columns: self.columns.clone(),
            rows: self.rows.clone(),
            cells: new_cells?,
            hlines: self.hlines.clone(),
            vlines: self.vlines.clone(),
            gutter: self.gutter,
            align: self.align,
            inset: self.inset,
            header: self.header.as_ref().map(|h| h.map_content(transform)).transpose()?,
            footer: self.footer.as_ref().map(|f| f.map_content(transform)).transpose()?,
            stroke: self.stroke.clone(),
            fill: self.fill,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Grid(Arc::new(GridElem {
            columns: self.columns.clone(),
            rows: self.rows.clone(),
            cells: self.cells.iter().map(|c| c.map_text(transform)).collect(),
            hlines: self.hlines.clone(),
            vlines: self.vlines.clone(),
            gutter: self.gutter,
            align: self.align,
            inset: self.inset,
            header: self.header.as_ref().map(|h| h.map_text(transform)),
            footer: self.footer.as_ref().map(|f| f.map_text(transform)),
            stroke: self.stroke.clone(),
            fill: self.fill,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> GridElem {
        GridElem {
            columns: vec![],
            rows: vec![],
            cells: vec![Content::text("a"), Content::text("b")],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::pt(0.0)),
            header: None,
            footer: None,
            stroke: None,
            fill: None,
        }
    }

    #[test]
    fn plain_text_concatena_cells() {
        assert_eq!(ex().plain_text(), "a b");
    }

    #[test]
    fn is_empty_so_sem_cells() {
        assert!(!ex().is_empty());
        let mut v = ex();
        v.cells = vec![];
        assert!(v.is_empty());
    }

    #[test]
    fn map_content_recurse_cells_e_header() {
        let mut v = ex();
        v.header = Some(Content::text("a"));
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match v.map_content(&mut f).unwrap() {
            Content::Grid(e) => {
                assert!(matches!(&e.cells[0], Content::Text(s) if s.as_str() == "Z"));
                assert!(
                    matches!(e.header.as_ref().unwrap(), Content::Text(s) if s.as_str() == "Z")
                );
            }
            _ => panic!("esperado Grid"),
        }
    }

    fn h(e: &GridElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let mut v = ex();
        v.cells = vec![];
        assert_ne!(h(&ex()), h(&v));
    }
}

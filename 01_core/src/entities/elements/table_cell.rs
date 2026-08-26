//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/table_cell.md
//! @prompt-hash a21b9074
//! @layer L1
//! @updated 2026-06-11
//!
//! `TableCellElem` — Lote 12 P327 (bloco grid/table cell). `table_cell(body, …)`.
//! Contentor: recurse no body em map_content E map_text. Não-locatável.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::Length;
use crate::entities::layout_types::{Align2D, Color};
use crate::entities::sides::Sides;
use crate::entities::source_result::SourceResult;

/// Célula de `Table`. `body` + 9 cosméticos (posição/span/estilo).
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellElem {
    pub body: Content,
    pub x: Option<usize>,
    pub y: Option<usize>,
    pub colspan: Option<usize>,
    pub rowspan: Option<usize>,
    pub stroke: Option<Stroke>,
    pub fill: Option<Color>,
    pub align: Option<Align2D>,
    pub inset: Option<Sides<Length>>,
    pub breakable: Option<bool>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Stroke`/`Color`/
// `Align2D`/`Sides<Length>` carregam `f64`. Ressalva `-0.0` vs `0.0` aceitável.
impl std::hash::Hash for TableCellElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for TableCellElem {
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
        Ok(Content::TableCell(Arc::new(TableCellElem {
            body: self.body.map_content(transform)?,
            x: self.x,
            y: self.y,
            colspan: self.colspan,
            rowspan: self.rowspan,
            stroke: self.stroke.clone(),
            fill: self.fill,
            align: self.align,
            inset: self.inset,
            breakable: self.breakable,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::TableCell(Arc::new(TableCellElem {
            body: self.body.map_text(transform),
            x: self.x,
            y: self.y,
            colspan: self.colspan,
            rowspan: self.rowspan,
            stroke: self.stroke.clone(),
            fill: self.fill,
            align: self.align,
            inset: self.inset,
            breakable: self.breakable,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> TableCellElem {
        TableCellElem {
            body: Content::text("a"),
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: None,
            fill: None,
            align: None,
            inset: None,
            breakable: None,
        }
    }

    #[test]
    fn plain_text_delega_ao_body() {
        assert_eq!(ex().plain_text(), "a");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        let mut v = ex();
        v.body = Content::Empty;
        assert!(v.is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_cosmeticos() {
        let mut c = ex();
        c.colspan = Some(2);
        let mut f = |x: &Content| -> SourceResult<Option<Content>> {
            match x {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match c.map_content(&mut f).unwrap() {
            Content::TableCell(e) => {
                assert_eq!(e.colspan, Some(2));
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado TableCell"),
        }
    }

    fn h(e: &TableCellElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let mut v = ex();
        v.colspan = Some(3);
        assert_ne!(h(&ex()), h(&v));
    }
}

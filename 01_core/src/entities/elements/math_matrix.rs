//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_matrix.md
//! @prompt-hash 0a978f42
//! @layer L1
//! @updated 2026-08-20
//!
//! `MathMatrixElem` — Lote 2 P317 (família math). Matriz `mat(...)`.

use std::sync::Arc;
use std::hash::{Hash, Hasher};

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// Matriz matemática (`mat(...)`).
#[derive(Debug, Clone, PartialEq)]
pub struct MathMatrixElem {
    pub rows: Vec<Vec<Content>>,
    pub delim: (char, char),
    pub row_gap: Option<Length>,
    pub column_gap: Option<Length>,
    pub gap: Option<Length>,
    pub augment: Option<usize>,
}

impl Hash for MathMatrixElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rows.hash(state);
        self.delim.hash(state);
        if let Some(g) = &self.row_gap {
            g.abs.0.to_bits().hash(state);
            g.em.to_bits().hash(state);
        }
        if let Some(g) = &self.column_gap {
            g.abs.0.to_bits().hash(state);
            g.em.to_bits().hash(state);
        }
        if let Some(g) = &self.gap {
            g.abs.0.to_bits().hash(state);
            g.em.to_bits().hash(state);
        }
        self.augment.hash(state);
    }
}

impl Element for MathMatrixElem {
    fn plain_text(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.iter().map(|c| c.plain_text()).collect::<Vec<_>>().join(", "))
            .collect::<Vec<_>>()
            .join("; ")
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let rows: SourceResult<Vec<Vec<Content>>> = self
            .rows
            .iter()
            .map(|row| row.iter().map(|c| c.map_content(transform)).collect())
            .collect();
        Ok(Content::MathMatrix(Arc::new(MathMatrixElem {
            rows: rows?,
            delim: self.delim,
            row_gap: self.row_gap,
            column_gap: self.column_gap,
            gap: self.gap,
            augment: self.augment,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathMatrix(Arc::new(self.clone()))
    }
}

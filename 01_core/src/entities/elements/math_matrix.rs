//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_matrix.md
//! @prompt-hash bf5a5c71
//! @layer L1
//! @updated 2026-09-17
//!
//! Carrier completo de `math.mat`.

use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{HAlign, Length};
use crate::entities::rel::Rel;
use crate::entities::source_result::SourceResult;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MathMatrixExplicit {
    pub delim: bool,
    pub align: bool,
    pub augment: bool,
    pub row_gap: bool,
    pub column_gap: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathMatrixAugment {
    pub hline: Vec<i64>,
    pub vline: Vec<i64>,
    /// `None` representa o stroke automático.
    pub stroke: Option<Stroke>,
}

impl Hash for MathMatrixAugment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hline.hash(state);
        self.vline.hash(state);
        format!("{:?}", self.stroke).hash(state);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathMatrixElem {
    pub rows: Vec<Vec<Content>>,
    pub delim: (Option<char>, Option<char>),
    pub align: HAlign,
    pub augment: Option<MathMatrixAugment>,
    pub row_gap: Rel<Length>,
    pub column_gap: Rel<Length>,
    pub explicit: MathMatrixExplicit,
}

impl Hash for MathMatrixElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        fn bits(value: f64) -> u64 {
            if value == 0.0 {
                0
            } else {
                value.to_bits()
            }
        }
        self.rows.hash(state);
        self.delim.hash(state);
        self.align.hash(state);
        self.augment.hash(state);
        for gap in [self.row_gap, self.column_gap] {
            bits(gap.rel).hash(state);
            bits(gap.abs.abs.0).hash(state);
            bits(gap.abs.em).hash(state);
        }
        self.explicit.hash(state);
    }
}

impl Element for MathMatrixElem {
    fn plain_text(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.iter().map(Content::plain_text).collect::<Vec<_>>().join(", "))
            .collect::<Vec<_>>()
            .join("; ")
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let rows = self
            .rows
            .iter()
            .map(|row| row.iter().map(|cell| cell.map_content(transform)).collect())
            .collect::<SourceResult<Vec<Vec<_>>>>()?;
        Ok(Content::MathMatrix(Arc::new(Self {
            rows,
            delim: self.delim,
            align: self.align,
            augment: self.augment.clone(),
            row_gap: self.row_gap,
            column_gap: self.column_gap,
            explicit: self.explicit,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathMatrix(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_explicitness_has_no_fabricated_presence() {
        assert_eq!(
            MathMatrixExplicit::default(),
            MathMatrixExplicit {
                delim: false,
                align: false,
                augment: false,
                row_gap: false,
                column_gap: false,
            }
        );
    }
}

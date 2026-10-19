//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/math_matrix.md
//! @prompt-hash f1510647
//! @layer L1
//! @updated 2026-10-10
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

    fn get_field(&self, field: &str) -> Option<crate::entities::value::Value> {
        use crate::entities::value::Value;
        match field {
            "rows" => Some(Value::Array(
                self.rows
                    .iter()
                    .map(|row| {
                        Value::Array(
                            row.iter()
                                .map(|cell| {
                                    Value::Content(if matches!(cell, Content::Empty) {
                                        Content::Sequence(Arc::from([]))
                                    } else {
                                        cell.clone()
                                    })
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            )),
            "delim" if self.explicit.delim => Some(Value::Array(
                [self.delim.0, self.delim.1]
                    .iter()
                    .map(|c| {
                        c.map(|c| Value::Str(c.to_string().into())).unwrap_or(Value::None)
                    })
                    .collect(),
            )),
            "align" if self.explicit.align => {
                Some(Value::Align(crate::entities::layout_types::Align2D {
                    h: Some(self.align),
                    v: None,
                }))
            }
            "row-gap" if self.explicit.row_gap => Some(Value::Relative(self.row_gap)),
            "column-gap" if self.explicit.column_gap => {
                Some(Value::Relative(self.column_gap))
            }
            "augment" if self.explicit.augment => Some(match &self.augment {
                None => Value::None,
                Some(a) => {
                    if a.stroke.is_none() && a.hline.is_empty() && a.vline.len() == 1 {
                        Value::Int(a.vline[0])
                    } else {
                        Value::Dict(
                            [
                                (
                                    "hline".into(),
                                    Value::Array(
                                        a.hline.iter().copied().map(Value::Int).collect(),
                                    ),
                                ),
                                (
                                    "vline".into(),
                                    Value::Array(
                                        a.vline.iter().copied().map(Value::Int).collect(),
                                    ),
                                ),
                                (
                                    "stroke".into(),
                                    a.stroke
                                        .as_ref()
                                        .map(|s| Value::from(s.clone()))
                                        .unwrap_or(Value::Auto),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )
                    }
                }
            }),
            _ => None,
        }
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathMatrix(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathMatrix(Arc::new(self.clone()))
    }
}

impl MathMatrixElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let mut rows: Option<Vec<Vec<Content>>> = None;
        for (index, row) in self.rows.iter().enumerate() {
            match Content::map_children_if_changed(row, transform)? {
                Some(changed) => {
                    let output = rows.get_or_insert_with(|| {
                        let mut output = Vec::with_capacity(self.rows.len());
                        output.extend_from_slice(&self.rows[..index]);
                        output
                    });
                    output.push(changed);
                }
                None => {
                    if let Some(output) = &mut rows {
                        output.push(row.clone());
                    }
                }
            }
        }
        Ok(rows.map(|rows| {
            Content::MathMatrix(Arc::new(Self {
                rows,
                delim: self.delim,
                align: self.align,
                augment: self.augment.clone(),
                row_gap: self.row_gap,
                column_gap: self.column_gap,
                explicit: self.explicit,
            }))
        }))
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

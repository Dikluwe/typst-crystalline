//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/transform.md
//! @prompt-hash 80b3de3e
//! @layer L1
//! @updated 2026-06-11
//!
//! `TransformElem` — Lote 9 P324 (por largura). `transform(matrix, body)`.
//! Contentor: recurse no body em map_content E map_text.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::{Align2D, Angle, Length, Ratio, TransformMatrix};
use crate::entities::rel::Rel;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Operação autoral; parâmetros dependentes do contexto são resolvidos no layout.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformElem {
    pub operation: TransformOperation,
    pub body: Content,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformScale {
    Auto,
    Ratio(Ratio),
    Length(Length),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformOperation {
    Matrix(TransformMatrix),
    Move {
        dx: Option<Rel<Length>>,
        dy: Option<Rel<Length>>,
    },
    Rotate {
        angle: Option<Angle>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
    Scale {
        x: Option<TransformScale>,
        y: Option<TransformScale>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
    Skew {
        ax: Option<Angle>,
        ay: Option<Angle>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
}

fn scale_value(value: TransformScale) -> Value {
    match value {
        TransformScale::Auto => Value::Auto,
        TransformScale::Ratio(r) => Value::Ratio(r),
        TransformScale::Length(l) => Value::Length(l),
    }
}

impl std::hash::Hash for TransformElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let bits = |v: f64| if v == 0.0 { 0_u64 } else { v.to_bits() };
        let length = |v: Length| (bits(v.abs.to_pt()), bits(v.em));
        let relative = |v: Rel<Length>| (bits(v.rel), length(v.abs));
        let angle = |v: Angle| bits(v.to_rad());
        let scale = |v: TransformScale| match v {
            TransformScale::Auto => (0_u8, 0, 0),
            TransformScale::Ratio(r) => (1, bits(r.0), 0),
            TransformScale::Length(l) => (2, length(l).0, length(l).1),
        };
        std::mem::discriminant(&self.operation).hash(state);
        match self.operation {
            TransformOperation::Matrix(m) => {
                [m.a, m.b, m.c, m.d, m.tx, m.ty].map(bits).hash(state)
            }
            TransformOperation::Move { dx, dy } => {
                (dx.map(relative), dy.map(relative)).hash(state)
            }
            TransformOperation::Rotate { angle: a, origin, reflow } => {
                (a.map(angle), origin, reflow).hash(state)
            }
            TransformOperation::Scale { x, y, origin, reflow } => {
                (x.map(scale), y.map(scale), origin, reflow).hash(state)
            }
            TransformOperation::Skew { ax, ay, origin, reflow } => {
                (ax.map(angle), ay.map(angle), origin, reflow).hash(state)
            }
        }
        self.body.plain_text().hash(state);
    }
}

impl Element for TransformElem {
    fn get_field(&self, field: &str) -> Option<Value> {
        use TransformOperation::{Matrix, Move, Rotate, Scale, Skew};
        match (self.operation, field) {
            (Matrix(_), _) => None,
            (_, "body") => Some(Value::Content(self.body.clone())),
            (Move { dx, .. }, "dx") => dx.map(Value::Relative),
            (Move { dy, .. }, "dy") => dy.map(Value::Relative),
            (Rotate { angle, .. }, "angle") => angle.map(Value::Angle),
            (Scale { x, .. }, "x") => x.map(scale_value),
            (Scale { y, .. }, "y") => y.map(scale_value),
            (Skew { ax, .. }, "ax") => ax.map(Value::Angle),
            (Skew { ay, .. }, "ay") => ay.map(Value::Angle),
            (
                Rotate { origin, .. } | Scale { origin, .. } | Skew { origin, .. },
                "origin",
            ) => origin.map(Value::Align),
            (
                Rotate { reflow, .. } | Scale { reflow, .. } | Skew { reflow, .. },
                "reflow",
            ) => reflow.map(Value::Bool),
            _ => None,
        }
    }

    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    // `is_empty` fica no default `false` — paridade `content.rs` (`Transform`
    // cai em `_ => false`, não delega ao body). Precedente Align (Lote 7).

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Transform(Arc::new(TransformElem {
            operation: self.operation,
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Transform(Arc::new(TransformElem {
            operation: self.operation,
            body: self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> TransformElem {
        TransformElem {
            operation: TransformOperation::Matrix(TransformMatrix::identity()),
            body: Content::text("a"),
        }
    }

    #[test]
    fn plain_text_delega_ao_body() {
        assert_eq!(ex().plain_text(), "a");
    }

    #[test]
    fn is_empty_default_false_nao_delega() {
        // Mesmo com body Empty, Transform não é "vazio" (paridade hub _ => false).
        let t = TransformElem {
            operation: TransformOperation::Matrix(TransformMatrix::identity()),
            body: Content::Empty,
        };
        assert!(!t.is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_matrix() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Transform(e) => {
                assert_eq!(
                    e.operation,
                    TransformOperation::Matrix(TransformMatrix::identity())
                );
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Transform"),
        }
    }

    fn h(e: &TransformElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let outro = TransformElem {
            operation: TransformOperation::Matrix(TransformMatrix::identity()),
            body: Content::text("b"),
        };
        assert_ne!(h(&ex()), h(&outro));
    }
    #[test]
    fn equal_signed_zero_parameters_have_equal_hashes() {
        let element = |v| TransformElem {
            operation: TransformOperation::Rotate {
                angle: Some(Angle::rad(v)),
                origin: None,
                reflow: Some(false),
            },
            body: Content::text("x"),
        };
        let a = element(-0.0);
        let b = element(0.0);
        assert_eq!(a, b);
        assert_eq!(h(&a), h(&b));
    }
}

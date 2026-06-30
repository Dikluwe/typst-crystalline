//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/curve.md
//! @layer L1
//! @updated 2026-06-30
//!
//! `CurveElem` — Passo 513 (curve elements: move/line/cubic/quad/close).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// Segmento de um path de curva.
#[derive(Debug, Clone, PartialEq)]
pub enum CurveSegment {
    /// Mover a caneta para um ponto.
    Move(CurvePoint),
    /// Linha recta até um ponto.
    Line(CurvePoint),
    /// Curva cúbica de Bézier (control1, control2, end).
    Cubic(CurvePoint, CurvePoint, CurvePoint),
    /// Curva quadrática de Bézier (control, end).
    Quad(CurvePoint, CurvePoint),
    /// Fechar o path.
    Close,
}

/// Ponto 2D com coordenadas em `Length`.
#[derive(Debug, Clone, PartialEq)]
pub struct CurvePoint {
    pub x: Length,
    pub y: Length,
}

/// Elemento que agrupa segmentos de curva.
#[derive(Debug, Clone, PartialEq)]
pub struct CurveElem {
    pub segments: Vec<CurveSegment>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Length` carrega `f64`.
impl std::hash::Hash for CurveElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for CurveElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Curve(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Curve(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn ex() -> CurveElem {
        CurveElem {
            segments: vec![
                CurveSegment::Move(CurvePoint { x: Length::pt(0.0), y: Length::pt(0.0) }),
                CurveSegment::Line(CurvePoint { x: Length::pt(10.0), y: Length::pt(10.0) }),
            ],
        }
    }

    #[test]
    fn plain_text_vazio() {
        assert_eq!(ex().plain_text(), "");
    }

    #[test]
    fn is_empty_false() {
        assert!(!ex().is_empty());
    }

    #[test]
    fn map_content_terminal() {
        let mut f = |_: &Content| -> SourceResult<Option<Content>> { Ok(Some(Content::text("X"))) };
        let c = ex().map_content(&mut f).unwrap();
        assert!(matches!(c, Content::Curve(_)));
    }

    #[test]
    fn map_text_terminal() {
        let mut f = |_: &str| -> String { "X".to_string() };
        let c = ex().map_text(&mut f);
        assert!(matches!(c, Content::Curve(_)));
    }

    fn h(e: &CurveElem) -> u64 {
        let mut s = DefaultHasher::new(); e.hash(&mut s); s.finish()
    }

    #[test]
    fn hash_diferente_com_segmentos_diferentes() {
        let mut outro = ex();
        outro.segments.pop();
        assert_ne!(h(&ex()), h(&outro));
    }
}

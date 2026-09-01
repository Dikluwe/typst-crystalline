//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_cancel.md
//! @prompt-hash 112fd354
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathCancelElem` — Lote 2 P317 (família math). Linha de cancelamento (P296).
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::func::Func;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{Angle, Length};
use crate::entities::rel::Rel;
use crate::entities::source_result::SourceResult;
use crate::entities::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum MathCancelAngle {
    Auto,
    Angle(Angle),
    Func(Func),
}

/// Presença morfológica dos argumentos nomeados de `math.cancel`.
///
/// Os bits não alteram a geometria. Eles distinguem omissão de um valor
/// explicitamente igual ao default para identidade e `repr`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MathCancelExplicit {
    pub length: bool,
    pub inverted: bool,
    pub cross: bool,
    pub angle: bool,
    pub stroke: bool,
    pub background: bool,
}

/// Linha de cancelamento sobre conteúdo matemático — vanilla `CancelElem`
/// minimal (P296). Layouter emite `FrameItem::Line` diagonal sobre o bbox.
#[derive(Debug, Clone)]
pub struct MathCancelElem {
    pub body: Content,
    pub length: Rel<Length>,
    pub inverted: bool,
    pub cross: bool,
    pub angle: MathCancelAngle,
    pub stroke: Option<Stroke>,
    pub background: bool,
    pub span: Span,
    pub explicit: MathCancelExplicit,
}

impl PartialEq for MathCancelElem {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
            && self.length == other.length
            && self.inverted == other.inverted
            && self.cross == other.cross
            && self.angle == other.angle
            && self.stroke == other.stroke
            && self.background == other.background
            && self.explicit == other.explicit
    }
}

impl std::hash::Hash for MathCancelElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        fn bits(value: f64) -> u64 {
            if value == 0.0 {
                0
            } else {
                value.to_bits()
            }
        }
        self.body.hash(state);
        bits(self.length.rel).hash(state);
        bits(self.length.abs.abs.0).hash(state);
        bits(self.length.abs.em).hash(state);
        self.inverted.hash(state);
        self.cross.hash(state);
        match &self.angle {
            MathCancelAngle::Auto => 0_u8.hash(state),
            MathCancelAngle::Angle(angle) => {
                1_u8.hash(state);
                bits(angle.to_rad()).hash(state);
            }
            MathCancelAngle::Func(func) => {
                2_u8.hash(state);
                func.hash(state);
            }
        }
        // Stroke payload may contain non-Hash paints. Presence participates;
        // equal values necessarily hash alike, while unequal strokes may collide.
        self.stroke.is_some().hash(state);
        self.background.hash(state);
        self.explicit.hash(state);
    }
}

impl Element for MathCancelElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathCancel(Arc::new(MathCancelElem {
            body: self.body.map_content(transform)?,
            length: self.length,
            inverted: self.inverted,
            cross: self.cross,
            angle: self.angle.clone(),
            stroke: self.stroke.clone(),
            background: self.background,
            span: self.span,
            explicit: self.explicit,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathCancel(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathCancelElem {
        MathCancelElem {
            body: Content::text("xy"),
            length: Rel::from_percent(100.0) + Length::em(0.3),
            inverted: false,
            cross: false,
            angle: MathCancelAngle::Auto,
            stroke: None,
            background: false,
            span: Span::detached(),
            explicit: MathCancelExplicit::default(),
        }
    }

    #[test]
    fn plain_text_transparente() {
        assert_eq!(ex().plain_text(), "xy");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.body = Content::text("z");
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_body() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "xy" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let r = ex().map_content(&mut f).unwrap();
        match r {
            Content::MathCancel(e) => assert_eq!(e.plain_text(), "Z"),
            _ => panic!("esperado MathCancel"),
        }
    }
}

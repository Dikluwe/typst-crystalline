//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/block.md
//! @prompt-hash b8220853
//! @layer L1
//! @updated 2026-10-10
//!
//! `BlockElem` — Lote 15 P330 (lote tardio da triagem DEBT-58; o ÚLTIMO lote).
//! `block(body, …)` — container block denso (14 campos). Contentor: recurse no
//! body. Não-locatável.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::corners::Corners;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{Color, Length};
use crate::entities::rel::Rel;
use crate::entities::sides::Sides;
use crate::entities::source_result::SourceResult;

/// Altura de bloco preservada até à fase de layout.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockHeight {
    Relative(Rel<Length>),
    Fractional(f64),
}

/// Bits de `BlockElem::omitted`.
pub(crate) mod omitted {
    /// Presente em todo valor do construtor nativo.
    pub(crate) const NATIVE: u32 = 1 << 31;
    pub(crate) const WIDTH: u32 = 1;
    pub(crate) const HEIGHT: u32 = 1 << 1;
    /// Quatro bits: esquerda, topo, direita, base.
    pub(crate) const INSET: u32 = 1 << 2;
    pub(crate) const OUTSET: u32 = 1 << 6;
    pub(crate) const RADIUS: u32 = 1 << 10;
    pub(crate) const BREAKABLE: u32 = 1 << 11;
    pub(crate) const CLIP: u32 = 1 << 12;
    pub(crate) const FILL: u32 = 1 << 13;
    pub(crate) const STROKE: u32 = 1 << 14;
    pub(crate) const SPACING: u32 = 1 << 15;
    pub(crate) const ABOVE: u32 = 1 << 16;
    pub(crate) const BELOW: u32 = 1 << 17;
    pub(crate) const STICKY: u32 = 1 << 18;
}

/// Container block (`block`). `body` + 13 cosméticos.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockElem {
    pub body: Content,
    pub width: Option<Rel<Length>>,
    pub height: Option<BlockHeight>,
    pub inset: Sides<Length>,
    pub breakable: bool,
    pub outset: Sides<Length>,
    pub radius: Corners<Length>,
    pub clip: bool,
    pub fill: Option<Color>,
    pub stroke: Option<Stroke>,
    pub spacing: Option<Length>,
    pub above: Option<Length>,
    pub below: Option<Length>,
    pub sticky: bool,
    /// Propriedades omitidas no construtor; zero conta todas como dadas.
    pub(crate) omitted: u32,
    /// Contorno de `rect`, `square`, `circle` ou `ellipse` com corpo.
    pub(crate) shape: Option<Arc<crate::entities::elements::shape::ShapeElem>>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Length`/`Sides`/`Corners`/
// `Color`/`Stroke` carregam `f64`. Ressalva `-0.0` vs `0.0` aceitável.
impl std::hash::Hash for BlockElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for BlockElem {
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
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::Block(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Block(Arc::new(BlockElem {
            body: self.body.map_text(transform),
            ..(*self).clone()
        }))
    }
}

impl BlockElem {
    /// Corpo de uma forma, com o seu contorno.
    pub(crate) fn outlined(
        body: Content,
        shape: Arc<crate::entities::elements::shape::ShapeElem>,
    ) -> Self {
        Self {
            body,
            width: None,
            height: None,
            inset: Sides::uniform(Length::ZERO),
            breakable: false,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
            omitted: 0,
            shape: Some(shape),
        }
    }

    /// Nome do elemento quando o bloco é o corpo de uma forma.
    pub fn outline_name(&self) -> Option<&'static str> {
        use crate::entities::elements::shape::ShapeOrigin;
        self.shape.as_ref().map(|shape| match shape.origin {
            ShapeOrigin::Square => "square",
            ShapeOrigin::Circle => "circle",
            ShapeOrigin::Ellipse => "ellipse",
            ShapeOrigin::Rect
            | ShapeOrigin::Geometry
            | ShapeOrigin::Line
            | ShapeOrigin::Polygon
            | ShapeOrigin::Curve => "rect",
        })
    }

    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        if body.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        Ok(Some(Content::Block(Arc::new(mapped))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn p1374_block_height_distinguishes_relative_and_fractional() {
        let relative = BlockHeight::Relative(Rel::from_percent(60.0));
        let fractional = BlockHeight::Fractional(2.0);
        assert_ne!(relative, fractional);
        assert!(matches!(relative, BlockHeight::Relative(value) if value.rel == 0.6));
        assert!(matches!(fractional, BlockHeight::Fractional(2.0)));
    }

    fn ex() -> BlockElem {
        BlockElem { shape: None,
            omitted: 0,
            body: Content::text("a"),
            width: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
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
        let mut v = ex();
        v.sticky = true;
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match v.map_content(&mut f).unwrap() {
            Content::Block(e) => {
                assert!(e.sticky);
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Block"),
        }
    }

    fn h(e: &BlockElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let mut v = ex();
        v.sticky = true;
        assert_ne!(h(&ex()), h(&v));
    }
}

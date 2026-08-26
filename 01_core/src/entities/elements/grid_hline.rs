//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/grid_hline.md
//! @prompt-hash 376ff1cf
//! @layer L1
//! @updated 2026-06-30
//!
//! `GridHLineElem` — linha horizontal em `grid()` (Passo 512).

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::source_result::SourceResult;

/// Linha horizontal num grid.
///
/// **P739A** — `stroke` é `Option`: `grid.hline(stroke: none)` compila e a
/// linha não é desenhada (paridade vanilla, medido). Zero-thickness não é
/// usado (width 0 em PDF é hairline — achado P726).
#[derive(Debug, Clone, PartialEq)]
pub struct GridHLineElem {
    pub start: usize,
    pub end: Option<usize>, // None = até à última coluna
    pub row: usize,         // linha do grid onde a hline se posiciona
    pub stroke: Option<Stroke>,
    pub position: EcoString, // "top" | "bottom"
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Stroke` carrega `f64`.
impl std::hash::Hash for GridHLineElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for GridHLineElem {
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
        Ok(Content::GridHLine(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::GridHLine(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> GridHLineElem {
        GridHLineElem {
            start: 0,
            end: Some(2),
            row: 1,
            stroke: Some(Stroke {
                paint: Paint::Solid(Color::rgb(0, 0, 0)),
                thickness: 1.0,
                overhang: false,
                ..Stroke::default()
            }),
            position: EcoString::from("top"),
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
        let mut f = |_: &Content| -> SourceResult<Option<Content>> {
            Ok(Some(Content::text("X")))
        };
        let c = ex().map_content(&mut f).unwrap();
        assert!(matches!(c, Content::GridHLine(_)));
    }

    #[test]
    fn map_text_terminal() {
        let mut f = |_: &str| -> String { "X".to_string() };
        let c = ex().map_text(&mut f);
        assert!(matches!(c, Content::GridHLine(_)));
    }

    fn h(e: &GridHLineElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn hash_diferente_com_row_diferente() {
        let mut outro = ex();
        outro.row = 2;
        assert_ne!(h(&ex()), h(&outro));
    }
}

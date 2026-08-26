//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/table_vline.md
//! @prompt-hash 872a2d94
//! @layer L1
//! @updated 2026-06-30
//!
//! `TableVLineElem` — linha vertical em `table()` (Passo 512).

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::source_result::SourceResult;

/// Linha vertical numa tabela.
///
/// **P739A** — `stroke` é `Option`: `table.vline(stroke: none)` compila e a
/// linha não é desenhada (paridade vanilla, medido).
#[derive(Debug, Clone, PartialEq)]
pub struct TableVLineElem {
    pub start: usize,
    pub end: Option<usize>, // None = até à última linha
    pub col: usize,         // coluna da tabela onde a vline se posiciona
    pub stroke: Option<Stroke>,
    pub position: EcoString, // "left" | "right"
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Stroke` carrega `f64`.
impl std::hash::Hash for TableVLineElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for TableVLineElem {
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
        Ok(Content::TableVLine(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::TableVLine(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> TableVLineElem {
        TableVLineElem {
            start: 0,
            end: Some(2),
            col: 1,
            stroke: Some(Stroke {
                paint: Paint::Solid(Color::rgb(0, 0, 0)),
                thickness: 1.0,
                overhang: false,
            }),
            position: EcoString::from("left"),
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
        assert!(matches!(c, Content::TableVLine(_)));
    }

    #[test]
    fn map_text_terminal() {
        let mut f = |_: &str| -> String { "X".to_string() };
        let c = ex().map_text(&mut f);
        assert!(matches!(c, Content::TableVLine(_)));
    }

    fn h(e: &TableVLineElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn hash_diferente_com_col_diferente() {
        let mut outro = ex();
        outro.col = 2;
        assert_ne!(h(&ex()), h(&outro));
    }
}

//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/shape.md
//! @prompt-hash 9ef75d9a
//! @layer L1
//! @updated 2026-06-11
//!
//! `ShapeElem` — Lote 11 P326 (por largura). `shape(kind, …)` — geometria
//! (rect/ellipse/…). Leaf não-locatável; `map_*` terminais.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::geometry::{ShapeKind, Stroke};
use crate::entities::paint::Paint;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Forma geométrica. `width`/`height` em `Value` (resolvidos em layout);
/// `fill`/`stroke` cosméticos.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeElem {
    pub kind:   ShapeKind,
    pub width:  Option<Box<Value>>,
    pub height: Option<Box<Value>>,
    pub fill:   Option<Paint>,
    pub stroke: Option<Stroke>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Value` (width/height)
// carrega `f64`; `Color`/`Stroke` idem. Ressalva `-0.0` vs `0.0` aceitável
// (`…Elem` não são chaves de mapa).
impl std::hash::Hash for ShapeElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for ShapeElem {
    fn plain_text(&self) -> String {
        // Geometria sem texto.
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Shape(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Shape(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn ex() -> ShapeElem {
        ShapeElem { kind: ShapeKind::Rect, width: None, height: None, fill: None, stroke: None }
    }

    #[test]
    fn plain_text_vazio() {
        assert_eq!(ex().plain_text(), "");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!ex().is_empty());
    }

    #[test]
    fn map_content_terminal() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        assert!(matches!(ex().map_content(&mut f).unwrap(), Content::Shape(_)));
    }

    #[test]
    fn eq_compara_campos() {
        assert_eq!(ex(), ex());
        assert_ne!(ex(), ShapeElem { kind: ShapeKind::Ellipse, width: None, height: None, fill: None, stroke: None });
    }

    fn h(e: &ShapeElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let outro = ShapeElem { kind: ShapeKind::Rect, width: Some(Box::new(Value::Int(5))), height: None, fill: None, stroke: None };
        assert_ne!(h(&ex()), h(&outro));
    }
}

//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_class_override.md
//! @prompt-hash 0631f2f7
//! @layer L1
//! @updated 2026-07-17
//!
//! `MathClassOverrideElem` — P772y. Força a `MathClass` de um símbolo/
//! expressão, override do valor inferido automaticamente por
//! `default_math_class`/`node_math_class`. Vanilla `ClassElem`
//! (`math/mod.rs`), `math.class(class, body)`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::math_class::MathClass;
use crate::entities::source_result::SourceResult;

/// Classe forçada + conteúdo — `math.class("relation", sym.suit.heart)`.
///
/// `class` afecta apenas decisões de espaçamento automático
/// (`rules/math/layout/spacing.rs`) e o `lclass`/`rclass` efectivo do nó
/// — não afecta layout/renderização do `body` (que é layoutado
/// normalmente via `layout_node`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathClassOverrideElem {
    pub class: MathClass,
    pub body: Content,
}

impl Element for MathClassOverrideElem {
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
        Ok(Content::MathClassOverride(Arc::new(MathClassOverrideElem {
            class: self.class,
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathClassOverride(Arc::new(MathClassOverrideElem {
            class: self.class,
            body: self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathClassOverrideElem {
        MathClassOverrideElem {
            class: MathClass::Relation,
            body: Content::text("loves"),
        }
    }

    #[test]
    fn plain_text_do_body() {
        assert_eq!(ex().plain_text(), "loves");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        assert!(MathClassOverrideElem {
            class: MathClass::Relation,
            body: Content::Empty
        }
        .is_empty());
    }

    #[test]
    fn map_content_recurse_preserva_class() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "loves" => {
                    Ok(Some(Content::text("LOVES")))
                }
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::MathClassOverride(e) => {
                assert_eq!(e.class, MathClass::Relation);
                assert_eq!(e.plain_text(), "LOVES");
            }
            _ => panic!("esperado MathClassOverride"),
        }
    }

    #[test]
    fn map_text_recurse_preserva_class() {
        match ex().map_text(&mut |s| s.to_uppercase()) {
            Content::MathClassOverride(e) => {
                assert_eq!(e.class, MathClass::Relation);
                assert_eq!(e.plain_text(), "LOVES");
            }
            _ => panic!("esperado MathClassOverride"),
        }
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        assert_ne!(
            ex(),
            MathClassOverrideElem {
                class: MathClass::Binary,
                body: Content::text("loves")
            }
        );
    }
}

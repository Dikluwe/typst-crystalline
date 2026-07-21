//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/stack.md
//! @prompt-hash 4ffa6ebf
//! @layer L1
//! @updated 2026-06-11
//!
//! `StackElem` — Lote 9 P324 (por largura). `stack(children, dir, spacing)`.
//! Contentor: recurse em cada child em map_content E map_text.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::dir::Dir;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// Empilha `children` na direcção `dir`, com `spacing` entre cópias.
#[derive(Debug, Clone, PartialEq)]
pub struct StackElem {
    pub children: Arc<[Content]>,
    pub dir: Dir,
    pub spacing: Option<Length>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `spacing: Option<Length>`
// carrega `f64`. Ressalva `-0.0` vs `0.0` aceitável (`…Elem` não são chaves de mapa).
impl std::hash::Hash for StackElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for StackElem {
    fn plain_text(&self) -> String {
        self.children.iter().map(|c| c.plain_text()).collect()
    }

    fn is_empty(&self) -> bool {
        self.children.iter().all(|c| c.is_empty())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let new_children: SourceResult<Vec<Content>> =
            self.children.iter().map(|c| c.map_content(transform)).collect();
        Ok(Content::Stack(Arc::new(StackElem {
            children: Arc::from(new_children?),
            dir: self.dir,
            spacing: self.spacing,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        let new_children: Vec<Content> =
            self.children.iter().map(|c| c.map_text(transform)).collect();
        Content::Stack(Arc::new(StackElem {
            children: Arc::from(new_children),
            dir: self.dir,
            spacing: self.spacing,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> StackElem {
        StackElem {
            children: Arc::from(vec![Content::text("a"), Content::text("b")]),
            dir: Dir::TTB,
            spacing: None,
        }
    }

    #[test]
    fn plain_text_concatena_children() {
        assert_eq!(ex().plain_text(), "ab");
    }

    #[test]
    fn is_empty_todos_vazios() {
        assert!(!ex().is_empty());
        let vazio = StackElem {
            children: Arc::from(vec![Content::Empty]),
            dir: Dir::TTB,
            spacing: None,
        };
        assert!(vazio.is_empty());
    }

    #[test]
    fn map_content_recurse_children_preserva_dir() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Stack(e) => {
                assert_eq!(e.dir, Dir::TTB);
                assert!(matches!(&e.children[0], Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Stack"),
        }
    }

    fn h(e: &StackElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let outro = StackElem {
            children: Arc::from(vec![Content::text("x")]),
            dir: Dir::TTB,
            spacing: None,
        };
        assert_ne!(h(&ex()), h(&outro));
    }
}

//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/list_item.md
//! @prompt-hash a5e523a9
//! @layer L1
//! @updated 2026-06-11
//!
//! `ListItemElem` — Lote 3 P318 (família lista/termos). Item de lista (`- …`).
//! Contentor de prosa: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Item de lista não ordenada (`- ...`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ListItemElem {
    pub body: Content,
}

impl Element for ListItemElem {
    fn plain_text(&self) -> String {
        format!("• {}", self.body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::ListItem(Arc::new(ListItemElem {
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::ListItem(Arc::new(ListItemElem {
            body: self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_com_bullet() {
        let li = ListItemElem { body: Content::text("um") };
        assert_eq!(li.plain_text(), "• um");
    }

    #[test]
    fn igualdade_estrutural() {
        let a = ListItemElem { body: Content::text("x") };
        assert_eq!(a.clone(), a.clone());
        assert_ne!(a, ListItemElem { body: Content::text("y") });
    }

    #[test]
    fn map_text_recurse_body() {
        let li = ListItemElem { body: Content::text("ab") };
        let mut up = |s: &str| s.to_uppercase();
        match li.map_text(&mut up) {
            Content::ListItem(e) => assert_eq!(e.plain_text(), "• AB"),
            _ => panic!("esperado ListItem"),
        }
    }
}

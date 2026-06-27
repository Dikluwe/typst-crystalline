//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/list_item.md
//! @prompt-hash f9756df3
//! @layer L1
//! @updated 2026-06-26
//!
//! `ListItemElem` — Lote 3 P318 (família lista/termos). Item de lista (`- …`).
//! Campo `marker` adicionado em P470. Contentor de prosa: recurse em
//! map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::list_marker::ListMarker;
use crate::entities::source_result::SourceResult;

/// Item de lista não ordenada (`- ...`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ListItemElem {
    pub body:   Content,
    pub marker: Option<ListMarker>,  // None = bullet padrão "•"
}

impl Element for ListItemElem {
    fn plain_text(&self) -> String {
        let m = self.marker.as_ref().map(|m| m.render()).unwrap_or("•");
        format!("{} {}", m, self.body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::ListItem(Arc::new(ListItemElem {
            body:   self.body.map_content(transform)?,
            marker: self.marker.clone(),
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::ListItem(Arc::new(ListItemElem {
            body:   self.body.map_text(transform),
            marker: self.marker.clone(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_com_bullet_default() {
        let li = ListItemElem { body: Content::text("um"), marker: None };
        assert_eq!(li.plain_text(), "• um");
    }

    #[test]
    fn plain_text_com_marcador_custom() {
        let li = ListItemElem {
            body:   Content::text("um"),
            marker: Some(ListMarker::Custom("→".into())),
        };
        assert_eq!(li.plain_text(), "→ um");
    }

    #[test]
    fn igualdade_estrutural() {
        let a = ListItemElem { body: Content::text("x"), marker: None };
        assert_eq!(a.clone(), a.clone());
        assert_ne!(a, ListItemElem { body: Content::text("y"), marker: None });
        assert_ne!(
            a,
            ListItemElem { body: Content::text("x"), marker: Some(ListMarker::Custom("→".into())) }
        );
    }

    #[test]
    fn map_text_recurse_body_preserva_marker() {
        let li = ListItemElem {
            body:   Content::text("ab"),
            marker: Some(ListMarker::Custom("*".into())),
        };
        let mut up = |s: &str| s.to_uppercase();
        match li.map_text(&mut up) {
            Content::ListItem(e) => {
                assert_eq!(e.plain_text(), "* AB");
                assert_eq!(e.marker, Some(ListMarker::Custom("*".into())));
            }
            _ => panic!("esperado ListItem"),
        }
    }

    #[test]
    fn map_content_preserva_marker() {
        let li = ListItemElem {
            body:   Content::text("x"),
            marker: Some(ListMarker::Default),
        };
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        match li.map_content(&mut f).unwrap() {
            Content::ListItem(e) => assert_eq!(e.marker, Some(ListMarker::Default)),
            _ => panic!("esperado ListItem"),
        }
    }
}

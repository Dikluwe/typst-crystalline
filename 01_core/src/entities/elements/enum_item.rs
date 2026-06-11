//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/enum_item.md
//! @prompt-hash 7c44b354
//! @layer L1
//! @updated 2026-06-11
//!
//! `EnumItemElem` — Lote 3 P318 (família lista/termos). Item de lista ordenada.
//! Contentor de prosa: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Item de lista ordenada (`+ ...` ou `1. ...`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EnumItemElem {
    pub number: Option<u32>,
    pub body:   Content,
}

impl Element for EnumItemElem {
    fn plain_text(&self) -> String {
        let n = self.number.map(|n| format!("{}. ", n)).unwrap_or_default();
        format!("{}{}", n, self.body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::EnumItem(Arc::new(EnumItemElem {
            number: self.number,
            body:   self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::EnumItem(Arc::new(EnumItemElem {
            number: self.number,
            body:   self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_com_e_sem_numero() {
        let com = EnumItemElem { number: Some(3), body: Content::text("x") };
        assert_eq!(com.plain_text(), "3. x");
        let sem = EnumItemElem { number: None, body: Content::text("x") };
        assert_eq!(sem.plain_text(), "x");
    }

    #[test]
    fn map_content_preserva_number() {
        let e = EnumItemElem { number: Some(7), body: Content::text("a") };
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        match e.map_content(&mut f).unwrap() {
            Content::EnumItem(el) => assert_eq!(el.number, Some(7)),
            _ => panic!("esperado EnumItem"),
        }
    }

    #[test]
    fn igualdade_estrutural() {
        let a = EnumItemElem { number: Some(1), body: Content::text("x") };
        assert_eq!(a.clone(), a.clone());
        assert_ne!(a, EnumItemElem { number: Some(2), body: Content::text("x") });
    }
}

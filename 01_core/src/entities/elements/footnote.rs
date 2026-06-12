//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/footnote.md
//! @prompt-hash 47dc41e8
//! @layer L1
//! @updated 2026-06-11
//!
//! `FootnoteElem` — Lote 11 P326 (por largura). `footnote(body)` — nota de rodapé.
//! Contentor: recurse no body em map_content E map_text. Não-locatável (P295
//! Fase 1 marker-only).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Nota de rodapé. `body` é o conteúdo diferido para o rodapé.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FootnoteElem {
    pub body: Content,
}

impl Element for FootnoteElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    // `is_empty` fica no default `false` — paridade `content.rs` (`Footnote`
    // tem arm explícito `=> false`: marker `[N]` sempre observable; não delega).

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Footnote(Arc::new(FootnoteElem {
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Footnote(Arc::new(FootnoteElem {
            body: self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_incorpora_body() {
        assert_eq!(FootnoteElem { body: Content::text("nota") }.plain_text(), "nota");
    }

    #[test]
    fn is_empty_default_false_nao_delega() {
        // Mesmo com body Empty, Footnote nunca é "vazio" (marker observable).
        assert!(!FootnoteElem { body: Content::Empty }.is_empty());
    }

    #[test]
    fn map_content_recurse_body() {
        let e = FootnoteElem { body: Content::text("a") };
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s, _) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match e.map_content(&mut f).unwrap() {
            Content::Footnote(el) => assert!(matches!(&el.body, Content::Text(s, _) if s.as_str() == "Z")),
            _ => panic!("esperado Footnote"),
        }
    }

    #[test]
    fn map_text_recurse_body() {
        let e = FootnoteElem { body: Content::text("hi") };
        match e.map_text(&mut |s| s.to_uppercase()) {
            Content::Footnote(el) => assert!(matches!(&el.body, Content::Text(s, _) if s.as_str() == "HI")),
            _ => panic!("esperado Footnote"),
        }
    }
}

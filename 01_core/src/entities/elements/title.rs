//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib_audit_methodology.md
//! @prompt-hash 0683fad7
//! @layer L1
//! @updated 2026-07-15
//!
//! `TitleElem` — título do documento (`#title()`), paridade com vanilla
//! CLI 0.15.0 (P765a). Renderiza o corpo com tamanho 1.7em e negrito,
//! equivalente ao `ShowSet` do vanilla.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Título do documento.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TitleElem {
    /// Corpo do título.
    pub body: Content,
}

impl TitleElem {
    /// Cria um título com o corpo dado.
    pub fn new(body: Content) -> Self {
        Self { body }
    }
}

impl crate::entities::elements::Element for TitleElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Title(Arc::new(TitleElem {
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Title(Arc::new(TitleElem { body: self.body.map_text(transform) }))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::elements::Element;

    #[test]
    fn plain_text_e_o_corpo() {
        let t = TitleElem::new(Content::text("Hello"));
        assert_eq!(t.plain_text(), "Hello");
    }

    #[test]
    fn get_field_body() {
        let t = TitleElem::new(Content::text("Hello"));
        assert!(matches!(t.get_field("body"), Some(Value::Content(_))));
        assert!(t.get_field("inexistente").is_none());
    }
}

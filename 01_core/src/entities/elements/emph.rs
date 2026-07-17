//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/f_fronteira_e1.md
//! @prompt-hash ff63160d
//! @layer L1
//! @updated 2026-06-18
//!
//! **F-5b fatia 1 (P371, `f_fronteira_e1.md` §3a.12)** — `emph` (`_italic_`) como
//! **variante própria** (modelo D, ADR-0105), par simétrico do `strong`. Devolve a
//! distinção semântica de tipo que o colapso do P101 removeu (era
//! `Content::Styled([Italic])`); `_italic_ X ≠ #set text(italic) X` (fidelidade
//! ADR-0107). **Sem vtable**. O render (italic) é replicado no layout (arm
//! `Content::Emph` espelha o push `Italic(true)` do arm `Styled`).

use super::Element;
use crate::entities::content::Content;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// `emph` — ênfase (`_italic_`). Container transparente ao `plain_text` (só o
/// body); o italic é estilo de render aplicado no layout.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EmphElem {
    pub body: Content,
}

impl EmphElem {
    pub fn new(body: Content) -> Self {
        Self { body }
    }
}

impl Element for EmphElem {
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
        Ok(Content::emph(self.body.map_content(transform)?))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::emph(self.body.map_text(transform))
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

    #[test]
    fn plain_text_transparente_ao_body() {
        assert_eq!(EmphElem::new(Content::text("em")).plain_text(), "em");
    }

    #[test]
    fn is_empty_segue_o_body() {
        assert!(EmphElem::new(Content::Empty).is_empty());
        assert!(!EmphElem::new(Content::text("x")).is_empty());
    }
}

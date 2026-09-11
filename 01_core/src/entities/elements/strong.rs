//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/strong.md
//! @prompt-hash 81114348
//! @layer L1
//! @updated 2026-06-18
//!
//! **F-5b fatia 1 (P371, `f_fronteira_e1.md` §3a.12)** — `strong` (`*bold*`) como
//! **variante própria** (modelo D, ADR-0105), devolvendo a distinção semântica de
//! tipo que o colapso do P101 removeu (era `Content::Styled([Bold])`). Dá à
//! ADR-0107 a fidelidade ao vanilla (`StrongElem` é tipo distinto, não `StyledElem`):
//! `*bold* X ≠ #set text(bold) X`. **Sem vtable** — variante de enum, distinção
//! estática (cláusula ADR-0026). O **render** (bold) é replicado no layout (arm
//! `Content::Strong` espelha o push `Bold(true)` do arm `Styled`); o bold **assado**
//! nos `Text` filhos (via `eval_body_with_delta`) fica intocado nesta fatia (é a
//! fatia 2 que o de-baka).

use super::Element;
use crate::entities::content::Content;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// `strong` — ênfase forte (`*bold*`). Container transparente ao `plain_text`
/// (só o body); o bold é estilo de render aplicado no layout.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct StrongElem {
    pub body: Content,
}

impl StrongElem {
    pub fn new(body: Content) -> Self {
        Self { body }
    }
}

impl Element for StrongElem {
    fn to_payload(&self) -> Option<crate::entities::element_payload::ElementPayload> {
        Some(crate::entities::element_payload::ElementPayload::NativeElement)
    }

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
        Ok(Content::strong(self.body.map_content(transform)?))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::strong(self.body.map_text(transform))
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
        assert_eq!(StrongElem::new(Content::text("bold")).plain_text(), "bold");
    }

    #[test]
    fn is_empty_segue_o_body() {
        assert!(StrongElem::new(Content::Empty).is_empty());
        assert!(!StrongElem::new(Content::text("x")).is_empty());
    }
}

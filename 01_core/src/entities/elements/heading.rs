//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/heading.md
//! @prompt-hash 0effb467
//! @layer L1
//! @updated 2026-06-10
//!
//! `HeadingElem` — teto do lote piloto D: locatável (prova a absorção de
//! `ElementKind`/`ElementPayload`).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::content_hash::hash_content;
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Cabeçalho com nível 1–6 e corpo.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HeadingElem {
    /// Nível clamped 1..=6.
    pub level: u8,
    /// Corpo do cabeçalho (era `Box<Content>`; agora `Content` dentro do `Arc`).
    pub body: Content,
    /// **Numeração ativa (Lote F-2 S1, P335)** — assada na criação a partir do
    /// `engine.styles.custom("heading.numbering")` (escopo léxico via a chain;
    /// fecha o canal global `SetHeadingNumbering`/StateRegistry). Paridade:
    /// só "ativo" (o pattern era descartado no canal antigo). O **valor** do
    /// contador continua via Introspector (`formatted_counter_at("heading")`).
    pub numbering_active: bool,
}

impl HeadingElem {
    /// Construtor com clamp de paridade (content.rs:1267). Numeração inativa.
    pub fn new(level: u8, body: Content) -> Self {
        Self { level: level.clamp(1, 6), body, numbering_active: false }
    }

    /// Construtor com numeração ativa (assada do `#set heading(numbering:)`).
    pub fn new_numbered(level: u8, body: Content, numbering_active: bool) -> Self {
        Self { level: level.clamp(1, 6), body, numbering_active }
    }
}

impl Element for HeadingElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Heading(Arc::new(HeadingElem {
            level: self.level,
            body: self.body.map_content(transform)?,
            numbering_active: self.numbering_active,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Heading(Arc::new(HeadingElem {
            level: self.level,
            body: self.body.map_text(transform),
            numbering_active: self.numbering_active,
        }))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "level" => Some(Value::Int(self.level as i64)),
            _ => None,
        }
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Heading)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::Heading {
            depth: self.level,
            body_hash: hash_content(&self.body),
            counter_update: CounterUpdate::Step,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h() -> HeadingElem {
        HeadingElem::new(2, Content::text("Intro"))
    }

    #[test]
    fn clamp_nivel() {
        assert_eq!(HeadingElem::new(9, Content::empty()).level, 6);
        assert_eq!(HeadingElem::new(0, Content::empty()).level, 1);
    }

    #[test]
    fn plain_text_e_o_corpo() {
        assert_eq!(h().plain_text(), "Intro");
    }

    #[test]
    fn get_field_body_level() {
        assert!(matches!(h().get_field("body"), Some(Value::Content(_))));
        assert!(matches!(h().get_field("level"), Some(Value::Int(2))));
        assert!(h().get_field("inexistente").is_none());
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(h().element_kind(), Some(ElementKind::Heading));
        assert!(matches!(
            h().to_payload(),
            Some(ElementPayload::Heading { depth: 2, .. })
        ));
    }
}
